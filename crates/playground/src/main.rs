use std::fs::File;

use everything_tff::parse::Parser;
use memmap2::Mmap;

fn main() {
    let file = File::open("examples/example.evts").unwrap();
    let content = unsafe { Mmap::map(&file) }.unwrap();

    let source = str::from_utf8(&content).unwrap();

    let root = Parser::new(source).parse_root().unwrap();

    println!("{root:?}")
}
