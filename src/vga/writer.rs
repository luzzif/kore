use super::buffer_manager::VgaBufferManager;
use super::character::VgaCharacter;
use super::color::Color;
use core::fmt;
use spin::Mutex;

pub struct VgaWriter {
    foreground: Color,
    background: Color,
    buffer_manager: VgaBufferManager,
}

impl VgaWriter {
    pub fn new() -> VgaWriter {
        VgaWriter {
            foreground: Color::White,
            background: Color::Black,
            buffer_manager: VgaBufferManager::new(),
        }
    }

    pub fn write_character(&mut self, character: char) {
        self.buffer_manager.write_character(VgaCharacter::new(
            character,
            self.foreground,
            self.background,
        ));
    }

    pub fn write(&mut self, string: &str) {
        for &character in string.as_bytes() {
            self.buffer_manager.write_character(VgaCharacter::new(
                character as char,
                self.foreground,
                self.background,
            ));
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for character in string.chars() {
            self.write_character(character);
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref VGA_WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new());
}
