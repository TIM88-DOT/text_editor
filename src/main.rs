use std::io::{self, Read, Result, Write};
use std::os::fd::AsRawFd;
use termios::*;

const fn ctrl_key(c: u8) -> u8 {
    c & 0x1f
}

struct RawMode {
    orig_termios: Termios,
}

impl RawMode {
    fn new() -> Result<Self> {
        let orig_termios = Termios::from_fd(io::stdin().as_raw_fd())?;
        Ok(RawMode { orig_termios })
    }

    fn enable(&self) -> Result<()>{
        let mut raw = self.orig_termios.clone();
        raw.c_lflag &= !(ECHO);
        raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        raw.c_oflag &= !(OPOST);
        raw.c_cflag |= CS8;
        raw.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    
        raw.c_cc[VMIN] = 0;
        raw.c_cc[VTIME] = 1;
        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &raw)?;
        Ok(())
    }

    fn disable(&self) -> Result<()> {
        tcsetattr(io::stdin().as_raw_fd(), TCSAFLUSH, &self.orig_termios)?;
        Ok(())
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        self.disable().unwrap();
    }
}

fn main() {
    let raw_mode = RawMode::new().unwrap();
    raw_mode.enable().unwrap();
    loop {
        editor_refresh_screen().unwrap();
        editor_process_keypress(&raw_mode);
    }
}

fn editor_read_key() -> u8 {
    loop {
        let mut c = [0; 1];
        match io::stdin().read(&mut c) {
            Ok(_) => {
                return c[0];
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(ref e) => {
                eprintln!("read: {}", e);
                die("read");
            }
        }
    }
}

fn editor_draw_rows() -> Result<()> {
    for _ in 0..24 {
        io::stdout().write_all(b"~\r\n")?;
    }
    Ok(())
}

fn editor_process_keypress(raw_mode : &RawMode) {
    match editor_read_key() {
        c if ctrl_key(c) == ctrl_key(b'x') => {
            io::stdout().write_all(b"\x1b[2J\x1b[H").unwrap();
            io::stdout().flush().unwrap();
            raw_mode.disable().unwrap();
            std::process::exit(0)},
        _ => (),
    }
}

fn editor_refresh_screen() -> io::Result<()> {
    io::stdout().write_all(b"\x1b[2J\x1b[H")?;
    editor_draw_rows()?;
    io::stdout().write_all(b"\x1b[H")?;
    io::stdout().flush()?;
    Ok(())
}
fn die(s: &str) -> ! {
    io::stdout().write_all(b"\x1b[2J\x1b[H").unwrap();
    io::stdout().flush().unwrap();
    eprintln!("{}", s);
    std::process::exit(1);
}
