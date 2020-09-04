pub use crate::libraries::dht_read::{read_raw, Delay, DhtError, InputOutputPin};

pub trait DhtReading: internal::FromRaw + Sized {
    fn read<P, E, D>(delay: &mut D, pin: &mut P) -> Result<Self, DhtError<E>>
    where
        P: InputOutputPin<E>,
        D: Delay,
    {
        read_raw(delay, pin).map(Self::raw_to_reading)
    }
}

mod internal {
    pub trait FromRaw {
        fn raw_to_reading(bytes: [u8; 4]) -> Self;
    }
}

pub mod dht11 {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Reading {
        pub temperature: i8,
        pub relative_humidity: u8,
    }

    impl internal::FromRaw for Reading {
        fn raw_to_reading(bytes: [u8; 4]) -> Reading {
            let [rh, _, temp_signed, _] = bytes;
            let temp = {
                let (signed, magnitude) = convert_signed(temp_signed);
                let temp_sign = if signed { -1 } else { 1 };
                temp_sign * magnitude as i8
            };
            Reading {
                temperature: temp,
                relative_humidity: rh,
            }
        }
    }

    impl DhtReading for Reading {}
}

pub mod dht22 {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Reading {
        pub temperature: f32,
        pub relative_humidity: f32,
    }

    impl internal::FromRaw for Reading {
        fn raw_to_reading(bytes: [u8; 4]) -> Reading {
            let [rh_h, rh_l, temp_h_signed, temp_l] = bytes;
            let rh = ((rh_h as u16) << 8 | (rh_l as u16)) as f32 / 10.0;
            let temp = {
                let (signed, magnitude) = convert_signed(temp_h_signed);
                let temp_sign = if signed { -1.0 } else { 1.0 };
                let temp_magnitude = ((magnitude as u16) << 8) | temp_l as u16;
                temp_sign * temp_magnitude as f32 / 10.0
            };
            Reading {
                temperature: temp,
                relative_humidity: rh,
            }
        }
    }

    impl DhtReading for Reading {}
}

fn convert_signed(signed: u8) -> (bool, u8) {
    let sign = signed & 0x80 != 0;
    let magnitude = signed & 0x7F;
    return (sign, magnitude);
}
