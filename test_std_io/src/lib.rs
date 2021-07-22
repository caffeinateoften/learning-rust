use std::io::{BufRead, Write};
pub struct ReaderWriter<R, W> {
    reader: R,
    writer: W
}

impl<R, W> ReaderWriter<R, W>
where
    R: BufRead,
    W: Write
    {
        pub fn new(r: R, w: W) -> ReaderWriter<R, W> {
            ReaderWriter {
                reader: r,
                writer: w,
            }
        }
        pub fn write_then_read(&mut self, question: &str) -> String {
            if let Err(err) = write!(&mut self.writer, "{}", question){
                eprintln!("Could not write output: {}", err);
            }
            if let Err(err) = self.writer.flush() {
                eprintln!("Could not flush output: {}", err);
            }
            let mut s = String::new();
            if let Err(err) = self.reader.read_line(&mut s){
                eprintln!("Could not read input: {}", err);
            }
            s
        }
    }

#[test]
fn test_reader_writer_in_memory() {
    let input = b"To seek the Holy Grail";
    let mut output = Vec::new();

    let mut in_memory_rw = ReaderWriter::new(&input[..], &mut output);

    let answer = in_memory_rw.write_then_read("What is your quest?");

    let output = String::from_utf8(output).expect("Not UTF-8");

    assert_eq!("What is your quest?", output);
    assert_eq!("To seek the Holy Grail", answer);
}
