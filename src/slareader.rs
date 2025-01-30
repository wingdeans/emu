use crate::slaformat::{AId, EId};

use core::str::from_utf8;
use std::io::Read;

#[derive(Debug)]
pub(crate) enum Attribute<'a> {
    Bool(bool),
    Int(i64),
    Uint(u64),
    BasicAddr(u64),
    SpecialAddr(u8),
    Str(&'a str),
}

#[derive(Debug)]
pub(crate) enum Tag<'a> {
    ElStart(EId),
    ElEnd,
    Attr(AId, Attribute<'a>),
}

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
    type Item = Tag<'a>;
    type IntoIter = SlaReader<'a>;
    fn into_iter(self) -> Self::IntoIter {
        SlaReader(self.0.iter())
    }
}

pub(crate) struct SlaReader<'a>(std::slice::Iter<'a, u8>);

impl<'a> Iterator for SlaReader<'a> {
    type Item = Tag<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // see Ghidra file: PackedDecode.java
        // commit: GP-4285 Compressed SLEIGH

        // ttXi_iiii 1iii_iiii
        // | | ^-id  ^-continued id (if xbit set)
        // | \-xbit
        // \-tagid
        let (tagid, id) = {
            let b0 = *self.0.next()?;
            let (tagid, xbit, mut id) = (b0 >> 6, b0 >> 5 & 1, b0 as u16 & 0b0001_1111);
            if xbit != 0 {
                let b1 = *self.0.next().unwrap();
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
                let b0 = *self.0.next().unwrap();
                let (typ, len) = (b0 >> 4, b0 & 0b1111);
                let attr = match typ {
                    1 => Attribute::Bool(len == 1),
                    6 => Attribute::SpecialAddr(len),
                    _ => {
                        // length-reliant attributes
                        let mut x = 0;
                        for b in take_slice(&mut self.0, len as usize) {
                            x <<= 7;
                            x |= *b as u64 & 0b0111_1111;
                        }
                        match typ {
                            2 => Attribute::Int(x.try_into().unwrap()),
                            3 => Attribute::Int(-TryInto::<i64>::try_into(x).unwrap()),
                            4 => Attribute::Uint(x),
                            5 => Attribute::BasicAddr(x),
                            7 => Attribute::Str(
                                from_utf8(take_slice(&mut self.0, x as usize)).unwrap(),
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
}
