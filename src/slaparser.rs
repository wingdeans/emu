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
pub(crate) struct TokenField {
    startbit: u8,
    endbit: u8,
    startbyte: u8,
    endbyte: u8,
}

#[derive(Debug)]
pub(crate) enum Operand {
    Subsym {
        off: u8,
        // minlen: u8,
        subsym: u16,
    },
    Tok(TokenField),
    Unk, // TODO
}

#[derive(Debug)]
pub(crate) enum Sym {
    Unknown,
    Op(Operand),
    Subtable {
        constructors: Vec<Constructor>,
        decision: Decision,
    },
    Varlist,
    Varnode,
}

#[derive(Debug)]
pub(crate) struct SymbolTable {
    pub(crate) syms: Vec<Sym>,
    pub(crate) sym_names: Vec<String>,
}

// PARSER

struct SlaParser<'a>(SlaReader<'a>);

impl SlaParser<'_> {
    fn parse_element_id(&mut self) -> u16 {
        let mut id = 0;
        while let Some(item) = self.0.next() {
            match item {
                Attr(AId::ID, Uint(x)) => id = x.try_into().unwrap(),
                Attr(AId::ID, Int(x)) => id = x.try_into().unwrap(),
                Elem(_) => self.0.skip_elem(),
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

        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::OPER) => operands.push(self.parse_element_id()),
                Elem(EId::OPPRINT) => prints.push(Print::OpPrint(self.parse_element_id())),
                // opprint
                Elem(EId::PRINT) => self.0.enter(),
                // opprint -> piece
                Attr(AId::PIECE, Str(s)) => prints.push(Print::Print(s)),
                // end
                Elem(EId::CONSTRUCT_TPL) => self.0.skip_elem(),
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

        while let Some(item) = self.0.next() {
            match item {
                // pair
                Attr(AId::ID, Int(x)) => id = x, // TODO
                // pair -> instruct pat
                Elem(EId::INSTRUCT_PAT) => self.0.enter(),
                // pair -> instruct pat -> pat block
                Elem(EId::PAT_BLOCK) => self.0.enter(),
                Attr(AId::OFF, Int(x)) => off = x,
                Attr(AId::NONZERO, Int(x)) => nonzero = x,
                // pair -> instruct pat -> pat block -> mask word
                Elem(EId::MASK_WORD) => self.0.enter(),
                Attr(AId::MASK, Uint(x)) => mask = x,
                Attr(AId::VAL, Uint(x)) => val = x,
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
        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::PAIR) => masks.push(self.parse_pair()),
                Elem(EId::DECISION) => options.push(self.parse_decision()),
                Attr(AId::STARTBIT, Int(x)) => start = x,
                Attr(AId::SIZE, Int(x)) => size = x,
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

    fn parse_tokenfield(&mut self) -> TokenField {
        let (mut startbit, mut endbit, mut startbyte, mut endbyte) = (0, 0, 0, 0);

        while let Some(item) = self.0.next() {
            match item {
                Attr(AId::STARTBIT, Int(x)) => startbit = x.try_into().unwrap(),
                Attr(AId::ENDBIT, Int(x)) => startbit = x.try_into().unwrap(),
                Attr(AId::STARTBYTE, Int(x)) => startbit = x.try_into().unwrap(),
                Attr(AId::ENDBYTE, Int(x)) => startbit = x.try_into().unwrap(),
                _ => (),
            }
        }

        TokenField {
            startbit,
            endbit,
            startbyte,
            endbyte,
        }
    }

    fn parse_operand(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;
        let (mut off, mut _minlen) = (0, 0);
        let mut subsym = None;
        let mut tokenfield = None;

        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::OPERAND_EXP) => self.0.skip_elem(),
                Elem(EId::TOKENFIELD) => tokenfield = Some(self.parse_tokenfield()),
                Elem(EId::PLUS_EXP) => self.0.skip_elem(),
                Elem(EId::LSHIFT_EXP) => self.0.skip_elem(),
                Elem(EId::MINUS_EXP) => self.0.skip_elem(),
                Attr(AId::ID, Uint(x)) => id = x.try_into().unwrap(),
                Attr(AId::OFF, Int(x)) => off = x.try_into().unwrap(),
                // Attr(AId::MINLEN, Int(x)) => minlen = x.try_into().unwrap(),
                Attr(AId::SUBSYM, Uint(x)) => subsym = Some(x),
                Attr(AId::BASE, Int(x)) => assert_eq!(x, -1),
                Attr(_, _) => (),
                _ => unreachable!("unknown operand item: {:?}", item),
            }
        }

        let op = if let Some(subsym) = subsym {
            Operand::Subsym {
                off,
                subsym: subsym.try_into().unwrap(),
            }
        } else if let Some(tokenfield) = tokenfield {
            Operand::Tok(tokenfield)
        } else {
            Operand::Unk
        };

        Self::push_sym_at(syms, id.try_into().unwrap(), Sym::Op(op));
    }

    fn parse_subtable(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;
        let mut decision = None;
        let mut constructors = Vec::new();

        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::CONSTRUCTOR) => constructors
                    .push(self.parse_constructor(constructors.len().try_into().unwrap())),
                Elem(EId::DECISION) => decision = Some(self.parse_decision()),
                Attr(AId::ID, Uint(x)) => id = x,
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

    fn parse_varlist(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;

        while let Some(item) = self.0.next() {
            match item {
                Attr(AId::ID, Uint(x)) => id = x,
                Elem(_) => self.0.skip_elem(),
                Attr(_, _) => (),
            }
        }

        let id = id.try_into().unwrap();
        let sym = Sym::Varlist;
        Self::push_sym_at(syms, id, sym);
    }

    fn parse_varnode(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;

        while let Some(item) = self.0.next() {
            match item {
                Attr(AId::ID, Uint(x)) => id = x,
                Elem(_) => self.0.skip_elem(),
                Attr(_, _) => (),
            }
        }

        let id = id.try_into().unwrap();
        let sym = Sym::Varnode;
        Self::push_sym_at(syms, id, sym);
    }

    // SYMBOL TABLE

    fn parse_head(&mut self, sym_names: &mut Vec<String>) {
        let mut name = None;
        let mut id = 0;
        for item in self.0.by_ref() {
            match item {
                Attr(AId::NAME, Str(aname)) => name = Some(aname),
                Attr(AId::ID, Uint(x)) => id = x,
                Attr(AId::SCOPE, Uint(_)) => (),
                _ => unreachable!("unknown head item: {:?}", item),
            }
        }

        assert_eq!(id, sym_names.len().try_into().unwrap());
        sym_names.push(name.unwrap());
    }

    fn parse_symbol_table(&mut self) -> SymbolTable {
        let mut syms = Vec::new();
        let mut sym_names = Vec::new();

        while let Some(item) = self.0.next() {
            match item {
                Elem(
                    EId::SUBTABLE_SYM_HEAD
                    | EId::START_SYM_HEAD
                    | EId::END_SYM_HEAD
                    | EId::NEXT2_SYM_HEAD
                    | EId::VARNODE_SYM_HEAD
                    | EId::VALUE_SYM_HEAD
                    | EId::VARLIST_SYM_HEAD
                    | EId::OPERAND_SYM_HEAD
                    | EId::USEROP_HEAD,
                ) => self.parse_head(&mut sym_names),
                Elem(EId::SUBTABLE_SYM) => self.parse_subtable(&mut syms),
                Elem(EId::OPERAND_SYM) => self.parse_operand(&mut syms),
                Elem(EId::VARLIST_SYM) => self.parse_varlist(&mut syms),
                Elem(EId::VARNODE_SYM) => self.parse_varnode(&mut syms),
                Elem(_) => self.0.skip_elem(),
                Attr(_, _) => (),
            }
        }

        SymbolTable { syms, sym_names }
    }

    fn parse_sleigh(&mut self) -> SymbolTable {
        let mut symtab = None;

        let Elem(EId::SLEIGH) = self.0.next().unwrap() else {
            unreachable!("root element is not sleigh");
        };

        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::SOURCEFILES | EId::SPACES) => self.0.skip_elem(),
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
        let mut parser = SlaParser(self.into_iter());
        parser.parse_sleigh()
    }
}
