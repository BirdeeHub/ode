use std::fs::File;
use std::io::{self, Read, BufReader};
use std::cmp::min;

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn mk_buf_reader(filepath: &str) -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open(filepath)?))
}

pub struct CharIterator<T>
where
    T: std::io::Read,
{
    reader: T,
    buf: Vec<u8>,
    buf_len: usize,
}
impl<T> CharIterator<T>
where
    T: std::io::Read,
{
    pub fn new(buf_len: usize, reader: T) -> CharIterator<T> {
        CharIterator {
            reader,
            buf: Vec::new(),
            buf_len,
        }
    }
}
impl<T> Iterator for CharIterator<T>
where
    T: std::io::Read,
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < 4 {
            let mut buf = vec![0; self.buf_len];
            let Ok(bytes_read) = self.reader.read(&mut buf) else {
                return None;
            };
            self.buf.extend_from_slice(&buf[..bytes_read]);
        }
        for i in 0..4 {
            if i >= self.buf.len() { break; };
            if let Ok(s) = std::str::from_utf8(&self.buf[0..i]) {
                if let Some(c) = s.chars().next() {
                    let char_len = c.len_utf8();
                    self.buf.drain(0..char_len);
                    return Some(c);
                }
            }
        }
        None
    }
}
