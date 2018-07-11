use super::character::VgaCharacter;
use volatile::Volatile;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

pub struct VgaBufferManager {
    line: usize,
    column: usize,
    content: &'static mut [[Volatile<VgaCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VgaBufferManager {
    pub fn new() -> VgaBufferManager {
        VgaBufferManager {
            line: 0,
            column: 0,
            content: unsafe {
                &mut *(0xb8000 as *mut [[Volatile<VgaCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT])
            },
        }
    }

    pub fn write_character(&mut self, character: VgaCharacter) {
        match character.character() {
            b'\n' => {
                self.increase_row_pointer();
                self.column = 0;
            }
            _ => {
                self.content[self.line][self.column].write(character);
                self.increase_column_pointer();
            }
        }
    }

    fn increase_row_pointer(&mut self) {
        if self.line >= BUFFER_HEIGHT {
            self.upshift();
        } else {
            self.line += 1;
        }
    }

    fn increase_column_pointer(&mut self) {
        if self.column >= BUFFER_WIDTH {
            self.increase_row_pointer();
        }
        self.column += 1;
    }

    fn upshift(&mut self) {
        for i in 1..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                let tmp = self.content[i][j].read();
                self.content[i - 1][j].write(tmp);
            }
        }
    }
}
