use hidapi::{HidApi, HidDevice};
use log::{info, trace};
use strum::{EnumIter, IntoEnumIterator};
use zerocopy::{Immutable, IntoBytes};

pub const HP_TRACERLED_PID: u16 = 0x84FD;
pub const HP_TRACERLED_VID: u16 = 0x103C;

#[repr(C)]
#[derive(IntoBytes, Clone, Copy, Immutable, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[repr(u8)]
#[derive(Clone, IntoBytes, Immutable, Debug)]
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

#[repr(u8)]
pub enum StaticTheme {
    None = 0x00,
}

#[repr(u8)]
pub enum BreathingTheme {
    None = 0x00,
    Rainbow = 0x01,
}

#[repr(C)]
#[derive(IntoBytes, Immutable, Clone, Debug)]
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
    pub fn new(mode: Mode, zone: Zone, colors: [Color; 12], brightness: u8, theme: u8, speed: u8) -> Self {
        Self { 
            report_id: 0x00,
            _header: [0x00, 0x12],
            mode,
            color_count: colors.len() as u8,
            number: colors.len() as u8,
            _padding_counts: [0x00; 2],
            colors,
            _padding_colors: [0x00; 4],
            brightness,
            _padding_brightness: [0x00; 5],
            zone,
            _padding_zone: 0x01,
            theme,
            speed,
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

    pub fn disable_all_zones(&self) {
        for zone in Zone::iter() {
            let res = self.device.write(LedReport::new(Mode::Static, zone.clone(), [Color(0x00, 0x00, 0x00); 12], 0x00, 0x00, 0x01).as_bytes());
            info!("{:?} {:?}", zone, res);
        }
    }

    pub fn apply_all_zones(&self, report: &LedReport) {
        for zone in Zone::iter() {
            let mut command = report.clone();
            command.zone = zone.clone();
            let _ = self.apply(&command);
        }
    }

    pub fn apply(&self, report: &LedReport) -> Result<usize, hidapi::HidError> {
        trace!("{:?}", report);
        let res = self.device.write(report.as_bytes());
        info!("{:?} {:?}", report.zone, res);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_alignment() {
        let report = LedReport::new(Mode::Static, Zone::Logo, [Color(0xFF, 0x00, 0x00); 12], 0x64, 0x00, 0xFF);
        let bytes = report.as_bytes();
        assert_eq!(bytes.len(), 58);
        assert_eq!(bytes, &[
            0x00, 0x00, 0x12, 0x01,
            0x0C, 0x0C, // [Custom color count, number]
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
            0x01, 0x01, 0x00, 0xFF  // [0x36-0x39] zone / 0x01 / theme / speed
        ]);
    }
}

