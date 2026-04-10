pub mod ui;

use ui::UI;

use std::ptr;
use std::os::windows::io::AsRawHandle;
use winapi::um::consoleapi::{AllocConsole, GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::{FreeConsole, SetConsoleOutputCP};
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING, WriteFile};
use winapi::um::processenv::{GetStdHandle, SetStdHandle};
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::{GENERIC_WRITE, FILE_SHARE_WRITE, HANDLE};

const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

pub struct Console {
    handle: HANDLE,
    
    pub ui: UI
}

impl Console {
    pub unsafe fn init() -> Self {
        FreeConsole();
        AllocConsole();
        SetConsoleOutputCP(65001);

        let default_out = GetStdHandle(STD_OUTPUT_HANDLE);
        if default_out != INVALID_HANDLE_VALUE {
            let mut mode: u32 = 0;
            if GetConsoleMode(default_out, &mut mode) != 0 {
                SetConsoleMode(default_out, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }

        let handle = CreateFileA(
            b"CONOUT$\0".as_ptr() as _,
            GENERIC_WRITE,
            FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        );

        let mut private: u32 = 0;
        if GetConsoleMode(handle, &mut private) != 0 {
            SetConsoleMode(handle, private | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }

        let (_reader, writer) = os_pipe::pipe().unwrap();
        let raw_writer = writer.as_raw_handle();

        SetStdHandle(STD_OUTPUT_HANDLE, raw_writer as _);
        SetStdHandle(STD_ERROR_HANDLE, raw_writer as _);

        let ui = UI::init();
        Self { handle, ui }
    }

    pub fn log<S: AsRef<str>>(&self, message: S) {
        let formatted = format!("\r\x1b[2K\x1b[94m[INFO]\x1b[0m {}\n", message.as_ref());
        self.raw_write(formatted);
        self.update();
    }

    pub fn update(&self) {
        let content = self.ui.to_string();
        let lines: Vec<&str> = content.lines().collect();
        let line_count = lines.len();

        let mut seq = String::from("\x1b[s");

        for (i, line) in lines.iter().enumerate() {
            let move_up = line_count - 1 - i;
            if move_up > 0 {
                seq.push_str(&format!("\x1b[999;1H\x1b[{}A\x1b[2K", move_up));
            } else {
                seq.push_str("\x1b[999;1H\x1b[2K");
            }
            
            seq.push_str("\x1b[1;33m");
            seq.push_str(line);
            seq.push_str("\x1b[0m");
        }

        seq.push_str("\x1b[u");
        self.raw_write(seq);
    }

    pub fn raw_write<S: AsRef<str>>(&self, text: S) {
        unsafe {
            let mut written = 0;
            let s = text.as_ref();
            WriteFile(
                self.handle,
                s.as_ptr() as _,
                s.len() as u32,
                &mut written,
                ptr::null_mut(),
            );
        }
    }
}