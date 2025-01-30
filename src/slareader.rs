use crate::slaformat::{AId, EId};

use core::str::from_utf8;
use std::io::Read;

#[derive(Debug)]
pub(crate) enum Attribute {
    Bool(bool),
    Int(i64),
    Uint(u64),
    BasicAddr(u64),
    SpecialAddr(u8),
    Str(String),
}

#[derive(Debug)]
enum Tag {
    ElStart(EId),
    ElEnd,
    Attr(AId, Attribute),
}

#[derive(Debug)]
pub(crate) enum SlaItem {
    Elem(EId),
    Attr(AId, Attribute),
}

// BUFFER

pub(crate) struct SlaBuf(Vec<u8>);

impl SlaBuf {
    pub(crate) fn new(path: &str) -> Result<SlaBuf, Box<dyn std::error::Error>> {
        let mut uncompressed = Vec::new();
        let mut file = std::fs::File::open(path)?;
        file.read_to_end(&mut uncompressed)?;

        assert!(&uncompressed[..4] == b"sla\x04");

        let bufvec = miniz_oxide::inflate::decompress_to_vec_zlib_with_limit(
            &uncompressed[4..],
            1024 * 1024 * 1024,
        )?;

        Ok(SlaBuf(bufvec))
    }
}

impl<'a> IntoIterator for &'a SlaBuf {
    type Item = SlaItem;
    type IntoIter = SlaReader<'a>;
    fn into_iter(self) -> Self::IntoIter {
        SlaReader {
            it: self.0.iter(),
            level: 0,
        }
    }
}

pub(crate) struct SlaReader<'a> {
    it: std::slice::Iter<'a, u8>,
    level: u32,
}

impl SlaReader<'_> {
    fn next_tag(&mut self) -> Option<Tag> {
        // see Ghidra file: PackedDecode.java
        // commit: GP-4285 Compressed SLEIGH

        // ttXi_iiii 1iii_iiii
        // | | ^-id  ^-continued id (if xbit set)
        // | \-xbit
        // \-tagid
        let (tagid, id) = {
            let b0 = *self.it.next()?;
            let (tagid, xbit, mut id) = (b0 >> 6, b0 >> 5 & 1, b0 as u16 & 0b0001_1111);
            if xbit != 0 {
                let b1 = *self.it.next().unwrap();
                id <<= 7;
                id |= b1 as u16 & 0b0111_1111;
            }
            (tagid, id)
        };

        fn take_slice<'a>(it: &mut std::slice::Iter<'a, u8>, len: usize) -> &'a [u8] {
            let slice = &it.as_slice()[..len];
            if len != 0 {
                it.nth(len - 1).unwrap();
            }
            slice
        }

        let tag = match tagid {
            0b01 => Tag::ElStart(id.into()),
            0b10 => Tag::ElEnd,
            0b11 => {
                let b0 = *self.it.next().unwrap();
                let (typ, len) = (b0 >> 4, b0 & 0b1111);
                let attr = match typ {
                    1 => Attribute::Bool(len == 1),
                    6 => Attribute::SpecialAddr(len),
                    _ => {
                        // length-reliant attributes
                        let mut x = 0;
                        for b in take_slice(&mut self.it, len as usize) {
                            x <<= 7;
                            x |= *b as u64 & 0b0111_1111;
                        }
                        match typ {
                            2 => Attribute::Int(x.try_into().unwrap()),
                            3 => Attribute::Int(-TryInto::<i64>::try_into(x).unwrap()),
                            4 => Attribute::Uint(x),
                            5 => Attribute::BasicAddr(x),
                            7 => Attribute::Str(
                                from_utf8(take_slice(&mut self.it, x as usize))
                                    .unwrap()
                                    .to_string(),
                            ),
                            _ => unreachable!(),
                        }
                    }
                };
                Tag::Attr(id.into(), attr)
            }
            _ => unreachable!("invalid tagid: {}", tagid),
        };
        Some(tag)
    }

    pub(crate) fn skip_elem(&mut self) {
        let mut level = 1;
        while let Some(tag) = self.next_tag() {
            match tag {
                Tag::ElStart(_) => level += 1,
                Tag::ElEnd => {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    pub(crate) fn enter(&mut self) {
        self.level += 1
    }
}

impl<'a> Iterator for SlaReader<'a> {
    type Item = SlaItem;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_tag().unwrap() {
                Tag::ElStart(id) => return Some(SlaItem::Elem(id)),
                Tag::Attr(id, attr) => return Some(SlaItem::Attr(id, attr)),
                Tag::ElEnd => {
                    if self.level == 0 {
                        return None;
                    }
                    self.level -= 1;
                }
            }
        }
    }
}
