use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1][..];

    let content = get_file_as_bytes(&path);
    let range = content.len() / 16;

    for i in 0..range {
        print!("{:#01$X} |", i, 10);
        print!("{} | ", bytes_to_hex_string(&content[i * 16..i * 16 + 16]));
        print!("{}", bytes_to_string(&content[i * 16..i * 16 + 16]));
        println!();
    }
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let mut hex_string = String::new();

    for byte in bytes {
        let fmt = format!(" {:02X}", byte);
        hex_string += &fmt;
    }

    hex_string
}

fn get_file_as_bytes(path: &str) -> Vec<u8> {
    let mut f = fs::File::open(&path).expect("Couldn't open file");
    let metadata = fs::metadata(&path).expect("Couldn't read file metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Critical wtf moment");

    buffer
}

fn bytes_to_string(bytes: &[u8]) -> String {
    let mut temp = vec![];
    temp.extend_from_slice(bytes);
    String::from_utf8(temp)
        .expect("Only utf-8 supported")
        .replace("\n", " ")
        .replace("\r", " ")
}
