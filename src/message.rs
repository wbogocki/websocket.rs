use crate::*;

impl<T> Message for &T
where
    T: Message + ?Sized,
{
    #[inline]
    fn frame_type(&self) -> (bool, u8, &[u8]) {
        T::frame_type(self)
    }
}

impl Message for str {
    #[inline]
    fn frame_type(&self) -> (bool, u8, &[u8]) {
        (true, 1, self.as_bytes())
    }
}

impl Message for [u8] {
    #[inline]
    fn frame_type(&self) -> (bool, u8, &[u8]) {
        (true, 2, self)
    }
}

impl<const N: usize> Message for [u8; N] {
    #[inline]
    fn frame_type(&self) -> (bool, u8, &[u8]) {
        (true, 2, self)
    }
}

impl<T> Message for Ping<T>
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn frame_type(&self) -> (bool, u8, &[u8]) {
        (true, 9, self.0.as_ref())
    }
}

impl<T> Message for Pong<T>
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn frame_type(&self) -> (bool, u8, &[u8]) {
        (true, 10, self.0.as_ref())
    }
}

// ------------------------------------------------------------------------------

impl CloseFrame for () {
    type Frame = &'static [u8];
    fn encode<const SIDE: bool>(self) -> Self::Frame {
        if SIDE == SERVER {
            &[136, 0]
        } else {
            &[136, 128, 0, 0, 0, 0]
        }
    }
}

impl CloseFrame for u16 {
    type Frame = Vec<u8>;
    fn encode<const SIDE: bool>(self) -> Self::Frame {
        todo!()
        // let mut bytes = Vec::new();
        // encode::<SIDE>(&mut bytes, true, 8, &self.to_be_bytes());
        // bytes
    }
}

impl CloseFrame for CloseCode {
    type Frame = Vec<u8>;

    fn encode<const SIDE: bool>(self) -> Self::Frame {
        todo!()
        // CloseFrame::encode::<SIDE>(u16::from(self))
    }
}

// impl<Code, Msg> CloseFrame for (Code, Msg)
// where
//     Code: Into<u16>,
//     Msg: AsRef<[u8]>,
// {
//     type Frame = Vec<u8>;

//     fn encode<const SIDE: bool>(self) -> Self::Frame {
//         let (code, reason) = (self.0.into(), self.1.as_ref());
//         let mut data = Vec::with_capacity(2 + reason.len());
//         data.extend_from_slice(&code.to_be_bytes());
//         data.extend_from_slice(reason);

//         let mut bytes = Vec::new();
//         encode::<SIDE>(&mut bytes, true, 8, &data);
//         bytes
//     }
// }

impl CloseFrame for &str {
    type Frame = Vec<u8>;
    fn encode<const SIDE: bool>(self) -> Self::Frame {
        todo!()
        // CloseFrame::encode::<SIDE>((CloseCode::Normal, self))
    }
}

// ------------------------------------------------------------------------------

// pub(crate) fn encode<const SIDE: bool>(writer: &mut Vec<u8>, fin: bool, opcode: u8, data: &[u8]) {
//     let data_len = data.len();
//     writer.reserve(if SERVER == SIDE { 10 } else { 14 } + data_len);
//     unsafe {
//         let filled = writer.len();
//         let start = writer.as_mut_ptr().add(filled);

//         let mask_bit = if SERVER == SIDE { 0 } else { 0x80 };

//         start.write(((fin as u8) << 7) | opcode);
//         let len = if data_len < 126 {
//             start.add(1).write(mask_bit | data_len as u8);
//             2
//         } else if data_len < 65536 {
//             let [b2, b3] = (data_len as u16).to_be_bytes();
//             start.add(1).write(mask_bit | 126);
//             start.add(2).write(b2);
//             start.add(3).write(b3);
//             4
//         } else {
//             let [b2, b3, b4, b5, b6, b7, b8, b9] = (data_len as u64).to_be_bytes();
//             start.add(1).write(mask_bit | 127);
//             start.add(2).write(b2);
//             start.add(3).write(b3);
//             start.add(4).write(b4);
//             start.add(5).write(b5);
//             start.add(6).write(b6);
//             start.add(7).write(b7);
//             start.add(8).write(b8);
//             start.add(9).write(b9);
//             10
//         };

//         let header_len = if SERVER == SIDE {
//             std::ptr::copy_nonoverlapping(data.as_ptr(), start.add(len), data_len);
//             len
//         } else {
//             let mask = rand::random::<u32>().to_ne_bytes();
//             let [a, b, c, d] = mask;
//             start.add(len).write(a);
//             start.add(len + 1).write(b);
//             start.add(len + 2).write(c);
//             start.add(len + 3).write(d);

//             let dist = start.add(len + 4);
//             // TODO: Use SIMD wherever possible for best performance
//             for i in 0..data_len {
//                 dist.add(i)
//                     .write(data.get_unchecked(i) ^ mask.get_unchecked(i & 3));
//             }
//             len + 4
//         };
//         writer.set_len(filled + header_len + data_len);
//     }
// }

// #[cfg(test)]
// mod encode {
//     use super::*;
//     const DATA: &[u8] = b"Hello";

//     #[test]
//     fn unmasked_txt_msg() {
//         let mut bytes = vec![];
//         encode::<SERVER>(&mut bytes, true, 1, DATA);
//         assert_eq!(bytes, [0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]);
//     }

//     #[test]
//     fn fragmented_unmasked_txt_msg() {
//         let mut bytes = vec![];
//         encode::<SERVER>(&mut bytes, false, 1, b"Hel");
//         encode::<SERVER>(&mut bytes, true, 0, b"lo");
//         assert_eq!(
//             bytes,
//             [
//                 0x01, 0x03, 0x48, 0x65, 0x6c, // fragmented frame
//                 0x80, 0x02, 0x6c, 0x6f, // final frame
//             ]
//         );
//     }

//     #[test]
//     fn unmasked_ping_req() {
//         let mut bytes = vec![];
//         encode::<SERVER>(&mut bytes, true, 9, DATA);
//         assert_eq!(bytes, [0x89, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,]);
//     }
// }
