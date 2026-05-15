use std::num::ParseIntError;

use clap::Parser;
use hp_tracerled_rs::{Color, HpTracerLedDevice, LedReport, Mode, Zone, REPORT_LED_PINS};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_parser=parse_color)]
    color: Color,

    #[arg(short, long, default_value_t = 100)]
    brightness: u8,

    #[arg(short, long, default_value_t = 0)]
    theme: u8,

    #[arg(short, long)]
    speed: u8,

    #[arg(short, long, value_parser=zone_from_str)]
    mode: Mode,
}

fn zone_from_str(arg: &str) -> Result<Mode, &'static str> {
    match arg {
        "static" => Ok(Mode::Static),
        "breathing" => Ok(Mode::Breathing),
        _ => Err("Invalid mode"),
    }
}

fn parse_color(arg: &str) -> Result<Color, ParseIntError> {
    Ok(Color(
        u8::from_str_radix(&arg[0..2], 16)?,
        u8::from_str_radix(&arg[2..4], 16)?,
        u8::from_str_radix(&arg[4..6], 16)?,
    ))
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let dev = HpTracerLedDevice::new();

    let result = dev.apply_all_zones(&LedReport::new(
        Mode::Static,
        Zone::Logo,
        [args.color; REPORT_LED_PINS],
        args.brightness,
        args.theme,
        args.speed,
    ));

    println!("{:?}", result);
}
