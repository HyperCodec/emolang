use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

/// 1 represented by 👌, 0 represented by 💀
/// bytes separated by 👀
fn main() -> std::io::Result<()> {
    let path = env::args().collect::<Vec<String>>()[1..].join(" ");

    let start = Instant::now();
    let data: String = fs::read_to_string(&path)?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let bytes: Vec<&str> = data.split("👀").collect();

    let mut out = fs::File::create(format!(
        "{}{}",
        Path::new(&path)
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .expect("Unable to convert filename to string"),
        ".bin"
    ))?;
    let mut content: Vec<u8> = Vec::new();
    for byte in bytes {
        let mut val = 0u8;

        let mut binary: Vec<bool> = Vec::new();
        for bit in byte.chars() {
            match bit {
                '👌' => binary.push(true),
                '💀' => binary.push(false),
                _ => panic!("Unexpected token: '{}'", bit),
            }
        }

        if binary.len() > 8 {
            panic!("Too many bits in byte");
        }

        for i in 0..binary.len() {
            if binary[i] {
                val += 2u8.pow((binary.len() - 1 - i) as u32);
            }
        }

        content.push(val);
    }

    out.write_all(&content)?;

    println!("Compiled successfully ({:#?})", start.elapsed());

    Ok(())
}
