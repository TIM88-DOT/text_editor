use std::io::{self, stdin, Read, Result};
use std::os::fd::AsRawFd;
use std::os::unix::io::RawFd;
use std::slice::from_mut;
use termios::*;

fn main() {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let original_termios = termios.clone();
    let _ = enable_raw_mode(stdin_fd, &mut termios);

    loop {
        let mut c: u8 = 0;
        stdin().read(from_mut(&mut c)).expect("Invalid");
        if c.is_ascii_control() {
            println!("{}\r\n", c);
        } else {
            println!("{} ('{}')\r\n", c, c as char);
        }
        if c == b'q' {
            break 
        };
    }

    disable_raw_mode(stdin_fd, &original_termios).unwrap();
}

fn disable_raw_mode(fd: RawFd, original_termios: &Termios) -> Result<()> {
    tcsetattr(fd, TCSAFLUSH, original_termios)?;
    Ok(())
}

fn enable_raw_mode(fd: RawFd, termios: &mut Termios) -> Result<()> {
    termios.c_iflag &= !(ICRNL | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);

    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    tcsetattr(fd, TCSAFLUSH, termios)?;
    Ok(())
}



