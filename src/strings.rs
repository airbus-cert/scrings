use crate::error::Result;
use std::io::{Read, Seek, SeekFrom};
use std::marker::PhantomData;
use std::result::Result as StdResult;


pub type Utf16le = u16;

pub trait Decode where Self: Sized {
    fn from_bytes(value: &[u8]) -> Result<Self>;
    fn from_vec(buf: Vec<Self>) -> Result<String>;
}

impl Decode for Utf16le {
    fn from_bytes(value: &[u8]) -> Result<u16> {
        Ok(u16::from_le_bytes(value.try_into()?))
    }

    fn from_vec(buf: Vec<Self>) -> Result<String> {
        Ok(std::char::decode_utf16(buf.into_iter()).collect::<StdResult<String, _>>()?)
    }
}

impl Decode for u8 {
    fn from_bytes(value: &[u8]) -> Result<u8> {
        Ok(u8::from_le_bytes(value.try_into()?))
    }

    fn from_vec(buf: Vec<Self>) -> Result<String> {
        Ok(String::from_utf8(buf)?)
    }
}


fn is_printable(c: u64) -> bool {
    c == 0xd || c == 0xa || (c >= 0x20 && c <= 0x7e)
}

fn decode_until_null_byte<T: Read + Seek, U: Decode + Into<u64> + Copy>(stream: &mut T) -> Result<String> {

    let position = stream.seek(SeekFrom::Current(0))?;
    let mut find_null_term = false;
    let mut result = vec![];
    while !find_null_term {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        for i in (0..1024).step_by(size_of::<U>()) {
            let tmp_char = U::from_bytes(&buffer[i..(i + size_of::<U>())])?;
            if !is_printable(tmp_char.into()) {
                find_null_term = true;
                break;
            }

            result.push(tmp_char);
        }
    }

    stream.seek(SeekFrom::Start(position + (size_of::<U>() as u64)*(result.len() as u64)))?;
    U::from_vec(result)
}

pub struct StringsIterator<T: Read + Seek, U> {
    buffer: T,
    step: usize,
    encoding: PhantomData<U>
}

impl<T: Read + Seek, U> StringsIterator<T, U> {
    pub fn new(buffer: T, step: usize) -> Self{
        Self {
            buffer,
            step,
            encoding: PhantomData
        }
    }
}

impl<'a, T: Read + Seek, U: Decode + Into<u64> + Copy> Iterator for StringsIterator<T, U> {
    type Item = (u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        next_strings::<T, U>(&mut self.buffer, self.step).ok()
    }
}

pub fn next_strings<T: Read + Seek, U: Decode + Into<u64> + Copy>(buf: &mut T, step: usize) -> Result<(u64, String)> {
    let increment = step * size_of::<U>();
    let mut cursor;
    loop {
        cursor = buf.seek(SeekFrom::Current(increment as i64))?;

        let mut find_printable = false;
        for i in 0..(size_of::<U>() as u64) {
            let mut char_buffer = vec![0; size_of::<U>()];
            buf.read_exact(char_buffer.as_mut_slice())?;

            let char = U::from_bytes(char_buffer.as_slice())?;

            if is_printable(char.into()) {
                find_printable = true;
                break;
            }

            buf.seek(SeekFrom::Start(cursor + i + 1))?;
        }

        if find_printable {

            // my cursor is the current minus the one matched
            cursor = buf.seek(SeekFrom::Current(0))? - size_of::<U>() as u64;

            // try to find what I missed
            buf.seek(SeekFrom::Start(cursor - increment as u64))?;

            // read back to my increment and find first non printable char
            let mut buffer = vec![0; increment as usize];
            buf.read_exact(&mut buffer)?;

            let mut position = cursor;
            for i in (0..increment).step_by(size_of::<U>()).rev() {
                // find non printable !
                if is_printable(U::from_bytes(&buffer[i..(i + size_of::<U>())])?.into()) {
                    position -= size_of::<U>() as u64;
                }
                else {
                    break;
                }
            }

            buf.seek(SeekFrom::Start(position))?;

            if let Ok(str) = decode_until_null_byte::<T, U>(buf) {
                if str.len() > step {
                    return Ok((position, str))
                }
            }
        }
    }
}

pub trait IterUtf16leStrings {
    fn iter_utf16le_strings(self, step: usize) -> StringsIterator<Self, Utf16le> where Self: Read + Seek + Sized;
}

impl<T: Read + Seek> IterUtf16leStrings for T {
    fn iter_utf16le_strings(self, step: usize) -> StringsIterator<Self, Utf16le> {
        StringsIterator::new(self, step)
    }
}

pub trait IterStrings {
    fn iter_strings(self, step: usize) -> StringsIterator<Self, u8> where Self: Read + Seek + Sized;
}

impl<T: Read + Seek> IterStrings for T {
    fn iter_strings(self, step: usize) -> StringsIterator<Self, u8> {
        StringsIterator::new(self, step)
    }
}