use std::io::{BufRead, BufReader, Read};

pub struct Reader<R> {
    pub(crate) reader: BufReader<R>,
}

impl<R> Read for Reader<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R> BufRead for Reader<R>
where
    R: Read,
{
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

impl<R> From<R> for Reader<R>
where
    R: Read,
{
    fn from(reader: R) -> Self {
        Reader {
            reader: BufReader::new(reader),
        }
    }
}
