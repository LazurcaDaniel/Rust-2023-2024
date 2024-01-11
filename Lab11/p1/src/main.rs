use std::fs::File;
use std::io::{Result, Write};

struct MyWriter {
    file: File,
}

impl MyWriter {
    fn new(file: File) -> MyWriter {
        MyWriter { file }
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut doubled_buf = Vec::with_capacity(buf.len() * 2);
        for &byte in buf.iter() {
            doubled_buf.push(byte);
            doubled_buf.push(byte);
        }
        self.file.write_all(&doubled_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}
