use crate::slaformat::{AId, EId};
use crate::slareader::Attribute::{Int, Str, Uint};
use crate::slareader::SlaItem::{Attr, Elem};
use crate::slareader::{SlaBuf, SlaReader};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SymIdx(u16);

macro_rules! cast {
    ($x:expr) => {
        $x.try_into().unwrap()
    };
}

macro_rules! sym_idx {
    ($x:expr) => {
        SymIdx(cast!($x))
    };
}

/*
impl quote::ToTokens for crate::slaparser::SymIdx {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
 */

impl quote::IdentFragment for crate::slaparser::SymIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// CONSTRUCTOR

#[derive(Debug)]
pub(crate) enum Print {
    OpPrint(u8),
    Print(String),
}

#[derive(Debug)]
pub(crate) struct Constructor {
    pub(crate) id: SymIdx,
    pub(crate) operands: Vec<SymIdx>,
    pub(crate) prints: Vec<Print>,
}

// DECISION

#[derive(Debug)]
pub(crate) struct Mask {
    pub(crate) id: u8,
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
        subsym: SymIdx,
    },
    Tok(TokenField),
    Unk, // TODO
}

#[derive(Debug)]
pub(crate) struct Subtable {
    pub(crate) constructors: Vec<Constructor>,
    pub(crate) decision: Option<Decision>,
}

#[derive(Debug)]
pub(crate) struct Varlist {
    tokenfield: TokenField,
    vars: Vec<Option<SymIdx>>,
}

#[derive(Debug)]
pub(crate) enum Sym {
    Unknown,
    Op(Operand),
    Subtable(Subtable),
    Varlist(Varlist),
    Varnode,
}

#[derive(Debug)]
pub(crate) struct SymbolTable {
    pub(crate) syms: Vec<Sym>,
    pub(crate) sym_names: Vec<String>,
}

impl SymbolTable {
    /*
    pub(crate) fn get_sym(&self, idx: SymIdx) -> &Sym {
        &self.syms[idx.0 as usize]
    }
    /*
    pub(crate) fn find_sym(&self, name: &str) -> Option<&Sym> {
        Some(&self.syms[self.sym_names.iter().position(|n| n == name)?])
    }
    */
    pub(crate) fn get_sym_name(&self, idx: SymIdx) -> &str {
        &self.sym_names[idx.0 as usize]
    }
    */
}

// PARSER

struct SlaParser<'a>(SlaReader<'a>);

impl SlaParser<'_> {
    fn parse_element_id(&mut self) -> u64 {
        let mut id = 0;
        while let Some(item) = self.0.next() {
            match item {
                Attr(AId::ID, Uint(x)) => id = x,
                Attr(AId::ID, Int(x)) => id = cast!(x),
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

    fn parse_constructor(&mut self, id: SymIdx) -> Constructor {
        let mut operands = Vec::new();
        let mut prints = Vec::new();

        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::OPER) => operands.push(sym_idx!(self.parse_element_id())),
                Elem(EId::OPPRINT) => prints.push(Print::OpPrint(cast!(self.parse_element_id()))),
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

        Mask {
            id: cast!(id),
            off: cast!(off),
            nonzero: cast!(nonzero),
            mask,
            val,
        }
    }

    fn parse_decision(&mut self) -> Option<Decision> {
        let (mut start, mut size) = (0, 0);
        let mut masks = Vec::new();
        let mut options = Vec::new();
        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::PAIR) => masks.push(self.parse_pair()),
                Elem(EId::DECISION) => options.push(
                    self.parse_decision()
                        .expect("non-root decisions must have mask"),
                ),
                Attr(AId::STARTBIT, Int(x)) => start = x,
                Attr(AId::SIZE, Int(x)) => size = x,
                Attr(_, _) => (),
                _ => unreachable!("unknown decision item: {:?}", item),
            }
        }

        if size != 0 {
            assert_eq!(options.len(), 1 << size);
            Some(Decision::Bits {
                start: cast!(start),
                size: cast!(size),
                options,
            })
        } else if masks.iter().any(|m| m.mask == 0) {
            assert_eq!(masks.len(), 1);
            None
        } else {
            Some(Decision::Masks(masks))
        }
    }

    // OPERAND

    fn parse_tokenfield(&mut self) -> TokenField {
        let (mut startbit, mut endbit, mut startbyte, mut endbyte) = (0, 0, 0, 0);

        for item in self.0.by_ref() {
            match item {
                Attr(AId::STARTBIT, Int(x)) => startbit = cast!(x),
                Attr(AId::ENDBIT, Int(x)) => endbit = cast!(x),
                Attr(AId::STARTBYTE, Int(x)) => startbyte = cast!(x),
                Attr(AId::ENDBYTE, Int(x)) => endbyte = cast!(x),
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
                Attr(AId::ID, Uint(x)) => id = cast!(x),
                Attr(AId::OFF, Int(x)) => off = cast!(x),
                Attr(AId::SUBSYM, Uint(x)) => subsym = Some(x),
                Attr(AId::BASE, Int(x)) => assert_eq!(x, -1),
                Attr(_, _) => (),
                _ => unreachable!("unknown operand item: {:?}", item),
            }
        }

        let op = if let Some(subsym) = subsym {
            Operand::Subsym {
                off,
                subsym: sym_idx!(subsym),
            }
        } else if let Some(tokenfield) = tokenfield {
            Operand::Tok(tokenfield)
        } else {
            Operand::Unk
        };

        Self::push_sym_at(syms, cast!(id), Sym::Op(op));
    }

    fn parse_subtable(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;
        let mut decision = None;
        let mut constructors = Vec::new();

        while let Some(item) = self.0.next() {
            match item {
                Elem(EId::CONSTRUCTOR) => {
                    constructors.push(self.parse_constructor(sym_idx!(constructors.len())))
                }
                Elem(EId::DECISION) => decision = Some(self.parse_decision()),
                Attr(AId::ID, Uint(x)) => id = x,
                Attr(_, _) => (),
                _ => unreachable!("unknown subtable item: {:?}", item),
            }
        }

        let sym = Sym::Subtable(Subtable {
            constructors,
            decision: decision.unwrap(),
        });
        Self::push_sym_at(syms, cast!(id), sym);
    }

    fn parse_varlist(&mut self, syms: &mut Vec<Sym>) {
        let mut id = 0;
        let mut tokenfield = None;
        let mut vars = Vec::new();

        while let Some(item) = self.0.next() {
            match item {
                Attr(AId::ID, Uint(x)) => id = x,
                Elem(EId::TOKENFIELD) => tokenfield = Some(self.parse_tokenfield()),
                Elem(EId::VAR) => vars.push(Some(sym_idx!(self.parse_element_id()))),
                Elem(EId::NULL) => self.0.skip_elem(),
                Attr(_, _) => (),
                _ => unreachable!("unknown varlist item: {:?}", item),
            }
        }

        let sym = Sym::Varlist(Varlist {
            tokenfield: tokenfield.unwrap(),
            vars,
        });
        Self::push_sym_at(syms, cast!(id), sym);
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

        let sym = Sym::Varnode;
        Self::push_sym_at(syms, cast!(id), sym);
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

        assert_eq!(sym_names.len(), cast!(id));
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
