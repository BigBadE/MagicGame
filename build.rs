use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() {
    {
        let mut writer = BufWriter::new(File::create("target/output.json").unwrap());

        writer.write_all("[".as_bytes()).unwrap();
        read_recursively(Path::new("resources/game"), &mut writer);
        writer.write_all("{}]".as_bytes()).unwrap();

        writer.flush().unwrap();
    }
}

pub fn read_recursively(input: &Path, output: &mut BufWriter<File>) {
    if input.is_dir() {
        for file in input.read_dir().unwrap() {
            read_recursively(file.unwrap().path().as_path(), output);
        }
    } else {
        let mut contents = String::new();
        BufReader::new(File::open(input).unwrap()).read_to_string(&mut contents).unwrap();
        output.write_all(contents.as_bytes()).unwrap();
        output.write_all(",".as_bytes()).unwrap();
    }
}