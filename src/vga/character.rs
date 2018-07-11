use super::color::Color;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VgaCharacter {
    character: u8,
    color: u8,
}

impl VgaCharacter {
    pub fn new(character: char, foreground: Color, background: Color) -> VgaCharacter {
        VgaCharacter {
            character: character as u8,
            color: (background as u8) << 4 | (foreground as u8),
        }
    }

    pub fn character(&self) -> u8 {
        return self.character;
    }
}
