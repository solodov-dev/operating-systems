use std::fs;

fn main() {
    fs::write("/tmp/file", "hello, world\n").expect("Error writing file");
}
