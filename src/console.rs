pub mod commands;

use std::ptr;
use std::io;
use std::os::windows::io::AsRawHandle;
use winapi::um::consoleapi::{AllocConsole, GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::FreeConsole;
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING, WriteFile};
use winapi::um::processenv::{GetStdHandle, SetStdHandle};
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::{GENERIC_WRITE, FILE_SHARE_WRITE, HANDLE};

const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

static mut CONSOLE_INIT: bool = false;
static mut out: HANDLE = INVALID_HANDLE_VALUE;

pub unsafe fn init_console() {
    if CONSOLE_INIT { return }
    CONSOLE_INIT = true;

    FreeConsole();
    AllocConsole();

    let default_out = GetStdHandle(STD_OUTPUT_HANDLE);
    if default_out != INVALID_HANDLE_VALUE {
        let mut mode: u32 = 0;
        if GetConsoleMode(default_out, &mut mode) != 0 {
            SetConsoleMode(default_out, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }

    out = CreateFileA(
        b"CONOUT$\0".as_ptr() as _,
        GENERIC_WRITE,
        FILE_SHARE_WRITE,
        ptr::null_mut(),
        OPEN_EXISTING,
        0,
        ptr::null_mut(),
    );

    let mut private: u32 = 0;
    if GetConsoleMode(out, &mut private) != 0 {
        SetConsoleMode(out, private | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }

    let (_reader, writer) = os_pipe::pipe().unwrap();
    let raw_writer = writer.as_raw_handle();

    SetStdHandle(STD_OUTPUT_HANDLE, raw_writer as _);
    SetStdHandle(STD_ERROR_HANDLE, raw_writer as _);

    std::thread::spawn(move || {
        let mut input = String::new();

        loop {
            input.clear();
            if io::stdin().read_line(&mut input).is_ok() {
                let cmd = input.trim();
                if cmd.is_empty() {
                    raw_write("\x1b[1A\r>> ");
                } else {
                    commands::run(cmd);
                }
            }
        }
    });
}

pub fn log<S: AsRef<str>>(message: S) {
    let formatted = format!("\r\x1b[2K\x1b[32m[LOG]\x1b[0m {}\n>> ", message.as_ref());
    raw_write(formatted);
}

fn raw_write<S: AsRef<str>>(text: S) {
    unsafe {
        if out == INVALID_HANDLE_VALUE { return; }
        let mut written = 0;
        let s = text.as_ref();
        WriteFile(
            out,
            s.as_ptr() as _,
            s.len() as u32,
            &mut written,
            ptr::null_mut(),
        );
    }
}