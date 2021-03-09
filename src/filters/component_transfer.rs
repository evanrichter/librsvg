use std::cmp::min;

use cssparser::Parser;
use markup5ever::{expanded_name, local_name, namespace_url, ns};

use crate::document::AcquiredNodes;
use crate::drawing_ctx::DrawingCtx;
use crate::element::{Draw, Element, ElementResult, SetAttributes};
use crate::error::*;
use crate::node::{Node, NodeBorrow};
use crate::parsers::{NumberList, NumberListLength, Parse, ParseValue};
use crate::surface_utils::{
    iterators::Pixels, shared_surface::ExclusiveImageSurface, ImageSurfaceDataExt, Pixel,
};
use crate::util::clamp;
use crate::xml::Attributes;

use super::context::{FilterContext, FilterOutput, FilterResult};
use super::{FilterEffect, FilterError, FilterRender, PrimitiveWithInput};

/// The `feComponentTransfer` filter primitive.
pub struct FeComponentTransfer {
    base: PrimitiveWithInput,
}

impl Default for FeComponentTransfer {
    /// Constructs a new `ComponentTransfer` with empty properties.
    #[inline]
    fn default() -> FeComponentTransfer {
        FeComponentTransfer {
            base: PrimitiveWithInput::new::<Self>(),
        }
    }
}

impl SetAttributes for FeComponentTransfer {
    fn set_attributes(&mut self, attrs: &Attributes) -> ElementResult {
        self.base.set_attributes(attrs)
    }
}

/// Pixel components that can be influenced by `feComponentTransfer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Channel {
    R,
    G,
    B,
    A,
}

/// Component transfer function types.
#[derive(Clone)]
enum FunctionType {
    Identity,
    Table,
    Discrete,
    Linear,
    Gamma,
}

impl Parse for FunctionType {
    fn parse<'i>(parser: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i>> {
        Ok(parse_identifiers!(
            parser,
            "identity" => FunctionType::Identity,
            "table" => FunctionType::Table,
            "discrete" => FunctionType::Discrete,
            "linear" => FunctionType::Linear,
            "gamma" => FunctionType::Gamma,
        )?)
    }
}

/// The compute function parameters.
struct FunctionParameters {
    table_values: Vec<f64>,
    slope: f64,
    intercept: f64,
    amplitude: f64,
    exponent: f64,
    offset: f64,
}

struct Functions {
    r: FeFuncR,
    g: FeFuncG,
    b: FeFuncB,
    a: FeFuncA,
}

/// The compute function type.
type Function = fn(&FunctionParameters, f64) -> f64;

/// The identity component transfer function.
fn identity(_: &FunctionParameters, value: f64) -> f64 {
    value
}

/// The table component transfer function.
fn table(params: &FunctionParameters, value: f64) -> f64 {
    let n = params.table_values.len() - 1;
    let k = (value * (n as f64)).floor() as usize;

    let k = min(k, n); // Just in case.

    if k == n {
        return params.table_values[k];
    }

    let vk = params.table_values[k];
    let vk1 = params.table_values[k + 1];
    let k = k as f64;
    let n = n as f64;

    vk + (value - k / n) * n * (vk1 - vk)
}

/// The discrete component transfer function.
fn discrete(params: &FunctionParameters, value: f64) -> f64 {
    let n = params.table_values.len();
    let k = (value * (n as f64)).floor() as usize;

    params.table_values[min(k, n - 1)]
}

/// The linear component transfer function.
fn linear(params: &FunctionParameters, value: f64) -> f64 {
    params.slope * value + params.intercept
}

/// The gamma component transfer function.
fn gamma(params: &FunctionParameters, value: f64) -> f64 {
    params.amplitude * value.powf(params.exponent) + params.offset
}

trait FeComponentTransferFunc {
    /// Returns the component transfer function.
    fn function(&self) -> Function;

    /// Returns the component transfer function parameters.
    fn function_parameters(&self) -> FunctionParameters;

    /// Returns the channel.
    fn channel(&self) -> Channel;
}

macro_rules! func_x {
    ($func_name:ident, $channel:expr) => {
        #[derive(Clone)]
        pub struct $func_name {
            channel: Channel,
            function_type: FunctionType,
            table_values: Vec<f64>,
            slope: f64,
            intercept: f64,
            amplitude: f64,
            exponent: f64,
            offset: f64,
        }

        impl Default for $func_name {
            #[inline]
            fn default() -> Self {
                Self {
                    channel: $channel,
                    function_type: FunctionType::Identity,
                    table_values: Vec::new(),
                    slope: 1.0,
                    intercept: 0.0,
                    amplitude: 1.0,
                    exponent: 1.0,
                    offset: 0.0,
                }
            }
        }

        impl FeComponentTransferFunc for $func_name {
            #[inline]
            fn function_parameters(&self) -> FunctionParameters {
                FunctionParameters {
                    table_values: self.table_values.clone(),
                    slope: self.slope,
                    intercept: self.intercept,
                    amplitude: self.amplitude,
                    exponent: self.exponent,
                    offset: self.offset,
                }
            }

            #[inline]
            fn function(&self) -> Function {
                match self.function_type {
                    FunctionType::Identity => identity,
                    FunctionType::Table => table,
                    FunctionType::Discrete => discrete,
                    FunctionType::Linear => linear,
                    FunctionType::Gamma => gamma,
                }
            }

            #[inline]
            fn channel(&self) -> Channel {
                self.channel
            }
        }

        impl SetAttributes for $func_name {
            #[inline]
            fn set_attributes(&mut self, attrs: &Attributes) -> ElementResult {
                for (attr, value) in attrs.iter() {
                    match attr.expanded() {
                        expanded_name!("", "type") => self.function_type = attr.parse(value)?,
                        expanded_name!("", "tableValues") => {
                            let NumberList(v) =
                                NumberList::parse_str(value, NumberListLength::Unbounded)
                                    .attribute(attr)?;
                            self.table_values = v;
                        }
                        expanded_name!("", "slope") => self.slope = attr.parse(value)?,
                        expanded_name!("", "intercept") => self.intercept = attr.parse(value)?,
                        expanded_name!("", "amplitude") => self.amplitude = attr.parse(value)?,
                        expanded_name!("", "exponent") => self.exponent = attr.parse(value)?,
                        expanded_name!("", "offset") => self.offset = attr.parse(value)?,

                        _ => (),
                    }
                }

                // The table function type with empty table_values is considered
                // an identity function.
                match self.function_type {
                    FunctionType::Table | FunctionType::Discrete => {
                        if self.table_values.is_empty() {
                            self.function_type = FunctionType::Identity;
                        }
                    }
                    _ => (),
                }

                Ok(())
            }
        }

        impl Draw for $func_name {}
    };
}

// The `<feFuncR>` element
func_x!(FeFuncR, Channel::R);

// The `<feFuncG>` element
func_x!(FeFuncG, Channel::G);

// The `<feFuncB>` element
func_x!(FeFuncB, Channel::B);

// The `<feFuncA>` element
func_x!(FeFuncA, Channel::A);

macro_rules! func_or_default {
    ($func_node:ident, $func_type:ident) => {
        match $func_node {
            Some(ref f) => match *f.borrow_element() {
                Element::$func_type(ref e) => e.element_impl.clone(),
                _ => unreachable!(),
            },
            _ => $func_type::default(),
        };
    };
}

macro_rules! get_func_x_node {
    ($func_node:ident, $func_type:ident, $channel:expr) => {
        $func_node
            .children()
            .rev()
            .filter(|c| c.is_element())
            .find(|c| match *c.borrow_element() {
                Element::$func_type(ref f) => f.channel() == $channel,
                _ => false,
            })
    };
}

impl FilterRender for FeComponentTransfer {
    fn render(
        &self,
        node: &Node,
        ctx: &FilterContext,
        acquired_nodes: &mut AcquiredNodes<'_>,
        draw_ctx: &mut DrawingCtx,
    ) -> Result<FilterResult, FilterError> {
        let functions = get_parameters(node)?;

        let input = self.base.get_input(ctx, acquired_nodes, draw_ctx)?;
        let bounds = self
            .base
            .get_bounds(ctx)?
            .add_input(&input)
            .into_irect(ctx, draw_ctx);

        // Create the output surface.
        let mut surface = ExclusiveImageSurface::new(
            ctx.source_graphic().width(),
            ctx.source_graphic().height(),
            input.surface().surface_type(),
        )?;

        #[inline]
        fn compute_func<F>(func: &F) -> impl Fn(u8, f64, f64) -> u8 + '_
        where
            F: FeComponentTransferFunc,
        {
            let compute = func.function();
            let params = func.function_parameters();

            move |value, alpha, new_alpha| {
                let value = f64::from(value) / 255f64;

                let unpremultiplied = if alpha == 0f64 { 0f64 } else { value / alpha };

                let new_value = compute(&params, unpremultiplied);
                let new_value = clamp(new_value, 0f64, 1f64);

                ((new_value * new_alpha * 255f64) + 0.5) as u8
            }
        }

        let compute_r = compute_func::<FeFuncR>(&functions.r);
        let compute_g = compute_func::<FeFuncG>(&functions.g);
        let compute_b = compute_func::<FeFuncB>(&functions.b);

        // Alpha gets special handling since everything else depends on it.
        let compute_a = functions.a.function();
        let params_a = functions.a.function_parameters();
        let compute_a = |alpha| compute_a(&params_a, alpha);

        // Do the actual processing.
        surface.modify(&mut |data, stride| {
            for (x, y, pixel) in Pixels::within(input.surface(), bounds) {
                let alpha = f64::from(pixel.a) / 255f64;
                let new_alpha = compute_a(alpha);

                let output_pixel = Pixel {
                    r: compute_r(pixel.r, alpha, new_alpha),
                    g: compute_g(pixel.g, alpha, new_alpha),
                    b: compute_b(pixel.b, alpha, new_alpha),
                    a: ((new_alpha * 255f64) + 0.5) as u8,
                };

                data.set_pixel(stride, output_pixel, x, y);
            }
        });

        Ok(FilterResult {
            name: self.base.result.clone(),
            output: FilterOutput {
                surface: surface.share()?,
                bounds,
            },
        })
    }
}

impl FilterEffect for FeComponentTransfer {
    fn is_affected_by_color_interpolation_filters(&self) -> bool {
        true
    }
}

/// Takes a feComponentTransfer and walks its children to produce the feFuncX arguments.
fn get_parameters(node: &Node) -> Result<Functions, FilterError> {
    let func_r_node = get_func_x_node!(node, FeFuncR, Channel::R);
    let func_g_node = get_func_x_node!(node, FeFuncG, Channel::G);
    let func_b_node = get_func_x_node!(node, FeFuncB, Channel::B);
    let func_a_node = get_func_x_node!(node, FeFuncA, Channel::A);

    for node in [&func_r_node, &func_g_node, &func_b_node, &func_a_node]
        .iter()
        .filter_map(|x| x.as_ref())
    {
        if node.borrow_element().is_in_error() {
            return Err(FilterError::ChildNodeInError);
        }
    }

    let r = func_or_default!(func_r_node, FeFuncR);
    let g = func_or_default!(func_g_node, FeFuncG);
    let b = func_or_default!(func_b_node, FeFuncB);
    let a = func_or_default!(func_a_node, FeFuncA);

    Ok(Functions { r, g, b, a })
}
