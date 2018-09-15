//! The following data types are used in the OpenType font file. All OpenType fonts use
//! Motorola-style byte ordering (Big Endian).
//!
//! https://docs.microsoft.com/en-gb/typography/opentype/spec/otff

use std::{fmt, str};

/// Short offset to a table, same as uint16, NULL offset = 0x0000
pub type Offset16 = u16;

/// Long offset to a table, same as uint32, NULL offset = 0x00000000
pub type Offset32 = u32;

pub type LongDateTime = i64;

pub type Fixed = i32;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TableTag {
    /// Axis Variations Table
    Avar,
    /// Baseline table
    Base,
    /// Color Bitmap Data Table
    Cbdt,
    /// Color Bitmap Location Table
    Cblc,
    /// Compact Font Format table
    Cff,
    /// Compact Font Format (CFF) Version 2
    Cff2,
    /// Character to Glyph Index Mapping Table
    Cmap,
    /// Color Table
    Colr,
    /// Color Palette Table
    Cpal,
    /// CVT Variations Table
    Cvar,
    /// Control Value Table
    Cvt,
    /// Digital Signature Table
    Dsig,
    /// Embedded Bitmap Data Table
    Ebdt,
    /// Embedded Bitmap Location Table
    Eblc,
    /// Embedded Bitmap Scaling Table
    Ebsc,
    /// Font Program
    Fpgm,
    /// Font Variations Table
    Fvar,
    /// Grid-fitting and Scan-conversion Procedure Table
    Gasp,
    /// Glyph Definition Table
    Gdef,
    /// Glyph Data
    Glyf,
    /// Glyph Positioning Table
    Gpos,
    /// Glyph Substitution Table
    Gsub,
    /// Glyph Variations Table
    Gvar,
    /// Horizontal Device Metrics
    Hdmx,
    /// Font Header Table (See ['head'](tables::head::Head) table)
    Head,
    /// Horizontal Header Table
    Hhea,
    /// Horizontal Metrics Table
    Hmtx,
    /// Horizontal Metrics Variations Table
    Hvar,
    /// Justification Table
    Jstf,
    /// Kerning
    Kern,
    /// Index to Location
    Loca,
    /// Linear Threshold
    Ltsh,
    /// The mathematical typesetting table
    Math,
    /// Maximum Profile
    Maxp,
    /// Merge Table
    Merg,
    /// Metadata Table
    Meta,
    /// Metrics Variations Table
    Mvar,
    /// Naming Table
    Name,
    /// OS/2 and Windows Metrics Table
    Os2,
    /// PCL 5 Table
    Pclt,
    /// PostScript Table
    Post,
    /// Control Value Program
    Prep,
    /// Standard Bitmap Graphics Table
    Sbix,
    /// Style Attributes Table
    Stat,
    /// The SVG (Scalable Vector Graphics) table
    Svg,
    /// Vertical Device Metrics
    Vdmx,
    /// Vertical Header Table
    Vhea,
    /// Vertical Metrics Table
    Vmtx,
    /// Vertical Origin Table
    Vorg,
    /// Vertical Metrics Variations Table
    Vvar
}

impl TableTag {
    /// Converts a Tag into Option<TableTag>.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::{Tag, TableTag};
    ///
    /// let tag = Tag::new(b"head");
    /// let table_tag = TableTag::parse(tag).unwrap();
    ///
    /// assert_eq!(table_tag, TableTag::Head);
    /// ```
    pub fn parse(tag: Tag) -> Option<TableTag> {
        match &tag.0 {
            b"avar" => Some(TableTag::Avar),
            b"BASE" => Some(TableTag::Base),
            b"CBDT" => Some(TableTag::Cbdt),
            b"CBLC" => Some(TableTag::Cblc),
            b"CFF " => Some(TableTag::Cff),
            b"CFF2" => Some(TableTag::Cff2),
            b"cmap" => Some(TableTag::Cmap),
            b"COLR" => Some(TableTag::Colr),
            b"CPAL" => Some(TableTag::Cpal),
            b"cvar" => Some(TableTag::Cvar),
            b"cvt " => Some(TableTag::Cvt),
            b"DSIG" => Some(TableTag::Dsig),
            b"EBDT" => Some(TableTag::Ebdt),
            b"EBLC" => Some(TableTag::Eblc),
            b"EBSC" => Some(TableTag::Ebsc),
            b"fpgm" => Some(TableTag::Fpgm),
            b"fvar" => Some(TableTag::Fvar),
            b"gasp" => Some(TableTag::Gasp),
            b"GDEF" => Some(TableTag::Gdef),
            b"glyf" => Some(TableTag::Glyf),
            b"GPOS" => Some(TableTag::Gpos),
            b"GSUB" => Some(TableTag::Gsub),
            b"gvar" => Some(TableTag::Gvar),
            b"hdmx" => Some(TableTag::Hdmx),
            b"head" => Some(TableTag::Head),
            b"hhea" => Some(TableTag::Hhea),
            b"hmtx" => Some(TableTag::Hmtx),
            b"HVAR" => Some(TableTag::Hvar),
            b"JSTF" => Some(TableTag::Jstf),
            b"kern" => Some(TableTag::Kern),
            b"loca" => Some(TableTag::Loca),
            b"LTSH" => Some(TableTag::Ltsh),
            b"MATH" => Some(TableTag::Math),
            b"maxp" => Some(TableTag::Maxp),
            b"MERG" => Some(TableTag::Merg),
            b"meta" => Some(TableTag::Meta),
            b"MVAR" => Some(TableTag::Mvar),
            b"name" => Some(TableTag::Name),
            b"OS/2" => Some(TableTag::Os2),
            b"PCLT" => Some(TableTag::Pclt),
            b"post" => Some(TableTag::Post),
            b"prep" => Some(TableTag::Prep),
            b"sbix" => Some(TableTag::Sbix),
            b"STAT" => Some(TableTag::Stat),
            b"SVG " => Some(TableTag::Svg),
            b"VDMX" => Some(TableTag::Vdmx),
            b"vhea" => Some(TableTag::Vhea),
            b"vmtx" => Some(TableTag::Vmtx),
            b"VORG" => Some(TableTag::Vorg),
            b"VVAR" => Some(TableTag::Vvar),
            _ => None
        }
    }
}

impl fmt::Display for TableTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Tag::from(*self))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tag ([u8; 4]);

impl Tag {
    pub fn new(s: &[u8]) -> Tag {
        assert!(s.len() >= 4);
        let mut arr : [u8; 4] = Default::default();
        arr.copy_from_slice(s);
        Tag(arr)
    }
}

impl PartialEq<[u8; 4]> for Tag {
    fn eq(&self, other: &[u8; 4]) -> bool {
        self.0.as_ref() == other
    }
}

impl<'a> PartialEq<Tag> for &'a [u8] {
    fn eq(&self, other: &Tag) -> bool {
        self[..] == other.0[..]
    }
}

impl From<TableTag> for Tag {
    fn from(table_tag: TableTag) -> Self {
        Tag::new(match table_tag {
            TableTag::Avar => b"avar",
            TableTag::Base => b"BASE",
            TableTag::Cbdt => b"CBDT",
            TableTag::Cblc => b"CBLC",
            TableTag::Cff =>  b"CFF ",
            TableTag::Cff2 => b"CFF2",
            TableTag::Cmap => b"cmap",
            TableTag::Colr => b"COLR",
            TableTag::Cpal => b"CPAL",
            TableTag::Cvar => b"cvar",
            TableTag::Cvt =>  b"cvt ",
            TableTag::Dsig => b"DSIG",
            TableTag::Ebdt => b"EBDT",
            TableTag::Eblc => b"EBLC",
            TableTag::Ebsc => b"EBSC",
            TableTag::Fpgm => b"fpgm",
            TableTag::Fvar => b"fvar",
            TableTag::Gasp => b"gasp",
            TableTag::Gdef => b"GDEF",
            TableTag::Glyf => b"glyf",
            TableTag::Gpos => b"GPOS",
            TableTag::Gsub => b"GSUB",
            TableTag::Gvar => b"gvar",
            TableTag::Hdmx => b"hdmx",
            TableTag::Head => b"head",
            TableTag::Hhea => b"hhea",
            TableTag::Hmtx => b"hmtx",
            TableTag::Hvar => b"HVAR",
            TableTag::Jstf => b"JSTF",
            TableTag::Kern => b"kern",
            TableTag::Loca => b"loca",
            TableTag::Ltsh => b"LTSH",
            TableTag::Math => b"MATH",
            TableTag::Maxp => b"maxp",
            TableTag::Merg => b"MERG",
            TableTag::Meta => b"meta",
            TableTag::Mvar => b"MVAR",
            TableTag::Name => b"name",
            TableTag::Os2 =>  b"OS/2",
            TableTag::Pclt => b"PCLT",
            TableTag::Post => b"post",
            TableTag::Prep => b"prep",
            TableTag::Sbix => b"sbix",
            TableTag::Stat => b"STAT",
            TableTag::Svg =>  b"SVG ",
            TableTag::Vdmx => b"VDMX",
            TableTag::Vhea => b"vhea",
            TableTag::Vmtx => b"vmtx",
            TableTag::Vorg => b"VORG",
            TableTag::Vvar => b"VVAR"
        })
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match str::from_utf8(&self.0) {
            Ok(s) =>  write!(f, "{}", s),
            _ => Err(fmt::Error)
        }
    }
}

/// A rectangular bounding box defined by two points (x_min, y_min) and (x_max, y_max).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect<T> {
    x_min: T,
    y_min: T,
    x_max: T,
    y_max: T,
}

impl<T> Rect<T> where T: Copy {
    pub fn new(x_min: T, y_min: T, x_max: T, y_max: T) -> Rect<T> {
        Rect {
            x_min,
            y_min,
            x_max,
            y_max
        }
    }

    pub fn x_min(&self) -> T {
        self.x_min
    }

    pub fn y_min(&self) -> T {
        self.y_min
    }

    pub fn x_max(&self) -> T {
        self.x_max
    }

    pub fn y_max(&self) -> T {
        self.y_max
    }
}

pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    pub fn new<T>(data: &'a T) -> HexSlice<'a>
        where T: ?Sized + AsRef<[u8]> + 'a
    {
        HexSlice(data.as_ref())
    }
}

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, byte) in self.0.iter().enumerate() {
            if index % 26 == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{:01$X} ", byte, 2)?;
        }
        Ok(())
    }
}