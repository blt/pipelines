const HEADERS: [&[u8]; 8] = [
    b"[spaces: 0]",
    b"[spaces: 1]",
    b"[spaces: 2]",
    b"[spaces: 3]",
    b"[spaces: 4]",
    b"[spaces: 5]",
    b"[spaces: 6]",
    b"[spaces: >6]",
];

pub fn get_header(spaces: usize) -> &'static [u8] {
    HEADERS[spaces.min(HEADERS.len() - 1)]
}

pub fn total_spaces(s: &str) -> u32 {
    s.chars()
        .fold(0, |acc, c| if c == ' ' { acc + 1 } else { acc })
}
