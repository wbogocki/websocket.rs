use tokio::io::AsyncBufReadExt;
use super::*;

macro_rules! default_impl_for_data {
    () => {
        impl Data<'_> {
            #[inline] pub fn ty(&self) -> DataType {
                self.ty
            }
            #[inline] pub fn len(&self) -> usize {
                self.len
            }

            #[inline] pub async fn read_exact(&mut self, mut buf: &mut [u8]) -> io::Result<()> {
                while !buf.is_empty() {
                    match self.read(buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            let tmp = buf;
                            buf = &mut tmp[n..];
                        }
                        Err(e) => return Err(e),
                    }
                }
                if !buf.is_empty() {
                    Err(io::Error::new(io::ErrorKind::UnexpectedEof,"failed to fill whole buffer",))
                } else {
                    Ok(())
                }
            }

            #[inline] pub async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
                let start = buf.as_mut_ptr();
                let mut end = unsafe { start.add(buf.len()) };
                let amt = end as usize;
                while self.len > 0 {
                    let additional = self.len;
                    buf.reserve(additional);
                    unsafe {
                        let uninit = std::slice::from_raw_parts_mut(end, additional);
                        self.read_exact(uninit).await?;
                        end = end.add(additional);
                    }
                }
                unsafe { buf.set_len(end as usize - start as usize) };
                Ok(end as usize - amt)
            }
        }    
    };
}

pub(crate) use default_impl_for_data;

pub async fn read_buf<const N: usize>(stream: &mut BufReader<TcpStream>) -> io::Result<[u8; N]> {
    let mut buf = [0; N];
    stream.read_exact(&mut buf).await?;
    Ok(buf)
}

#[inline]
pub async fn read_bytes<F>(stream: &mut BufReader<TcpStream>, len: usize, cb: F) -> io::Result<usize>
where
    F: FnOnce(&[u8]) -> usize,
{
    let bytes = stream.fill_buf().await?;
    let amt = bytes.len().min(len);
    let count = cb(unsafe { bytes.get_unchecked(..amt) });
    stream.consume(amt);
    Ok(count)
}
