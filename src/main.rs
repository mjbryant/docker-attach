extern crate termion;

use std::io::{stdin, stdout, Read, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;
use termion::{color, cursor, style, terminal_size};
use termion::cursor::DetectCursorPos;
use termion::raw::IntoRawMode;

struct ContainerInfo {
    id: String,
    image: String,
    name: String,
}

fn get_container_info(line: &[u8]) -> ContainerInfo {
    let s = String::from_utf8_lossy(line);
    let mut iter = s.split_whitespace();
    // ID and IMAGE are the first two words in the line
    let id = iter.next().unwrap();
    let image = iter.next().unwrap();
    // NAME is the last word in the line
    let mut name = "";
    while let Some(word) = iter.next() {
        name = word;
    }
    return ContainerInfo {
        id: String::from(id),
        image: String::from(image),
        name: String::from(name),
    };
}

fn draw_lines(
    containers: &Vec<ContainerInfo>,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    highlighted_line: usize,
    x: u16,
    y: u16,
) {
    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
    stdout.flush().unwrap();
    let mut leader: &str;
    for (i, container) in containers.iter().enumerate() {
        if i == highlighted_line {
            leader = "--> ";
        } else {
            leader = "    ";
        }
        write!(
            stdout,
            "{}{}{} {}({} - {}){}\r\n",
            leader,
            color::Fg(color::Green),
            container.name,
            color::Fg(color::Red),
            container.image,
            container.id,
            style::Reset,
        ).unwrap();
    }
    stdout.flush().unwrap();
}

fn main() {
    let dps_cmd = Command::new("docker").arg("ps").output().expect(
        "Failed to run 'docker'");

    // Read stdout from the `docker ps` as a slice as u8 ints
    let mut output = dps_cmd.stdout.as_slice();
    let mut containers = vec![];

    // Split stdout on newline (10) characters, into lists of u8 ints
    // corresponding to each line.
    while let Some(i) = output.iter().position(|&r| r == 10) {
        let (first, rest) = output.split_at(i + 1);
        output = rest;
        containers.push(get_container_info(first));
    }
    // Remove the header line
    containers.remove(0);

    let mut highlighted_line = 0;
    let mut do_exec = true;
    // Put the stdout inside an explicit context so it gets dropped before we
    // exec into the docker container. Otherwise the raw mode setting will
    // persist after the program exits.
    {
        let mut stdout = stdout().into_raw_mode().expect(
            "Failed to get terminal in raw mode");
        // Original position on the screen, so we can overwrite the output.
        // This apparently causes the first byte of user input to get swalled;
        // it's a known termion error, and nbd, since all it means is you have to
        // push e.g. 'j' twice before it goes down.
        let (x, mut y) = stdout.cursor_pos().expect("Failed to get cursor_pos");
        let (_, size_y) = terminal_size().expect("Failed to get terminal size");

        draw_lines(&containers, &mut stdout, highlighted_line, x, y);

        if size_y == y {
            y = y - containers.len() as u16;
        }

        let mut character = [0];
        while let Ok(_) = stdin().read(&mut character) {
            if character[0] == 3 || character[0] == 27 {  // ctrl+c or esc
                do_exec = false;
                break;
            } else if character[0] == 106 {  // 'j'
                highlighted_line = (highlighted_line + 1) % containers.len();
                draw_lines(&containers, &mut stdout, highlighted_line, x, y);
            } else if character[0] == 107 {  // 'k'
                if highlighted_line == 0 {
                    highlighted_line = containers.len() - 1;
                } else {
                    highlighted_line -= 1;
                }
                draw_lines(&containers, &mut stdout, highlighted_line, x, y);
            } else if character[0] == 13 {  // enter
                write!(stdout, "{}", style::Reset).unwrap();
                stdout.flush().unwrap();
                break;
            }
        }
    }
    if do_exec {
        Command::new("docker")
            .args(&["exec", "-ti", &containers[highlighted_line].id, "/bin/bash"])
            .exec();
    }
}
