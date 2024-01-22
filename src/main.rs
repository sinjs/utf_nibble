use std::env::args;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use ux::u4;
use std::os::unix::fs::FileExt;
use std::io::prelude::*;
use std::mem::size_of;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Letter {
    a = 0b0000,
    b = 0b0001,
    d = 0b0010,
    e = 0b0011,
    f = 0b0100,
    g = 0b0101,
    i = 0b0110,
    j = 0b0111,
    k = 0b1000,
    n = 0b1001,
    m = 0b1010,
    o = 0b1011,
    p = 0b1100,
    r = 0b1101,
    s = 0b1110,
    t = 0b1111,
}

impl Letter {
    fn get_value(&self) -> u4 {
        use Letter::*;
        u4::new(*self as u8)
    }

    fn to_char(&self) -> char {
        match self {
            Letter::a => 'a',
            Letter::b => 'b',
            Letter::d => 'd',
            Letter::e => 'e',
            Letter::f => 'f',
            Letter::g => 'g',
            Letter::i => 'i',
            Letter::j => 'j',
            Letter::k => 'k',
            Letter::n => 'n',
            Letter::m => 'm',
            Letter::o => 'o',
            Letter::p => 'p',
            Letter::r => 'r',
            Letter::s => 's',
            Letter::t => 't',
        }
    }
}

impl From<u8> for Letter {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => Letter::a,
            0b0001 => Letter::b,
            0b0010 => Letter::d,
            0b0011 => Letter::e,
            0b0100 => Letter::f,
            0b0101 => Letter::g,
            0b0110 => Letter::i,
            0b0111 => Letter::j,
            0b1000 => Letter::k,
            0b1001 => Letter::n,
            0b1010 => Letter::m,
            0b1011 => Letter::o,
            0b1100 => Letter::p,
            0b1101 => Letter::r,
            0b1110 => Letter::s,
            0b1111 => Letter::t,
            _ => panic!("The Nibble is too big for UTF-Nibble™"),
        }
    }
}

impl From<char> for Letter {
    fn from(value: char) -> Self {
        match value {
            'a' => Letter::a,
            'b' => Letter::b,
            'd' => Letter::d,
            'e' => Letter::e,
            'f' => Letter::f,
            'g' => Letter::g,
            'i' => Letter::i,
            'j' => Letter::j,
            'k' => Letter::k,
            'n' => Letter::n,
            'm' => Letter::m,
            'o' => Letter::o,
            'p' => Letter::p,
            'r' => Letter::r,
            's' => Letter::s,
            't' => Letter::t,
            _ => panic!("Character {} not supported in UTF-Nibble™", value)
        }
    }
}

fn encode_str(input: &str) -> Vec<u8> {
    let mut letters: Vec<Letter> = vec![];
    let mut bytes: Vec<u8> = vec![];

    for char in input.chars() {
        letters.push(Letter::from(char));
    }

    for letter in letters {
        bytes.push(u8::from(letter.get_value()));
    }

    bytes
}

fn decode_bytes(input: Vec<u8>) -> String {
    let mut letters: Vec<Letter> = vec![];

    for byte in input {
        letters.push(Letter::from(byte));
    };

    let chars: String = letters.into_iter().map(|letter| letter.to_char()).collect();

    chars
}

fn main() {
    let args: Vec<String> = args().collect();

    if (args.len() < 3) {
        panic!("Not enough arguments.")
    }

    match args.get(1).unwrap().as_str() {
        "encode" => {
            let filename = args.get(2).unwrap();
            let content = std::fs::read_to_string(filename).unwrap();
            let encoded = encode_str(content.as_str());

            std::io::stdout().write_all(&*encoded).unwrap();
        },
        "decode" => {
            let filename = args.get(2).unwrap();
            let mut buffer = fs::read(filename).unwrap();
            let decoded = decode_bytes(buffer);

            print!("{}", decoded);
        },
        _ => panic!("Invalid usage: {} [encode|decode] [file]", args.get(0).unwrap())
    }
}
