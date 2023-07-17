use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, IntEnum, Copy, Clone, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    return _color as u32;
}

fn color_to_string(_color: ResistorColor) -> String {
    return format!("{:?}", _color);
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => color_to_string(color),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    vec.sort();
    return vec;
}
