#[macro_use]
extern crate clap;

mod cli;
mod input;
mod output;
mod size;
mod surface;

use cssparser::Color;
use librsvg::rsvg_convert_only::LegacySize;
use librsvg::{CairoRenderer, Loader, RenderingError};

use crate::cli::Args;
use crate::output::Stream;
use crate::size::{ResizeStrategy, Size};
use crate::surface::Surface;

#[macro_export]
macro_rules! exit {
    () => (exit!("Error"));
    ($($arg:tt)*) => ({
        std::eprintln!("{}", std::format_args!($($arg)*));
        std::process::exit(1);
    })
}

fn size_limit_exceeded() -> ! {
    exit!(
        "The resulting image would be larger than 32767 pixels on either dimension.\n\
           Librsvg currently cannot render to images bigger than that.\n\
           Please specify a smaller size."
    );
}

fn load_stylesheet(args: &Args) -> std::io::Result<Option<String>> {
    match args.stylesheet {
        Some(ref filename) => std::fs::read_to_string(filename).map(Some),
        None => Ok(None),
    }
}

fn main() {
    let args = Args::new().unwrap_or_else(|e| e.exit());

    let stylesheet =
        load_stylesheet(&args).unwrap_or_else(|e| exit!("Error reading stylesheet: {}", e));

    let mut target = None;

    for input in args.input() {
        let mut handle = Loader::new()
            .with_unlimited_size(args.unlimited)
            .keep_image_data(args.keep_image_data)
            .read_stream(input.stream(), input.file(), None::<&gio::Cancellable>)
            .unwrap_or_else(|e| exit!("Error reading SVG {}: {}", input, e));

        if let Some(ref css) = stylesheet {
            handle
                .set_stylesheet(&css)
                .unwrap_or_else(|e| exit!("Error applying stylesheet: {}", e));
        }

        let renderer = CairoRenderer::new(&handle).with_dpi(args.dpi.x, args.dpi.y);

        if target.is_none() {
            let (width, height) = renderer
                .legacy_document_size_in_pixels()
                .unwrap_or_else(|e| exit!("Error rendering SVG {}: {}", input, e));

            let strategy = match (args.width, args.height) {
                // when w and h are not specified, scale to the requested zoom (if any)
                (None, None) => ResizeStrategy::Scale(args.zoom),

                // when w and h are specified, but zoom is not, scale to the requested size
                (Some(w), Some(h)) if args.zoom.is_identity() => ResizeStrategy::Fit(w, h),

                // if only one between w and h is specified and there is no zoom, scale to the
                // requested w or h and use the same scaling factor for the other
                (Some(w), None) if args.zoom.is_identity() => ResizeStrategy::FitWidth(w),
                (None, Some(h)) if args.zoom.is_identity() => ResizeStrategy::FitHeight(h),

                // otherwise scale the image, but cap the zoom to match the requested size
                _ => ResizeStrategy::FitLargestScale(args.zoom, args.width, args.height),
            };

            target = {
                let output = Stream::new(args.output())
                    .unwrap_or_else(|e| exit!("Error opening output: {}", e));

                let size = strategy
                    .apply(Size::new(width, height), args.keep_aspect_ratio)
                    .unwrap_or_else(|_| exit!("The SVG {} has no dimensions", input));

                match Surface::new(args.format, size, output) {
                    Ok(surface) => Some(surface),
                    Err(cairo::Status::InvalidSize) => size_limit_exceeded(),
                    Err(e) => exit!("Error creating output surface: {}", e),
                }
            };
        }

        if let Some(ref surface) = target {
            let cr = cairo::Context::new(surface);

            if let Some(Color::RGBA(rgba)) = args.background_color {
                cr.set_source_rgba(
                    rgba.red_f32().into(),
                    rgba.green_f32().into(),
                    rgba.blue_f32().into(),
                    rgba.alpha_f32().into(),
                );
            }

            cr.scale(args.zoom.x, args.zoom.y);

            surface
                .render(&renderer, &cr, args.export_id())
                .unwrap_or_else(|e| match e {
                    RenderingError::IdNotFound => exit!(
                        "File {} does not have an object with id \"{}\")",
                        input,
                        args.export_id().unwrap()
                    ),
                    _ => exit!("Error rendering SVG {}: {}", input, e),
                });
        }
    }

    if let Some(ref mut surface) = target {
        surface
            .finish()
            .unwrap_or_else(|e| exit!("Error saving output: {}", e));
    }
}
