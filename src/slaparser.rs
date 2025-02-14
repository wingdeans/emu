use crate::pcodeop::PcodeOp;
use crate::slaformat::{AId, EId};

use std::collections::HashMap;

use std::io::Read;

#[derive(Debug)]
pub(crate) enum Attribute {
    Bool(bool),
    Int(i64),
    Uint(u64),
    BasicAddr(u64),
    SpecialAddr(u8),
    Str(String),
    Pcode(PcodeOp), // unofficial
}

#[derive(Debug)]
pub(crate) struct Sla {
    pub(crate) eid: EId,
    pub(crate) attrs: HashMap<AId, Attribute>,
    pub(crate) els: Vec<Sla>,
}

// BUF/READER

#[derive(Debug)]
enum Tag {
    ElStart(EId),
    ElEnd,
    Attr(AId, Attribute),
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

    pub(crate) fn parse(self) -> Sla {
        let mut reader = SlaReader(self.0.into_iter());
        let Tag::ElStart(EId::SLEIGH) = reader.read_tag() else {
            unreachable!("sla must start with sleigh tag");
        };
        reader.parse(EId::SLEIGH)
    }
}

struct SlaReader(std::vec::IntoIter<u8>);

impl SlaReader {
    fn read_tag(&mut self) -> Tag {
        // see Ghidra file: PackedDecode.java
        // commit: GP-4285 Compressed SLEIGH

        // ttXi_iiii 1iii_iiii
        // | | ^-id  ^-continued id (if xbit set)
        // | \-xbit
        // \-tagid
        let (tagid, id) = {
            let b0 = self.0.next().unwrap();
            let (tagid, xbit, mut id) = (b0 >> 6, b0 >> 5 & 1, b0 as u16 & 0b0001_1111);
            if xbit != 0 {
                let b1 = self.0.next().unwrap();
                id <<= 7;
                id |= b1 as u16 & 0b0111_1111;
            }
            (tagid, id)
        };

        match tagid {
            0b01 => Tag::ElStart(id.into()),
            0b10 => Tag::ElEnd,
            0b11 => {
                let b0 = self.0.next().unwrap();
                let (typ, len) = (b0 >> 4, b0 & 0b1111);
                let attr = match typ {
                    1 => Attribute::Bool(len == 1),
                    6 => Attribute::SpecialAddr(len),
                    _ => {
                        // length-reliant attributes
                        let mut x = 0;
                        for b in self.0.by_ref().take(len as usize) {
                            x <<= 7;
                            x |= b as u64 & 0b0111_1111;
                        }
                        match typ {
                            2 => Attribute::Int(x.try_into().unwrap()),
                            3 => Attribute::Int(-TryInto::<i64>::try_into(x).unwrap()),
                            4 => Attribute::Uint(x),
                            5 => Attribute::BasicAddr(x),
                            7 => Attribute::Str(
                                String::from_utf8(self.0.by_ref().take(x as usize).collect())
                                    .unwrap(),
                            ),
                            _ => unreachable!("invalid attr type: {}", typ),
                        }
                    }
                };
                Tag::Attr(id.into(), attr)
            }
            _ => unreachable!("invalid tag id: {}", tagid),
        }
    }

    fn parse(&mut self, eid: EId) -> Sla {
        let mut attrs = HashMap::new();
        let mut els = Vec::new();

        loop {
            match self.read_tag() {
                Tag::ElStart(eid) => els.push(self.parse(eid)),
                Tag::Attr(aid, mut attr) => {
                    if aid == AId::CODE {
                        if let Attribute::Int(x) = attr {
                            attr = Attribute::Pcode(TryInto::<u8>::try_into(x).unwrap().into());
                        }
                    }
                    if attrs.insert(aid, attr).is_some() {
                        unreachable!("duplicate attribute: {:?}", aid)
                    }
                }
                Tag::ElEnd => break,
            }
        }

        Sla { eid, attrs, els }
    }
}

// SLA

impl Sla {
    fn write(&self, f: &mut std::fmt::Formatter, mut space: usize) -> std::fmt::Result {
        writeln!(f, "{:space$}{:?}", "", self.eid, space = space)?;
        space += 4;
        for (aid, val) in &self.attrs {
            writeln!(f, "{:space$}{:?} = {:?}", "", aid, val, space = space)?;
        }
        for el in &self.els {
            el.write(f, space)?
        }
        Ok(())
    }

    pub(crate) fn get_int<T>(&self, key: AId) -> T
    where
        T: TryFrom<u64> + TryFrom<i64>,
        <T as TryFrom<u64>>::Error: std::fmt::Debug,
        <T as TryFrom<i64>>::Error: std::fmt::Debug,
    {
        match self.attrs.get(&key) {
            Some(Attribute::Int(x)) => (*x).try_into().unwrap(),
            Some(Attribute::Uint(x)) => (*x).try_into().unwrap(),
            _ => unreachable!("{}", self), // TODO
        }
    }

    pub(crate) fn get_id<T>(&self) -> T
    where
        T: TryFrom<u64> + TryFrom<i64>,
        <T as TryFrom<u64>>::Error: std::fmt::Debug,
        <T as TryFrom<i64>>::Error: std::fmt::Debug,
    {
        self.get_int(AId::ID)
    }

    pub(crate) fn get_str(&self, key: AId) -> String {
        let Attribute::Str(s) = &self.attrs[&key] else {
            unreachable!() // TODO
        };
        s.clone()
    }

    pub(crate) fn get_el(&self, eid: EId) -> &Sla {
        let mut matches = self.els.iter().filter(|e| e.eid == eid);
        let Some(first) = matches.next() else {
            unreachable!("No occurrences of {:?} in Sla: {}", eid, self);
        };
        if matches.next().is_some() {
            unreachable!("Item {:?} is not unique in Sla: {}", eid, self);
        }
        first
    }
}

impl std::fmt::Display for Sla {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.write(f, 0)
    }
}
