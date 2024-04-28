use std::io::{self, Read, Result};
use std::os::fd::AsRawFd;
use std::os::unix::io::RawFd;
use std::slice::from_mut;
use termios::*;

fn main() {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let original_termios = termios.clone();
    let _ = enable_raw_mode(stdin_fd, &mut termios);

    let mut c: u8 = 0;
    while io::stdin().read(from_mut(&mut c)).is_ok() && c != b'q' {
        // Continue reading until 'q' is entered
        if c.is_ascii_control() {
            println!("{}", c);
        } else {
            println!("{} ('{}')", c, c as char);
        }
    }

    disable_raw_mode(stdin_fd, &original_termios).unwrap();
}

fn disable_raw_mode(fd: RawFd, original_termios: &Termios) -> Result<()> {
    tcsetattr(fd, TCSAFLUSH, original_termios)?;
    Ok(())
}

fn enable_raw_mode(fd: RawFd, termios: &mut Termios) -> Result<()> {
    termios.c_lflag &= !(ECHO | ICANON);
    tcsetattr(fd, TCSAFLUSH, termios)?;
    Ok(())
}
