use std::process::exit;

use everything_structures_ff::ErrorInfo;

fn lc_from_index(source: &str, index: u32) -> (u32, u32) {
    let slice = &source[..index as usize];

    let mut lines = 0;
    let mut cr = false;

    let mut chars = slice.chars();
    let mut last_line = slice;

    while let Some(c) = chars.next() {
        if c == '\r' {
            cr = true;

            lines += 1;
            last_line = chars.as_str();
        } else if c == '\n' {
            if !cr {
                lines += 1;
                last_line = chars.as_str();
            }

            cr = false;
        } else {
            cr = false;
        }
    }

    (lines, last_line.chars().count() as u32)
}

pub fn handle_parse_error(input: &str, error: &ErrorInfo) -> ! {
    match &error.found {
        Some(found) => {
            let (start_line, start_col) = lc_from_index(input, found.range.start);
            let (end_line, end_col) = lc_from_index(input, found.range.end);

            eprintln!(
                "error while parsing at {}:{} (to {}:{}): found {:?}, expected {:?}",
                start_line + 1,
                start_col + 1,
                end_line + 1,
                end_col + 1,
                error.found,
                error.expected
            )
        }
        None => {
            eprintln!(
                "error while parsing at the end: expected {:?}",
                error.expected
            )
        }
    }

    exit(-1)
}
