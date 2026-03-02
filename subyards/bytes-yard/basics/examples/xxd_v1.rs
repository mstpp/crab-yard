use std::fmt::Write;

fn inspect(data: &[u8]) -> String {
    let mut offset = 0_usize;
    let mut res = String::new();
    let mut ascii_line = String::new();

    data.iter().enumerate().for_each(|(i, b)| {
        if b.is_ascii_control() {
            ascii_line.push('.');
        } else {
            ascii_line.push(char::from(b.clone()));
        }
        match i % 16 {
            0 => {
                write!(&mut res, "{offset:08x}: {b:02x}").unwrap();
                offset += 16;
            }
            15 => {
                write!(&mut res, "{b:02x} {ascii_line}\n").unwrap();
                ascii_line.clear();
            }
            1 | 3 | 5 | 7 | 9 | 11 | 13 => write!(&mut res, "{b:02x} ").unwrap(),
            _ => write!(&mut res, "{b:02x}").unwrap(),
        }
    });
    let rem_bytes = 16 - data.len() % 16;
    let rem_spaces = rem_bytes / 2;
    let space_count = rem_bytes * 2 + rem_spaces;
    let spaces = " ".to_string().repeat(space_count);
    write!(&mut res, "{spaces} {ascii_line}").unwrap();

    res
}

fn main() {
    let res = inspect(b"Hello World!And others!");
    println!("{res}");
    assert_eq!(
        inspect(b"Hello World\n"),
        "00000000: 4865 6c6c 6f20 576f 726c 640a            Hello World.".to_string()
    );
}
