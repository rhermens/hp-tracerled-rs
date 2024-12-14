use std::num::ParseIntError;

use clap::Parser;
use hp_tracerled_rs::{Color, HpTracerLedDevice};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_parser=parse_color)]
    color: Color,
}

fn parse_color(arg: &str) -> Result<Color, ParseIntError> {
    Ok((
        u8::from_str_radix(&arg[0..2], 16)?,
        u8::from_str_radix(&arg[2..4], 16)?,
        u8::from_str_radix(&arg[4..6], 16)?,
    ))
}

fn main() {
    let args = Args::parse();
    let dev = HpTracerLedDevice::new();

    println!("{:?}", dev.set_static_color(args.color));
}
