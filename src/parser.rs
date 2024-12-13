use std::io::{Read, Seek};
use std::marker::PhantomData;
use crate::error::Result;
use crate::strings::{Decode, StringsIterator};

pub trait Parser {
    fn parse(&mut self, src: &str) -> Result<Option<(u64, String)>>;
}

pub trait Parsable {
    fn is<P: Parser + Default>(&self) -> Result<Option<(u64, String)>>;
}

impl Parsable for String {
    fn is<P: Parser + Default>(&self) -> Result<Option<(u64, String)>> {
        P::default().parse(self.as_str())
    }
}

pub struct LanguageIterator<T: Read + Seek, U, P> {
    strings_iterator: StringsIterator<T, U>,
    language: PhantomData<P>
}

impl <T: Read + Seek, U, P> LanguageIterator<T, U, P> {
    pub fn new(buffer: T, step: usize) -> Self {
        Self {
            strings_iterator: StringsIterator::new(buffer, step),
            language: PhantomData
        }
    }
}

impl<T: Read + Seek, U: Decode + Into<u64> + Copy, P: Parser + Default> Iterator for LanguageIterator<T, U, P> {
    type Item = (u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((position, s)) = self.strings_iterator.next() {
            if let Some((offset, result)) = s.is::<P>().unwrap_or(None) {
                return Some((position + offset, result));
            }
        }
        None
    }
}


pub trait IterScrings {
    fn iter_scrings<U: Decode + Into<u64> + Copy, P: Parser>(self, step: usize) -> LanguageIterator<Self, U, P> where Self: Read + Seek + Sized;
}

impl<T: Read + Seek> IterScrings for T {
    fn iter_scrings<U: Decode + Into<u64> + Copy, P: Parser>(self, step: usize) -> LanguageIterator<Self, U, P> {
        LanguageIterator::new(self, step)
    }
}
