/// A 24-bit triplet for hex colors. Defaults to *White* `(0xFF,0xFF,0xFF)`
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

#[cfg(feature = "serde")]
impl serde::Serialize for RGB {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let RGB(r, g, b) = *self;
        let mut rgb = serializer.serialize_struct("rgb", 3)?;
        rgb.serialize_field("r", &r)?;
        rgb.serialize_field("g", &g)?;
        rgb.serialize_field("b", &b)?;
        rgb.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RGB {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Into::into)
    }
}

impl Default for RGB {
    /// Default color of #FFFFFF (White)
    fn default() -> Self {
        RGB(255, 255, 255)
    }
}

impl std::fmt::Display for RGB {
    /// Formats the RGB as #RRGGBB (in hex)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl From<&str> for RGB {
    /// Tries to parse an RGB from the str, defaulting if invalid
    fn from(s: &str) -> Self {
        RGB::from_hex(s)
    }
}

impl From<String> for RGB {
    /// Tries to parse an RGB from the String, defaulting if invalid
    fn from(s: String) -> Self {
        RGB::from_hex(&s)
    }
}

impl RGB {
    /// Tries to parse a string (`'#FFFFFF'` or `'FFFFFF'`) into the RGB,
    /// `default`s if it can't
    pub fn from_hex(input: &str) -> Self {
        let input = input.trim();
        let input = match (input.chars().next(), input.len()) {
            (Some('#'), 7) => &input[1..],
            (_, 6) => input,
            _ => return Self::default(),
        };

        u32::from_str_radix(&input, 16)
            .map(|s| {
                RGB(
                    ((s >> 16) & 0xFF) as u8,
                    ((s >> 8) & 0xFF) as u8,
                    (s & 0xFF) as u8,
                )
            })
            .unwrap_or_default()
    }

    /// Returns the `red` field
    pub fn red(self) -> u8 {
        self.0
    }

    /// Returns the `green` field
    pub fn green(self) -> u8 {
        self.1
    }

    /// Returns the `blue` field
    pub fn blue(self) -> u8 {
        self.2
    }
}

impl From<Trovo> for RGB {
    /// Tries to turn the [`TrovoColor`](./enum.TrovoColor.html) color into an [`RGB`](./struct.RGB.html)
    ///
    /// If the color is, somehow, unknown, it'll use [`RGB::default`](./struct.RGB.html#method.default)
    fn from(color: Trovo) -> Self {
        if let Trovo::Turbo(rgb) = color {
            return rgb;
        }

        trovo_colors()
            .iter()
            .find(|(c, _)| *c == color)
            .map(|&(_, rgb)| rgb)
            .unwrap_or_default()
    }
}

/// These are the default Trovo colors
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Trovo {
    /// RGB (hex): #0000FF
    Blue,
    /// RGB (hex): #8A2BE2
    BlueViolet,
    /// RGB (hex): #5F9EA0
    CadetBlue,
    /// RGB (hex): #D2691E
    Chocolate,
    /// RGB (hex): #FF7F50
    Coral,
    /// RGB (hex): #1E90FF
    DodgerBlue,
    /// RGB (hex): #B22222
    Firebrick,
    /// RGB (hex): #DAA520
    GoldenRod,
    /// RGB (hex): #008000
    Green,
    /// RGB (hex): #FF69B4
    HotPink,
    /// RGB (hex): #FF4500
    OrangeRed,
    /// RGB (hex): #FF0000
    Red,
    /// RGB (hex): #2E8B57
    SeaGreen,
    /// RGB (hex): #00FF7F
    SpringGreen,
    /// RGB (hex): #ADFF2F
    YellowGreen,
    /// Turbo colors are custom user-selected colors    
    Turbo(RGB),
}

#[cfg(feature = "serde")]
impl serde::Serialize for Trovo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let RGB(r, g, b) = (*self).into();
        let mut rgb = serializer.serialize_struct("color", 4)?;

        match self {
            Trovo::Turbo(..) => rgb.serialize_field("name", &"Turbo")?,
            e => rgb.serialize_field("name", &e.to_string())?,
        }

        rgb.serialize_field("r", &r)?;
        rgb.serialize_field("g", &g)?;
        rgb.serialize_field("b", &b)?;
        rgb.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Trovo {
    fn deserialize<D>(deserializer: D) -> Result<Trovo, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct C<'a> {
            name: &'a str,
            r: u8,
            g: u8,
            b: u8,
        }

        C::deserialize(deserializer).map(|c| {
            if c.name == "Turbo" {
                Trovo::Turbo(RGB(c.r, c.g, c.b))
            } else {
                c.name.into()
            }
        })
    }
}

impl Default for Trovo {
    /// Defaults to Trovo::Turbo(RGB(0xFF,0xFF,0xFF))
    fn default() -> Self {
        Trovo::Turbo(RGB::default())
    }
}

impl From<&str> for Trovo {
    /// Tries to parse the trovo color name from a string, or as a #RRGGBB/RRGGBB string
    ///
    /// view source to see valid strings
    fn from(input: &str) -> Self {
        use Trovo::*;
        match input {
            "Blue" | "blue" => Blue,
            "BlueViolet" | "blue_violet" | "blueviolet" | "blue violet" => BlueViolet,
            "CadetBlue" | "cadet_blue" | "cadetblue" | "cadet blue" => CadetBlue,
            "Chocolate" | "chocolate" => Chocolate,
            "Coral" | "coral" => Coral,
            "DodgerBlue" | "dodger_blue" | "dodgerblue" | "dodger blue" => DodgerBlue,
            "Firebrick" | "firebrick" => Firebrick,
            "GoldenRod" | "golden_rod" | "goldenrod" | "golden rod" => GoldenRod,
            "Green" | "green" => Green,
            "HotPink" | "hot_pink" | "hotpink" | "hot pink" => HotPink,
            "OrangeRed" | "orange_red" | "orangered" | "orange red" => OrangeRed,
            "Red" | "red" => Red,
            "SeaGreen" | "sea_green" | "seagreen" | "sea green" => SeaGreen,
            "SpringGreen" | "spring_green" | "springgreen" | "spring green" => SpringGreen,
            "YellowGreen" | "yellow_green" | "yellowgreen" | "yellow green" => YellowGreen,
            s => Turbo(RGB::from_hex(s)),
        }
    }
}

impl From<RGB> for Trovo {
    /// Tries to turn the RGB Color into a Trovo Color
    ///
    /// Defaults to a Turbo(RGB(0xFF,0xFF,0xFF))
    fn from(rgb: RGB) -> Self {
        trovo_colors()
            .iter()
            .find(|(_, color)| *color == rgb)
            .map(|&(c, _)| c)
            .unwrap_or_else(|| Trovo::Turbo(rgb))
    }
}

impl std::fmt::Display for Trovo {
    /// Gets the Trovo color name as a string, as those listed on the Trovo site
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Trovo::*;
        match self {
            Blue => write!(f, "Blue"),
            BlueViolet => write!(f, "BlueViolet"),
            CadetBlue => write!(f, "CadetBlue"),
            Chocolate => write!(f, "Chocolate"),
            Coral => write!(f, "Coral"),
            DodgerBlue => write!(f, "DodgerBlue"),
            Firebrick => write!(f, "Firebrick"),
            GoldenRod => write!(f, "GoldenRod"),
            Green => write!(f, "Green"),
            HotPink => write!(f, "HotPink"),
            OrangeRed => write!(f, "OrangeRed"),
            Red => write!(f, "Red"),
            SeaGreen => write!(f, "SeaGreen"),
            SpringGreen => write!(f, "SpringGreen"),
            YellowGreen => write!(f, "YellowGreen"),
            Turbo(rgb) => write!(f, "{}", rgb),
        }
    }
}

/// A helper method that returns a const array of [`TrovoColor`](./enum.TrovoColor.html) colors to [`RGB`](./struct.RGB.html)
pub const fn trovo_colors() -> [(Trovo, RGB); 15] {
    use Trovo::*;
    [
        (Blue, RGB(0x00, 0x00, 0xFF)),
        (BlueViolet, RGB(0x8A, 0x2B, 0xE2)),
        (CadetBlue, RGB(0x5F, 0x9E, 0xA0)),
        (Chocolate, RGB(0xD2, 0x69, 0x1E)),
        (Coral, RGB(0xFF, 0x7F, 0x50)),
        (DodgerBlue, RGB(0x1E, 0x90, 0xFF)),
        (Firebrick, RGB(0xB2, 0x22, 0x22)),
        (GoldenRod, RGB(0xDA, 0xA5, 0x20)),
        (Green, RGB(0x00, 0x80, 0x00)),
        (HotPink, RGB(0xFF, 0x69, 0xB4)),
        (OrangeRed, RGB(0xFF, 0x45, 0x00)),
        (Red, RGB(0xFF, 0x00, 0x00)),
        (SeaGreen, RGB(0x2E, 0x8B, 0x57)),
        (SpringGreen, RGB(0x00, 0xFF, 0x7F)),
        (YellowGreen, RGB(0xAD, 0xFF, 0x2F)),
    ]
}
