# opentype-rs [![Build Status](https://travis-ci.org/fveilly/opentype-rs.svg?branch=master)](https://github.com/fveilly/opentype-rs)
Safe, fast and memory efficient OpenType font file parser

## Overview
[overview]: #overview

The OpenType font format is an extension of the TrueType font format, adding support for PostScript font data. The OpenType font format was developed jointly by Microsoft and Adobe. OpenType fonts and the operating system services which support OpenType fonts provide users with a simple way to install and use fonts, whether the fonts contain TrueType outlines or CFF (PostScript) outlines.

See [OpenType® specification](https://docs.microsoft.com/en-gb/typography/opentype/spec/) for details.

## Example
[example]: #example

```rust
#[macro_use]
extern crate opentype-rs;

#[test]
fn parse_otff() {
    // TODO
}
```

## Support
[support]: #support

<aside class="warning">
opentype-rs is in early stage and the APIs are far from stable.
</aside>

#### Required Tables

- [ ] **cmap**: Character to glyph mapping
- [x] **head**: Font header
- [x] **hhea**: Horizontal header
- [ ] **hmtx**: Horizontal metrics
- [ ] **maxp**: Maximum profile
- [ ] **name**: Naming table
- [ ] **OS/2**: OS/2 and Windows specific metrics
- [ ] **post**: PostScript information

#### Tables Related to TrueType Outlines

- [ ] **cvt**: Control Value Table (optional table)
- [ ] **fpgm**: Font program (optional table)
- [ ] **glyf**: Glyph data
- [ ] **loca**: Index to location
- [ ] **prep**: CVT Program (optional table)
- [ ] **gasp**: Grid-fitting/Scan-conversion (optional table)

#### Tables Related to CFF Outlines

- [ ] **CFF**: Compact Font Format 1.0
- [ ] **CFF2**: Compact Font Format 2.0
- [ ] **VORG**: Vertical Origin (optional table)

#### Table Related to SVG Outlines

- [ ] **SVG**: The SVG (Scalable Vector Graphics) table

#### Tables Related to Bitmap Glyphs

- [ ] **EBDT**: Embedded bitmap data
- [ ] **EBLC**: Embedded bitmap location data
- [ ] **EBSC**: Embedded bitmap scaling data
- [ ] **CBDT**: Color bitmap data
- [ ] **CBLC**: Color bitmap location data
- [ ] **sbix**: Standard bitmap graphics

#### Advanced Typographic Tables

- [ ] **BASE**: Baseline data
- [ ] **GDEF**: Glyph definition data
- [ ] **GPOS**: Glyph positioning data
- [ ] **GSUB**: Glyph substitution data
- [ ] **JSTF**: Justification data
- [ ] **MATH**: Math layout data

#### Tables used for OpenType Font Variations

- [ ] **avar**: Axis variations
- [ ] **cvar**: CVT variations (TrueType outlines only)
- [ ] **fvar**: Font variations
- [ ] **gvar**: Glyph variations (TrueType outlines only)
- [ ] **HVAR**: Horizontal metrics variations
- [ ] **MVAR**: Metrics variations
- [ ] **STAT**: Style attributes (required for variable fonts, optional for non-variable fonts)
- [ ] **VVAR**: Vertical metrics variations

#### Tables Related to Color Fonts

- [ ] **COLR**: Color table
- [ ] **CPAL**: Color palette table
- [ ] **CBDT**: Color bitmap data
- [ ] **CBLC**: Color bitmap location data
- [ ] **sbix**: Standard bitmap graphics
- [ ] **SVG**: The SVG (Scalable Vector Graphics) table

#### Other OpenType Tables

- [ ] **DSIG**: Digital signature
- [ ] **hdmx**: Horizontal device metrics
- [ ] **kern**: Kerning
- [ ] **LTSH**: Linear threshold data
- [ ] **MERG**: Merge
- [ ] **meta**: Metadata
- [ ] **STAT**: Style attributes
- [ ] **PCLT**: PCL 5 data
- [ ] **VDMX**: Vertical device metrics
- [ ] **vhea**: Vertical Metrics header
- [ ] **vmtx**: Vertical Metrics

## License
[license]: #license

opentype-rs is distributed under the terms of the MIT license.

See [LICENSE-MIT](LICENSE-MIT) for details.