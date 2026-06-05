use std::fs::File;

use base64::display::Base64Display;
use everything::base::BASE;
use everything_tff::{encode::Encoder, parse::Parser};
use memmap2::Mmap;

fn main_decode() {
    let file = File::open("base.evts").unwrap();
    file.lock().unwrap();
    let content = unsafe { Mmap::map(&file) }.unwrap();
    let source = str::from_utf8(&content).unwrap();

    let root = Parser::new(source).parse_root().unwrap();

    println!("{:?}", &root == &*BASE)
}

fn main_encode() {
    let mut out = String::new();

    Encoder::new(&mut out).encode_root(BASE.clone()).unwrap();

    println!("{out}");
}

fn main() {
    main_decode();
}
