use volatile::Volatile;

#[allow(dead_code)] // Tells the compiler to ignore not used coded warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Auto generate implementations for these traits
#[repr(u8)] // Tells the compiler to represent each color with 1 byte digits (necessary for VGA)
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // ColorCode is basically an u8 wrapper, without it the compiler can change layout and include paddings.
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // The first nibble is the background color, the last is the text color (foreground)
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // It grants the struct to be equals in Rust and C (without Rust struct optimization)
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // Matrix with 80 ScreenChar's and 25 lines for it.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}


impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Ascii characters
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // Invalid characters
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position > BUFFER_WIDTH - 1 {
                    self.new_line();
                }

                let curr_row = BUFFER_HEIGHT - 1; // ?
                let curr_col = self.column_position;

                self.buffer.chars[curr_row][curr_col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {}
}

pub fn test() {
    static HELLO: &[u8] = b"Hello World!";
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // We can create a raw pointer to 0xb8000
        // casting the address to Buffer because Buffer has 4000 sequential bytes (80x25x2), matching with VGA Buffer.
    };

    writer.write_string("teste");
    writer.write_string("T e s t e");
    writer.write_string("öö");
}
