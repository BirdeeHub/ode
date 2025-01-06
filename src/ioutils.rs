use std::io::Read;
pub struct CharIterator<T: Read> {
    reader: T,
    buf: Vec<u8>,
}
impl<T: Read> CharIterator<T> {
    pub fn new(reader: T) -> CharIterator<T> {
        CharIterator {
            reader,
            buf: Vec::new(),
        }
    }
}
impl<T: Read> Iterator for CharIterator<T> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < 4 {
            let mut buf = [0; 4092];
            let Ok(bytes_read) = self.reader.read(&mut buf) else {
                return None;
            };
            self.buf.extend_from_slice(&buf[..bytes_read]);
        }
        for i in 1..5 {
            if i > self.buf.len() { break; };
            if let Some(c) = std::str::from_utf8(&self.buf[0..i]).ok().and_then(|s|s.chars().next()) {
                self.buf.drain(0..c.len_utf8());
                return Some(c);
            }
        }
        None
    }
}
