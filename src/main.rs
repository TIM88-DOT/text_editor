use std::io::{self, Read, Result};
use std::os::fd::AsRawFd;
use std::os::unix::io::RawFd;
use std::slice::from_mut;
use termios::*;


fn main() {
    let stdin_fd = io::stdin().as_raw_fd();
    let _ = enable_raw_mode(stdin_fd);

    let mut c: u8 = 0;
    while io::stdin().read_exact(from_mut(&mut c)).is_ok() && c != b'q' {
        // Continue reading until 'q' is entered
    }
}

fn enable_raw_mode(fd : RawFd) -> Result<()> {
    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag = !(ECHO);
    tcsetattr(fd, TCSANOW, &termios).unwrap();
    Ok(())
}


