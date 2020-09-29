use specs::{Component, VecStorage};
use termion::color;

#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    White,
    Yellow,
    Reset,
    // Default,
}

// #[derive(Debug, Clone, Copy)]
// pub enum ColorType {
//     Black(color::Black),
//     Blue(color::Blue),
//     Cyan(color::Cyan),
//     Green(color::Green),
//     LightBlack(color::LightBlack),
//     LightBlue(color::LightBlue),
//     LightCyan(color::LightCyan),
//     LightGreen(color::LightGreen),
//     LightMagenta(color::LightMagenta),
//     LightRed(color::LightRed),
//     LightWhite(color::LightWhite),
//     LightYellow(color::LightYellow),
//     Magenta(color::Magenta),
//     Red(color::Red),
//     White(color::White),
//     Yellow(color::Yellow),
// }

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Color {
    color: ColorType,
}

impl Color {
    pub fn new(color: ColorType) -> Self {
        Self { color }
    }

    pub fn get_color(self) -> &'static dyn color::Color {
        match self.color {
            ColorType::Black => &color::Black,
            ColorType::Blue => &color::Blue,
            ColorType::Cyan => &color::Cyan,
            ColorType::Green => &color::Green,
            ColorType::LightBlack => &color::LightBlack,
            ColorType::LightBlue => &color::LightBlue,
            ColorType::LightCyan => &color::LightCyan,
            ColorType::LightGreen => &color::LightGreen,
            ColorType::LightMagenta => &color::LightMagenta,
            ColorType::LightRed => &color::LightRed,
            ColorType::LightWhite => &color::LightWhite,
            ColorType::LightYellow => &color::LightYellow,
            ColorType::Magenta => &color::Magenta,
            ColorType::Red => &color::Red,
            ColorType::White => &color::White,
            ColorType::Yellow => &color::Yellow,
            ColorType::Reset => &color::Reset,
            // ColorType::Default => &color::White,
        }
    }
}
