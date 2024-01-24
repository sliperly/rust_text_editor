mod editor;

use std::io::{self, Read, Write};
use std::io::{Stdin, Bytes};
use termion::raw::RawTerminal;
use termion::raw::IntoRawMode;

const MASK: u8 = 0b0001_1111;

fn main() {
    let _raw_terminal: RawTerminal<std::io::Stdout> = match io::stdout().into_raw_mode() {
        Ok(raw) => raw,
        Err(e) => {
            panic!("Failed to enter raw mode: {:?}", e);
        }
    };
    let stdin: Stdin = io::stdin();
    let bytes_iter: Bytes<Stdin> = stdin.bytes();
    for byte in bytes_iter {
        let byte: u8 = byte.unwrap();
        let c: char = byte as char;
        print!("byte: {}\r", byte);
        print!("char: {}\r", byte as char);
        print!("binary: {:#b}\r", byte);
        if byte == ('q' as u8 & MASK) {
            break;
        }

        std::io::stdout().flush().unwrap();
    }
}

//use std::io::{self, Read};

//fn main() {
//    let stdin: Stdin = io::stdin();
//    let bytes_iter: Bytes<Stdin> = stdin.bytes();
//    for byte: Result<u8, Err0r> in bytes_iter {
//        let byte: u8 = byte.unwrap();
//        println!("{} {}", byte as char, byte);
//    }
//}
