Version 2.55.0-alpha
====================

This is the first release in the new development series.

- The Minimum Supported Rust Version (MSRV) is now Rust 1.58.

- #855: The release tarball no longer contains vendored Rust
  dependencies.  Most distributions now have infrastructure to pull
  these themselves, so let's make the tarball smaller.

- #880 - Accept patterns with userSpaceOnUse units for the stroke of
  axis-aligned lines.

- #706: Small reductions in memory consumption of the DOM tree
  (Michael Howell).

- Updates for the gtk-rs API (Bilal Elmoussaoui).

Version 2.54.4
==============

- #817 - Support CSS Color 4 syntax for `<alpha-value>`.  Opacities
  can be specified as numbers or percentages now, e.g. `0.5` or `50%`
  (Michael Howell).

- #870 - Roll back minimum required version of Pango to 1.46.0
  (Kleis Auke Wolthuizen).

- #867 - Fix Windows NMake install when documentation is not built.
  (Peter Williams).

Version 2.54.3
==============

- #866 - Fix detection of gi-docgen (David King, Jan Alexander Steffens).

- Install the generated documentation in the correct place so that
  Devhelp can find it (thanks to Jan Tojnar for pointing this out).

Version 2.54.2
==============

- #823 - Fix regressions when computing element geometries.

- Add a --disable-gtk-doc option for the configure script, so people
  can disable generating documentation for cross-compiling (Matt
  Turner).

- MSVC: Support generating documentation, and passing introspection
  paths (Chun-wei Fan)

Version 2.54.1
==============

This release just has some build-time fixes.

- Fix oversight in the Minimum Supported Rust Version (MSRV) - this
  release requires Rust 1.56 or later.  This has been true since
  librsvg 2.53.1, but it was not checked correctly. (Matt Turner)

- #859 - Make rst2man and gi-docgen optional. (Kleis Auke Wolthuizen)

- #856 - Fix documentation comments.

Version 2.54.0
==============

This is a summary of the 2.53.x release notes; you can also read them
for more detail.  Not many new features; we've been working on
infrastructure and documentation.

- Librsvg now supports SVG2 geometry properties for these elements:
  rect, circle, ellipse, image, svg (Jeremias Ortega).

- #721 - Catch circular references when rendering patterns.

- The C API documentation now uses gi-docgen instead of gtk-doc.
  Rsvg-convert's man page is now converted to reStructuredText instead
  of troff.

- The "Recommendations for applications" chapter in the documentation
  is much improved!

Version 2.53.2
==============

- #851 - Output filled text as text for PDF; fixes regression due to
  outputting all text as paths.

- Fix taller-than-wide proportional scaling and size limiting in
  rsvg-convert.

- #772, #773, #774, #775, #776 - Implement SVG2 geometry properties
  for these elements: rect, circle, ellipse, image, svg (Jeremias
  Ortega).

- #450 - Fix potential unaligned accesses in surface iterators
  (Michael Howell).

- Actually use GDK_PIXBUF_MODULEDIR when calling
  gdk-pixbuf-query-loaders (Fabrice Fontaine).

- Windows build fixes (Chun-wei Fan).

- Add links to functions and types throughout the C API documentation.

Version 2.53.1
==============

While it is not a user-visible change, Jordan Petridis has ported
librsvg's Continuous Integration (CI) infrastructure to use
Freedesktop CI Templates.  This lets us test librsvg easily on
different environments.  If you want your environment tested, now is
the time to add it!

- #599 - Fix incorrect text rendering when text has different scales
  in the X/Y axes.  This regressed after librsvg 2.52.5, when Pango
  had to revert its fix for the same bug.  Now librsvg renders all
  text as paths, and does the scaling itself.  Please file a bug if
  you have evidence that this presents a performance problem for you.

- Update to the latest gtk-rs release (Bilal Elmoussaoui).

- Port to Freedesktop CI Templates (Jordan Petridis).

- Visual Studio builds can work from a git checkout now (Chun-wei Fan)

- Windows build fixes (Martin Hertz, Chun-wei Fan)

Version 2.53.0
==============

This is the first release in the new development series.  There are no
new features, just changes to how the documentation is built.

The man page for rsvg-convert is now generated from a reStructuredText
document, and the C API reference is generated using gi-docgen.

Please make sure you install python3-docutils (for rst2man) and
gi-docgen before compiling librsvg from a tarball.

- #755 - rsvg-convert's man page is now generated from reStructuredText.

- #827 - Move C API reference to gi-docgen.

- Test suite updates (Michael Howell).

- Link against bcrypt for Visual Studio builds (Martin Hertz).

Version 2.52.5
==============

Just bugfixes this time:

- #812 - Fix mangled output in rsvg-convert when redirecting output to
  a pipe on Windows (Michael Howell).

- #766 - When outputting to SVG, rsvg-convert now uses the
  width/height units specified in the command line; it always used
  pixels before (Daniel Petri Rocha).

- #814 - Fix incorrect top/left margins for SVG/PS/EPS/PDF output
  (Daniel Petri Rocha).

- #599 - Fix incorrect placement of glyphs when text has non-uniform
  scaling in the X/Y axes.  This is not a librsvg bug, but is fixed by
  Pango 1.49.3 and later.  Hopefully Pango 1.48.11 will be released
  soon with this fix as well.  Note that this release of librsvg
  cannot increase the minimum Pango version to 1.48.11 because it is
  not released yet.

Miscellaneous:

- Updated crate dependencies: assert_cmd, cast, clap cssparser,
  float-cmp, itertools, nalgebra, png, proptest, rctree, selectors,
  system-deps.

Version 2.52.4
==============

New features:

- #816 - Support the isolation property from the Compositing and
  Blending Level 1 specification.

- Support Visual Studio 2022 (Chun-wei Fan).

Fixes:

- #818 - The opacity and mix-blend-mode properties were not being
  applied when an element has a mask.

- Fix panic when an empty group has a pattern fill and filters.

- Fix the tests on Windows; the still only work when Fontconfig is
  present (Chun-wei Fan).

- Work around a bug in the cairo-rs bindings in the test suite, that
  only manifests itself in s/390x due to its calling convention.  See
  https://github.com/gtk-rs/gtk-rs-core/issues/335

Version 2.52.3
==============

Bugfixes, mostly for text layout.  Also, text links in PDF!

- #17 - Support text-decoration=overline.

- #249 - Basic support for the unicode-bidi property.  Librsvg still
  considers each tspan independently of others, which is incorrect, but
  at least bidi-override works now for a single embedding level.

- #804 - Fix placement of tspan that changes the text direction.

- #805 - :lang() selector should now match lang attribute from an
  element's parent. (Michael Howell)

- #806 - Fix the text-anchor property for right-to-left text.

- #807 - PDF now includes links inside text elements. (Michael Howell)

Version 2.52.2
==============

Bugfixes and new features!

## New features

Thanks to Michael Howell, rsvg-convert now supports generating
multi-page PDFs in a sensible way.

With one SVG document per page, each page with the
SVG's natural size:

  rsvg-convert --format=pdf -o out.pdf a.svg b.svg c.svg

With all pages sized as portrait US Letter, and each SVG scaled to fit
so that there is a 1in margin around each page:

  rsvg-convert --format=pdf -o out.pdf \
    --page-width=8.5in --page-height=11in \
    --width=6.5in --height=8.5in --keep-aspect-ratio \
    --top=1in --left=1in \
    a.svg b.svg c.svg

Please see the man page for details.

- #738 - Support <a> elements inside <text>.  Also, support the CSS :link
  pseudo-class for matching against links. (Michael Howell)

- #649 - Support the CSS :lang() pseudo-class for matching against an
  element's xml:lang attribute. (Michael Howell)

- #790 - Support the mask-type property from SVG2.

## Fixes

- #800 - Don't panic when a shorthand property is set to
  inherit. (Michael Howell)

- #788 - Fix regression with the viewport size of interior <svg>
  elements. (Michael Howell)

- #731 - Allow length units to be case-insensitive, per SVG2. (Kolja Lampe)

## Documentation

- There is now a FEATURES.md in the repository, where you can see all
  the elements, attributes, and properties that librsvg supports.  We
  will be adding detail to this gradually.

- For developers, there is now devel-docs/adding-a-property.md with a
  tutorial on how to add support for new CSS properties.

Version 2.52.1
==============

This is a bugfix release; there are no new features this time.

## Changes:

- #791 - Fix ordering of tspan inside text elements for right-to-left
  languages.

- #789 - Fix text-anchor positioning for right-to-left languages.

- #797 - Fix regression in computing sizes when an SVG has only one of
  width/height and a viewBox.  Thanks to Joshua Fogg for compiling a
  list of test cases for this.

- #565 - Spec compliance - the writing-mode property applies only to
  text elements, no to individual tspan elements.

- #794 - Fix build on big-endian platforms.

- Clarify documentation for the rsvg_handle_write() /
  rsvg_handle_close() deprecated APIs.

Version 2.52.0
==============

This is a big release!  What follows is a summary from the 2.51.x
release notes; you can also read them for more detail.

The biggest user-visible change is that rsvg-convert has been ported
to Rust (Sven Neumann, Paolo Borelli), and it has new features!

## New features in rsvg-convert

### Support for physical units

rsvg-convert is now aware of physical units, and
fixes a bug where PDFs were created at the wrong size.  Do you need to
render an SVG in a PDF file, scaled to 10x10 cm, placed at a certain
position of a landscape A4 page?

  rsvg-convert --format=pdf \
    --page-width=297mm --page-height=210mm \
    --width=10cm --height=10cm --keep-aspect-ratio \
    --top=5cm --left=8cm \
    foo.svg > foo.pdf

Please see the rsvg-convert(1) man page for more details and plenty of
examples.

### Support for Accept-Language

Previously, librsvg picked up the user's language preferences through
environment variables like LANG and LC_MESSAGES.  This is inconvenient
for applications that call rsvg-convert but don't want to synthesize a
LANG variable.

There is a new option in rsvg-convert so you can pass
--accept-language=<languages> formatted as an HTTP Accept-Language
header.  This is used to specify which languages will be chosen from
elements with the "systemLanguage" attribute:

    rsvg-convert --accept-language=es-MX,en foo.svg

That command will select Mexican Spanish and English from suitable SVG
elements.  Please see the man page for details.

### Miscellaneous

rsvg-convert's default DPI is now 96, to better match W3C
standards.  It was 90 before for historical reasons.  We can change
this back to 90 if it breaks too many scripts.  You can use the
options "--dpi-x=90 --dpi-y=90" to restore the old behavior.

rsvg-convert no longer supports the "xml" or "recording" output
formats.  These are useful only for debugging Cairo, not for general
usage.

## SVG2/CSS3 features

The following features are supported now.  Madds H, John Ledbetter,
worked on these features.

- transform property from SVG2; previously librsvg only supported the
  transform attribute from SVG1.1, which has different syntax.

- context-fill and context-stroke for <marker> and <use> elements.

- markers now support orient="auto-start-reverse".

- paint-order for text elements.

- "auto" values for the width and height attributes of the <image>,
  <rect>, and <svg> elements.

- All the <filter-function> types from the Filter Effects Module Level
  1 specification: blur(), brightness(), contrast(), drop-shadow(),
  grayscale(), hue-rotate(), invert(), opacity(), sepia(), saturate().

- The filter property now supports chains of uri() filters or
  <filter-function> shortcuts.

- Support CSS selectors for attribute matching, like rect[attr^="prefix"]

## New APIs

See the HTML documentation for details:

- rsvg_handle_get_intrinsic_size_in_pixels()

- rsvg_major_version / rsvg_minor_version / rsvg_micro_version
  variables - used to obtain the librsvg version from languages other
  than C, since they do not have access to the C macros like
  LIBRSVG_MAJOR_VERSION.

## Deprecations

The following APIs are deprecated but still available:

- rsvg_handle_render_cairo() - use rsvg_handle_render_document() instead.

- rsvg_handle_render_cairo_sub() - use rsvg_handle_render_layer() or
  rsvg_handle_render_element() depending on what you want to do.

Please see the "Migrating from old APIs" chapter in the HTML
documentation for details.

## News for developers

If you want to run the librsvg test suite easily, there are now Docker
scripts to do so.  Please see the tools/docker/README.md file for
details. (Madds H)

There is no leftover C code in the library; all of the implementation
and the publically-visible symbols are defined in the Rust code.  The
remaining .h files are all public and do not reflect any .c
code. (Sven Neumann)

The test suite is now ported to Rust.  The only remaining tests in C
are for the C API itself (tests/api.c).  With the test suite in
Rust, the tests are automatically run in parallel across CPU cores,
making test runs much faster.  (Sven Neumann, Dunja Lalic)

"cargo build", "cargo test" now work without running autotools first,
so you can in general develop librsvg as a normal Rust project.

The HTML documentation has new chapters; you may find interesting
things there!

## News for distributors

There is a new list of librsvg releases with security fixes in
SECURITY.md.  That file also contains security-related information
on librsvg's dependencies.

The Minimum Supported Rust Version (MSRV) is now Rust 1.52.

## Special thanks

Paolo Borelli and Sven Neumann did a lot of painstaking work to finish
porting the library and rsvg-convert to Rust.

Sven Neumann and Dunja Lalic ported the test suite to Rust, making it
much faster.

Dunja Lalic rewrote the Continuous Integration infrastructure, making
it MUCH faster.

Madds H did their Outreachy internship for librsvg and implemented a
bunch of useful SVG2/CSS3 features.

John Ledbetter methodically went through all the <filter-function>
shortcuts and implemented them for SVG2.

Ismael Luceno has been cleaning up our autotools scripts.

Andre Klapper has been wrangling numerous bug reports from Wikimedia
as usual.

Chun-wei Fan, Abraham Toriz, Christian Hergert, Ignacio Casal Quinteiro
have been keeping the Windows and MacOS builds working.


Version 2.51.4
==============

- #618 - SVG2: Implement context-fill and context-stroke for markers
  (Madds H).

- #727 - SVG2: Implement paint-order for text elements (Madds H).

- #747 - SVG2: Support width="auto" and height="auto" for the image element.

- Fix the Windows build (Chun-wei Fan).

- The tools/docker directory now has scripts that developers can use
  to test librsvg on containers for various Linux distributions.

- Gtk-rs dependency is updated to 0.14.0 (Bilal Elmoussaoui, Chun-wei Fan).

- #758 - Panic when rendering with masks or opacity to a non-image surface.

- #757 - Fix 32-bit builds.


Version 2.51.3
==============

The big news is that rsvg-convert is now aware of physical units, and
fixes a bug where PDFs were created at the wrong size.  Do you need to
render an SVG in a PDF file, scaled to 10x10 cm, placed at a certain
position of a landscape A4 page?

  rsvg-convert --format=pdf \
    --page-width=297mm --page-height=210mm \
    --width=10cm --height=10cm --keep-aspect-ratio \
    --top=5cm --left=8cm \
    foo.svg > foo.pdf

Please see the rsvg-convert(1) man page for more details and plenty of
examples.

SVG2 features:

Markers now implement orient="auto-start-reverse".  The work on
markers is by Madds H., who is doing their Outreachy internship for
librsvg.

All the <filter-function> types in SVG2 are now supported, thanks to
John Ledbetter.

- The Minimum Supported Rust Version (MSRV) is now Rust 1.52.  This
  takes care of CVE-2021-28878 in the Rust standard library.

- #514 - rsvg-convert is now aware of physical units.

- #484 - Markers can now have orient="auto-start-reverse" per SVG2
  (Madds H - Outreachy internship).

- #711 - Implement the drop-shadow() filter function (John Ledbetter).

- #713 - Implement the hue-rotate() filter function (John Ledbetter).

- #677 - rsvg-convert, do not clip the rightmost/bottomost pixels of
  an image with partial pixel coverage.

- Partial fix for #668 - Render small caps for fonts that support the
  "smcp" OpenType feature.  Librsvg and Pango are not yet able to
  synthesize small caps for fonts that do not support them, but for
  those that do, they should work fine now.

- #566 - Restrict which elements can appear inside a clipPath, to be
  spec compliant.

- #746 - Possible cairo_save() without cairo_restore() in render_layer().

- Various updates to the developer's documentation.


Version 2.51.2
==============

This release fixes an important bug about text spacing.  The bug fix
requires an update to at least Pango 1.44.  Sorry for the increased
requirements!

Librsvg now supports most of the filter function shortcuts in SVG2;
see below.

Previously, librsvg picked up the user's language preferences through
environment variables like LANG and LC_MESSAGES.  This is inconvenient
for applications that call rsvg-convert but don't want to synthesize a
LANG variable.

There is a new option in rsvg-convert so you can pass
--accept-language=<languages> formatted as an HTTP Accept-Language
header.  This is used to specify which languages will be chosen from
elements with the "systemLanguage" attribute:

    rsvg-convert --accept-language=es-MX,en foo.svg

That command will select Mexican Spanish and English from suitable SVG
elements.  Please see the man page for details.

- The Minimum Supported Rust Version (MSRV) is now Rust 1.51.

- Librsvg now requires at least Pango 1.44.

- #730 - Incorrect text spacing when the transform is not 1:1.  You
  can see this when a small font-size is scaled up due to a
  transform.  It is less visible for a large font-size scaled down.

- #709 #710 #712 714 #715 #716 #717 - The "filter" attribute now
  accepts lists of "<filter-function>" per SVG2.  There is support for
  blur(), contrast(), grayscale(), invert(), opacity(), saturate(),
  sepia() (John Ledbetter).

- #356 - Add --accept-language option to rsvg-convert.

- #704 - Fix circle/ellipse in paths when they are made out of a
  single Arc command.

- #691 - Don't allow number lists with unbounded lengths in
  tableValues attributes, for feComponentTransfer and
  feConvolveMatrix (Madds H).

- #718 - Negative rx/ry in rect element should be ignored.

- #687 - Reduce memory pressure when rendering text.

- Fix build on 32-bit ARM (Lovell Fuller).

- Update the Rust crate dependencies (Bastien Orivel).

- Refactoring parsers (Paolo Borelli).

- There is a new list of librsvg releases with security fixes in
  SECURITY.md.  That file also contains security-related information
  on librsvg's dependencies.

Special thanks to John Ledbetter for carefully implementing each of
the new shortcuts for filter functions in SVG2.


Version 2.51.1
==============

- The Minimum Supported Rust Version is now 1.48.

- rsvg-convert should fully work on Windows again (Abraham Toriz).

- rsvg-convert's SVG output format uses pixel units instead of points again.

- #699 - Images embedded as data: URLs didn't render if they had a
  MIME type with a charset parameter.

- #698 - Add limit for too-large radiuses on the feMorphology filter (Madds H).

- #686 - Reduced stack usage (Sebastian Dröge).

- #261 - Parse the enable-background property.

- #703 - Properly ignore elements in an error state inside the "switch" element.

- #695 - Fix cascading mode for the "feImage" element.

- Fix cascading for the "filter" element and filter primitives in general.

- Remove constraints on the types of units used within the "filter"
  element and filter primitives.

- Reduced memory consumption in general by about 300 bytes per SVG element.

- Update vulnerable crates:
    smallvec to 1.6.1 (RUSTSEC-2021-0003)
    generic-array to 0.13.3 (RUSTSEC-2020-0146)

- Lots of cleanups to the build (Sven Neumann).

- Update to gtk-rs 0.9 (Bilal Elmoussaoui).

- Updated ARCHITECTURE.md and documentation in general.

Special thanks to Dunja Lalic for adding the start of code coverage
analysis to the build.


Version 2.51.0
==============

There are many changes in this development release!  This experimental
release is meant for early testing, with the understanding that some
things may be broken, especially since we have large changes to the
way librsvg's artifacts are built.

The biggest user-visible change is that rsvg-convert has been ported
to Rust (Sven Neumann, Paolo Borelli).

Please file bugs at https://gitlab.gnome.org/GNOME/librsvg/-/issues/new
if these changes break your scripts; they are experimental for the
2.51.x development series:

- #646 - rsvg-convert's default DPI is now 96, to better match W3C
  standards.  It was 90 before for historical reasons.  We can change
  this back to 90 if it breaks too many scripts.  You can use the
  options "--dpi-x=90 --dpi-y=90" to restore the old behavior.

- rsvg-convert's --export-id (-i) option should now have more useful
  behavior.  This extracts a particular element from the SVG document
  and renders it scaled to the size specified by the --width/--height
  arguments, or to the pixel size of the element as if it had no
  transformations applied.

- rsvg-convert no longer supports the "xml" or "recording" output
  formats.  These are useful only for debugging Cairo, not for general
  usage.

Known missing features, which should be restored for the stable release:

- rsvg-convert does not allow stdin/stdout streams on Windows; this is
  being tracked in issue #676.

The following are changes related to porting to Rust; they are not
necessarily user-visible, but important for developers of the library:

- There is no leftover C code in the library; all of the
  implementation and the publically-visible symbols are defined in the
  Rust code.  The remaining .h files are all public and do not reflect
  any .c code.  (Sven Neumann)

- The test suite is now ported to Rust.  The only remaining tests in C
  are for the C API itself (tests/api.c).  With the test suite in
  Rust, the tests are automatically run in parallel across CPU cores,
  making test runs much faster.  (Sven Neumann, Dunja Lalic)

- "cargo build", "cargo test" should now work without running
  autotools first.

- The C API is now implemented on top of the public Rust API, without
  special hooks into the library's internals.

The following APIs are deprecated but still available:

- rsvg_handle_render_cairo() - use rsvg_handle_render_document() instead.

- rsvg_handle_render_cairo_sub() - use rsvg_handle_render_layer() or
  rsvg_handle_render_element() depending on what you want to do.

- These deprecations are because the new APIs conform with the web
  world's view of how SVGs should be positioned and scaled in
  surrounding content.  Whereas the old APIs were about rendering SVGs
  to whatever current transformation matrix a Cairo context may
  contain, the new APIs take a rectangular viewport and librsvg
  automatically scales the SVG document to fit in it.  The "natural
  sizing" that was implicit in the deprecated APIs is now explicitly
  documented, and available through the new API
  rsvg_handle_get_intrinsic_size_in_pixels().

New APIs:

- rsvg_handle_get_intrinsic_size_in_pixels() converts an SVG
  document's intrinsic dimensions to pixels, i.e. transforms the SVG
  document's "width" and "height" attributes to CSS pixels.

New features and bug fixes:

- #615: SVG2: Support a chain of uri() filters in the "filter" property
  (John Ledbetter, Sven Neumann).

- #483: Support CSS selectors for attribute matching, like rect[attr^="prefix"]

- #554: Fixed the geometry_for_layer() APIs to not ignore the passed viewport.

- Fixed CSS "import" so it allows only files from the same base directory
  (Lars Schmertmann).

- #642 - Fix dx/dy offsets in nested <tspan> elements.

- #601 - Compute correct bounds for objects with stroke-width=0.

- Slight speed improvements in the RGBA premultiplication code (Sven Neumann).

- #623 - The pkg-config files (*.pc) do not define the 'svgz_supported' and
  'css_supported' variables anymore.  These variables were hardcoded
  to 'true' and unchanged since 2011.

- #624 - The source repository no longer produces a librsvg-uninstalled.pc file.

- Fix the MacOS build (Christian Hergert, Ignacio Casal Quinteiro).

- Deal with missing pkg-config (Ismael Luceno).

- For cross-compilation, check for target-specific prefixed tools like
  rustc/cargo (Heiko Becker).

Changes in the Rust API:

- #597 - The LoadingError and RenderingError enums have changed, and
  are now marked #[non_exhaustive].  They will probably change again
  to hide details of error variants before we make librsvg available
  as a crate on crates.io.

Special thanks for this release:

- Dunja Lalic for revamping the CI infrastructure and making our CI
  runs much, much faster - !398.

- Sven Neumann, Dunja Lalic for porting the test suite to Rust.

- Sven Neumann, Paolo Borelli for porting rsvg-convert to Rust and
  unraveling all the little historical details that were embedded in
  it.

- Paolo Borelli for constant refactoring.


Version 2.50.7
==============

Two cairo-related bug fixes:

- #745 - Fix mismatched cairo_save/restore when running in inside the Cairo test suite.
- #746 - Possible cairo_save() without cairo_restore() in render_layer().


Version 2.50.6
==============

This release fixes an important bug about text spacing.  The bug fix
requires an update to at least Pango 1.44.  Sorry for the increased
requirements!

- Librsvg now requires at least Pango 1.44.

- #730 - Incorrect text spacing when the transform is not 1:1.  You
  can see this when a small font-size is scaled up due to a
  transform.  It is less visible for a large font-size scaled down.

- #704 - Fix circle/ellipse in paths when they are made out of a
  single Arc command.


Version 2.50.5
==============

- #699 - Images embedded as data: URLs didn't render if they had a
  MIME type with a charset parameter.

- #691 - Don't allow number lists with unbounded lengths in
  tableValues attributes, for feComponentTransfer and
  feConvolveMatrix (Madds H).

- #718 - Negative rx/ry in rect element should be ignored.


Version 2.50.4
==============

Update dependent crates that had security vulnerabilities:

  generic-array to 0.13.3 - RUSTSEC-2020-0146

- #686 - Reduced stack usage (Sebastian Dröge).

- #698 - Add limit for too-large radiuses on the feMorphology filter (Madds H).

- #703 - Properly ignore elements in an error state inside the "switch" element.



Version 2.50.3
==============

- #601 - Compute correct bounds for objects with stroke-width=0.

- #545 - Fix MacOS build (Ignacio Casal Quinteiro, Christian Hergert).

- Fix test suite on Rust 1.49 (Sven Neumann).

Version 2.50.2
==============

- #642 - Fix dx/dy offsets in nested <tspan> elements.

Version 2.50.1
==============

- #615: SVG2: Support a chain of uri() filters in the "filter" property
  (John Ledbetter, Sven Neumann).

- #483: Support CSS selectors for attribute matching, like rect[attr^="prefix"]

- #554: Fixed the geometry_for_layer() APIs to not ignore the passed viewport.

- Fixed CSS "import" so it allows only files from the same base directory
  (Lars Schmertmann).

- #623 - The pkg-config files (*.pc) do not define the 'svgz_supported' and
  'css_supported' variables anymore.  These variables were hardcoded
  to 'true' and unchanged since 2011.

- #624 - The source repository no longer produces a librsvg-uninstalled.pc file.

Version 2.50.0
==============

- The following is a summary of changes between 2.48.x and 2.50.0.  For
  full details, please see the 2.49.x release notes below.

- This release requires at least Rust 1.40.

- Windows builds now support ARM64, thanks to Chun-wei Fan.

- Librsvg now consumes much less memory for large SVG files.  The limit for
  the maximum number of elements in an SVG is bigger now at 1 million
  elements (this limit exists to avoid unbounded memory consumption by
  maliciously large files, and is plenty to render even very detailed
  maps).  Thanks to Sergey "Shnatsel" Davidoff and Adam Reichold for making
  this possible.

- #34 - The 'font' shorthand in is now supported in CSS.  Librsvg
  ignores the 'line-height' sub-property because it cannot be done
  easily with Pango, but everything else in 'font' should work now.

- Many new features from SVG2:

- #508 - radialGradient now supports the "fr" property from SVG2
  (Dunja Lalic, Corentin Rossignon).

- #568 - Support href attribute in addition to xlink:href per SVG2.

- #560 - Ignore missing filter references per SVG2.

- #607 - Support the mix-blend-mode property from SVG2 and the
  Compositing and Blending Level 1 specification, so layers can be
  composited with operators like multiply/screen/color-burn, etc.
  https://www.w3.org/TR/compositing-1/ (John Ledbetter).

- #473 - Support the paint-order property from SVG2, so one can pick
  the order in which a path's fill/stroke/markers are drawn (John Ledbetter).

- Many bug fixes and little optimizations; see the 2.49.x release notes for
  details.  Thanks to Paolo Borelli, Daniel Kolesa, Ernestas Kulik.
  Sven Neumann, Bastien Orivel, Jordan Petridis, Emile Snyder.

- Special thanks to Sven Neumann for fixing all the cargo-clippy lints.


Version 2.49.5
==============

- #607 - Support the mix-blend-mode property from SVG2 and the
  Compositing and Blending Level 1 specification, so layers can be
  composited with operators like multiply/screen/color-burn, etc.
  https://www.w3.org/TR/compositing-1/ (John Ledbetter).

- #473 - Support the paint-order property from SVG2, so one can pick
  the order in which a path's fill/stroke/markers are drawn (John Ledbetter).

- Cleanups of the basic drawing and paint-handling code (Paolo Borelli).

- Many style fixes per cargo-clippy (Sven Neumann).

- Improved documentation on the library's internals.

Version 2.49.4
==============

- Support ARM64 Windows builds (Chun-wei Fan).

- #165 - Draw zero-length paths with square caps correctly (Bastien Orivel).

- #568 - Support href attribute in addition to xlink:href per SVG2.

- #560 - Ignore missing filter references per SVG2.

- #609 - Support the transform attribute again in clipPath (Sven Neumann).

- Update Criterion benchmarks (Ernestas Kulik).

- Update to newer dependencies (Bastien Orivel).

- Many style fixes per cargo-clippy (Dunja Lalic, Sven Neumann).

Version 2.49.3
==============

- #34 - The 'font' shorthand in is now supported in CSS.  Librsvg
  ignores the 'line-height' sub-property because it cannot be done
  easily with Pango, but everything else in 'font' should work now.

- #605 - Compute 'bolder' and 'lighter' font-weight correctly.

- #219 - Make the path parser have a tokenizer/parser (Emile Snyder).

- Fixes for the CI after the gitlab.gnome.org changes (Jordan Petridis).

- Fixed some typos in the documentation (codespell is awesome!).

Version 2.49.2
==============

- Fix failing tests.

Version 2.49.1
==============

- (#604) - Fix build on big-endian machines.  Thanks to Daniel Kolesa
  for testing this repeatedly.

Version 2.49.0
==============

- Memory consumption of parsed SVG data is greatly reduced.  Property
  values, path data, and text nodes now use much less memory than
  before.  Thanks to Sergey "Shnatsel" Davidoff and Adam Reichold for
  their contributions.

- With that reduction in memory consumption, librsvg can now load up
  to 1 million XML elements.  This is a hard limit to avoid malicious
  files consuming arbitrary amounts of memory.

- #508 - radialGradient now supports the "fr" property from SVG2
  (Dunja Lalic, Corentin Rossignon).

- Sven Neumann has written a fantastic test suite for rsvg-convert(1).
  Now we can modify this program with confidence.

- Many, many cleanups (Paolo Borelli).

- #594 - Only compute locale information for elements that need it.

- As usual, thanks to Bastien Orivel for keeping dependent crates updated.

- Fix Windows builds without PangoFT2.

- Benchmarks of internal functions work now.  You can run "cargo bench"
  in librsvg/ and rsvg_internals/.

Version 2.48.5
==============

- #577 - Support multiple fonts in the font-family property (Bastien Orivel).
  Previously in font-family="Foo, Bar, Baz" only Foo would get used.

- #591 - Catch overflow when rendering files with a huge viewBox.

- #593 - Don't panic with an empty objectBoundingBox for a mask.

- #596 - Fix introspection data for rsvg_handle_set_stylesheet (Sutou Kouhei).

- Fixes to the librsvg_crate documentation (Abraham Toriz).

- Loading raster images for inclusion in an SVG and producing
  GdkPixbufs is now faster.

Version 2.48.4
==============

- #590 - Fix regression when rendering masks.

- #588 - Fix panic when a big viewBox creates an invalid transform.

- #592 - Fix invalid argument to g_log_structured_array()

Version 2.48.3
==============

- Librsvg now requires glib 2.50.0 or later.

- rsvg-convert's --background-color option now accepts colors with
  transparency like #rrggbbaa, rgba(...), hsla(...); the transparency
  value was ignored before (Hao Guan).

- #584 - Fix arithmetic overflow when rendering to large GdkPixbufs.

- #582 - Make librsvg build with Rust 1.39 again.

- #581 - Don't require upcalls into C code for Debian powerpc.

Version 2.48.2
==============

- Fix linking of the test suite against Harfbuzz.

Version 2.48.1
==============

- #129 - Fix baseline-shift for simple subscripts/superscripts and
  absolute offsets.  This should fix a lot of Wikimedia images with
  formulas.

- #548 - Support images with data: URLs that don't have a MIME-type.
  This fixes some Open Clip Art images generated by old versions
  of Adobe Illustrator.

- Fix build of the test suite on Windows (Chun-wei Fan).

- Support running the rsvg_internals tests on Windows (Chun-wei Fan).

Version 2.48.0
==============

- The following is a summary of changes between 2.46.x and 2.48.0.
  For full details, please see the 2.47.x release notes below.

- This release requires at least Rust 1.39.

- #379 - New API, rsvg_handle_set_stylesheet(), to set a CSS
  stylesheet independent of the SVG document.

- #510 - support opacity in patterns.

- Librsvg's XML parser now supports namespaces (xmlns), and is
  stricter than before about it.  Files may fail to parse if there are
  attributes or elements with namespace prefixes (e.g. foo:bar instead
  of plain bar), but without a corresponding namespace declaration
  (e.g. xmlns:foo="http://example.com/foo").

  This may happen especially with incorrectly-written SVGs that use
  xlink:href or xi:include attributes without the corresponding
  namespace declarations.  If you run into this, just add the
  following to your toplevel SVG element:

      <svg xmlns="http://www.w3.org/2000/svg"
           xmlns:xlink="http://www.w3.org/1999/xlink"
	   xmlns:xi="http://www.w3.org/2001/XInclude">
           ^^^^^^^^^ these ones

- Librsvg no longer depends on libcroco, and now does all CSS
  processing using Rust crates from Mozilla Servo.  As a result,
  librsvg can now handle much more complex CSS selectors than before.

- Link-time optimization (LTO) is disabled by default on release
  builds, as this increased build time too much.  Downstream
  distributors may want to turn it back on in the toplevel Cargo.toml.

- #515 (CVE-2019-20446) - Librsvg now has limits on the number of
  loaded XML elements, and the number of referenced elements within an
  SVG document.  This is to mitigate malicious SVGs which try to
  consume all memory, and those which try to consume an exponential
  amount of CPU time.

- Many bugfixes; please see the 2.47.x release notes below.

Version 2.47.4
==============

- (#240) - Fix rsvg-convert's multipage PDF output when the zoom
  option is used (Sven Neumann).

- (#547) - Do not stop rendering if an <image> element references a
  nonexistent file.  This fixes a number of Open Clipart cases.

- (#558) - Compute the font-size cascade correctly when there are "em"
   #and "ex" units involved.

- Updated the man page for rsvg-convert (Sven Neumann).

Version 2.47.3
==============

- #379 - New API, rsvg_handle_set_stylesheet(), to set a CSS
  stylesheet independent of the SVG document.

- #510 - support opacity in patterns (Sven Neumann).

- Move away from the Cairo transform type to our own (Paolo Borelli).

- Update the gtk-rs version.

Version 2.47.2
==============

- Handling of the "result", "in", "in2" attributes in filter
  primitives is slightly stricter now, and spec compliant.  Their
  arguments must be of type CSS custom-ident, so "default", "inherit",
  "initial", and "unset" are disallowed.  Most SVGs should still work
  fine.

- #542 - Fix infinite loop when processing CSS sibling combinators.

- #408 - feImage filters no longer clip their output to integer
  coordinates.

- #504 - Documentation for the Rust crate (available at
  https://gnome.pages.gitlab.gnome.org/librsvg/doc/librsvg/) now has
  API usage examples.

- Debug logs from RSVG_LOG=1 should now be more legible and contain
  better information on invalid CSS.

- Remove link-time workarounds for Rust pre-1.35 (Kleis Auke Wolthuizen).

- Unify internal error types to share the CSS code with gnome-shell.

- Made handling of XML namespaces more spec-compliant.

- Lots of refactoring to start moving away from Cairo internals
  (Paolo Borelli).

Version 2.47.1
==============

- Librsvg no longer depends on libcroco!  It now does all CSS
  processing using Rust crates from Mozilla Servo; these are also the
  crates that are in use in recent versions of Firefox.  As a result,
  librsvg can now handle much more complex CSS selectors than before.
  Fixes #79, #167, #237, #283, #336, #428, #441, #466, #525, #525
  (Paolo Borelli, Federico Mena).  Thanks to Evgeniy Reizner
  for fixing https://github.com/servo/servo/issues/22972, which made
  it possible to use Servo's selectors crate.

- #524 - Panic when reading an invalid stylesheet URL in an XML
  processing instruction (Paolo Borelli)

- Lots of little improvements to the documentation.

- Link-time optimization (LTO) is disabled by default on release
  builds, as this increased build time too much.  Downstream
  distributors may want to turn it back on in the toplevel Cargo.toml.

- We now have the start of documentation on the library's internals at
  https://gnome.pages.gitlab.gnome.org/librsvg/doc/rsvg_internals/index.html
  This should be interest of newcomers to librsvg's source code.

Version 2.47.0
==============

- Librsvg's XML parser now supports namespaces (xmlns), and is
  stricter than before about it.  Files may fail to parse if there are
  attributes or elements with namespace prefixes (e.g. foo:bar instead
  of plain bar), but without a corresponding namespace declaration
  (e.g. xmlns:foo="http://example.com/foo").

  This may happen especially with incorrectly-written SVGs that use
  xlink:href or xi:include attributes without the corresponding
  namespace declarations.  If you run into this, just add the
  following to your toplevel SVG element:

      <svg xmlns="http://www.w3.org/2000/svg"
           xmlns:xlink="http://www.w3.org/1999/xlink"
	   xmlns:xi="http://www.w3.org/2001/XInclude">
           ^^^^^^^^^ these ones

- Patterns and gradients reused across more than one element will only
  get resolved once now; this should make things marginally faster for
  patterns or gradients with fallbacks.

- #515 - Librsvg now has limits on the number of loaded XML elements,
  and the number of referenced elements within an SVG document.  This
  is to mitigate malicious SVGs which try to consume all memory, and
  those which try to consume an exponential amount of CPU time.

- #521 - Compute geometries correctly if there is a viewBox attribute.

- #308 - Fix stack exhaustion with circular references in <use> elements.

- Consistently use the LGPL 2.1 wherever it is mentioned.

- Patterns and gradients reused across more than one element will only
  get resolved once now; this should make things marginally faster for
  patterns or gradients with fallbacks.

- #515 - Librsvg now has limits on the number of loaded XML elements,
  and the number of referenced elements within an SVG document.  This
  is to mitigate malicious SVGs which try to consume all memory, and
  those which try to consume an exponential amount of CPU time.

- #506 - Fix empty patterns which reference a fallback pattern with
  children.

Version 2.46.0
==============

- The following are highlights compared to the 2.44.x series.  For
  full details, please see the release notes for the 2.45.x series.

- All of librsvg.so is now implemented in Rust!  That is, except for a
  very thin wrapper over the public API functions.  Hopefully we can
  remove this wrapper when Cargo gets some more features around
  controlling the linking step.  This release requires at least Rust 1.34.

- Librsvg now comes with a Rust crate that can be used from Rust
  applications.  See librsvg_crate/examples.  This Rust API is
  designed to be idiomatic; if you want a Rust binding to the shared
  library instead, please use the "rsvg-rs" crate from crates.io.

- The following API functions are new in the C library:
    rsvg_handle_get_intrinsic_dimensions()
    rsvg_handle_render_document()
    rsvg_handle_render_layer()
    rsvg_handle_render_element()
    rsvg_handle_get_geometry_for_layer()
    rsvg_handle_get_geometry_for_element()

  Correspondingly, there is a new chapter in the documentation, called
  "Recommendations for Applications".  These new APIs conform better
  with the web platform's idea of how SVG sizing/positioning should
  work.  Applications should now find it easier to scale and render
  SVGs in a single call, instead of having to obtain image dimensions
  first.

- A bunch of functions have been deprecated but are still available:

    - rsvg_handle_write()/close() are deprecated in favor of the
      stream functions.  Unfortunately the write()/close() pair
      require buffering the entire document, in case it is a .svgz
      compressed file; the streaming functions do not have this
      problem.

    - Functions that return RsvgDimensionData and RsvgPositionData are
      deprecated, since they just use integers instead of floating
      point numbers.  They are replaced with the _get_geometry_*()
      functions above.

- The library is a lot more strict now in terms of detecting that the
  API functions are called in the correct order.  For example, calling
  rsvg_handle_get_dimensions() before rsvg_handle_close() will now
  emit a critical warning.

- Librsvg is gradually moving towards using code from Mozilla's Servo.
  We haven't quite gotten rid of libcroco and libxml2 yet, but this is
  in progress.

- Many thanks to all the people who participated in the long cycle for
  2.45.  Having the whole library's functionality in Rust is a big
  accomplishment!

Version 2.45.92
===============

- #496 - Ensure all lengths and angles parse as finite numbers

- #497 - Don't panic on paths with all-invalid commands

- #500 - Added additional SVG blend-modes for the feBlend filter primitive (Andargor)

- Some changes in the build structure to allow for faster builds.

Version 2.45.91
===============

- The documentation has a new chapter, Recommendations for Applications.

- #451 - Make rsvg-convert detect images larger than pixman's limit.

- #410 - Update introspection annotations.

- #449 - librsvg_crate: Make cancellable arguments consistent with gio-rs

- librsvg_crate: Take all gio arguments as IsA<SomeGioType> generics

- Updated Rust crates to avoid duplicates (Bastien Orivel)

Version 2.45.90
===============

- New API functions:
    rsvg_handle_render_document()
    rsvg_handle_render_layer()
    rsvg_handle_render_element()
    rsvg_handle_get_geometry_for_layer()
    rsvg_handle_get_geometry_for_element()

  CairoRenderer in the librsvg_crate has corresponding functions
  as well.

- Fix builds with gettext ≥ 0.20 (Ting-Wei Lan).

- If the C API is called out of order, downgrade hard panics to
  g_critical() to cope with incorrect/old applications that called
  rsvg_handle_get_dimensions() before rsvg_handle_close().

- API reference documentation is much improved.

Version 2.45.8
==============

- This version requires at least Rust 1.34.

- #485 - Fix build on Rust earlier than 1.36 (Kleis Auke Wolthuizen).

- More polishing of the Windows build (Chun-wei Fan).

- Update gtk-rs and dependent crates (Bastien Orivel)

Version 2.45.7
==============

- Fix #463 - Don't panic if an SVG has character data outside the
  first element.

- Fix #467: Don't panic when there's an xi:include fallback with no
  parent element.  Thanks to Bastien Orivel for running afl-fuzz
  on librsvg.

- Fix #471: Fix blurry semi-opaque objects when rendering with a
  scaled transformation.  Thanks to the gnome-games people for
  isolating a test case and to Evgeniy Reizner for providing a more
  minimal one.

- Fix #481: Don't ignore the first x/y/dx/dy in text/tspan elements if
  there is more than one position specified.

- #452 - In librsvg_crate, SvgHandle now has a ::has_element_with_id()
  method.

- rsvg-convert now catches the case where the SVG has no dimensions.

- Replaced the Visual Studio build infrastructure for NMake
  (Chun-wei Fan).

- This version no longer contains the rsvg-view program, so librsvg no
  longer depends on GTK.  Please see
  https://people.gnome.org/~federico/blog/removing-rsvg-view.html for
  the rationale behind this change.

- The poly element no longer supports "verts" as an alias for the
  "points" attribute.  The "verts" name was only used in SVG pre-1.0,
  and we had been cargo-culting that name ever since.

- We now use more machinery from Mozilla Servo, in this case the
  markup5ever and rust-selectors crates.  This is in line with
  gradually replacing libcroco with a Rust-only CSS machinery.

- Lots and lots of refactoring and cleanups:  use the rctree crate
  instead of our own tree representation; remove interior mutability
  in element structs; make the gradients and patterns code less
  repetitive (Paolo Borelli).

- Update some dependencies (Bastien Orivel).

- New section in COMPILING.md about cross-compilation to Windows using
  mingw (Takuro Ashie)

- Fix static linking and Windows builds (Kleis Auke Wolthuizen).

Version 2.45.6
==============
- Librsvg now requires Rust 1.30.0 or later.

- Librsvg now requires Cairo 1.16.0 or later.  Thanks to
  Julian Sparber for keeping up with cairo-rs API changes.

- This version introduces librsvg_crate, an idiomatic Rust crate for
  using librsvg from Rust programs directly, without using GObject
  machinery.  This API is subject to change, but you can start using
  it now in an experimental fashion.  Thanks to Paolo Borelli and
  Jordan Petridis for fine-tuning this new API.

- All of the librsvg internals are now in Rust!  The C code is just a
  thin wrapper over Rust functions.

- The internals library has been converted to Rust 2018 (Jordan Petridis).

- Within librsvg_crate, there is a new infrastructure for doing
  reftests in Rust, that does not depend on PNG reference files.  See
  librsvg_crate/tests for details.

- This release introduces the following new APIs:
  rsvg_handle_get_intrinsic_dimensions(),
  rsvg_handle_get_geometry_for_element().

- Parsing of the "style" attribute, which has a plain list of CSS
  property declarations, is now done with rust-cssparser.
- CSS selector matching should be marginally faster than before.

- Fix Visual Studio builds (Chun-wei Fan).

- Fix #11 - Respect the "direction" property for bidirectional text (Khaled Hosny).

- Fix #295 - Ensure the initial viewport fits into temporary surfaces for compositing

- Fix #425 - Don't fail parsing if the system's locale is broken (Paolo Borelli).

- Fix #438, #443 - Don't create intermediate raster surfaces unless
  absolutely needed.  This was causing blurred output for SVGs from
  Inkscape and Illustrator, since they include an "enable-background"
  property even when there are no filters in use.  Thanks to Julian
  Sparber, Jordan Petridis, Zeeshan Ali for doing a huge "git bisect"
  to find the cause of this bug.

- Fix #443 - Fix blurry output when enable-background is used without filters.

- Fix #455 - Fix rounding error on i386 (Olivier Tilloy).

- Check for Cairo errors when constructing paths.

Version 2.45.5
==============

- At build time, you can now pass $CARGO and $RUSTC environment
  variables if you need to override the default Rust toolchain.
  Please see COMPILING.md for details.  (Tobias Kortkamp)
- Fix #405 - In the gdk-pixbuf loader, don't crash if the write()
  function doesn't receive a GError.
- Fix #268 - Remove the comp-op property; it's not in SVG 1.1 nor SVG 2.
- Fix #415 - register RsvgHandleFlags and the RsvgError enum values in
  a thread-safe fashion (Sebastian Dröge).
- All of the library's non-GObject functionality is implemented in
  Rust now.
- Update the cairo crate (Kornel Lesiński).
- Clean up the loading code paths (Paolo Borelli).
- Updated compilation docs for Debian (Jordan Petridis)
- Updated parts of the reference documentation.

Version 2.45.4
==============

- Brown paper bag release, my apologies.
- Fix #402 - Fix the library's soname.  Thanks to Gabriele Balducci
  for reporting it, and Kalev Lember for fixing it.

Version 2.45.3
==============

- Big news!  All the real work in the library is now implemented in
  Rust.  The public API is implemented in C, but most it calls
  immediately into the Rust code.  Special thanks to Paolo Borelli and
  Carlos Martín Nieto for making this possible.

- rsvg_handle_set_base_uri() now really assumes that it is passed a
  URI.  Previously it would try to differentiate between real URIs,
  and absolute or relative file paths.  If this breaks your code
  (i.e. you are passing a filename, not a URI), please tell us so we
  can restore the old behavior!

- Fix #395 - Don't panic in feMorphology if it ends up with a negative
  scaling transformation.
- Fix #398 - Detect circular references in gradients.
- Match the Firefox/Chrome behavior on gradients and patterns with
  circular references for fallbacks (Paolo Borelli).
- Fixes for Rust 1.30 and below (Jordan Petridis).
- Lots and lots of refactoring (Paolo Borelli, Federico Mena).

Version 2.45.2
==============

- rsvg_cleanup() is now deprecated.  This was only meant to be called
  from code to be checked by Valgrind.  Leak checkers may show
  reachable memory from libxml2; real memory leaks should still be
  reported, of course.  (Kornel Lesiński).

- As an experimental change, librsvg no longer calls xmlInitParser()
  from libxml2.  Please tell us if this causes problems for
  multithreaded programs.

- Added g_warning()s to ensure the API is called in the correct
  sequence.

- The text handling code has been completely refactored and
  simplified.  This will allow us to implement the x/y/dx/dy
  properties for multiple glyphs in the future.  Please report any
  problems you experience in text rendering.

- Fix #385 - Don't crash if there is no rsvg_handle_write() before
  rsvg_handle_close().
- Fix #391 - Avoid undefined behavior when casting opaque pointers
  (Jordan Petridis).
- Fix crash when a linear RGB filter is followed by an SRGB filter
  (Ivan Molodetskikh).
- Fix #393 - Stack overflow when freeing thousands of sibling elements.
- Fix positioning of adjacent <tspan> elements.
- All the toplevel loading and drawing code is implemented in Rust now.
- Pixbuf conversion code is in Rust now (Paolo Borelli).
- Cleanups in the code for XML processing, markers, attributes (Paolo
  Borelli).
- Many build/link fixes (Jordan Petridis).
- Cleanups in the code that calls libxml2 (Kornel Lesiński).

Version 2.45.1
==============

- New public API, rsvg_handle_get_geometry_sub(), to get the exact
  geometry of an element.  The functions
  rsvg_handle_get_position_sub() and rsvg_handle_get_dimensions_sub()
  are deprecated now; these returned incomplete data with integer
  coordinates (Julian Sparber).

- rsvg_handle_write() and rsvg_handle_close() are now deprecated in
  favor of the functions which use a GInputStream.  The former need to
  buffer the entire SVG data first; the latter don't need buffering.

- Librsvg no longer tries to load XML entities which reference
  external resources, either parameter or general entities, declared
  in the DTD.  This never worked properly, and it is better to do so
  via the xi:include mechanism.  Also, unparsed external entities with
  a notation are not really supported in SVG; it has its own <image>
  element and similar for that purpose.  Only internal general
  entities are supported now, for example:

     <!ENTITY foo "<some xml here>">
     <!ENTITY bar "some text here">

- Started support for localized error messages (Daniel García Moreno).

- Ported to Rust: loading code, XML processing code, data: URL parsing, 

- Many code cleanups and refactorings, courtesy of Paolo Borelli.
- Fix undefined behavior in casts (Jordan Petridis).
- Cairo/Rust API updates by Julian Sparber.
- configure.ac cleanups by Maya Rashish.

Version 2.45.0
==============

- Librsvg now requires Rust 1.27.
- Librsvg now requires Cairo 1.15.12.
- Fix building when srcdir != builddir (Mathieu Bridon).
- Fix #339 - Panic in filters with primitiveUnits="objectBoundingBox"
  on zero-sized elements (Ivan Molodetskikh).
- Fix #335 - Don't panic if the toplevel node is not <svg>.
- Fixes from fuzz testing (Ivan Molodetskikh): don't panic when the
  feConvolveMatrix kernel is not set; fix upper bounds in filter pixel
  getters.
- Fix #337 - Don't panic with "em" or "ex" units in the font-size property.
- Fix #338 - Don't panic when an image element doesn't have
  width/height attributes.
- Fix #340 - Don't panic when a marker has a zero-sized viewBox attribute.
- Fix #341 - Don't infinite-loop with cyclic pattern references.
- Fix #342 - Don't crash if a <use> node references one of its ancestors.
- Special thanks to Ivan Molodetskikh for doing a fuzz-testing run for
  this round.
- Fix #344 - Don't panic when a viewBox has overflowing numbers.
- Fix #345 - Fix panics due to bad path parsing and overflows in
  surface size.
- Updates to the CI infrastructure and the build documentation (Jordan
  Petridis).
- Cleanups and refactoring (Paolo Borelli, Linus Unnebäck, Federico Mena).
- Fix #343 - Handle child being in error in feComponentTransfer.
- Fix #346 - Handle filter primitives producing errors.
- Fix #347 - Regression in computation of text element extents.
- Fix #348 - Fix incorrect font sizing.
- Fix #349 - Don't panic when loading an external image that is bigger
  than Cairo's limits.
- Fix - rsvg_handle_get_dimensions_sub() no longer panics if passed a
  nonexistent fragment identifier.
- Fix 32-bit builds (Jordan Petridis).
- Fix #256 - Correctly match the systemLanguage attribute with the
  user's locale.
- Fix #320 - Parse xml:lang correctly.
- Fix #334 - Don't modify the caller's cairo_t state during rendering.
- Fix #349 - Don't panic if we get a "data:" URI with empty data.
- Fix #352 - Don't panic on getting a very large "order" for
  feConvolveMatrix (Ivan Molodetskikh).
- Fix #363 - Don't drop spaces around <tspan> elements.
- Fix #365 - rsvg-convert now uses pixel units for SVG output, instead
  of points.  This requires cairo 1.15.12 (Antonio Ospite).
- Fix #358, #366 - tweaks to have the test suite pass on i386 and
  non-x86_64 platforms (Simon McVittie, Federico Mena).
- Fix #368 - With RSVG_LOG=1, librsvg will now report when SVGs have
  references to nonexistent elements, to aid debugging (for example,
  in an xlink:href attribute).
- Fix #371 - rsvg-convert was positioning extracted elements
  incorrectly when using the -w/-h options together with --export-id.
- Fix #372 - Mis-rendering in small arc segments.
- Fix #373 - Rendering of gradients for horizontal/vertical stroked
  lines with gradientUnits="userSpaceOnUse".
- Fix a couple of memory leaks in the error paths of the GdkPixbuf
  loader (Benedikt Heine).
- Allow reference tests with a small difference to pass without
  breaking the build (Simon McVittie).
- CSS processing code is now in Rust, although it still calls libcroco
  to do the parsing.
- XML processing code is partially moved to Rust, although it still
  calls libxml2 for the XML parsing.
- Special thanks to Alex Crichton for ensuring that MacOS builds work.

Version 2.44.2
==============

- A *double brown paper bag* release!  Super strong, this one!
- Fix #325 - Don't leak all the elements at the toplevel.
- Fix #328 - Make masking work on big-endian.  Thanks to
  Simon McVittie for quick testing of this on s390x.
- Fix library ordering so -Wl,--as-needed works (Simon McVittie).

Version 2.44.1
==============

- This is a brown paper bag release!  Apologies for the bugs:
- Fix #324 - Don't panic if trying to render a non-empty SVG with no
  elements (Paolo Borelli).
- Fix #325 - All the elements were being leaked at the toplevel.
- Fix #326 - Fix COMPILING.md - librsvg 2.44 does not build on Ubuntu
  18.04 because it has a rust version that is too old.

Version 2.44.0
==============

- Librsvg now has minimal logging for debugging by setting the
  RSVG_LOG environment variable.  See CONTRIBUTING.md for details.
- Speed improvements for Gaussian blur, SRGB conversions, and various
  filters (Ivan Molodetskikh).
- Fix #264 - the letter-spacing property now supports "normal" in
  addition to lengths (Paolo Borelli).
- Fix #318 - the font-weight property was being parsed incorrectly.
- Fix #323 - don't use 100% "forever" with malicious SVGs that cause
  an exponential number of elements to be instanced through the
  <use> element.  We limit the number of <use> instances now.
- Fix #293 - Don't panic when masking an empty group.
- Fix #319 - Parse single font-family correctly.
- Cleanups for the internal representation of elliptical arcs in paths
  (letheed).

Version 2.43.4
==============

- This is an early release to test the effects of threading in librsvg.
- The lighting and Gaussian blur filters are now parallelized with
  Rayon; they will use all available cores (Ivan Molodetskikh).

Version 2.43.3
==============

- Tentative fix: #309 - Don't panic if rendering to a non-image Cairo surface.
- Optimizations for Gaussian blur and other filters (Ivan Molodetskikh).
- Optimizations in SRGB <-> linear RGB conversions (Ivan Molodetskikh).
- More C to Rust conversion in the loading code (Paolo Borelli).

Version 2.43.2
==============

- All the filter effects have been ported to Rust as part of Ivan
  Molodetskikh's Summer of Code project!!!!!
- We now require Rust 1.26.
- We now include Rust debug information even in release builds, to make it
  easier to obtain stack traces.
- Fix #310 - Respect DPI in the font-size property (Mike Marcacci).
- Fix: draw the circle/ellipse elements with the same orientation as
  the SVG 1.1 test suite; this is relevant for stroke-dasharray (Ivan
  Molodetskikh).
- Refactoring of the drawing code and font sizes (Paolo Borelli).
- New filters in Rust: feConvolveMatrix, feColorMatrix, feMorphology,
  feDisplacementMap, feGaussianBlur, feDistantLight, feSpotLight,
  fePointLight, feTile.
- Updated Rust dependencies (Igor Gnatenko).

Version 2.43.1
==============

- Fix: #259 - Test fonts should now work with --enable-installed-tests
- Fix: #277 - Don't panic when trying to filter an empty group
  (Ivan Molodetskikh).
- Fix: #292 - Don't panic if we try to clip an empty group.
- Fix the feOffset filter's coordinate parsing.  (Ivan Molodetskikh).
- Fix linearization of SRGB data in the feComposite filter (Ivan
  Molodetskikh).
- Fix CSS cascading in filters (Ivan Molodetskikh).
- Fix, don't render filters if they are in error (Ivan Molodetskikh).
- Fixed a couple of memory leaks in the test suite.
- Filters now support FillPaint and StrokePaint for input (Ivan Molodetskikh).
- Filters now support the color-interpolation-filters property
  (Ivan Molodetskikh).
- The feImage, feBlend, feComponentTransfer filters are now in Rust
  (Ivan Molodetskikh).
- The feOffset filter now supports fractional offsets (Ivan Molodetskikh).
- The drawing context code is now in Rust.  This is a tremendous
  amount of work! (Paolo Borelli)
- All the style property parsers have been moved to rust-cssparser.

Version 2.43.0
==============

- This is the start of the 2.43 development series, and you bet we are
  doing extensive changes!
- Many, many thanks to Jordan Petridis for keeping our Continuous
  Infrastructure updated all the time, and for emergency fixes when
  they were necessary.
- As part of the Summer of Code 2018, Ivan Molodetskikh is porting the
  filter effects to Rust.  These are done so far:  core filters
  infrastructure, feComposite, feMerge, feOffset.  The only remaining
  SVG elements done in C pertain to filter effects, and Ivan is taking
  care of them.
- The feComposite filter now operates in linear RGB space, for better
  spec compliance.  We transform back to SRGB for the final results
  (see issue #275 for pending work on fully supporting this
  elsewhere).
- Filters now compute their bounds to floating-point values, instead
  of clipping them to integers.
- The text, tspan, tref elements are now in Rust (Paolo Borelli).
- Text rendering should be better.  We now perform text measurement
  operations with the actual affine transformation that the text will
  use in the end.  This should give Pango/Freetype a better chance of
  doing scale-appropriate hinting.
- The basic styling infrastructure has moved to Rust (Paolo Borelli,
  Federico Mena).  We don't use the old and limited cascading code
  anymore.  We also audited which CSS properties are supposed to
  inherit automatically or not; this should be working per the SVG
  spec now.  Special thanks to Paolo for doing the largest part of the
  work in moving the style data to Rust; this was a painstaking,
  months-long process of constant refactoring.
- The internals of the drawing infrastructure and bounding-box
  computation are now done in Rust (Paolo Borelli).
- Element creation from the parsing stage is now in Rust (Saurav
  Sachidanand).
- Clipping and Masking are now done in Rust (Paolo Borelli).
- Our tests now include the full Adwaita icon theme, so it doesn't
  break.
- Fix: #241 - feDistantLight and feSpotLight now work again.
- Fix: #282 - feComposite is fixed not to overwrite the source image
  in some cases; this fixes drop shadows generated from Inkscape (Ivan
  Molodetskikh).
- We have the beginnings of Windows CI, courtesy of Guillaume Gomez.
- Changes from 2.42.4:
- Fix: elements with systemLanguage attributes without variants now
  work better (Paolo Borelli).
- gitlab#227 - Fix: we now catch negative values in stroke-dasharray
  properties instead of leaving the cairo_t in an error state.
- gitlab#228 - Fix: empty transform attribute now correctly yields an
  identity transform (Dmitry Kontsevoy).
- Fix possible crash on invalid gradient references.
- Make robust against patterns and gradients with no children.
- Lots of code cleanups and refactorings (Jordan Petridis, Dmitry
  Kontsevoy, Ivan Molodetskikh).
- Code moved to Rust: low-level path and PangoLayout drawing, "switch"
  element (Paolo Borelli).
- New Rust dependencies: float_cmp, pangocairo crates.
- We now require Rust 1.21 or later.
- Changes from 2.42.5:
- #276 - rsvg_handle_render_cairo() will now refuse to render if the
  cairo_t passed to it is in an error state.  Fixes a panic in the
  cairo-dock program.
- #206 - The test suite now writes test artifacts to tests/output
  instead of /tmp (Saurav Sachidanand).

Version 2.42.3
==============

- gitlab#205 - The configure script now checks for Rust 1.20.0 -
  previously this minimum requirement was not well-defined.
- gitlab#204 - New feature: If an SVG has an <a> link element, we now
  generate the corresponding link when outputting to a Cairo PDF
  surface.  If you use rsvg-convert(1) with PDF output, <a> links in
  the SVG will work in the PDF (Dmitry Kontsevoy).
- gitlab#108 - New feature: support font-size:larger and font-size:smaller
  relative sizes.
- New feature: rsvg-convert now supports SOURCE_DATE_EPOCH per
  https://reproducible-builds.org/specs/source-date-epoch/ to generate
  reproducible output for PDFs (Chris Lamb).
- gitlab#197 - New requirement: We now require Freetype2 2.9.0, which
  fixes font rendering bugs.  The test reference PNGs have been
  regenerated with this version.  You may see changes in font
  rendering as a result of this updated requirement.
- gitlab#91 - Fix rendering of masks and clips when the initial
  transformation has a translation component (Massimo).
- gitlab#112 - Fix: apply style attributes for all SVG elements, not just
  for the toplevel one.
- gitlab#161 - Fix the marker angle for the last vertex of closed
  paths (Juraj Fiala).
- gitlab#198 - Fix: Make rsvg_pixbuf_from_file() and its derived
  functions work again.  Now we have tests for the whole public API.
- gitlab#143 - Minor optimizations for Gaussian blurs (Timm Bäder).
- gitlab#201 - Minor speedups in the code to parse SVG attributes.
- gitlab#178 - Fix some tests that failed on 32-bit machines.
- COMPILING.md now lists our build dependencies, and has command lines
  which you can use on openSUSE/Fedora/Debian/Ubuntu/MacOS to set up a
  development environment for librsvg (Jordan Petridis, Brion Vibber).
- gitlab#211 - Running the configure script on MacOS now works and
  doesn't try to use -Bsymbolic for linking (Brion Vibber).
- gitlab#In addtion to --enable-debug/--disable-debug to control the Rust
  compilation, now you can use an environment variable
  LIBRSVG_DEBUG=yes / LIBRSVG_DEBUG=no if you wish.
- Code moved to Rust: SVG paint servers (Dmitry Kontsevoy), SVG
  attribute parsing.
- We now use a Cargo workspace internally, to move more things to Rust
  (Jordan Petridis, Chun-wei Fan).
- Special thanks in this release to Jordan Petridis for MAJOR
  improvements to our Continuous Integration infrastructure, the
  repository structure, and updates to the compilation documentation.
  Also for setting up 32-bit builds for continuous integration.

Version 2.42.2
==============

- gitlab#193 - Don't crash when feConvolveMatrix doesn't specify
  orderx/ordery attributes.
- gitlab#136 - Parse stroke-dasharray property correctly.  This code
  is in Rust now; yay! (Jordan Petridis).
- Don't render markers if they are zero-sized, per the spec.
- Performance: eliminate a bunch of string copies during parsing.
- Update rust-cssparser to 0.23 (Igor Gnatenko).

Version 2.42.1
==============

- gitlab#182 - Parse the transform attribute in a faster/simpler way.
  We now use rust-cssparser instead of lalrpop.  This is especially
  noticeable on SVGs with lots of "transform" attributes.
- gitlab#187 - Don't crash when setting a gradient on a zero-sized object.
- gitlab#184 - (Windows) Don't use PangoFT2 if not available (Chun-wei Fan).
- gitlab#181 - Inherit attributes in the <svg> element properly.
- gitlab#160 - rsvg-convert - fix error reporting when reading from
  stdin (Phlip Withnall).
- gitlab#152 - Fix detection of image type in "data:" URIs when they
  don't specify a MIME type (Andreas Smas).
- gitlab#117 - (Windows) rsvg-convert - Set stdout to O_BINARY
  (Bakhtiar Hasmanan).
- gitlab#133 - More stringent parsing of path data; better tests.
- (Windows/MSVC) Fix linking of the Rust internals library (Chun-wei Fan).
- Fix typos and links in the .md files.

Version 2.42.0
==============

- Fix a memory leak in rsvg_handle_new_from_file() (Lovell Fuller).
- Optimize the xml:space normalization function (Jordan Petridis).
- gitlab#179 - fix a runtime warning in the feMergeNode code.
- gitlab#175 - Clarify documentation about the rsvg_*_sub() APIs.
- Stylistic fixes from cargo-clippy (Jordan Petridis).
- Port the Pango glue code to Rust.
- New ARCHITECTURE.md with a description of librsvg's internals.

Version 2.41.2
==============

- We now require glib 2.52.0.
- bgo#787895 - Fix mis-use of libxml2.  Thanks to Nick Wellnhofer for
  the guidance.
- bgo#761175 - Allow masks and clips to reuse a node being drawn.
- Fix xml:space normalization, per the spec.
- Don't access the file system when deciding whether to load a remote
  file with a UNC path for a paint server (i.e. don't try to load it at all).
- We now support cross-compilation of the Rust code (David Michael).
  See COMPILING.md for details.
- Fixed bugs from Coverity runs (Philip Withnall).
- Vastly improved README.md, new COMPILING.md, improved CONTRIBUTING.md.
- Markers now have the correct default size per the SVG spec.
- Visual Studio: We now use HIGHENTROPYVA linker option on x64 builds,
  to enhance the security of built binaries (Chun-wei Fan).
- Cargo is now verbose as well if you use "make V=1".
- Fixed some memory leaks.
- Don't render elements that establish a viewport if their viewBox
  size is 0, per the spec.
- SVG elements ported to Rust: image, clipPath, mask, character data in elements.
- Fixed loading files one byte at a time.
- Reference documentation is now DocBook 5.1.
- Reference docs now have an overview of the library.
- Distribute README.md in the tarball properly.
- Expanded the test suite.
- Lots of internal refactoring.

Version 2.41.1
==============

- The feConvolveMatrix filter primitive wasn't being rendered at all;
  now it works.
- Pattern specifications can now have a fallback color, per the spec -
  https://www.w3.org/TR/SVG/painting.html#SpecifyingPaint
- Tests now use a very basic form of reproducible font rendering.
  This means that "make check" should pass even if you have a custom
  Fontconfig setup.
- Fixed recursive fallbacks in gradients.
- Per the spec, we now don't render elements which have invalid
  attributes.
- Windows build: support building with Fontconfig; support Visual
  Studio 2017; generate .pc files upon install (Chun-wei Fan)
- Fixed bgo#621088 - Text elements can now be used as clipping paths.
- Fixed bgo#587721 - Fix rendering of text elements with transformations
  (Massimo)
- Fixed bgo#776932 - Don't crash on elements with an invalid
  "transform" attribute.
- Fixed bgo#777155 - Ignore patterns that have close-to-zero dimensions.
- Fixed bgo#776297 - Don't render markers for rect / circle elements;
  fix marker angles in some cases (Massimo).
- Fixed bgo#777834 - Don't crash when rendering text with empty
  extents.
- Fixed bgo#634324 - Gaussian blur with negative-scaling
  transformation was being rendered incorrectly.
- Fixed bgo#783835 - Don't divide by zero in Gaussian blurs
- Fixed division by zero in feTile filter when the input surface is
  empty.
- Fixed bgo#779489 - Link to pangoft2 as required.
- Don't crash in filters when one of them yields an invalid surface
  for an intermediate result.
- Update for bgo#778666 - Use our own thumbnailer specification file
  (Jeremy Bicha).
- Fixed bgo#782098 - Don't pass deprecated options to gtkdoc-scangobj
  (Ting-Wei Lan).
- Fixed bgo#777833 - Various memory leaks (Philip Withnall, Federico Mena)
- Fixed bgo#786372 - Use the correct default for the style element's
  "type" attribute.
- Fixed bgo#634514 - Don't render unknown elements and their children.
- Fix parsing of "azimuth", "elevation", "limitingConeAngle" for
  filter effects.
- Fixed bgo#785276 - Don't crash on empty or single-byte files.
- Made the <switch> element work; SVG feature names were being
  tested incorrectly.
- Fixed a few cases of uninitialized struct fields.
- Code converted to Rust: preserveAspectRatio attribute, viewBox
  attribute, core Node structure, path/line/rect/circle/ellipse basic
  shapes, group/defs/switch/svg/use/symbol structural elements, pattern element
  and pattern fallbacks, marker rendering, various parsers, error
  propagation from parsers, gradient stops, gradient element, 
- Added a bunch of new test cases for the new features and the
  code converted to Rust.
- We now require cairo-rs 0.2.0
- We now require lalrpop 0.13.1
- The librsvg tarball now comes with the Rust dependencies embedded
  using "cargo vendor".  Linux distros can replace these dependencies
  with their own versions using the infrastructure described in
  http://doc.crates.io/source-replacement.html

Version 2.41.0
==============

- The big news is that parts of librsvg are now implemented in the
  Rust programming language, instead of C.  The public API remains
  identical.  Rust should provide us with memory safety and nicer
  built-in abstractions for the code, as well as an easier way to do
  unit tests.  Special thanks to all the people who sent tips on Rust
  idioms, and to Sebastian Dröge and Hubert Figuière for the Automake bits.
- Added an "--enable-debug" option to configure.ac - this will tell
  the Rust compiler to generate debugging code, instead of working in
  release mode.  Note that you must still pass CFLAGS by hand by the
  regular means for the C code.
- For Windows builds, only MSVC 2012 and upward are supported now.
- Chun-wei Fan made it possible to regenerate the MSVC project files
  when Makefile.am changes.
- Fixed bgo#763386 - handle curveto segments where three control
  points are coincident.  Thanks to Massimo for the detailed test cases.
- Fixed bgo#603550 - Compute the luminance correctly when generating a
  mask.  Thanks to Mike Lewis for the patch.
- Fixed bgo#776297 - Only render markers in path, line, polygon,
  polyline elements.
- Fixed feImage filters when they reference SVG nodes; they were
  translated incorrectly.
- Fixed feComponentTransferFunction when there are duplicated feFuncX
  elements.
- Fixed bgo#761871 - handle reflection points for quadratic and cubic
  curves correctly.
- Fixed bgo#686953 - support the "marker" shorthand property.
- Fixed a few minor issues pointed out by Coverity.
- The path data parser now handles boolean values in Arc elements correctly.
- Fixed conformance bugs in gradient inheritance.
- Radial gradients now adjust the focus point correctly to be within
  the gradient's radius.
- Stroke width normalization is now conformant to the spec.
- Viewport-relative length normalization is now conformant to the spec.
- Added some of the official SVG 1.1 test files to our test suite.  Fixed
  a little bunch of conformance bugs.
- As a small optimization, we only push/pop CSS states when rendering
  will actually happen, instead of for all (potentially invisible) nodes.
- Code that has been converted to Rust:  marker orientations and
  rendering, path data parser, path building, length normalization,
  gradient inheritance, bounding boxes with affine transformations.
- Lots of refactoring to accomodate the Rust code, and general cleanups as well.
- Added tests/README.md with instructions on how to run the test suite
  and update it.
  rsvg-test can now skip files or directories that start with "ignore".
- Improved the README.

Version 2.40.16
===============

- Chun-wei Fan added support for building the introspection files under MSVC.
- Fixed bgo#760262 - Make the zooming options in rsvg-convert(1) work again
  for scaling the resulting image.  Fix by Menner <mix@gmx.org>.
- Fixed bgo#764808 - Wikipedia generates equations as SVGs and renders
  them, but uses fill="currentColor".  Since we don't let callers
  specify a starting state for CSS, we need to start with opaque black
  as the default current color.
- Added documentation for how to replace the deprecated
  rsvg_handle_set_size_callback().

Version 2.40.15
===============

- Apologies for the lack of 2.40.14.  I mistakenly tagged the
  repository before updating the NEWS file.
- librsvg now uses the Contributor Covenant Code of Conduct, 
  version 1.4, to which all contributors and maintainers are expected
  to abide. Please see the code_of_conduct.md file for details.
- Chun-wei Fan fixed builds on Visual Studio pre-2012.
- Fixed bgo#759084 - Don't crash when filters don't actually exist
  Fix by Benjamin Otte.
- Javier Jardón updated our autogen.sh to use modern autotools.
- Fixed bgo#761728 - Memory leak in the PrimitiveComponentTransfer
  filter.  Fix by Ron Hopper.

Version 2.40.13
===============

- Chun-wei Fan and Paolo Borelli fixed the Windows build.
- Menner added basic support for the "baseline-shift" attribute in
  text objects.  We support "sub", "super", and plain "baseline", so
  you can at least have subscripts and superscripts for formulas and
  such.  There is no support for percentages or explicit lengths yet.
  bgo#340047.
- Menner fixed some duplicate logic when rendering paths, which would
  try to decide whether to create intermediate surfaces.  Now we have
  a single place where this is done.  This fixes inconsistent text
  spacing in some situations.  bgo#749415.
- Rewrote the markers engine, for bgo#685906 and bgo#760180 - Our
  machinery for rendering SVG markers (like arrowheads and such)
  didn't handle several cases correctly.  Curves with coincident
  control points produced incorrect orientations for the markers, as
  did multiple contiguous zero-length segments.  We follow the spec
  for this now and handle things correctly.  Also, markers didn't
  render in the correct position if they had the viewBox attribute
  set.

Version 2.40.12
===============

- Benjamin Otte did *great* work in refactoring the test harness to
  use Glib's gtest infrastructure, instead of using home-grown
  machinery.  Tests can simply be put as SVG files in the tests/
  subdirectories; it is not necessary to list them explicitly in some
  text file.  Yay!
- Gzipped SVGs now work if read from streams.
- References to objects/filters/URIs/etc. are now handled lazily.
  Also, there is a general-purpose cycle detector so malformed SVGs
  don't cause infinite loops.  Work by Benjamin Otte.
- Removed parsing of Adobe blend modes; they were not implemented, anyway.
- Fixed bgo#700911 - feComponentTransfer filter functions did not work at all.
- Fixed bgo#630732 - out-of-bounds read in feComponentTransfer with tableValues.
- Fixed bgo#677068 - incorrect reflection points in paths.

Version 2.40.11
===============

- Add project files for building on Visual Studio (bgo#753555).  Work
  by Chun-wei Fan.
- Added an "--export-id" option to rsvg-convert(1).  This lets you
  select a single object to export, for example, to pick out a group
  from a multi-part drawing.  Note that this is mostly useful for PNG
  output right now; for SVG output we don't preserve many attributes
  which could be useful in the extracted version.  Doing this properly
  requires an internal "output to SVG" backend instead of just telling
  Cairo to render to SVG.

Version 2.40.10
===============

- Fixed bgo#748608 - Memory leak when Gaussian-blurring.  Thanks to
  Carlos Garnacho for fixing my leaky code.
- Fixed bgo#739329 - font-family attributes with singly-quoted names
  were not handled correctly, yielding incorrect fonts.  Patch by Menner.
- Fixed bgo#476507 - Path start/end markers didn't have the correct angle
  if the path was a curve with coincident control points.  Patch by Menner.
- Fixed bgo#688689 - Support font-style="normal" within a non-normal
  styled text block.  Patch by Paolo.
- Fixed builddir != srcdir.  Patch by Matthias Clasen.
- Remove a bunch of deprecated GTK+ calls.
- This version of librsvg requires GTK+ 3.10.0, which is, ahem, only two years old.
  We previously required a version of GTK+ which is four years old.  Out with
  the old, in with the old.

Version 2.40.9
==============

- Fixed bgo#738367 - V/v/H/h commands in path elements were not
  working.  Patch by Andrea Griffini.
- Fixed bgo#605875 - Gaussian-blurred objects were sometimes missing.
  Based on a patch by Eduard Braun.
- Fixed bgo#710163 - use _wfullpath() on Windows when canonicalizing
  filenames.  Patch by LRN.

Version 2.40.8
==============

- Bugs fixed from fuzz testing: #744688 - possible double g_free()
  when processing stroke-dasharray
- Optimize rendering of polylines, lines, rectangles, circles, and
  ellipses.  These should be marginally faster, marginally more
  precise, and should put less pressure on the memory allocator.

Version 2.40.7
==============

- Bugs fixed from fuzz testing: #703102, #738050, #738169, #744270, #744299
- Fixed unfiled bug from fuzz testing, where the convolution filter
  had an integer multiplication overflow.
- Fix build of rsvg-convert on Windows.
- Fix a bunch of compiler warnings.

Version 2.40.6
==============

- MinGW build fixes
- Fix path data number parsing
- Fix build with newer libtool

Version 2.40.1
==============

- Build fixes
- Crash fixes

Version 2.40
============

- Add support for parsing rgba() colours and improve colour parsing
- rsvg-filter: Fix memory leak
- Remove support for GTK+2 along with the old theme engine
- Require at least version 3.2 of GTK+
- Remove support for old versions of gdk-pixbuf
- Add eps support to rsvg-convert

Version 2.39.0
- don't load resources from the net (#691708, CVE-2013-1881)

Version 2.37.0

- bump pango requirement to 1.32.6
- mark pixbuf loader as threadsafe

Version 2.36.4
==============

- build fixes

Version 2.36.3
==============

- build fixes

Version 2.36.2
==============

- GObject introspection fixes (#677674)
- added Vala bindings (#677676)
- deprecate including headers apart from rsvg.h directly
- build fixes (#677661)

Version 2.36.1
==============

- Bugs fixed: #672725, #672791, #672885, #673748

Version 2.36.0
==============

- Bugs fixed: #669563, #672414, #672792, #672725

Version 2.35.2
==============

_ Bug fixes

Version 2.35.1
==============

- Bugs fixed: #664684, #664533, #665905, #665824

Version 2.35.0
==============

This version contains many bug fixes and improvements.
- Update libxml2 dependency to 2.7.0
- Make GIO and libcroco hard requirements
- Use attributes for deprecations instead of defines
- Many memory leaks plugged
- Bugs fixed: #621636, #630112, #624820, #624835, #581108, #614157, 
              #630733, #473862, #590788, #626559, #663049, #663049

Version 2.34.1
==============

This version contains fixes for CVE-2011-3146.

- cairo: reduce cost of measuring bounding boxes
- Use "const" instead G_CONST_RETURN (#652213)
- Call xmlFreeParserCtxt after using the context (#655472)
- Store node type separately in RsvgNode (#658014)

Version 2.34.0
==============

- Allow building rsvg-view with gtk 3

Bugs fixed:
  629392 - stroke-dasharray is inherited by sibling texts
  626802 - NULL-ptr crash in g_str_equal in rsvg-styles.c
  641586 - configure should honour aclocal flags ACLOCAL_FLAGS 
  640336 - Typo in rsvg.1
  635214 - svgz never supported by pixbuf loader
  FDO 30071 - crash when rendering this svg with librsvg to a pdf or ps or recording surface
  629412 - Missing test files on librsvg-2.31.0
  630714 - fix stray comma 

Version 2.32.1
==============

Version 2.32.0
==============

Version 2.31.0
==============

- buildable against standalone gdk-pixbuf
- require GIO

- Bug fixed:

  337979 text size not rendered correctly
  545158 Segfault or bad rendering when displaying a SVG file
  614556 Background pixbuf in filter process should not be created if it's not needed.
  616018 cairo-freetype font rendering code should be removed.
  620130 Decode base64 inplace
  620238 crashes rending a trivial file
  620592 Use correct free func
  620649 Simplify code by using g_clear_error
  620693 presentation attributes in svg element are ignored
  620822 Build fails with make -jx"
  621699 make librsvg gio friendly
  622790 use standalone gdk-pixbuf
  623383 [PATCH] crash on rsvg-gobject.c:141, in instance_dispose function

Version 2.26.3
==============

- Bug fixed:
  143300 wrong bounding box when importing SVG
  403274 text in thumbnail too large
  404976 Unicode decomposed chars are not rendered well
  524690 text alignment incorrect with text-anchor:end
  545158 Segfault or bad rendering when displaying a SVG file
  563933 corrupted rendering of a card in 'Paris' aisleriot card theme
  564527 rsvg_handle_get_dimensions_sub weird behaviour
  564544 shape-rendering crispEdges property is antialiasing line elements
  566433 Could not  read  a valid svg file (inkspace read it)
  579286 This SVG-File crashes nautilus/rsvg-view
  581491 rsvg rendering is broken when encounters a 0px styled <text>
  589612 EOG error when loading a large SVG
  592207 Object cannot be rendered with more than 1 CSS {} rule
  597873 glib-mkenums cannot be invoked when GLib is uninstalled.
  597988 incorrect pkgconfig file let's others fail to detect librsvg
  598151 Incorrect rendering of svg file
  608575 Hang on particular SVG input
  612951 SVG not rendered if header contains width or height in percentage
  614123 librsvg builds tests even if tests are not run
  614555 should remove unused rsvg_filter_adobe_blend function
  614566 Needless G_OBJECT macro should be removed.
  614606 !important is not respected
  614643 does not handle comma separated CSS selector without libcroco
  614703 Need tests for get_dimensions.
  614704 css style doesn't override presentation attributes
  614730 Rendering not disabled for 0 sized objects
  614866 tests for CSS handling
  615490 rsvg-view should scale image size by default if the image has huge canvas.
  615699 rsvg-view should show zoom ratio.
  615701 class directive in svg element is not used at all
  615715 .class#id type selector is not supported.
  616187 rsvg-view crashes when open a svg image
  616835 Fix linking with pedantic linkers
  617163 !important directive support without libcroco.

Version 2.26.2
==============

- Enable silent build by default
- Use GDK_DRAWABLE() instead of non-existing GTK_DRAWABLE()
- Fix compiler warnings

Version 2.26.1
==============

- Remove some deprecated gtk+/gdk functions and bump gtk+ requirement
- Fix linking with pedantic linkers
- Make librsvg compile with -DGSEAL_ENABLED
- Drop mozilla plugin

Version 2.22.3
==============

- Make librsvg buildable on windows without freetype dependency
- Fix build and tests
- Fix the handling of the stroke-dashoffset property
- Bad rendering for some wacko SVG path data
- Fix some arcs not being well drawn in SVG path
- Fix various crashes
- Set license field of GdkPixBufFormat in the SVG loader
- Migrate use of GMemChunk to GSlice
- If an ID is not found, return an error and/or do not render whole SVG

Version 2.22.2
==============

- Fix rsvg-convert crash in  _rsvg_acquire_xlink_href_resource()

Version 2.22.1
==============

- If a moveto is followed by multiple pairs of coordinates, the subsequent
  pairs are treated as implicit lineto commands.
- Handle display:none
- Handle xml:space="preserve"

Version 2.22.0
==============

- Text rendering fixes (especially for non-ASCII text)
- Better cairo integration (fixed bugs when rendering inside of cairo groups)
- Migrate from GVFS to GIO
- Migrate rsvg-view from GnomePrint to GtkPrint

Version 2.18.0
==============

- Support for some of the SVG 1.2 comp-op properties
- Less-blurry output
- Marker orientation is fixed
- Support compressed SVG from the GdkPixbuf plugin
- CSS fixes
- Better support for SVG conditionals
- Support for SVG 1.1's shape-rendering and text-rendering properties

Version 2.7.x
=============

- Filters
- Hugely improved text support
- GTK+ theme engine
- Masks
- Path markers
- Mozilla plugin
- Patterns
- Standalone viewer
- Decent <image> support
- Numerous bug fixes

== gnome-2-6 branch ==

Version 2.6.x
=============

- A slew of bug fixes

Version 2.5.x
=============

- A slew of bug fixes

Version 2.4.0
=============

- gdk-pixbuf loader fix
- support for title and desc elements

Version 2.3.0
=============

- Support Paths inside of <defs> and use them in <use/> statements
- Vastly improved CSS support (libcroco and builtin)
- LibCroco CSS support
- Improved support for Gimp plugin

== gnome-2-2 branch ==

Version 2.2.5
=============

- Gimp Import/Load plugin
- Bugfixes

Version 2.2.4
=============

- Few bugfixes

Version 2.2.3
=============

- All sorts of build and installation fixes
- "Bugfix" for a regression in GTK+'s pixbuf-io loader scheme

Version 2.2.2
=============

- Handle SVGZ files (Gzipped SVG)
- Handle objectBoundingBox coordinates on gradients
- API extensions
- Various bugfixes and crash fixes

Version 2.2.1
=============

- Better CSS style handling
- Fix for a crasher exploited by Sodipodi. Improved gradient handling
- RSVG allows you to specify the JPEG quality
- Handles Kontour's conicalGradients to some extent

Version 2.2.0
=============

- Better CSS style handling
- Works in "international" locales (non-C locales) in a threadsafe manner
- Handle rounded rects
- Add manpage
- Handle "ex" units better

Version 2.1.5
=============

- Missed files in tarball added. 

Version 2.1.4
=============

- Fixed memory leak

- Better handling of display and visibility styles.

Version 2.1.3
=============

- Better font handling (stretch, style, variant, weight supported, inherited)

Version 2.1.2
=============

- Fixed crash with non-utf8 characters.

Version 2.1.1
=============

- Better ellipse rendering

- New functions to set rendering DPI

- Better handling of SVGs colors, gradients and images

- support dashes

- Includes a new gdk-pixbuf svg loader

- Includes a new gimp svg loader

- Now installs a binary that can rasterize svgs.

Version 2.1.0
=============

- New co-maintainer Dom Lachowicz

- Added new svg based gtk+ engine

- Handles SVGs a lot better. Especially with regards to units.

Version 2.0.1
=============

- Fixed misrender of the ScalableGorilla trashcan.

Version 2.0.0
=============

- Fixed crash when trying to open empty files (Anders)

Version 1.1.6
=============

- Fix division by zero crashes when scaling an .svg without dimensions (Darin)

Version 1.1.5
=============

- Add another new function for use in Nautilus icons (Alex Larsson)

Version 1.1.4
=============

- Add new function for auto-scaling svg for use in Nautilus icons (Alex Larsson)
- Fix xml parsing to read much faster and work around libxml bug (Michael Meeks)
- Add back handling for .svg file without width and height (Darin)
- Improved internal error handling (Darin)

Version 1.1.3
=============

- Fix major storage leaks (Anders)

Version 1.1.2
=============

- Use the new intersector (Alex)

Version 1.1.1
=============

- Ported to Gnome 2 (Ramiro, Darin)
- Ported to Pango instead of using FreeType directly (Darin)
- Replace FILE-based API with data pushing API (Jonathan)

- Use pkgconfig rather than config scripts or .m4 files (Ramiro)
- Use stuff from libart rather than having our own copies (Darin)
- Move art_render_mask into libart (Darin)
- Use locale-independent functions for parsing the XML files,
  not <ctype.h>, which is locale dependent (Darin)
- Other misc. API cleanups (Jonathan).
- Fix NULL-dereference bugs in gradient code (Darin)
- Fix handling of empty .svg files (Darin)
- Measures strings propertly taking affine into account (Darin)
- Fix empty struct portability problem (Darin)
- Other build fixes (Owen, Jacob, Anders)
