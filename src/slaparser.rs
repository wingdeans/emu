use std::collections::HashMap;

use crate::slaformat::{AId, EId};
use crate::slareader::Attribute::{Int, Str, Uint};
use crate::slareader::SlaItem::{Attr, Elem};
use crate::slareader::{SlaBuf, SlaReader};

// CONSTRUCTOR
#[derive(Debug)]
pub(crate) enum Print {
    OpPrint(u16),
    Print(String),
}

#[derive(Debug)]
pub(crate) struct Constructor {
    pub(crate) id: u16,
    pub(crate) operands: Vec<u16>,
    pub(crate) prints: Vec<Print>,
}

// DECISION

#[derive(Debug)]
pub(crate) struct Mask {
    pub(crate) id: u16,
    pub(crate) off: u8,
    pub(crate) nonzero: u8,
    pub(crate) mask: u64,
    pub(crate) val: u64,
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
pub(crate) enum Sym {
    Unknown,
    Operand {
        off: u8,
        // minlen: u8,
    },
    Subtable {
        constructors: Vec<Constructor>,
        decision: Decision,
    },
}

#[derive(Debug)]
pub(crate) struct SymbolTable {
    pub(crate) syms: Vec<Sym>,
    pub(crate) subtables: HashMap<String, usize>,
}

// PARSER

struct SlaParser<'a> {
    r: SlaReader<'a>,
}

impl SlaParser<'_> {
    fn parse_element_id(&mut self) -> u16 {
        let mut id = 0;
        for item in self.r.by_ref() {
            match item {
                Attr(AId::ID, Uint(aid)) => id = aid.try_into().unwrap(),
                Attr(AId::ID, Int(aid)) => id = aid.try_into().unwrap(),
                _ => (),
            }
        }
        id
    }

    fn push_sym_at(vec: &mut Vec<Sym>, id: usize, val: Sym) {
        vec.resize_with(id + 1, || Sym::Unknown);
        vec[id] = val
    }

    // CONSTRUCTOR

    fn parse_constructor(&mut self, id: u16) -> Constructor {
        let mut operands = Vec::new();
        let mut prints = Vec::new();

        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::OPER) => operands.push(self.parse_element_id()),
                Elem(EId::OPPRINT) => prints.push(Print::OpPrint(self.parse_element_id())),
                // opprint
                Elem(EId::PRINT) => self.r.enter(),
                // opprint -> piece
                Attr(AId::PIECE, Str(s)) => prints.push(Print::Print(s)),
                // end
                Elem(EId::CONSTRUCT_TPL) => self.r.skip_elem(),
                Attr(_, _) => (),
                _ => unreachable!("unknown constructor item: {:?}", item),
            }
        }

        Constructor {
            id,
            prints,
            operands,
        }
    }

    // DECISION

    fn parse_pair(&mut self) -> Mask {
        let mut id = 0;
        let (mut off, mut nonzero) = (0, 0);
        let (mut mask, mut val) = (0, 0);

        while let Some(item) = self.r.next() {
            match item {
                // pair
                Attr(AId::ID, Int(aid)) => id = aid, // TODO
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

    // OPERAND

    fn parse_operand(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;
        let (mut off, mut _minlen) = (0, 0);
        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::OPERAND_EXP) => self.r.skip_elem(),
                Elem(EId::TOKENFIELD) => self.r.skip_elem(),
                Elem(EId::PLUS_EXP) => self.r.skip_elem(),
                Elem(EId::LSHIFT_EXP) => self.r.skip_elem(),
                Elem(EId::MINUS_EXP) => self.r.skip_elem(),
                // Elem(_) => self.r.skip_elem(),
                Attr(AId::ID, Uint(aid)) => id = aid.try_into().unwrap(),
                Attr(AId::OFF, Int(aoff)) => off = aoff.try_into().unwrap(),
                // Attr(AId::MINLEN, Int(aminlen)) => minlen = aminlen.try_into().unwrap(),
                Attr(AId::BASE, Int(abase)) => assert_eq!(abase, -1),
                Attr(_, _) => (),
                _ => unreachable!("unknown operand item: {:?}", item),
            }
        }

        Self::push_sym_at(syms, id.try_into().unwrap(), Sym::Operand { off });
    }

    fn parse_subtable(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;
        let mut decision = None;
        let mut constructors = Vec::new();

        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::CONSTRUCTOR) => constructors
                    .push(self.parse_constructor(constructors.len().try_into().unwrap())),
                Elem(EId::DECISION) => decision = Some(self.parse_decision()),
                Attr(AId::ID, Uint(aid)) => id = aid,
                Attr(_, _) => (),
                _ => unreachable!("unknown subtable item: {:?}", item),
            }
        }

        let id = id.try_into().unwrap();
        let sym = Sym::Subtable {
            constructors,
            decision: decision.unwrap(),
        };
        Self::push_sym_at(syms, id, sym);
    }

    fn parse_head(&mut self, subtables: &mut HashMap<String, usize>) {
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

        subtables.insert(name.unwrap(), id.try_into().unwrap());
    }

    fn parse_symbol_table(&mut self) -> SymbolTable {
        let mut syms = Vec::new();
        let mut subtables = HashMap::new();

        while let Some(item) = self.r.next() {
            match item {
                Elem(EId::SUBTABLE_SYM_HEAD) => self.parse_head(&mut subtables),
                Elem(EId::SUBTABLE_SYM) => self.parse_subtable(&mut syms),
                Elem(EId::OPERAND_SYM) => self.parse_operand(&mut syms),
                Elem(_) => self.r.skip_elem(),
                Attr(_, _) => (),
            }
        }

        SymbolTable { syms, subtables }
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
        };
        parser.parse_sleigh()
    }
}
