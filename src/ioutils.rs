pub struct CharIterator<T: std::io::Read> {
    reader: T,
    buf: Vec<u8>,
}
impl<T: std::io::Read> CharIterator<T> {
    pub fn new(reader: T) -> CharIterator<T> {
        CharIterator { reader, buf: Vec::new(), }
    }
}
impl<T: std::io::Read> Iterator for CharIterator<T> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < 4 {
            let mut buf = [0; 64];
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
