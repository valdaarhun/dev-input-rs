use std::fmt;
use std::error::Error;

#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
struct TimeVal {
    seconds: i64,
    microseconds: i64,
}

#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
pub struct Input {
    timeval: TimeVal,
    ev_type: u16,
    ev_code: u16,
    value: i32,   
}

impl Input {
    pub fn copy_from_bytes(&mut self, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
        self.timeval.seconds = i64::from_le_bytes(buffer[0..8].try_into()?);
        self.timeval.microseconds = i64::from_le_bytes(buffer[8..16].try_into()?);
        self.ev_type = u16::from_le_bytes(buffer[16..18].try_into()?);
        self.ev_code = u16::from_le_bytes(buffer[18..20].try_into()?);
        self.value = i32::from_le_bytes(buffer[20..24].try_into()?);

        Ok(())
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}.{}] Type={} Code={}, Value={}",
            self.timeval.seconds,
            self.timeval.microseconds,
            EvType::from(self.ev_type).tostring(),
            self.ev_code,
            self.value
        )
    }
}

macro_rules! enum_to_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        enum $name {
            $($variant),*,
        }

        impl $name {
            fn tostring(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

enum_to_str! {enum EvType {
    EvSyn,
    EvKey,
    EvRel,
    EvAbs,
    EvMsc,
    EvSw,
    EvLed,
    EvSnd,
    EvRep,
    EvFf,
    EvPwr,
    EvFfStatus,
    EvMax,
    EvCnt,
}}

impl From<u16> for EvType {
    fn from(ev_type: u16) -> Self {
        match ev_type {
            0x0 => EvType::EvSyn,
            0x1 => EvType::EvKey,
            0x2 => EvType::EvRel,
            0x3 => EvType::EvAbs,
            0x4 => EvType::EvMsc,
            0x5 => EvType::EvSw,
            0x11 => EvType::EvLed,
            0x12 => EvType::EvSnd,
            0x14 => EvType::EvRep,
            0x15 => EvType::EvFf,
            0x16 => EvType::EvPwr,
            0x17 => EvType::EvFfStatus,
            0x1f => EvType::EvMax,
            _ => EvType::EvCnt,
        }
    }
}
