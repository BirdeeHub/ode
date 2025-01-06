use std::io::Read;
pub struct CharIterator<T: Read>
{
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
        if self.buf.is_empty() {
            let mut buf = [0; 4096];
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
