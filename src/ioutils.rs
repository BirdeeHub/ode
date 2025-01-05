use std::fs::File;
use std::io::{self, Bytes, Read};

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
pub struct CharIterator<T>
where
    T: std::io::Read,
{
    reader: Bytes<T>,
    buf: Vec<u8>,
}
impl<T> CharIterator<T>
where
    T: std::io::Read,
{
    pub fn new(reader: T) -> io::Result<CharIterator<T>> {
        Ok(CharIterator {
            reader: reader.bytes(),
            buf: Vec::new(),
        })
    }
}
impl<T> Iterator for CharIterator<T>
where
    T: std::io::Read,
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < 4 {
            while self.buf.len() < 4 {
                match self.reader.next() {
                    Some(Ok(b)) => {
                        self.buf.push(b);
                    }
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
