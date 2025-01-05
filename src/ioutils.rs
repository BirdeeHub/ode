use std::fs::File;
use std::io::{self, Bytes, Read, BufReader};

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
pub struct CharIterator {
    reader: Bytes<BufReader<File>>,
    buf: Vec<u8>,
}
impl CharIterator {
    pub fn new(filepath: &str) -> io::Result<CharIterator> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file).bytes();
        Ok(CharIterator { reader, buf: Vec::new() })
    }
}
impl Iterator for CharIterator {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < 4 {
            while self.buf.len() < 4 {
                match self.reader.next() {
                    Some(Ok(b)) => {
                        self.buf.push(b);
                    },
                    _ => break,
                }
            }
        }
        let Ok(charval1) = std::str::from_utf8(&self.buf) else {
            return None;
        };
        let charval = charval1.chars().next();
        match charval {
            Some(c) => self.buf.drain(0..c.len_utf8()),
            _ => return None,
        };
        charval
    }
}

