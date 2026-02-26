extern crate clap;
extern crate contrast;
extern crate rgb;
extern crate hsluv;
use clap::{App, Arg};
use rgb::RGB;
use hsluv::hex_to_rgb;

fn main() {
  contrast_checker();
}

fn contrast_checker() {
  let matches = App::new("contrast-checker")
                                .version("v0.1.1")
                                .about("Simple tool to calculate the contrast ratio between any two colours.")
                                .arg(Arg::with_name("colour1")
                                          .required(true)
                                          .index(1)
                                          .help("enter a colour as a Hex code containing 6 numbers or as a comma separated R,G,B list"))
                                .arg(Arg::with_name("colour2")
                                          .required(true)
                                          .index(2)
                                          .help("enter a colour as a Hex code containing 6 numbers or as a comma separated R,G,B list"))
                                .get_matches();

  let arg1: &str = matches.value_of("colour1").unwrap();
  let arg2: &str = matches.value_of("colour2").unwrap();

  let ratio: f32 = calculate_contrast(arg1, arg2);

  print_result(ratio);
}

fn calculate_contrast(arg1: &str, arg2: &str) -> f32 {
  let colour1: RGB<u8> = convert_to_rgb(arg1, 1).unwrap();
  let colour2: RGB<u8> = convert_to_rgb(arg2, 2).unwrap();
  return contrast::contrast(colour1,colour2);
}

fn print_result(ratio: f32) {
  println!("Contrast ratio: {}:1", ratio);
  match ratio {
    r if r >= 7.0 => println!("✅ This meets WCAG level AAA standards for text!"),
    r if r >= 4.5 => println!("✅ This meets WCAG level AA standards for text!"),
    _ => {
      println!("❌ This does not meet WCAG level AA standards for text.");
      println!("To meet level AA, text must have at least a 4.5:1 ratio against its background.");
    }
  }

}

fn convert_to_rgb(arg: &str, num: i32) -> Result<rgb::RGB8, Box<dyn std::error::Error>> {
  let len: usize = arg.chars().count();

  // argument is Hex color code
  if len == 6 && !arg.contains(",") {
    let hex: String = parse_hex(arg);
    let rgb = hex_to_rgb(hex.as_str());
    let r: u8 = (rgb.0 * 255.0) as u8;
    let g: u8 = (rgb.1 * 255.0) as u8;
    let b: u8 = (rgb.2 * 255.0) as u8;
    println!("Color {}: {}", num, hex);
    return Ok(RGB::new(r, g, b));
  }

  // argument is RGB color code
  if arg.contains(",") {
    let rgb: Vec<&str> = arg.split(",").collect();
    if rgb.len() == 3 {
      let r: u8 = rgb[0].parse::<u8>().unwrap();
      let g: u8 = rgb[1].parse::<u8>().unwrap();
      let b: u8 = rgb[2].parse::<u8>().unwrap();
      println!("Color {}: rgb({}, {}, {})", num, r, g, b);
      return Ok(RGB::new(r, g, b));
    }
  }

  panic!("Color {}: {} is invalid", num, arg);
}

fn parse_hex(arg: &str) -> String {
  let mut hex: String = "#".to_owned();
  hex.push_str(&arg);
  return hex;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_white() {
        let result = convert_to_rgb("ffffff", 1).unwrap();
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255);
        assert_eq!(result.b, 255);
    }

    #[test]
    fn test_hex_black() {
        let result = convert_to_rgb("000000", 1).unwrap();
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
    }

    #[test]
    fn test_hex_red() {
        let result = convert_to_rgb("ff0000", 1).unwrap();
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
    }

    #[test]
    fn test_rgb_white() {
        let result = convert_to_rgb("255,255,255", 1).unwrap();
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255);
        assert_eq!(result.b, 255);
    }

    #[test]
    fn test_rgb_black() {
        let result = convert_to_rgb("0,0,0", 1).unwrap();
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
    }

    #[test]
    fn test_rgb_arbitrary_color() {
        let result = convert_to_rgb("100,150,200", 1).unwrap();
        assert_eq!(result.r, 100);
        assert_eq!(result.g, 150);
        assert_eq!(result.b, 200);
    }

    #[test]
    #[should_panic(expected = "Color 1")]
    fn test_invalid_input_panics() {
        convert_to_rgb("notacolor", 1).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rgb_too_few_components_panics() {
        // Two components instead of three — falls through both branches and panics
        convert_to_rgb("255,255", 1).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_rgb_value_out_of_range_panics() {
        // 256 overflows u8::parse, causing a panic in unwrap()
        convert_to_rgb("256,0,0", 1).unwrap();
    }

    #[test]
    fn test_parse_hex_prepends_hash() {
        assert_eq!(parse_hex("ff0000"), "#ff0000");
    }

    #[test]
    fn test_parse_hex_black() {
        assert_eq!(parse_hex("000000"), "#000000");
    }

    #[test]
    fn test_contrast_black_on_white_is_max() {
        // Black on white should give the maximum ratio of 21:1
        let ratio = calculate_contrast("000000", "ffffff");
        assert!((ratio - 21.0).abs() < 0.1, "Expected ~21.0, got {}", ratio);
    }

    #[test]
    fn test_contrast_identical_colors_is_min() {
        // Same color on same color should give minimum ratio of 1:1
        let ratio = calculate_contrast("ff0000", "ff0000");
        assert_eq!(ratio, 1.0)
    }

    #[test]
    fn test_contrast_is_symmetric() {
        // Contrast of A on B should equal contrast of B on A
        let ratio_ab = calculate_contrast("336699", "ffffff");
        let ratio_ba = calculate_contrast("ffffff", "336699");
        assert_eq!(ratio_ab, ratio_ba);
    }

    #[test]
    fn test_high_contrast_meets_wcag_aaa() {
        let ratio = calculate_contrast("000000", "ffffff");
        assert!(ratio >= 7.0);
    }

    #[test]
    fn test_mid_contrast_meets_aa_but_not_aaa() {
        // #767676 on white is the classic AA boundary colour (~4.54:1), below AAA
        let ratio = calculate_contrast("767676", "ffffff");
        assert!(ratio >= 4.5 && ratio < 7.0);
    }

    #[test]
    fn test_low_contrast_fails_wcag_aaa() {
        let ratio = calculate_contrast("aaaaaa", "bbbbbb");
        assert!(ratio < 7.0);
    }
}