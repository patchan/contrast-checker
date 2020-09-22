extern crate contrast;
extern crate rgb;
use std::env;
use rgb::RGB;


fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{}", args.len());
  if args.len() != 3 {
    panic!("not enough arguments");
  }

  let col1 = &args[1];
  let col2 = &args[2];
  println!("Color 1: {}", col1);
  println!("Color 2: {}", col2);
  let ratio: f32 = contrast::contrast(parse_arg(col1).unwrap(), parse_arg(col2).unwrap());
  println!("Contrast ratio {}:1", ratio);
}

fn parse_arg(arg: &str) -> Result<rgb::RGB8, Box<dyn std::error::Error>> {
  let len = arg.chars().count();
  if len == 2 || len == 6 {
    convert_hex();
  }
  if arg.contains(",") {
    let args: Vec<&str> = arg.split(",").collect();
    let r = args[0].parse::<u8>().unwrap();
    let g = args[1].parse::<u8>().unwrap();
    let b = args[2].parse::<u8>().unwrap();
    return Ok(RGB::new(r, g, b));
  }
  panic!("invalid color");
}

fn convert_hex() {

}