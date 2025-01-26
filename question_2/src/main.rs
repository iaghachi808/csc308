#![no_std]
#![no_main]
use x86_64::instructions::hlt;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

pub struct FrameBufferWriter {
    framebuffer: *mut u8,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
    color: u8,
}

impl FrameBufferWriter {
    // Initializes the framebuffer writer
    pub fn new(framebuffer: *mut u8, width: usize, height: usize) -> Self {
        FrameBufferWriter {
            framebuffer,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            color: 0xb, // Default color: light cyan on black
        }
    }

    // Write a character at the current cursor position
    pub fn write_char(&mut self, c: u8) {
        let position = self.cursor_y * self.width + self.cursor_x;
        unsafe {
            *self.framebuffer.offset(position as isize * 2) = c; // ASCII value
            *self.framebuffer.offset(position as isize * 2 + 1) = self.color; // Attribute byte
        }

        // Update cursor
        self.cursor_x += 1;
        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        // If we overflow the screen height, scroll up
        if self.cursor_y >= self.height {
            self.scroll_up();
            self.cursor_y = self.height - 1;
        }
    }

    // Scroll the screen up by one line
    fn scroll_up(&mut self) {
        let len = self.width * self.height * 2;
        unsafe {
            for i in 0..len - self.width * 2 {
                *self.framebuffer.offset(i as isize) = *self.framebuffer.offset((i + self.width * 2) as isize);
            }
            // Clear the last line
            for i in len - self.width * 2..len {
                *self.framebuffer.offset(i as isize) = 0;
            }
        }
    }

    // Clear the screen
    pub fn clear_screen(&mut self) {
        for i in 0..self.width * self.height * 2 {
            unsafe {
                *self.framebuffer.offset(i as isize) = 0;
            }
        }
    }

    // Set a new color
    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    // Write a string, interpreting escape sequences
    pub fn write_str(&mut self, text: &[u8]) {
        let mut i = 0;
        while i < text.len() {
            match text[i] {
                b'\n' => {
                    // Newline: Move cursor to the start of the next line
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
                b'\t' => {
                    // Tab: Move the cursor to the next multiple of 8
                    self.cursor_x = (self.cursor_x + 8) / 8 * 8;
                }
                b'\x1b' => {
                    // Handle escape sequences: \c for color change
                    if i + 2 < text.len() && text[i + 1] == b'c' {
                        let color = match text[i + 2] {
                            b'R' => 0xc, // Red
                            b'G' => 0xa, // Green
                            b'B' => 0x9, // Blue
                            _ => self.color, // Default to current color
                        };
                        self.set_color(color);
                        i += 2; // Skip the \c and color character
                    }
                }
                _ => {
                    self.write_char(text[i]);
                }
            }

            i += 1;
        }
    }
}

// Define the print! macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        let framebuffer = 0xb8000 as *mut u8;
        let mut writer = $crate::FrameBufferWriter::new(framebuffer, 80, 25);

        // Convert arguments to bytes and write them to the framebuffer
        let text = concat!($($arg)*);
        writer.write_str(text.as_bytes());
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Use the print! macro
    print!("Hello, world!\nThis is a test.\nBlue Text\tIndented Text");

    loop {
        hlt();
    }
}
