use error::Error;
use nom::be_u16;
use traits::{Parser, TableParser};

/// Naming Table
///
/// The naming table allows multilingual strings to be associated with the OpenType™ font.
/// These strings can represent copyright notices, font names, family names, style names, and so
/// on. To keep this table short, the font manufacturer may wish to make a limited set of entries
/// in some small set of languages; later, the font can be “localized” and the strings translated
/// or added. Other parts of the OpenType font that require these strings can refer to them using
/// a language-independent name ID. In addition to language variants, the table also allows for
/// platform-specific character-encoding variants. Clients that need a particular string can look
/// it up by its platform ID, encoding ID, language ID and name ID. Note that different platforms
/// may have different requirements for the encoding of strings.
///
/// Many newer platforms can use strings intended for different platforms if a font does not
/// include strings for that platform. Some applications might display incorrect strings, however,
/// if strings for the current platform are not included.
///
/// There are two formats for the Naming Table. Format 0 uses platform-specific, numeric language
/// identifiers. Format 1 allows for use of language-tag strings to indicate the language of
/// strings. Both formats include variable-size string-data storage, and an array of name records
/// that are used to identify the type of string (name ID), platform, encoding and language
/// variants of the string, and the location within the storage.
///
/// **Format 0**: Format 0 differs from format 1 in regard to handling of language identification:
/// it uses only numeric language IDs, which generally are values less than 0x8000 and have
/// platform-specific interpretations.
///
/// **Format 1**: When format 1 is used, the language IDs in name records can be less than or
/// greater than 0x8000. If a language ID is less than 0x8000, it has a platform-specific
/// interpretation as with a format 0 naming table. If a language ID is equal to or greater than
/// 0x8000, it is associated with a language-tag record (LangTagRecord) that references a
/// language-tag string. In this way, the language ID is associated with a language-tag string that
/// specifies the language for name records using that language ID, regardless of the platform.
/// These can be used for any platform that supports this language-tag mechanism.
///
/// A font using a format 1 naming table may use a combination of platform-specific language IDs
/// as well as language-tag records for a given platform and encoding.
///
/// More information on ['name'](https://docs.microsoft.com/en-gb/typography/opentype/spec/name)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamingTable {
    string_offset: u16,
    name_records: Vec<NameRecord>,
    lang_tag_records: Option<Vec<LangTagRecord>>
}

impl NamingTable {
    /// Offset to start of string storage (from start of table).
    pub fn string_offset(&self) -> u16 {
        self.string_offset
    }

    /// The name records.
    pub fn name_records(&self) -> &Vec<NameRecord> {
        &self.name_records
    }

    /// The language-tag records.
    pub fn lang_tag_records(&self) -> Option<&Vec<LangTagRecord>> {
        self.lang_tag_records.as_ref()
    }
}

impl<'otf> Parser<'otf> for NamingTable {
    type Item = NamingTable;

    /// Parse Naming Table.
    ///
    /// # Example
    ///
    /// Naming Table format 0
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::tables::name::{NamingTable, Platform, MacintoshEncoding, MacintoshLanguage, NameId};
    /// use otf::traits::Parser;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x00, 0x00, 0x1A, 0x01, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x2F, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x06,
    ///     0x00, 0x2F, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x07, 0x00, 0x35,
    ///     0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x06, 0x00, 0x2F, 0x00, 0x01,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x06, 0x00, 0x2F, 0x00, 0x01, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x05, 0x00, 0x13, 0x00, 0x3C, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x06, 0x00, 0x0E, 0x00, 0x4F, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07,
    ///     0x00, 0x20, 0x00, 0x5D, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x06,
    ///     0x00, 0x7D, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x0A, 0x00, 0x83,
    ///     0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x13, 0x00, 0x8D, 0x00, 0x01,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x00, 0x2E, 0x00, 0xA0, 0x00, 0x01, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x0E, 0x00, 0x2A, 0x00, 0xCE, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09,
    ///     0x00, 0x00, 0x00, 0x5E, 0x00, 0xF8, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x01,
    ///     0x00, 0x0C, 0x01, 0x56, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x02, 0x00, 0x0E,
    ///     0x01, 0x62, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x03, 0x00, 0x0C, 0x01, 0x56,
    ///     0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x04, 0x00, 0x0C, 0x01, 0x56, 0x00, 0x03,
    ///     0x00, 0x01, 0x04, 0x09, 0x00, 0x05, 0x00, 0x26, 0x01, 0x70, 0x00, 0x03, 0x00, 0x01,
    ///     0x04, 0x09, 0x00, 0x06, 0x00, 0x1C, 0x01, 0x96, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09,
    ///     0x00, 0x07, 0x00, 0x40, 0x01, 0xB2, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x09,
    ///     0x00, 0x0C, 0x01, 0xF2, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x0B, 0x00, 0x14,
    ///     0x01, 0xFE, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x0C, 0x00, 0x26, 0x02, 0x12,
    ///     0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x0D, 0x00, 0x5C, 0x02, 0x38, 0x00, 0x03,
    ///     0x00, 0x01, 0x04, 0x09, 0x00, 0x0E, 0x00, 0x54, 0x02, 0x94];
    ///
    /// let naming_table = NamingTable::parse(bytes).unwrap();
    ///
    /// assert_eq!(naming_table.string_offset(), 318);
    /// assert_eq!(naming_table.name_records().len(), 26);
    /// assert!(naming_table.lang_tag_records().is_none());
    ///
    /// let first_name_record = naming_table.name_records().get(0).unwrap();
    ///
    /// match first_name_record.platform() {
    ///     Platform::Macintosh(encoding_id, language_id) => {
    ///         assert_eq!(encoding_id, MacintoshEncoding::Roman);
    ///         assert_eq!(language_id.unwrap(), MacintoshLanguage::English);
    ///     },
    ///     _ => assert!(false)
    /// }
    ///
    /// assert_eq!(first_name_record.name_id(), NameId::Copyright);
    /// assert_eq!(first_name_record.length(), 47);
    /// assert_eq!(first_name_record.offset(), 0);
    /// ```
    ///
    /// Naming Table format 1
    /// ```
    /// // TODO
    /// ```
    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error> {
        Ok(parse_naming_table(buf)?.1)
    }
}

impl<'otf> TableParser<'otf> for NamingTable {}

/// The platform, encoding and language IDs of a name record allow for platform-specific
/// implementations. Different platforms can support different encodings, and different languages.
/// All encoding IDs are platform-specific. Language IDs are similarly platform-specific, except in
/// the case of IDs used in conjuction with the language-tag mechanism of naming table format 1.
///
/// Note: Platform IDs, platform-specific encoding IDs and, in some cases, platform-specific
/// language IDs are also used in the 'cmap' table.
///
/// Language IDs refer to a value that identifies the language in which a particular string is
/// written. Values less than 0x8000 are defined on a platform-specific basis. A format 0 naming
/// $table must use only language ID values less than 0x8000 from the platform-specific enumerations
/// given below. (Exceptions to this requirement are permitted, however, in the case of
/// user-defined platforms — platform IDs 240 to 255.) Values greater than or equal to 0x8000 can
/// be used in a format 1 naming table in conjunction with language-tag records, as described above.
/// Not all platforms have platform-specific language IDs, and not all platforms support
/// language-tag records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    Unicode(UnicodeEncoding, Option<u16>),
    Macintosh(MacintoshEncoding, Option<MacintoshLanguage>),
    /// Platform ID 2 (ISO) has been deprecated as of OpenType version v1.3. It was intended to
    /// represent ISO/IEC 10646, as opposed to Unicode. It is redundant, however, since both
    /// standards have identical character code assignments.
    #[deprecated]
    Iso(IsoEncoding, Option<u16>),
    Windows(WindowsEncoding, Option<WindowsLanguage>),
    Custom(u16, Option<u16>),
    /// Platform ID values 240 through 255 are reserved for user-defined platforms. This
    /// specification will never assign these values to a registered platform.
    UserDefined(u16, Option<u16>)
}

impl Platform {
    pub fn new(platform_id: u16, encoding_id: u16, language_opt: Option<u16>) -> Option<Platform> {
        match platform_id {
            0 => {
                UnicodeEncoding::from_u16(encoding_id).map(|unicode_encoding_id| {
                    Platform::Unicode(unicode_encoding_id, language_opt)
                })
            },
            1 => {
                MacintoshEncoding::from_u16(encoding_id).and_then(|macintosh_encoding_id| {
                    let macintosh_language_id = language_opt.and_then(|language_id| {
                        MacintoshLanguage::from_u16(language_id)
                    });

                    Some(Platform::Macintosh(macintosh_encoding_id, macintosh_language_id))
                })
            },
            2 => {
                IsoEncoding::from_u16(encoding_id).map(|iso_encoding_id| {
                    Platform::Iso(iso_encoding_id, language_opt)
                })
            },
            3 => {
                WindowsEncoding::from_u16(encoding_id).and_then(|windows_encoding_id| {
                    let windows_language_id = language_opt.and_then(|language_id| {
                        WindowsLanguage::from_u16(language_id)
                    });

                    Some(Platform::Windows(windows_encoding_id, windows_language_id))
                })
            },
            4 => {
                Some(Platform::Custom(encoding_id, language_opt))
            },
            240...255 => {
                Some(Platform::UserDefined(encoding_id, language_opt))
            },
            _ => None
        }
    }
}

/// Unicode encoding IDs
///
/// A new encoding ID for the Unicode platform will be assigned if a new version of Unicode moves
/// characters, in order to properly specify character code semantics. (Because of Unicode
/// stability policies, such a need is not anticipated.) The distinction between Unicode
/// platform-specific encoding IDs 1 and 2 is for historical reasons only; the Unicode Standard is
/// in fact identical in repertoire and encoding to ISO 10646. For all practical purposes in
/// current fonts, the distinctions provided by encoding IDs 0, 1 and 2 are not important, thus
/// these encoding IDs are deprecated.
///
/// A new encoding ID for the Unicode platform is also sometimes assigned when new 'cmap' subtable
/// formats are added to the specification, so as to allow for compatibility with existing parsers.
/// For example, when 'cmap' subtable formats 10 and 12 were added to the specification, encoding
/// ID 4 was added as well, and when 'cmap' subtable format 13 was added to the specification,
/// encoding ID 6 was added. The 'cmap' subtable formats listed in the table above are the only
/// ones that may be used for the corresponding encoding ID.
///
/// Unicode platform encoding ID 5 can be used for encodings in the 'cmap' table but not for
/// strings in the 'name' table.
///
/// There are no platform-specific language IDs defined for the Unicode platform. Language ID = 0
/// may be used for Unicode-platform strings, but this does not indicate any particular language.
/// Language IDs greater than or equal to 0x8000 may be used together with language-tag records,
/// as described above.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum UnicodeEncoding {
    /// Unicode 1.0 semantics
    Unicode_1_0 = 0,
    /// Unicode 1.1 semantics
    Unicode_1_1 = 1,
    /// ISO/IEC 10646 semantics
    Iso10646 = 2,
    /// Unicode 2.0 and onwards semantics, Unicode BMP only ('cmap' subtable formats 0, 4, 6).
    Unicode_2_0_Bmp = 3,
    /// Unicode 2.0 and onwards semantics, Unicode full repertoire ('cmap' subtable formats 0,
    /// 4, 6, 10, 12).
    Unicode_2_0_Full = 4,
    /// Unicode Variation Sequences ('cmap' subtable format 14).
    UnicodeVariationSequences = 5,
    /// Unicode full repertoire ('cmap' subtable formats 0, 4, 6, 10, 12, 13).
    UnicodeFullRepertoire = 6
}

impl UnicodeEncoding {
    pub fn from_u16(v: u16) -> Option<UnicodeEncoding> {
        match v {
            0 => Some(UnicodeEncoding::Unicode_1_0),
            1 => Some(UnicodeEncoding::Unicode_1_1),
            2 => Some(UnicodeEncoding::Iso10646),
            3 => Some(UnicodeEncoding::Unicode_2_0_Bmp),
            4 => Some(UnicodeEncoding::Unicode_2_0_Full),
            5 => Some(UnicodeEncoding::UnicodeVariationSequences),
            6 => Some(UnicodeEncoding::UnicodeFullRepertoire),
            _ => None
        }
    }
}

/// Windows encoding IDs
///
/// When building a Unicode font for Windows, the platform ID should be 3 and the encoding ID
/// should be 1, and the referenced string data must be encoded in UTF-16BE. When building a
/// symbol font for Windows, the platform ID should be 3 and the encoding ID should be 0, and
/// the referenced string data must be encoded in UTF-16BE. When building a font that will be used
/// on the Macintosh, the platform ID should be 1 and the encoding ID should be 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowsEncoding {
    Symbol = 0,
    UnicodeBmp = 1,
    ShiftJis = 2,
    Prc = 3,
    Big5 = 4,
    Wansung = 5,
    Johab = 6,
    UnicodeFullRepertoire = 10
}

impl WindowsEncoding {
    pub fn from_u16(v: u16) -> Option<WindowsEncoding> {
        match v {
            0 => Some(WindowsEncoding::Symbol),
            1 => Some(WindowsEncoding::UnicodeBmp),
            2 => Some(WindowsEncoding::ShiftJis),
            3 => Some(WindowsEncoding::Prc),
            4 => Some(WindowsEncoding::Big5),
            5 => Some(WindowsEncoding::Wansung),
            6 => Some(WindowsEncoding::Johab),
            10 => Some(WindowsEncoding::UnicodeFullRepertoire),
            _ => None
        }
    }
}

/// Platform-specific Language IDs assigned by Microsoft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowsLanguage {
    Afrikaans = 0x0436,
    Albanian = 0x041C,
    Alsatian = 0x0484,
    Amharic = 0x045E,
    ArabicAlgeria = 0x1401,
    ArabicBahrain = 0x3C01,
    ArabicEgypt = 0x0C01,
    ArabicIraq = 0x0801,
    ArabicJordan = 0x2C01,
    ArabicKuwait = 0x3401,
    ArabicLebanon = 0x3001,
    ArabicLibya = 0x1001,
    ArabicMorocco = 0x1801,
    ArabicOman = 0x2001,
    ArabicQatar = 0x4001,
    ArabicSaudi = 0x0401,
    ArabicSyria = 0x2801,
    ArabicTunisia = 0x1C01,
    ArabicUAE = 0x3801,
    ArabicYemen = 0x2401,
    Armenian = 0x042B,
    Assamese = 0x044D,
    AzeriCyrillic = 0x082C,
    AzeriLatin = 0x042C,
    Bashkir = 0x046D,
    Basque = 0x042D,
    Belarusian = 0x0423,
    BengaliBangladesh = 0x0845,
    BengaliIndia = 0x0445,
    BosnianCyrillic = 0x201A,
    BosnianLatin = 0x141A,
    Breton = 0x047E,
    Bulgarian = 0x0402,
    Catalan = 0x0403,
    ChineseHongKongSAR = 0x0C04,
    ChineseMacaoSAR = 0x1404,
    ChineseRepublicOfChina = 0x0804,
    ChineseSingapore = 0x1004,
    ChineseTaiwan = 0x0404,
    Corsican = 0x0483,
    Croatian = 0x041A,
    CroatianLatin = 0x101A,
    Czech = 0x0405,
    Danish = 0x0406,
    Dari = 0x048C,
    Divehi = 0x0465,
    DutchBelgium = 0x0813,
    DutchNetherlands = 0x0413,
    EnglishAustralia = 0x0C09,
    EnglishBelize = 0x2809,
    EnglishCanada = 0x1009,
    EnglishCaribbean = 0x2409,
    EnglishIndia = 0x4009,
    EnglishIreland = 0x1809,
    EnglishJamaica = 0x2009,
    EnglishMalaysia = 0x4409,
    EnglishNewZealand = 0x1409,
    EnglishPhilippines = 0x3409,
    EnglishSingapore = 0x4809,
    EnglishSouthAfrica = 0x1C09,
    EnglishTrinidadAndTobago = 0x2C09,
    EnglishUnitedKingdom = 0x0809,
    EnglishUnitedStates = 0x0409,
    EnglishZimbabwe = 0x3009,
    Estonian = 0x0425,
    Faroese = 0x0438,
    Filipino = 0x0464,
    Finnish = 0x040B,
    FrenchBelgium = 0x080C,
    FrenchCanada = 0x0C0C,
    FrenchFrance = 0x040C,
    FrenchLuxembourg = 0x140c,
    FrenchMonaco = 0x180C,
    FrenchSwitzerland = 0x100C,
    Frisian = 0x0462,
    Galician = 0x0456,
    Georgian = 0x0437,
    GermanAustria = 0x0C07,
    GermanGermany = 0x0407,
    GermanLiechtenstein = 0x1407,
    GermanLuxembourg = 0x1007,
    GermanSwitzerland = 0x0807,
    Greek = 0x0408,
    Greenlandic = 0x046F,
    Gujarati = 0x0447,
    Hausa = 0x0468,
    Hebrew = 0x040D,
    Hindi = 0x0439,
    Hungarian = 0x040E,
    Icelandic = 0x040F,
    Igbo = 0x0470,
    Indonesian = 0x0421,
    Inuktitut = 0x045D,
    InuktitutLatin = 0x085D,
    Irish = 0x083C,
    IsiXhosa = 0x0434,
    IsiZulu = 0x0435,
    ItalianItaly = 0x0410,
    ItalianSwitzerland = 0x0810,
    Japanese = 0x0411,
    Kannada = 0x044B,
    Kazakh = 0x043F,
    Khmer = 0x0453,
    Kiche = 0x0486,
    Kinyarwanda = 0x0487,
    Kiswahili = 0x0441,
    Konkani = 0x0457,
    Korean = 0x0412,
    Kyrgyz = 0x0440,
    Lao = 0x0454,
    Latvian = 0x0426,
    Lithuanian = 0x0427,
    LowerSorbian = 0x082E,
    Luxembourgish = 0x046E,
    Macedonian = 0x042F,
    MalayBrunei = 0x083E,
    MalayMalaysia = 0x043E,
    Malayalam = 0x044C,
    Maltese = 0x043A,
    Maori = 0x0481,
    Mapudungun = 0x047A,
    Marathi = 0x044E,
    Mohawk = 0x047C,
    MongolianCyrillic = 0x0450,
    MongolianTraditional = 0x0850,
    Nepali = 0x0461,
    NorwegianBokmal = 0x0414,
    NorwegianNynorsk = 0x0814,
    Occitan = 0x0482,
    Odia = 0x0448,
    Pashto = 0x0463,
    Polish = 0x0415,
    PortugueseBrazil = 0x0416,
    PortuguesePortugal = 0x0816,
    Punjabi = 0x0446,
    QuechuaBolivia = 0x046B,
    QuechuaEcuador = 0x086B,
    QuechuaPeru = 0x0C6B,
    Romanian = 0x0418,
    Romansh = 0x0417,
    Russian = 0x0419,
    SamiInariFinland = 0x243B,
    SamiLuleNorway = 0x103B,
    SamiLuleSweden = 0x143B,
    SamiNorthernFinland = 0x0C3B,
    SamiNorthernNorway = 0x043B,
    SamiNorthernSweden = 0x083B,
    SamiSkoltFinland = 0x203B,
    SamiSouthernNorway = 0x183B,
    SamiSouthernSweden = 0x1C3B,
    Sanskrit = 0x044F,
    SerbianCyrillicBosniaAndHerzegovina = 0x1C1A,
    SerbianCyrillicSerbia = 0x0C1A,
    SerbianLatinBosniAndHerzegovina = 0x181A,
    SerbianLatinSerbia = 0x081A,
    Sesotho = 0x046C,
    Setswana = 0x0432,
    Sinhala = 0x045B,
    Slovak = 0x041B,
    Slovenian = 0x0424,
    SpanishArgentina = 0x2C0A,
    SpanishBolivia = 0x400A,
    SpanishChile = 0x340A,
    SpanishColombia = 0x240A,
    SpanishCostaRica = 0x140A,
    SpanishDominicanRepublic = 0x1C0A,
    SpanishEcuador = 0x300A,
    SpanishElSalvador = 0x440A,
    SpanishGuatemala = 0x100A,
    SpanishHonduras = 0x480A,
    SpanishMexico = 0x080A,
    SpanishNicaragua = 0x4C0A,
    SpanishPanama = 0x180A,
    SpanishParaguay = 0x3C0A,
    SpanishPeru = 0x280A,
    SpanishPuertoRico = 0x500A,
    SpanishModernSpain = 0x0C0A,
    SpanishTraditionalSpain = 0x040A,
    SpanishUnitedStates = 0x540A,
    SpanishUruguay = 0x380A,
    SpanishVenezuela = 0x200A,
    SwedenFinland = 0x081D,
    SwedishSweden = 0x041D,
    Syriac = 0x045A,
    Tajik = 0x0428,
    Tamazight = 0x085F,
    Tamil = 0x0449,
    Tatar = 0x0444,
    Telugu = 0x044A,
    Thai = 0x041E,
    Tibetan = 0x0451,
    Turkish = 0x041F,
    Turkmen = 0x0442,
    Uighur = 0x0480,
    Ukrainian = 0x0422,
    Upper = 0x042E,
    Urdu = 0x0420,
    UzbekCyrillic = 0x0843,
    UzbekLatin = 0x0443,
    Vietnamese = 0x042A,
    Welsh = 0x0452,
    Wolof = 0x0488,
    Yakut = 0x0485,
    Yi = 0x0478,
    Yoruba = 0x046A
}

impl WindowsLanguage {
    pub fn from_u16(v: u16) -> Option<WindowsLanguage> {
        match v {
            0x0436 => Some(WindowsLanguage::Afrikaans),
            0x041C => Some(WindowsLanguage::Albanian),
            0x0484 => Some(WindowsLanguage::Alsatian),
            0x045E => Some(WindowsLanguage::Amharic),
            0x1401 => Some(WindowsLanguage::ArabicAlgeria),
            0x3C01 => Some(WindowsLanguage::ArabicBahrain),
            0x0C01 => Some(WindowsLanguage::ArabicEgypt),
            0x0801 => Some(WindowsLanguage::ArabicIraq),
            0x2C01 => Some(WindowsLanguage::ArabicJordan),
            0x3401 => Some(WindowsLanguage::ArabicKuwait),
            0x3001 => Some(WindowsLanguage::ArabicLebanon),
            0x1001 => Some(WindowsLanguage::ArabicLibya),
            0x1801 => Some(WindowsLanguage::ArabicMorocco),
            0x2001 => Some(WindowsLanguage::ArabicOman),
            0x4001 => Some(WindowsLanguage::ArabicQatar),
            0x0401 => Some(WindowsLanguage::ArabicSaudi),
            0x2801 => Some(WindowsLanguage::ArabicSyria),
            0x1C01 => Some(WindowsLanguage::ArabicTunisia),
            0x3801 => Some(WindowsLanguage::ArabicUAE),
            0x2401 => Some(WindowsLanguage::ArabicYemen),
            0x042B => Some(WindowsLanguage::Armenian),
            0x044D => Some(WindowsLanguage::Assamese),
            0x082C => Some(WindowsLanguage::AzeriCyrillic),
            0x042C => Some(WindowsLanguage::AzeriLatin),
            0x046D => Some(WindowsLanguage::Bashkir),
            0x042D => Some(WindowsLanguage::Basque),
            0x0423 => Some(WindowsLanguage::Belarusian),
            0x0845 => Some(WindowsLanguage::BengaliBangladesh),
            0x0445 => Some(WindowsLanguage::BengaliIndia),
            0x201A => Some(WindowsLanguage::BosnianCyrillic),
            0x141A => Some(WindowsLanguage::BosnianLatin),
            0x047E => Some(WindowsLanguage::Breton),
            0x0402 => Some(WindowsLanguage::Bulgarian),
            0x0403 => Some(WindowsLanguage::Catalan),
            0x0C04 => Some(WindowsLanguage::ChineseHongKongSAR),
            0x1404 => Some(WindowsLanguage::ChineseMacaoSAR),
            0x0804 => Some(WindowsLanguage::ChineseRepublicOfChina),
            0x1004 => Some(WindowsLanguage::ChineseSingapore),
            0x0404 => Some(WindowsLanguage::ChineseTaiwan),
            0x0483 => Some(WindowsLanguage::Corsican),
            0x041A => Some(WindowsLanguage::Croatian),
            0x101A => Some(WindowsLanguage::CroatianLatin),
            0x0405 => Some(WindowsLanguage::Czech),
            0x0406 => Some(WindowsLanguage::Danish),
            0x048C => Some(WindowsLanguage::Dari),
            0x0465 => Some(WindowsLanguage::Divehi),
            0x0813 => Some(WindowsLanguage::DutchBelgium),
            0x0413 => Some(WindowsLanguage::DutchNetherlands),
            0x0C09 => Some(WindowsLanguage::EnglishAustralia),
            0x2809 => Some(WindowsLanguage::EnglishBelize),
            0x1009 => Some(WindowsLanguage::EnglishCanada),
            0x2409 => Some(WindowsLanguage::EnglishCaribbean),
            0x4009 => Some(WindowsLanguage::EnglishIndia),
            0x1809 => Some(WindowsLanguage::EnglishIreland),
            0x2009 => Some(WindowsLanguage::EnglishJamaica),
            0x4409 => Some(WindowsLanguage::EnglishMalaysia),
            0x1409 => Some(WindowsLanguage::EnglishNewZealand),
            0x3409 => Some(WindowsLanguage::EnglishPhilippines),
            0x4809 => Some(WindowsLanguage::EnglishSingapore),
            0x1C09 => Some(WindowsLanguage::EnglishSouthAfrica),
            0x2C09 => Some(WindowsLanguage::EnglishTrinidadAndTobago),
            0x0809 => Some(WindowsLanguage::EnglishUnitedKingdom),
            0x0409 => Some(WindowsLanguage::EnglishUnitedStates),
            0x3009 => Some(WindowsLanguage::EnglishZimbabwe),
            0x0425 => Some(WindowsLanguage::Estonian),
            0x0438 => Some(WindowsLanguage::Faroese),
            0x0464 => Some(WindowsLanguage::Filipino),
            0x040B => Some(WindowsLanguage::Finnish),
            0x080C => Some(WindowsLanguage::FrenchBelgium),
            0x0C0C => Some(WindowsLanguage::FrenchCanada),
            0x040C => Some(WindowsLanguage::FrenchFrance),
            0x140c => Some(WindowsLanguage::FrenchLuxembourg),
            0x180C => Some(WindowsLanguage::FrenchMonaco),
            0x100C => Some(WindowsLanguage::FrenchSwitzerland),
            0x0462 => Some(WindowsLanguage::Frisian),
            0x0456 => Some(WindowsLanguage::Galician),
            0x0437 => Some(WindowsLanguage::Georgian),
            0x0C07 => Some(WindowsLanguage::GermanAustria),
            0x0407 => Some(WindowsLanguage::GermanGermany),
            0x1407 => Some(WindowsLanguage::GermanLiechtenstein),
            0x1007 => Some(WindowsLanguage::GermanLuxembourg),
            0x0807 => Some(WindowsLanguage::GermanSwitzerland),
            0x0408 => Some(WindowsLanguage::Greek),
            0x046F => Some(WindowsLanguage::Greenlandic),
            0x0447 => Some(WindowsLanguage::Gujarati),
            0x0468 => Some(WindowsLanguage::Hausa),
            0x040D => Some(WindowsLanguage::Hebrew),
            0x0439 => Some(WindowsLanguage::Hindi),
            0x040E => Some(WindowsLanguage::Hungarian),
            0x040F => Some(WindowsLanguage::Icelandic),
            0x0470 => Some(WindowsLanguage::Igbo),
            0x0421 => Some(WindowsLanguage::Indonesian),
            0x045D => Some(WindowsLanguage::Inuktitut),
            0x085D => Some(WindowsLanguage::InuktitutLatin),
            0x083C => Some(WindowsLanguage::Irish),
            0x0434 => Some(WindowsLanguage::IsiXhosa),
            0x0435 => Some(WindowsLanguage::IsiZulu),
            0x0410 => Some(WindowsLanguage::ItalianItaly),
            0x0810 => Some(WindowsLanguage::ItalianSwitzerland),
            0x0411 => Some(WindowsLanguage::Japanese),
            0x044B => Some(WindowsLanguage::Kannada),
            0x043F => Some(WindowsLanguage::Kazakh),
            0x0453 => Some(WindowsLanguage::Khmer),
            0x0486 => Some(WindowsLanguage::Kiche),
            0x0487 => Some(WindowsLanguage::Kinyarwanda),
            0x0441 => Some(WindowsLanguage::Kiswahili),
            0x0457 => Some(WindowsLanguage::Konkani),
            0x0412 => Some(WindowsLanguage::Korean),
            0x0440 => Some(WindowsLanguage::Kyrgyz),
            0x0454 => Some(WindowsLanguage::Lao),
            0x0426 => Some(WindowsLanguage::Latvian),
            0x0427 => Some(WindowsLanguage::Lithuanian),
            0x082E => Some(WindowsLanguage::LowerSorbian),
            0x046E => Some(WindowsLanguage::Luxembourgish),
            0x042F => Some(WindowsLanguage::Macedonian),
            0x083E => Some(WindowsLanguage::MalayBrunei),
            0x043E => Some(WindowsLanguage::MalayMalaysia),
            0x044C => Some(WindowsLanguage::Malayalam),
            0x043A => Some(WindowsLanguage::Maltese),
            0x0481 => Some(WindowsLanguage::Maori),
            0x047A => Some(WindowsLanguage::Mapudungun),
            0x044E => Some(WindowsLanguage::Marathi),
            0x047C => Some(WindowsLanguage::Mohawk),
            0x0450 => Some(WindowsLanguage::MongolianCyrillic),
            0x0850 => Some(WindowsLanguage::MongolianTraditional),
            0x0461 => Some(WindowsLanguage::Nepali),
            0x0414 => Some(WindowsLanguage::NorwegianBokmal),
            0x0814 => Some(WindowsLanguage::NorwegianNynorsk),
            0x0482 => Some(WindowsLanguage::Occitan),
            0x0448 => Some(WindowsLanguage::Odia),
            0x0463 => Some(WindowsLanguage::Pashto),
            0x0415 => Some(WindowsLanguage::Polish),
            0x0416 => Some(WindowsLanguage::PortugueseBrazil),
            0x0816 => Some(WindowsLanguage::PortuguesePortugal),
            0x0446 => Some(WindowsLanguage::Punjabi),
            0x046B => Some(WindowsLanguage::QuechuaBolivia),
            0x086B => Some(WindowsLanguage::QuechuaEcuador),
            0x0C6B => Some(WindowsLanguage::QuechuaPeru),
            0x0418 => Some(WindowsLanguage::Romanian),
            0x0417 => Some(WindowsLanguage::Romansh),
            0x0419 => Some(WindowsLanguage::Russian),
            0x243B => Some(WindowsLanguage::SamiInariFinland),
            0x103B => Some(WindowsLanguage::SamiLuleNorway),
            0x143B => Some(WindowsLanguage::SamiLuleSweden),
            0x0C3B => Some(WindowsLanguage::SamiNorthernFinland),
            0x043B => Some(WindowsLanguage::SamiNorthernNorway),
            0x083B => Some(WindowsLanguage::SamiNorthernSweden),
            0x203B => Some(WindowsLanguage::SamiSkoltFinland),
            0x183B => Some(WindowsLanguage::SamiSouthernNorway),
            0x1C3B => Some(WindowsLanguage::SamiSouthernSweden),
            0x044F => Some(WindowsLanguage::Sanskrit),
            0x1C1A => Some(WindowsLanguage::SerbianCyrillicBosniaAndHerzegovina),
            0x0C1A => Some(WindowsLanguage::SerbianCyrillicSerbia),
            0x181A => Some(WindowsLanguage::SerbianLatinBosniAndHerzegovina),
            0x081A => Some(WindowsLanguage::SerbianLatinSerbia),
            0x046C => Some(WindowsLanguage::Sesotho),
            0x0432 => Some(WindowsLanguage::Setswana),
            0x045B => Some(WindowsLanguage::Sinhala),
            0x041B => Some(WindowsLanguage::Slovak),
            0x0424 => Some(WindowsLanguage::Slovenian),
            0x2C0A => Some(WindowsLanguage::SpanishArgentina),
            0x400A => Some(WindowsLanguage::SpanishBolivia),
            0x340A => Some(WindowsLanguage::SpanishChile),
            0x240A => Some(WindowsLanguage::SpanishColombia),
            0x140A => Some(WindowsLanguage::SpanishCostaRica),
            0x1C0A => Some(WindowsLanguage::SpanishDominicanRepublic),
            0x300A => Some(WindowsLanguage::SpanishEcuador),
            0x440A => Some(WindowsLanguage::SpanishElSalvador),
            0x100A => Some(WindowsLanguage::SpanishGuatemala),
            0x480A => Some(WindowsLanguage::SpanishHonduras),
            0x080A => Some(WindowsLanguage::SpanishMexico),
            0x4C0A => Some(WindowsLanguage::SpanishNicaragua),
            0x180A => Some(WindowsLanguage::SpanishPanama),
            0x3C0A => Some(WindowsLanguage::SpanishParaguay),
            0x280A => Some(WindowsLanguage::SpanishPeru),
            0x500A => Some(WindowsLanguage::SpanishPuertoRico),
            0x0C0A => Some(WindowsLanguage::SpanishModernSpain),
            0x040A => Some(WindowsLanguage::SpanishTraditionalSpain),
            0x540A => Some(WindowsLanguage::SpanishUnitedStates),
            0x380A => Some(WindowsLanguage::SpanishUruguay),
            0x200A => Some(WindowsLanguage::SpanishVenezuela),
            0x081D => Some(WindowsLanguage::SwedenFinland),
            0x041D => Some(WindowsLanguage::SwedishSweden),
            0x045A => Some(WindowsLanguage::Syriac),
            0x0428 => Some(WindowsLanguage::Tajik),
            0x085F => Some(WindowsLanguage::Tamazight),
            0x0449 => Some(WindowsLanguage::Tamil),
            0x0444 => Some(WindowsLanguage::Tatar),
            0x044A => Some(WindowsLanguage::Telugu),
            0x041E => Some(WindowsLanguage::Thai),
            0x0451 => Some(WindowsLanguage::Tibetan),
            0x041F => Some(WindowsLanguage::Turkish),
            0x0442 => Some(WindowsLanguage::Turkmen),
            0x0480 => Some(WindowsLanguage::Uighur),
            0x0422 => Some(WindowsLanguage::Ukrainian),
            0x042E => Some(WindowsLanguage::Upper),
            0x0420 => Some(WindowsLanguage::Urdu),
            0x0843 => Some(WindowsLanguage::UzbekCyrillic),
            0x0443 => Some(WindowsLanguage::UzbekLatin),
            0x042A => Some(WindowsLanguage::Vietnamese),
            0x0452 => Some(WindowsLanguage::Welsh),
            0x0488 => Some(WindowsLanguage::Wolof),
            0x0485 => Some(WindowsLanguage::Yakut),
            0x0478 => Some(WindowsLanguage::Yi),
            0x046A => Some(WindowsLanguage::Yoruba),
            _ => None
        }
    }
}

/// Macintosh encoding IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacintoshEncoding {
    Roman = 0,
    Japanese = 1,
    Chinese = 2,
    Korean = 3,
    Arabic = 4,
    Hebrew = 5,
    Greek = 6,
    Russian = 7,
    RSymbol = 8,
    Devanagari = 9,
    Gurmukhi = 10,
    Gujarati = 11,
    Oriya = 12,
    Bengali = 13,
    Tamil = 14,
    Telugu = 15,
    Kannada = 16
}

impl MacintoshEncoding {
    pub fn from_u16(v: u16) -> Option<MacintoshEncoding> {
        match v {
            0 => Some(MacintoshEncoding::Roman),
            1 => Some(MacintoshEncoding::Japanese),
            2 => Some(MacintoshEncoding::Chinese),
            3 => Some(MacintoshEncoding::Korean),
            4 => Some(MacintoshEncoding::Arabic),
            5 => Some(MacintoshEncoding::Hebrew),
            6 => Some(MacintoshEncoding::Greek),
            7 => Some(MacintoshEncoding::Russian),
            8 => Some(MacintoshEncoding::RSymbol),
            9 => Some(MacintoshEncoding::Devanagari),
            10 => Some(MacintoshEncoding::Gurmukhi),
            11 => Some(MacintoshEncoding::Gujarati),
            12 => Some(MacintoshEncoding::Oriya),
            13 => Some(MacintoshEncoding::Bengali),
            14 => Some(MacintoshEncoding::Tamil),
            15 => Some(MacintoshEncoding::Telugu),
            16 => Some(MacintoshEncoding::Kannada),
            _ => None
        }
    }
}

/// Platform-specific Language IDs assigned by Apple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacintoshLanguage {
    English = 0,
    French = 1,
    German = 2,
    Italian = 3,
    Dutch = 4,
    Swedish = 5,
    Spanish = 6,
    Danish = 7,
    Portuguese = 8,
    Norwegian = 9,
    Hebrew = 10,
    Japanese = 11,
    Arabic = 12,
    Finnish = 13,
    Greek = 14,
    Icelandic = 15,
    Maltese = 16,
    Turkish = 17,
    Croatian = 18,
    ChineseTraditional = 19,
    Urdu = 20,
    Hindi = 21,
    Thai = 22,
    Korean = 23,
    Lithuanian = 24,
    Polish = 25,
    Hungarian = 26,
    Estonian = 27,
    Latvian = 28,
    Sami = 29,
    Faroese = 30,
    Farsi = 31,
    Russian = 32,
    ChineseSimplified = 33,
    Flemish = 34,
    Irish = 35,
    Albanian = 36,
    Romanian = 37,
    Czech = 38,
    Slovak = 39,
    Slovenian = 40,
    Yiddish = 41,
    Serbian = 42,
    Macedonian = 43,
    Bulgarian = 44,
    Ukrainian = 45,
    Byelorussian = 46,
    Uzbek = 47,
    Kazakh = 48,
    AzerbaijaniCyrillic = 49,
    AzerbaijaniArabic = 50,
    Armenian = 51,
    Georgian = 52,
    Moldavian = 53,
    Kirghiz = 54,
    Tajiki = 55,
    Turkmen = 56,
    Mongolian = 57,
    MongolianCyrillic = 58,
    Pashto = 59,
    Kurdish = 60,
    Kashmiri = 61,
    Sindhi = 62,
    Tibetan = 63,
    Nepali = 64,
    Sanskrit = 65,
    Marathi = 66,
    Bengali = 67,
    Assamese = 68,
    Gujarati = 69,
    Punjabi = 70,
    Oriya = 71,
    Malayalam = 72,
    Kannada = 73,
    Tamil = 74,
    Telugu = 75,
    Sinhalese = 76,
    Burmese = 77,
    Khmer = 78,
    Lao = 79,
    Vietnamese = 80,
    Indonesian = 81,
    Tagalog = 82,
    MalayRoman = 83,
    MalayArabic = 84,
    Amharic = 85,
    Tigrinya = 86,
    Galla = 87,
    Somali = 88,
    Swahili = 89,
    Kinyarwanda = 90,
    Rundi = 91,
    Nyanja = 92,
    Malagasy = 93,
    Esperanto = 94,
    Welsh = 128,
    Basque = 129,
    Catalan = 130,
    Latin = 131,
    Quechua = 132,
    Guarani = 133,
    Aymara = 134,
    Tatar = 135,
    Uighur = 136,
    Dzongkha = 137,
    Javanese = 138,
    Sundanese = 139,
    Galician = 140,
    Afrikaans = 141,
    Breton = 142,
    Inuktitut = 143,
    Scottish = 144,
    Manx = 145,
    IrishGaelicWithDotAbove = 146,
    Tongan = 147,
    GreekPolytonic = 148,
    Greenlandic = 149,
    AzerbaijaniRoman = 150
}

impl MacintoshLanguage {
    pub fn from_u16(v: u16) -> Option<MacintoshLanguage> {
        match v {
            0 => Some(MacintoshLanguage::English),
            1 => Some(MacintoshLanguage::French),
            2 => Some(MacintoshLanguage::German),
            3 => Some(MacintoshLanguage::Italian),
            4 => Some(MacintoshLanguage::Dutch),
            5 => Some(MacintoshLanguage::Swedish),
            6 => Some(MacintoshLanguage::Spanish),
            7 => Some(MacintoshLanguage::Danish),
            8 => Some(MacintoshLanguage::Portuguese),
            9 => Some(MacintoshLanguage::Norwegian),
            10 => Some(MacintoshLanguage::Hebrew),
            11 => Some(MacintoshLanguage::Japanese),
            12 => Some(MacintoshLanguage::Arabic),
            13 => Some(MacintoshLanguage::Finnish),
            14 => Some(MacintoshLanguage::Greek),
            15 => Some(MacintoshLanguage::Icelandic),
            16 => Some(MacintoshLanguage::Maltese),
            17 => Some(MacintoshLanguage::Turkish),
            18 => Some(MacintoshLanguage::Croatian),
            19 => Some(MacintoshLanguage::ChineseTraditional),
            20 => Some(MacintoshLanguage::Urdu),
            21 => Some(MacintoshLanguage::Hindi),
            22 => Some(MacintoshLanguage::Thai),
            23 => Some(MacintoshLanguage::Korean),
            24 => Some(MacintoshLanguage::Lithuanian),
            25 => Some(MacintoshLanguage::Polish),
            26 => Some(MacintoshLanguage::Hungarian),
            27 => Some(MacintoshLanguage::Estonian),
            28 => Some(MacintoshLanguage::Latvian),
            29 => Some(MacintoshLanguage::Sami),
            30 => Some(MacintoshLanguage::Faroese),
            31 => Some(MacintoshLanguage::Farsi),
            32 => Some(MacintoshLanguage::Russian),
            33 => Some(MacintoshLanguage::ChineseSimplified),
            34 => Some(MacintoshLanguage::Flemish),
            35 => Some(MacintoshLanguage::Irish),
            36 => Some(MacintoshLanguage::Albanian),
            37 => Some(MacintoshLanguage::Romanian),
            38 => Some(MacintoshLanguage::Czech),
            39 => Some(MacintoshLanguage::Slovak),
            40 => Some(MacintoshLanguage::Slovenian),
            41 => Some(MacintoshLanguage::Yiddish),
            42 => Some(MacintoshLanguage::Serbian),
            43 => Some(MacintoshLanguage::Macedonian),
            44 => Some(MacintoshLanguage::Bulgarian),
            45 => Some(MacintoshLanguage::Ukrainian),
            46 => Some(MacintoshLanguage::Byelorussian),
            47 => Some(MacintoshLanguage::Uzbek),
            48 => Some(MacintoshLanguage::Kazakh),
            49 => Some(MacintoshLanguage::AzerbaijaniCyrillic),
            50 => Some(MacintoshLanguage::AzerbaijaniArabic),
            51 => Some(MacintoshLanguage::Armenian),
            52 => Some(MacintoshLanguage::Georgian),
            53 => Some(MacintoshLanguage::Moldavian),
            54 => Some(MacintoshLanguage::Kirghiz),
            55 => Some(MacintoshLanguage::Tajiki),
            56 => Some(MacintoshLanguage::Turkmen),
            57 => Some(MacintoshLanguage::Mongolian),
            58 => Some(MacintoshLanguage::MongolianCyrillic),
            59 => Some(MacintoshLanguage::Pashto),
            60 => Some(MacintoshLanguage::Kurdish),
            61 => Some(MacintoshLanguage::Kashmiri),
            62 => Some(MacintoshLanguage::Sindhi),
            63 => Some(MacintoshLanguage::Tibetan),
            64 => Some(MacintoshLanguage::Nepali),
            65 => Some(MacintoshLanguage::Sanskrit),
            66 => Some(MacintoshLanguage::Marathi),
            67 => Some(MacintoshLanguage::Bengali),
            68 => Some(MacintoshLanguage::Assamese),
            69 => Some(MacintoshLanguage::Gujarati),
            70 => Some(MacintoshLanguage::Punjabi),
            71 => Some(MacintoshLanguage::Oriya),
            72 => Some(MacintoshLanguage::Malayalam),
            73 => Some(MacintoshLanguage::Kannada),
            74 => Some(MacintoshLanguage::Tamil),
            75 => Some(MacintoshLanguage::Telugu),
            76 => Some(MacintoshLanguage::Sinhalese),
            77 => Some(MacintoshLanguage::Burmese),
            78 => Some(MacintoshLanguage::Khmer),
            79 => Some(MacintoshLanguage::Lao),
            80 => Some(MacintoshLanguage::Vietnamese),
            81 => Some(MacintoshLanguage::Indonesian),
            82 => Some(MacintoshLanguage::Tagalog),
            83 => Some(MacintoshLanguage::MalayRoman),
            84 => Some(MacintoshLanguage::MalayArabic),
            85 => Some(MacintoshLanguage::Amharic),
            86 => Some(MacintoshLanguage::Tigrinya),
            87 => Some(MacintoshLanguage::Galla),
            88 => Some(MacintoshLanguage::Somali),
            89 => Some(MacintoshLanguage::Swahili),
            90 => Some(MacintoshLanguage::Kinyarwanda),
            91 => Some(MacintoshLanguage::Rundi),
            92 => Some(MacintoshLanguage::Nyanja),
            93 => Some(MacintoshLanguage::Malagasy),
            94 => Some(MacintoshLanguage::Esperanto),
            128 => Some(MacintoshLanguage::Welsh),
            129 => Some(MacintoshLanguage::Basque),
            130 => Some(MacintoshLanguage::Catalan),
            131 => Some(MacintoshLanguage::Latin),
            132 => Some(MacintoshLanguage::Quechua),
            133 => Some(MacintoshLanguage::Guarani),
            134 => Some(MacintoshLanguage::Aymara),
            135 => Some(MacintoshLanguage::Tatar),
            136 => Some(MacintoshLanguage::Uighur),
            137 => Some(MacintoshLanguage::Dzongkha),
            138 => Some(MacintoshLanguage::Javanese),
            139 => Some(MacintoshLanguage::Sundanese),
            140 => Some(MacintoshLanguage::Galician),
            141 => Some(MacintoshLanguage::Afrikaans),
            142 => Some(MacintoshLanguage::Breton),
            143 => Some(MacintoshLanguage::Inuktitut),
            144 => Some(MacintoshLanguage::Scottish),
            145 => Some(MacintoshLanguage::Manx),
            146 => Some(MacintoshLanguage::IrishGaelicWithDotAbove),
            147 => Some(MacintoshLanguage::Tongan),
            148 => Some(MacintoshLanguage::GreekPolytonic),
            149 => Some(MacintoshLanguage::Greenlandic),
            150 => Some(MacintoshLanguage::AzerbaijaniRoman),
            _ => None
        }
    }
}

/// ISO encoding IDs
///
/// There are no ISO-specific language IDs, and language-tag records are not supported on this
/// platform. This means that it could potentially be used for encodings in the 'cmap' table, but
/// not for strings in the 'name' table. Note that use of the ISO platform in the 'cmap' table
/// is deprecated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum IsoEncoding {
    Ascii = 0,
    Iso10646 = 1,
    Iso8859_1 = 2
}

impl IsoEncoding {
    pub fn from_u16(v: u16) -> Option<IsoEncoding> {
        match v {
            0 => Some(IsoEncoding::Ascii),
            1 => Some(IsoEncoding::Iso10646),
            2 => Some(IsoEncoding::Iso8859_1),
            _ => None
        }
    }
}

/// Name IDs
///
/// The following name IDs are pre-defined, and they apply to all platforms unless indicated
/// otherwise. Name IDs 26 to 255, inclusive, are reserved for future standard names. Name IDs 256
/// to 32767, inclusive, are reserved for font-specific names such as those referenced by a font’s
/// layout features.
///
/// Note that while both Apple and Microsoft support the same set of name strings, the
/// interpretations may be somewhat different. But since name strings are stored by platform,
/// encoding and language (placing separate strings for both Apple and MS platforms), this should
/// not present a problem.
///
/// The key information for this table for Microsoft platforms relates to the use of name IDs 1, 2,
/// 4, 16 and 17. Note that some newer applications will use name IDs 16 and 17, while some legacy
/// applications require name IDs 1 and 2 and also assume certain limitations on these values
/// (see descriptions of name IDs 1 and 2). Fonts should include all of these strings for the
/// broadest application compatibility.
///
/// All naming table strings for the Windows platform (platform ID 3) must be encoded in UTF-16BE.
/// Strings for the Macintosh platform (platform ID 1) use platform-specific single- or
/// double-byte encodings.
///
/// Note that, for a typographic family that includes member faces that differ from Regular in
/// relation to attributes other than weight, width or slope, there may also be some member faces
/// that differ only in relation to these three attributes. IDs 21 and 22 should be used only in
/// those fonts that differ from the Regular face in terms of an attribute other than weight,
/// width or slope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NameId {
    /// Copyright notice.
    Copyright,

    /// Font Family name. The Font Family name is used in combination with Font Subfamily name
    /// (name ID 2), and should be shared among at most four fonts that differ only in weight or
    /// style (as described below).
    ///
    /// This four-way distinction should also be reflected in the OS/2.fsSelection field, using
    /// bits 0 and 5.
    ///
    /// While some platforms or applications do not have this constraint, many existing
    /// applications that use this pair of names assume that a Font Family name is shared by at
    /// most four fonts that form a font style-linking group: regular, italic (or oblique), bold,
    /// and bold italic (or bold oblique). To be compatible with the broadest range of platforms
    /// and applications, it is strongly recommended that fonts limit use of Font Family name in
    /// this manner.
    ///
    /// For extended typographic families that includes fonts other than the four basic styles
    /// (regular, italic, bold, bold italic), it is strongly recommended that name IDs 16 and 17
    /// be used in fonts to create an extended, typographic grouping.
    ///
    /// It is also strongly recommended that applications support extended typographic-family
    /// groupings using name IDs 16 and 17. Note, in particular, that variable fonts can include a
    /// large number of named instances, each of which will use a shared typographic family name
    /// (name ID 16) and will have a typographic subfamily name (equivalent to name ID 17).
    /// Applications that assume a four-style family grouping based on name IDs 1 and 2 are likely
    /// to provide a poor user experience with variable fonts.
    ///
    /// For fonts within an extended typographic family that fall outside the basic four-way
    /// distinction, the distinguishing attributes should be reflected in the Font Family name so
    /// that those fonts appear as a separate font family. For example, the Font Family name for
    /// the Arial Narrow font is “Arial Narrow”; the Font Family name for the Arial Black font is
    /// “Arial Black”. Note that, in such cases, name ID 16 should also be included with a shared
    /// name that reflects the full, typographic family.
    FontFamilyName,

    /// Font Subfamily name. The Font Subfamily name is used in combination with Font Family name
    /// (name ID 1), and distinguishes the fonts in a group with the same Font Family name. This
    /// should be used for style and weight variants only (as described below).
    ///
    /// This four-way distinction should also be reflected in the OS/2.fsSelection field, using
    /// bits 0 and 5. A font with no distinctive weight or style (e.g. medium weight, not italic,
    /// and OS/2.fsSelection bit 6 set) should use the string “Regular” as the Font Subfamily name
    /// (for English language).
    ///
    /// While some platforms or applications do not have this constraint, many existing
    /// applications that use this pair of names assume that a Font Family name is shared by at
    /// most four fonts that form a font style-linking group, and that Font Subfamily names would
    /// reflect one of the four basic styles, regular, italic (or oblique), bold, and bold italic
    /// (or bold oblique). To be compatible with the broadest range of platforms and applications,
    /// it is strongly recommended that fonts should limit use of Font Family in this manner.
    ///
    /// For extended typographic families that includes fonts other than the four basic styles
    /// (regular, italic, bold, bold italic), it is strongly recommended that name IDs 16 and 17 be
    /// used in fonts to create an extended, typographic grouping.
    ///
    /// Within an extended typographic family that includes fonts beyond regular, bold, italic, or
    /// bold italic, distinctions are made in the Font Family name, so that fonts appear to be in
    /// separate families. In some cases, this may lead to specifying a Subfamily name of “Regular”
    /// for a font that might not otherwise be considered a regular font. For example, the Arial
    /// Black font has a Font Family name of “Arial Black” and a Subfamily name of “Regular”. Note
    /// that, in such cases, name IDs 16 and 17 should also be included, using a shared value for
    /// name ID 16 that reflects the full typographic family, and values for name ID 17 that
    /// appropriately reflect the actual design variant of each font.
    FontSubfamilyName,

    /// Unique font identifier.
    UniqueFontIdentifier,

    /// Full font name that reflects all family and relevant subfamily descriptors. The full font
    /// name is generally a combination of name IDs 1 and 2, or of name IDs 16 and 17, or a similar
    /// human-readable variant.
    ///
    /// For fonts in extended typographic families (that is, families that include more than
    /// regular, italic, bold, and bold italic variants), values for name IDs 1 and 2 are normally
    /// chosen to provide compatibility with certain applications that assume a family has at most
    /// four style-linked fonts. In that case, some fonts may end up with a Subfamily name
    /// (name ID 2) of “Regular” even though the font would not be considered, typographically, a
    /// regular font. For such non-regular fonts in which name ID 2 is specified as “Regular”, the
    /// “Regular” descriptor would generally be omitted from name ID 4. For example, the Arial
    /// Black font has a Font Family name (name ID 1) of “Arial Black” and a Subfamily name (name
    /// ID 2) of “Regular”, but has a full font name (name ID 4) of “Arial Black”. Note that name
    /// IDs 16 and 17 should also be included in these fonts, and that name ID 4 would typically
    /// be a combination of name IDs 16 and 17, without needing any additional qualifications
    /// regarding “Regular”.
    FullFontName,

    /// Version string. Should begin with the syntax “Version <number>.<number>” (upper case, lower
    /// case, or mixed, with a space between “Version” and the number).
    ///
    /// The string must contain a version number of the following form: one or more digits (0-9)
    /// of value less than 65,535, followed by a period, followed by one or more digits of value
    /// less than 65,535. Any character other than a digit will terminate the minor number. A
    /// character such as “;” is helpful to separate different pieces of version information.
    ///
    /// The first such match in the string can be used by installation software to compare font
    /// versions. Note that some installers may require the string to start with “Version ”,
    /// followed by a version number as above.
    VersionString,

    /// PostScript name for the font; Name ID 6 specifies a string which is used to invoke a
    /// PostScript language font that corresponds to this OpenType font. When translated to
    /// ASCII, the name string must be no longer than 63 characters and restricted to the printable
    /// ASCII subset, codes 33 to 126, except for the 10 characters '[', ']', '(', ')', '{', '}',
    /// '<', '>', '/', '%'.
    ///
    /// In a CFF OpenType font, there is no requirement that this name be the same as the font name
    /// in the CFF’s Name INDEX. Thus, the same CFF may be shared among multiple font components
    /// in a Font Collection. See the ['name' table section](https://docs.microsoft.com/en-gb/typography/opentype/spec/recom#name)
    /// of “Recommendations for OpenType fonts” for additional information.
    PostScript,

    /// Trademark; this is used to save any trademark notice/information for this font. Such
    /// information should be based on legal advice. This is distinctly separate from the copyright.
    Trademark,

    /// Manufacturer Name.
    ManufacturerName,

    /// Designer; name of the designer of the typeface.
    Designer,

    /// Description; description of the typeface. Can contain revision information, usage
    /// recommendations, history, features, etc.
    Description,

    /// URL Vendor; URL of font vendor (with protocol, e.g., http://, ftp://). If a unique serial
    /// number is embedded in the URL, it can be used to register the font.
    URLVendor,

    /// URL Designer; URL of typeface designer (with protocol, e.g., http://, ftp://).
    URLDesigner,

    /// License Description; description of how the font may be legally used, or different example
    /// scenarios for licensed use. This field should be written in plain language, not legalese.
    LicenseDescription,

    /// License Info URL; URL where additional licensing information can be found.
    LicenseInfoURL,

    /// Typographic Family name: The typographic family grouping doesn’t impose any constraints on
    /// the number of faces within it, in contrast with the 4-style family grouping (ID 1), which
    /// is present both for historical reasons and to express style linking groups. If name ID 16
    /// is absent, then name ID 1 is considered to be the typographic family name. (In earlier
    /// versions of the specification, name ID 16 was known as “Preferred Family”.)
    TypographicFamilyName,

    /// Typographic Subfamily name: This allows font designers to specify a subfamily name within
    /// the typographic family grouping. This string must be unique within a particular typographic
    /// family. If it is absent, then name ID 2 is considered to be the typographic subfamily name.
    /// (In earlier versions of the specification, name ID 17 was known as “Preferred Subfamily”.)
    TypographicSubfamilyName,

    /// Compatible Full (Macintosh only); On the Macintosh, the menu name is constructed using the
    /// FOND resource. This usually matches the Full Name. If you want the name of the font to
    /// appear differently than the Full Name, you can insert the Compatible Full Name in ID 18.
    CompatibleFull,

    /// Sample text; This can be the font name, or any other text that the designer thinks is the
    /// best sample to display the font in.
    SampleText,

    /// PostScript CID findfont name; Its presence in a font means that the nameID 6 holds a
    /// PostScript font name that is meant to be used with the “composefont” invocation in order to
    /// invoke the font in a PostScript interpreter. See the definition of name ID 6.
    ///
    /// The value held in the name ID 20 string is interpreted as a PostScript font name that is
    /// meant to be used with the “findfont” invocation, in order to invoke the font in a
    /// PostScript interpreter.
    ///
    /// When translated to ASCII, this name string must be restricted to the printable ASCII
    /// subset, codes 33 through 126, except for the 10 characters: '[', ']', '(', ')', '{', '}',
    /// '<', '>', '/', '%'.
    ///
    /// See ["Recommendations for OTF fonts"](https://docs.microsoft.com/en-gb/typography/opentype/spec/recom)
    /// for additional information
    PostScriptCIDFindfontName,

    /// WWS Family Name. Used to provide a WWS-conformant family name in case the entries for IDs
    /// 16 and 17 do not conform to the WWS model. (That is, in case the entry for ID 17 includes
    /// qualifiers for some attribute other than weight, width or slope.) If bit 8 of the
    /// fsSelection field is set, a WWS Family Name entry should not be needed and should not be
    /// included. Conversely, if an entry for this ID is include, bit 8 should not be set.
    /// (See OS/2.fsSelection field for details.) Examples of name ID 21: “Minion Pro Caption”
    /// and “Minion Pro Display”. (Name ID 16 would be “Minion Pro” for these examples.)
    WWSFamilyName,

    /// WWS Subfamily Name. Used in conjunction with ID 21, this ID provides a WWS-conformant
    /// subfamily name (reflecting only weight, width and slope attributes) in case the entries for
    /// IDs 16 and 17 do not conform to the WWS model. As in the case of ID 21, use of this ID
    /// should correlate inversely with bit 8 of the fsSelection field being set. Examples of name
    /// ID 22: “Semibold Italic”, “Bold Condensed”. (Name ID 17 could be “Semibold Italic Caption”,
    /// or “Bold Condensed Display”, for example.)
    WWSSubfamilyName,

    /// Light Background Palette. This ID, if used in the CPAL table’s Palette Labels Array,
    /// specifies that the corresponding color palette in the CPAL table is appropriate to use with
    /// the font when displaying it on a light background such as white. Strings for this ID are
    /// for use as user interface strings associated with this palette.
    LightBackgroundPalette,

    /// Dark Background Palette. This ID, if used in the CPAL table’s Palette Labels Array,
    /// specifies that the corresponding color palette in the CPAL table is appropriate to use with
    /// the font when displaying it on a dark background such as black. Strings for this ID are for
    /// use as user interface strings associated with this palette.
    DarkBackgroundPalette,

    /// Variations PostScript Name Prefix. If present in a variable font, it may be used as the
    /// family prefix in the PostScript Name Generation for Variation Fonts algorithm. The
    /// character set is restricted to ASCII-range uppercase Latin letters, lowercase Latin
    /// letters, and digits. All name strings for name ID 25 within a font, when converted to
    /// ASCII, must be identical. See [Adobe Technical Note #5902: “PostScript Name Generation for Variation Fonts”](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5902.AdobePSNameGeneration.pdf)
    /// for reasons to include name ID 25 in a font, and for examples. For general information on
    /// OpenType Font Variations, see the chapter, [OpenType Font Variations Overview](https://docs.microsoft.com/en-gb/typography/opentype/spec/otvaroverview).
    VariationsPostScriptNamePrefix,

    /// Name IDs 256 to 32767, inclusive, are reserved for font-specific names such as those
    /// referenced by a font’s layout features.
    FontSpecificName(u16)
}

impl NameId {
    pub fn from_u16(v: u16) -> Option<NameId> {
        match v {
            0 => Some(NameId::Copyright),
            1 => Some(NameId::FontFamilyName),
            2 => Some(NameId::FontSubfamilyName),
            3 => Some(NameId::UniqueFontIdentifier),
            4 => Some(NameId::FullFontName),
            5 => Some(NameId::VersionString),
            6 => Some(NameId::PostScript),
            7 => Some(NameId::Trademark),
            8 => Some(NameId::ManufacturerName),
            9 => Some(NameId::Designer),
            10 => Some(NameId::Description),
            11 => Some(NameId::URLVendor),
            12 => Some(NameId::URLDesigner),
            13 => Some(NameId::LicenseDescription),
            14 => Some(NameId::LicenseInfoURL),
            16 => Some(NameId::TypographicFamilyName),
            17 => Some(NameId::TypographicSubfamilyName),
            18 => Some(NameId::CompatibleFull),
            19 => Some(NameId::SampleText),
            20 => Some(NameId::PostScriptCIDFindfontName),
            21 => Some(NameId::WWSFamilyName),
            22 => Some(NameId::WWSSubfamilyName),
            23 => Some(NameId::LightBackgroundPalette),
            24 => Some(NameId::DarkBackgroundPalette),
            25 => Some(NameId::VariationsPostScriptNamePrefix),
            256...32767 => Some(NameId::FontSpecificName(v)),
            _ => None
        }
    }
}

/// Each string in the string storage is referenced by a name record. The name record has a
/// multi-part key, to identify the logical type of string and its language or platform-specific
/// implementation variants, plus the location of the string in the string storage.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NameRecord {
    platform: Platform,
    name_id: NameId,
    length: u16,
    offset: u16
}

impl NameRecord {
    /// The platform, encoding and language IDs.
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Name ID.
    ///
    /// The name ID identifies a logical string category, such as family name or copyright.
    pub fn name_id(&self) -> NameId {
        self.name_id
    }

    /// String length (in bytes).
    pub fn length(&self) -> u16 {
        self.length
    }

    /// String offset from start of storage area (in bytes).
    pub fn offset(&self) -> u16 {
        self.offset
    }
}

/// Language-tag record.
///
/// Language-tag strings stored in the naming table must be encoded in UTF-16BE. The language tags
/// must conform to IETF specification BCP 47. This provides tags such as “en”, “fr-CA” and
/// “zh-Hant” to identify languages, including dialects, written form and other language variants.
///
/// The language-tag records are associated sequentially with language IDs starting with 0x8000.
/// Each language-tag record corresponds to a language ID one greater than that for the previous
/// language-tag record. Thus, language IDs associated with language-tag records must be within the
/// range 0x8000 to 0x8000 + langTagCount - 1. If a name record uses a language ID that is greater
/// than this, the identity of the language is unknown; such name records should not be used.
///
/// For example, suppose a font has two language-tag records referencing strings in the storage:
/// the first references the string “en”, and the second references the string “zh-Hant-HK” In this
/// case, the language ID 0x8000 is used in name records to index English-language strings. The
/// language ID 0x8001 is used in name records to index strings in Traditional Chinese as used in
/// Hong Kong.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LangTagRecord {
    length: u16,
    offset: u16
}

impl LangTagRecord {
    /// Language-tag string length (in bytes).
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Language-tag string offset from start of storage area (in bytes).
    pub fn offset(&self) -> u16 {
        self.offset
    }
}


named!(pub parse_naming_table<&[u8],NamingTable>,
    alt!(parse_naming_table_format0 | parse_naming_table_format1)
);

named!(parse_naming_table_format0<&[u8],NamingTable>,
    do_parse!(
        verify!(be_u16, |format| format == 0) >>
        count: be_u16 >>
        string_offset: be_u16 >>
        name_records: count!(parse_name_record, count as usize) >>
        (
            NamingTable {
                string_offset,
                name_records,
                lang_tag_records: None
            }
        )
    )
);

named!(parse_naming_table_format1<&[u8],NamingTable>,
    do_parse!(
        verify!(be_u16, |format| format == 1) >>
        count: be_u16 >>
        string_offset: be_u16 >>
        name_records: count!(parse_name_record, count as usize) >>
        lang_tag_records: length_count!(be_u16, parse_lang_tag_record) >>
        (
            NamingTable {
                string_offset,
                name_records,
                lang_tag_records: Some(lang_tag_records)
            }
        )
    )
);

named!(parse_name_record<&[u8],NameRecord>,
    do_parse!(
        platform_id: be_u16 >>
        encoding_id: be_u16 >>
        language_id: be_u16 >>
        platform: expr_opt!(Platform::new(platform_id, encoding_id, Some(language_id))) >>
        name_id: map_opt!(be_u16, |v| NameId::from_u16(v)) >>
        length: be_u16 >>
        offset: be_u16 >>
        (
            NameRecord {
                platform,
                name_id,
                length,
                offset
            }
        )
    )
);

named!(parse_lang_tag_record<&[u8],LangTagRecord>,
    do_parse!(
        length: be_u16 >>
        offset: be_u16 >>
        (
            LangTagRecord {
                length,
                offset
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context, Needed};

    #[test]
    fn case_naming_table_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(2)));
        assert_eq!(parse_naming_table(bytes), expected);
    }

    #[test]
    fn case_naming_table_invalid_format() {
        let bytes: &[u8] = &[0x01, 0x01];

        let expected =  Result::Err(Err::Error(Context::Code(bytes, ErrorKind::Alt)));
        assert_eq!(parse_naming_table(bytes), expected);
    }

    #[test]
    fn case_naming_table_invalid_incomplete() {
        let bytes: &[u8] = &[0x00, 0x00, 0x00, 0x1A, 0x01, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x2F, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x06, 0x00, 0x2F, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x07, 0x00, 0x35];

        let expected = Result::Err(Err::Incomplete(Needed::Size(2)));
        assert_eq!(parse_naming_table(bytes), expected);
    }

    #[test]
    fn case_name_record_name_id_font_specific_name() {
        let bytes: &[u8] = &[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x0F, 0xFF,
            0x00, 0x00, 0x00, 0x00];

        let expected = (&b""[..], NameRecord {
            platform: Platform::new(1, 0, Some(0)).unwrap(),
            name_id: NameId::FontSpecificName(0x0FFF),
            offset: 0,
            length: 0
        });

        let res = parse_name_record(bytes).unwrap();
        assert_eq!(res,  expected);
    }

    #[test]
    fn case_name_record_invalid_platform_id() {
        let bytes: &[u8] = &[0x00, 0x05, 0x00, 0x00, 0x00, 0x00];

        let expected =  Result::Err(Err::Error(Context::Code(&b""[..], ErrorKind::ExprOpt)));
        assert_eq!(parse_name_record(bytes), expected);
    }

    #[test]
    fn case_name_record_invalid_macintosh_encoding_id() {
        let bytes: &[u8] = &[0x00, 0x01, 0x00, 0xFF, 0x00, 0x00];

        let expected =  Result::Err(Err::Error(Context::Code(&b""[..], ErrorKind::ExprOpt)));
        assert_eq!(parse_name_record(bytes), expected);
    }

    #[test]
    fn case_name_record_malformed_macintosh_language_id() {
        let bytes: &[u8] = &[0x00, 0x01, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00];

        let expected = (&b""[..], NameRecord {
            platform: Platform::new(1, 0, None).unwrap(),
            name_id: NameId::FontFamilyName,
            offset: 0,
            length: 0
        });

        let res = parse_name_record(bytes).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn case_name_record_invalid_name_id() {
        let bytes: &[u8] = &[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF];

        let expected =  Result::Err(Err::Error(Context::Code(&bytes[6..], ErrorKind::MapOpt)));
        assert_eq!(parse_name_record(bytes), expected);
    }
}