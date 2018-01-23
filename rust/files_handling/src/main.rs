use std::io::{BufReader, BufRead, BufWriter};
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::fs;


fn read(file_path: &str) -> Vec<String> {
    let mut lines_upper = vec![];
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Failed to open file");
    let file = BufReader::new(&file);
    for line in file.lines() {
        let current_line = line.unwrap();
        lines_upper.push(current_line.to_uppercase());
    }
    println!("{:?}", lines_upper);
    lines_upper
}

fn create_and_write(lines: &Vec<String>, file_name: &str) {
    let file = File::create(file_name).expect("Failed to create file");
    let mut file = BufWriter::new(file);
    for line in lines {
        writeln!(&mut file, "{}, Lenght {}", line, line.len()).expect("Failed to save line");
    }

}

fn delete(file_name: &str) {
    fs::remove_file(file_name).expect("Can't remove file");
}

fn main() {
    let lines_with_len = read("src/zen.txt");
    create_and_write(&lines_with_len, "src/zen_with_len.txt");
    delete("src/zen_with_len.txt");
}
