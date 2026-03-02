use std::fmt::Write;

fn inspect(data: &[u8]) -> Result<String, std::fmt::Error> {
    // len: offset 10 + hex 16*2 + spaces 8 + ascii 16 + newline 1
    // each line is the same since it's padding spaces until ascii
    let res_len = 67 * (data.len() / 16);
    let mut res = String::with_capacity(res_len);

    for (i, line) in data.chunks(16).enumerate() {
        let offset = i * 16;
        write!(&mut res, "{offset:08x}: ")?;

        let mut ascii_line = String::new();

        for (i, b) in line.iter().enumerate() {
            write!(&mut res, "{b:02x}")?;
            if i % 2 == 1 {
                write!(&mut res, " ")?;
            }
            if b.is_ascii_control() {
                ascii_line.push('.');
            } else {
                ascii_line.push(char::from(b.to_owned()));
            }
        }
        let padding = 40 - line.len() * 2 - line.len() / 2 + line.len();
        write!(&mut res, "{ascii_line:>padding$}\n", padding = padding)?;
    }
    Ok(res)
}

fn main() {
    let res = inspect(b"Hello World\nAnd others!").unwrap();
    println!("{res}");
    assert_eq!(
        inspect(b"Hello World\n"),
        Ok("00000000: 4865 6c6c 6f20 576f 726c 640a           Hello World.\n".to_string())
    );
}
