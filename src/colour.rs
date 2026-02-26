use hsluv::hex_to_rgb;
use rgb::RGB8;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Colour {
  pub rgb: RGB8
}

impl Colour {
    pub fn to_rgb8(&self) -> RGB8 {
        self.rgb
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.rgb.r, self.rgb.g, self.rgb.b)
    }
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle CSV format: 255,255,255
        if s.contains(',') {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 3 {
                return Err("RGB format must be R,G,B".to_string());
            }
            let r = parts[0].trim().parse::<u8>().map_err(|_| "Invalid R value")?;
            let g = parts[1].trim().parse::<u8>().map_err(|_| "Invalid G value")?;
            let b = parts[2].trim().parse::<u8>().map_err(|_| "Invalid B value")?;
            return Ok(Colour { rgb: RGB8::new(r, g, b) });
        }

        // Handle Hex format: 6 characters, strip optional leading #
        let hex = s.strip_prefix('#').unwrap_or(s);
        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
          let (r, g, b) = hex_to_rgb(&format!("#{}", hex.to_lowercase()));
            return Ok(Colour {
              rgb: RGB8::new(
                (r * 255.0).round() as u8,
                (g * 255.0).round() as u8,
                (b * 255.0).round() as u8,
              )
            });
        }

        Err(format!("Invalid colour '{}'. Use hex (FFFFFF or #FFFFFF) or RGB (0,0,0)", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Parsing: valid inputs ---

    #[test]
    fn test_parse_hex_uppercase() {
        let colour = Colour::from_str("FFFFFF").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(255, 255, 255));
    }

    #[test]
    fn test_parse_hex_lowercase() {
        let colour = Colour::from_str("000000").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(0, 0, 0));
    }

    #[test]
    fn test_parse_hex_with_hash_prefix() {
        let colour = Colour::from_str("#ff0000").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(255, 0, 0));
    }

    #[test]
    fn test_parse_hex_mixed_case() {
        let colour = Colour::from_str("fF0000").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(255, 0, 0));
    }

    #[test]
    fn test_parse_rgb_no_spaces() {
        let colour = Colour::from_str("10,20,30").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(10, 20, 30));
    }

    #[test]
    fn test_parse_rgb_with_spaces() {
        let colour = Colour::from_str("10, 20, 30").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(10, 20, 30));
    }

    #[test]
    fn test_parse_rgb_black() {
        let colour = Colour::from_str("0,0,0").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(0, 0, 0));
    }

    #[test]
    fn test_parse_rgb_white() {
        let colour = Colour::from_str("255,255,255").unwrap();
        assert_eq!(colour.to_rgb8(), RGB8::new(255, 255, 255));
    }

    // --- Parsing: invalid inputs ---

    #[test]
    fn test_err_on_five_char_hex() {
        assert!(Colour::from_str("12345").is_err());
    }

    #[test]
    fn test_err_on_seven_char_hex_without_hash() {
        assert!(Colour::from_str("FFFFFFF").is_err());
    }

    #[test]
    fn test_err_on_non_hex_characters() {
        assert!(Colour::from_str("GGGGGG").is_err());
    }

    #[test]
    fn test_err_on_rgb_too_few_components() {
        assert!(Colour::from_str("10,20").is_err());
    }

    #[test]
    fn test_err_on_rgb_too_many_components() {
        assert!(Colour::from_str("10,20,30,40").is_err());
    }

    #[test]
    fn test_err_on_rgb_non_numeric() {
        assert!(Colour::from_str("abc,def,ghi").is_err());
    }

    #[test]
    fn test_err_on_rgb_out_of_range() {
        assert!(Colour::from_str("256,0,0").is_err());
    }

    #[test]
    fn test_err_on_empty_string() {
        assert!(Colour::from_str("").is_err());
    }
}