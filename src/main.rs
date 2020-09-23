extern crate contrast;
extern crate rgb;
extern crate hsluv;
use rgb::RGB;
use hsluv::hex_to_rgb;


fn main() {
  let arg1 = std::env::args().nth(1).expect("color 1 not given");
  let arg2 = std::env::args().nth(2).expect("color 2 not given");

  let ratio: f32 = contrast::contrast(parse_arg(arg1, 1).unwrap(), parse_arg(arg2, 2).unwrap());
  println!("Contrast ratio: {}:1", ratio);
}

fn parse_arg(arg: String, num: i32) -> Result<rgb::RGB8, Box<dyn std::error::Error>> {
  let len = arg.chars().count();
  if len == 6 {
    let hex: String = create_hex_string(arg);
    println!("Color {}: {}", num, hex);
    let rgb = hex_to_rgb(hex.as_str());
    let r = rgb.0 as u8;
    let g = rgb.1 as u8;
    let b = rgb.2 as u8;
    return Ok(RGB::new(r, g, b));
  }
  if arg.contains(",") {
    let args: Vec<&str> = arg.split(",").collect();
    if args.len() == 3 {
      let r = args[0].parse::<u8>().unwrap();
      let g = args[1].parse::<u8>().unwrap();
      let b = args[2].parse::<u8>().unwrap();
      println!("Color {}: rgb({}, {}, {})", num, r, g, b);
      return Ok(RGB::new(r, g, b));
    }
  }
  panic!("Color {} is invalid", num);
}

fn create_hex_string(arg: String) -> String {
  let mut hex: String = "#".to_owned();
  hex.push_str(&arg);
  return hex;
}