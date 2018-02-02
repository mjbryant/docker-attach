extern crate termion;

use termion::{color, cursor};
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Read, Write};
use std::process::Command;


fn main() {
    let dps_cmd = Command::new("docker").arg("ps").output().expect(
        "Failed to run 'docker'");

    // Read stdout from the `docker ps` as a slice as u8 ints
    let mut output = dps_cmd.stdout.as_slice();
    let mut lines = vec![];

    // Split stdout on newline (10) characters, into lists of u8 ints
    // corresponding to each line.
    while let Some(i) = output.iter().position(|&r| r == 10) {
        let (first, rest) = output.split_at(i+1);
        output = rest;
        lines.push(first);
    }
    // Remove the header line
    lines.remove(0);

    let mut s = String::new();
    let si = stdin();
    loop {
        println!("{}", cursor::Goto(1, 1));
        for line in &lines {
            println!("{}{}", color::Fg(color::Red), String::from_utf8_lossy(line));
        }
        si.read_line(&mut s).unwrap();
    }
}

fn _main() {
    // Puts the terminal in "raw" mode, where there is:
    // * no line buffering (input comes in byte-by-byte
    // * the input isn't written to the terminal
    // * the input isn't canonicalized (e.g. \n means one line down, not line
    //   break)
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "Type some characters:\r\n").unwrap();
    stdout.flush().unwrap();
    let mut character = [0];
    while let Ok(_) = stdin().read(&mut character) {
        if character[0] == 3 {
            break
        } else {
            write!(stdout, "Character: {:?}\r\n", character[0]).unwrap();
            stdout.flush().unwrap();
        }
    }
}
