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
  if ratio >= 4.5 {
    println!("This meets WCAG level AA standards for text!");
  } else {
    println!("This does not meet WCAG level AA standards for text.");
    println!("To meet level AA, text must have at least a 4.5:1 ratio against its background.");
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