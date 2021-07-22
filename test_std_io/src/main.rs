use std::io;

use test_std_io::ReaderWriter;

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let mut std_rw = ReaderWriter::new(input, output);

    let answer = std_rw.write_then_read("What is your quest?\n");
    println!("\nYou response was: {}", answer);
}
