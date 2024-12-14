use hidapi::{HidApi, HidDevice};
use strum::{EnumIter, IntoEnumIterator};

pub const HP_TRACERLED_PID: u16 = 0x84FD;
pub const HP_TRACERLED_VID: u16 = 0x103C;

pub type Color = (u8, u8, u8);

#[derive(Clone)]
pub enum Mode {
    Static = 0x01,
    Breathing = 0x06,
    Cycle = 0x07,
    Blinking = 0x08,
}

#[derive(EnumIter, Debug, Clone)]
pub enum Zone {
    Logo = 0x01,
    Bar = 0x02,
    Fan = 0x03,
    Cpu = 0x04,
    FrontFanBottom = 0x05,
    FrontFanMiddle = 0x06,
    FrontFanTop = 0x07,
}

pub struct LedReport {
    pub mode: Mode,
    pub zone: Zone,
    pub brightness: u8,
    pub colors: [Color; 12]
}

impl LedReport {
    pub fn serialize(&self) -> [u8; 58] {
        let mut packet = [
            0x00, 0x00, 0x12,
            self.mode.clone() as u8,
            0x01 * self.colors.len() as u8, 0x01, // [Custom color count, number]
            0x00, 0x00,
            // [0x08 - 0x2C]  R, G, B // 12 x 3 = 36
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            self.brightness, // Brightness
            0x00,
            0x00, 0x00, 0x00, 0x00,
            self.zone.clone() as u8, 0x01, 0x00, 0x01  // [0x36-0x39] zone / 0x01 / theme / speed
        ];
        for (i, color) in self.colors.iter().enumerate() {
            packet[0x08 + (i * 3)] = color.0;
            packet[0x09 + (i * 3)] = color.1;
            packet[0x0A + (i * 3)] = color.2;
        }
        packet
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
        self.device.write(&LedReport { zone, mode: Mode::Static, brightness: 0x64, colors: [color; 12]}.serialize())
    }
}
