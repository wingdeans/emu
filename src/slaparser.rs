use crate::slaformat::{AId, EId};
use crate::slareader::Attribute::{Int, Str, Uint};
use crate::slareader::SlaItem::{Attr, Elem};
use crate::slareader::{SlaBuf, SlaReader};

#[derive(Debug)]
pub(crate) struct Mask {
    id: u16,
    off: u8,
    nonzero: u8,
    mask: u64,
    val: u64,
}

#[derive(Debug)]
pub(crate) enum Decision {
    Bits {
        start: u8,
        size: u8,
        options: Vec<Decision>,
    },
    Masks(Vec<Mask>),
}

#[derive(Debug)]
pub(crate) struct Subtable {
    id: u16,
    name: String,
    pub(crate) decision: Decision,
}

#[derive(Debug)]
pub(crate) struct SymbolTable {
    subtables: Vec<Subtable>,
}

impl SymbolTable {
    pub(crate) fn get_subtable_by_id(&self, id: u16) -> Option<&Subtable> {
        let idx = self.subtables.binary_search_by_key(&id, |st| st.id).ok()?;
        Some(&self.subtables[idx])
    }

    pub(crate) fn get_subtable_by_name(&self, name: &str) -> Option<&Subtable> {
        self.subtables.iter().find(|st| st.name == name)
    }
}

struct SlaParser<'a> {
    r: SlaReader<'a>,
    heads: Vec<(u16, String)>,
}

impl SlaParser<'_> {
    fn parse_constructor(&mut self) {
        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::OPER | EId::PRINT | EId::OPPRINT | EId::CONSTRUCT_TPL) => {
                    self.r.skip_elem()
                }
                Attr(_, _) => (),
                _ => unreachable!("unknown constructor item: {:?}", item),
            }
        }
    }

    fn parse_pair(&mut self) -> Mask {
        let mut id = 0;
        let (mut off, mut nonzero) = (0, 0);
        let (mut mask, mut val) = (0, 0);
        while let Some(item) = self.r.next() {
            match item {
                // pair
                Attr(AId::ID, Int(aid)) => id = aid,
                // pair -> instruct pat
                Elem(EId::INSTRUCT_PAT) => self.r.enter(),
                // pair -> instruct pat -> pat block
                Elem(EId::PAT_BLOCK) => self.r.enter(),
                Attr(AId::OFF, Int(aoff)) => off = aoff,
                Attr(AId::NONZERO, Int(anonzero)) => nonzero = anonzero,
                // pair -> instruct pat -> pat block -> mask word
                Elem(EId::MASK_WORD) => self.r.enter(),
                Attr(AId::MASK, Uint(amask)) => mask = amask,
                Attr(AId::VAL, Uint(aval)) => val = aval,
                // end
                Attr(_, _) => (),
                _ => unreachable!("unknown pair item: {:?}", item),
            }
        }
        let id = id.try_into().unwrap();
        let off = off.try_into().unwrap();
        let nonzero = nonzero.try_into().unwrap();
        Mask {
            id,
            off,
            nonzero,
            mask,
            val,
        }
    }

    fn parse_decision(&mut self) -> Decision {
        let (mut start, mut size) = (0, 0);
        let mut masks = Vec::new();
        let mut options = Vec::new();
        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::PAIR) => masks.push(self.parse_pair()),
                Elem(EId::DECISION) => options.push(self.parse_decision()),
                Attr(AId::STARTBIT, Int(astart)) => start = astart,
                Attr(AId::SIZE, Int(asize)) => size = asize,
                Attr(_, _) => (),
                _ => unreachable!("unknown decision item: {:?}", item),
            }
        }

        if size != 0 {
            assert_eq!(options.len(), 1 << size);
            let start = start.try_into().unwrap();
            let size = size.try_into().unwrap();
            Decision::Bits {
                start,
                size,
                options,
            }
        } else {
            Decision::Masks(masks)
        }
    }

    fn parse_subtable(&mut self) -> Subtable {
        let mut id = 0;
        let mut decision = None;
        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::CONSTRUCTOR) => self.parse_constructor(),
                Elem(EId::DECISION) => decision = Some(self.parse_decision()),
                Attr(AId::ID, Uint(aid)) => id = aid,
                Attr(_, _) => (),
                _ => unreachable!("unknown subtable item: {:?}", item),
            }
        }

        let id: u16 = id.try_into().unwrap();
        let name = {
            let idx = self.heads.binary_search_by_key(&id, |&(id, _)| id).unwrap();
            self.heads[idx].1.clone()
        };
        Subtable {
            id,
            name,
            decision: decision.unwrap(),
        }
    }

    fn parse_head(&mut self) {
        let mut name = None;
        let mut id = 0;
        for item in self.r.by_ref() {
            match item {
                Attr(AId::NAME, Str(aname)) => name = Some(aname),
                Attr(AId::ID, Uint(aid)) => id = aid,
                Attr(AId::SCOPE, Uint(_)) => (),
                _ => unreachable!("unknown head item: {:?}", item),
            }
        }
        self.heads.push((id.try_into().unwrap(), name.unwrap()))
    }

    fn parse_symbol_table(&mut self) -> SymbolTable {
        let mut subtables = Vec::new();
        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::SUBTABLE_SYM_HEAD) => self.parse_head(),
                Elem(EId::SUBTABLE_SYM) => subtables.push(self.parse_subtable()),
                Elem(_) => self.r.skip_elem(),
                Attr(_, _) => (),
            }
        }

        // sorted
        assert!(subtables.windows(2).all(|w| w[0].id <= w[1].id));

        SymbolTable { subtables }
    }

    fn parse_sleigh(&mut self) -> SymbolTable {
        let mut symtab = None;

        let Elem(EId::SLEIGH) = self.r.next().unwrap() else {
            unreachable!("root element is not sleigh");
        };

        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::SOURCEFILES | EId::SPACES) => self.r.skip_elem(),
                Elem(EId::SYMBOL_TABLE) => symtab = Some(self.parse_symbol_table()),
                Attr(_, _) => (),
                _ => unreachable!("unknown sleigh item: {:?}", item),
            }
        }

        symtab.unwrap()
    }
}

impl SlaBuf {
    pub(crate) fn parse_sleigh(&self) -> SymbolTable {
        let mut parser = SlaParser {
            r: self.into_iter(),
            heads: Vec::new(),
        };
        parser.parse_sleigh()
    }
}
