use hidapi::{HidApi, HidDevice};
use strum::{EnumIter, IntoEnumIterator};
use zerocopy::{Immutable, IntoBytes};

pub const HP_TRACERLED_PID: u16 = 0x84FD;
pub const HP_TRACERLED_VID: u16 = 0x103C;

#[repr(C)]
#[derive(IntoBytes, Clone, Copy, Immutable, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[repr(u8)]
#[derive(Clone, IntoBytes, Immutable)]
pub enum Mode {
    Static = 0x01,
    Breathing = 0x06,
    Cycle = 0x07,
    Blinking = 0x08,
}

#[repr(u8)]
#[derive(EnumIter, Debug, Clone, IntoBytes, Immutable)]
pub enum Zone {
    Logo = 0x01,
    Bar = 0x02,
    Fan = 0x03,
    Cpu = 0x04,
    FrontFanBottom = 0x05,
    FrontFanMiddle = 0x06,
    FrontFanTop = 0x07,
}

#[repr(C)]
#[derive(IntoBytes, Immutable)]
pub struct LedReport {
    report_id: u8,
    _header: [u8; 2],
    mode: Mode,
    color_count: u8,
    number: u8,
    _padding_counts: [u8; 2],
    colors: [Color; 12],
    _padding_colors: [u8; 4],

    brightness: u8,
    _padding_brightness: [u8; 5],

    zone: Zone,
    _padding_zone: u8,
    theme: u8,
    speed: u8,
}

impl LedReport {
    pub fn new(mode: Mode, zone: Zone, brightness: u8, colors: [Color; 12]) -> Self {
        Self { 
            report_id: 0x00,
            _header: [0x00, 0x12],
            mode,
            color_count: colors.len() as u8,
            number: 0x01,
            _padding_counts: [0x00; 2],
            colors,
            _padding_colors: [0x00; 4],
            brightness,
            _padding_brightness: [0x00; 5],
            zone,
            _padding_zone: 0x01,
            theme: 0x00,
            speed: 0x01,
        }
    }
}

pub struct HpTracerLedDevice {
    pub device: HidDevice,
}

impl HpTracerLedDevice {
    pub fn new() -> Self {
        match HidApi::new() {
            Ok(api) => match api.open(HP_TRACERLED_VID, HP_TRACERLED_PID) {
                Ok(device) => Self { device },
                Err(e) => panic!("Failed to open device: {}", e),
            },
            Err(e) => panic!("Failed to create HID API: {}", e),
        }
    }

    pub fn set_static_color(&self, color: Color) {
        for zone in Zone::iter() {
            let res = self.set_zone_static_color(zone.clone(), color);
            println!("{:?} {:?}", zone, res);
        }
    }

    pub fn set_zone_static_color(&self, zone: Zone, color: Color) -> Result<usize, hidapi::HidError> {
        self.device.write(LedReport::new(Mode::Static, zone, 0x64, [color; 12]).as_bytes())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_alignment() {
        let report = LedReport::new(Mode::Static, Zone::Logo, 0x64, [Color(0xFF, 0x00, 0x00); 12]);
        let zero_bytes = report.as_bytes();
        assert_eq!(zero_bytes.len(), 58);
        assert_eq!(zero_bytes, &[
            0x00, 0x00, 0x12, 0x01,
            0x0C, 0x01, // [Custom color count, number]
            0x00, 0x00,
            // [0x08 - 0x2C]  R, G, B // 12 x 3 = 36
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            0xFF, 0x00, 0x00,
            // Pad
            0x00, 0x00, 0x00, 0x00,
            0x64, // Brightness
            0x00,
            0x00, 0x00, 0x00, 0x00,
            0x01, 0x01, 0x00, 0x01  // [0x36-0x39] zone / 0x01 / theme / speed
        ]);
    }
}

