//! This module contain some utility function to work with http protocol.

use super::*;
use std::collections::HashMap;

/// Provides a interface for formatting HTTP headers
///
/// # Example
///
/// ```rust
/// use web_socket::http::Header;
///
/// assert_eq!(Header::fmt(&("val", 2)), "val: 2\r\n");
/// assert_eq!(Header::fmt(&["key", "value"]), "key: value\r\n");
/// ```
pub trait Header {
    /// Format a single http header field
    fn fmt(_: &Self) -> String;
}

impl<T: Header> Header for &T {
    fn fmt(this: &Self) -> String {
        T::fmt(this)
    }
}
impl<T: fmt::Display> Header for [T; 2] {
    fn fmt([key, value]: &Self) -> String {
        format!("{key}: {value}\r\n")
    }
}
impl<K: fmt::Display, V: fmt::Display> Header for (K, V) {
    fn fmt((key, value): &Self) -> String {
        format!("{key}: {value}\r\n")
    }
}

/// It represents an HTTP message with a schema and a header.
///
/// ### Example
///
/// ```rust
/// let mut bytes = "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n".as_bytes();
/// let header = web_socket::http::Http::parse(&mut bytes).unwrap();
///
/// assert_eq!(header.schema, "HTTP/1.1 101 Switching Protocols".as_bytes());
/// assert_eq!(header.get("upgrade"), Some("websocket".as_bytes()));
/// assert_eq!(
///     header.get("sec-websocket-accept"),
///     Some("s3pPLMBiTxaQ9kYGzzhZRbK+xOo=".as_bytes())
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Http {
    /// prefix of the http message (e.g. `HTTP/1.1 101 Switching Protocols`)
    pub prefix: String,
    ///  key-value pairs of http headers
    pub headers: HashMap<String, String>,
}

struct Reader<T>(T);

impl<T: AsyncRead + Unpin> Reader<T> {
    async fn get_byte(&mut self) -> Result<u8> {
        let mut byte = [0; 1];
        self.0.read_exact(&mut byte).await?;
        Ok(byte[0])
    }

    async fn split_once(&mut self, ascii: u8) -> Result<String> {
        let mut bytes = Vec::new();
        loop {
            let byte = self.get_byte().await?;
            if byte == ascii {
                return match String::from_utf8(bytes) {
                    Ok(data) => Ok(data),
                    Err(err) => io_err!(InvalidData, err),
                };
            }
            bytes.push(byte);
        }
    }
}

impl Http {
    /// parse an HTTP headers,
    pub async fn parse(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self> {
        let mut reader = Reader(reader);
        let prefix = reader.split_once(b'\n').await?;
        let mut headers = HashMap::new();
        {
            reader.split_once("ascii")
        }
        Ok(Self { prefix, headers })
    }
}

mod awd {
    #![allow(warnings)]

    #[test]
    fn test_name() {
        let s = String::from("Hello!\r");
        println!("{:?}", s.trim_end());
    }
}

// impl<'a> Http<'a> {
//     /// get http header value.
//     pub fn get(&self, key: impl AsRef<str>) -> Option<&[u8]> {
//         self.headers.get(key.as_ref()).copied()
//     }
//     fn _is_ws_upgrade(&self) -> Option<bool> {
//         let upgrade = self.get("upgrade")?.eq_ignore_ascii_case(b"websocket");
//         let connection = self.get("connection")?.eq_ignore_ascii_case(b"upgrade");
//         Some(upgrade && connection)
//     }
//     /// Determine if the incoming HTTP request is an upgrade request to the WebSocket protocol.
//     pub fn is_ws_upgrade(&self) -> bool {
//         matches!(self._is_ws_upgrade(), Some(true))
//     }
//     /// get http `sec-websocket-key` header value.
//     pub fn get_sec_ws_key(&self) -> Option<&[u8]> {
//         self.get("sec-websocket-version")?
//             .windows(2)
//             .any(|version| version == b"13")
//             .then_some(self.get("sec-websocket-key")?)
//     }

//     /// parse an HTTP message from a byte slice
//     pub fn parse(bytes: &mut &'a [u8]) -> std::result::Result<Self, &'static str> {
//         const HTTP_EOF_ERR: &str = "http parse error: unexpected end";
//         let schema = trim_ascii_end(split_once(bytes, b'\n').ok_or(HTTP_EOF_ERR)?);
//         let mut header = HashMap::new();
//         loop {
//             match split_once(bytes, b'\n').ok_or(HTTP_EOF_ERR)? {
//                 b"" | b"\r" => {
//                     return Ok(Self {
//                         prefix: schema,
//                         headers: header,
//                     })
//                 }
//                 line => {
//                     let mut value = line;
//                     let key = split_once(&mut value, b':')
//                         .ok_or("http parse error: invalid header field")?
//                         .to_ascii_lowercase();
//                     header.insert(
//                         String::from_utf8(key).map_err(|_| "invalid utf-8 bytes")?,
//                         trim_ascii_start(trim_ascii_end(value)),
//                     );
//                 }
//             }
//         }
//     }
// }

// // --------------------------------------------------------------------------------------------------

// fn split_once<'a>(reader: &mut &'a [u8], ascii: u8) -> Option<&'a [u8]> {
//     let index = reader.iter().position(|&byte| ascii == byte)?;
//     let val = &reader[..index];
//     *reader = &reader[index + 1..];
//     Some(val)
// }

// const fn trim_ascii_start(mut bytes: &[u8]) -> &[u8] {
//     while let [first, rest @ ..] = bytes {
//         if first.is_ascii_whitespace() {
//             bytes = rest;
//         } else {
//             break;
//         }
//     }
//     bytes
// }

// const fn trim_ascii_end(mut bytes: &[u8]) -> &[u8] {
//     while let [rest @ .., last] = bytes {
//         if last.is_ascii_whitespace() {
//             bytes = rest;
//         } else {
//             break;
//         }
//     }
//     bytes
// }
