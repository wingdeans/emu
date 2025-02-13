use crate::pcodeop::PcodeOp;
use crate::slaformat::{AId, EId};
use crate::slaparser::{Attribute, Sla};

#[derive(Debug, Clone, Copy)]
pub(crate) struct SymIdx(u16);

impl quote::IdentFragment for SymIdx {
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
    pub(crate) operands: Vec<SymIdx>,
    pub(crate) prints: Vec<Print>,
    pub(crate) construct_tpl: Vec<PcodeOp>,
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
    pub(crate) startbit: u8,
    pub(crate) endbit: u8,
    pub(crate) startbyte: u8,
    pub(crate) endbyte: u8,
    pub(crate) shift: u8,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Lshift(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>),
    Const(u64),
    InsnEnd,
    Operand(u8), // TODO
}

#[derive(Debug)]
pub(crate) enum OpExpr {
    Subsym(SymIdx),
    Tok(TokenField),
    Expr(Expr),
}

#[derive(Debug)]
pub(crate) struct Operand {
    pub(crate) off: u8,
    pub(crate) expr: OpExpr,
    pub(crate) args: Vec<(u8, u8, u8)>,
}

#[derive(Debug)]
pub(crate) struct Subtable {
    pub(crate) constructors: Vec<Constructor>,
    pub(crate) decision: Option<Decision>,
}

#[derive(Debug)]
pub(crate) struct Varlist {
    pub(crate) tokenfield: TokenField,
    pub(crate) vars: Vec<Option<SymIdx>>,
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

#[derive(Debug)]
pub(crate) struct Sleigh {
    pub(crate) symtab: SymbolTable,
}

// CONSTRUCTOR

fn model_varnode_tpl(tpl: &Sla) {
    assert_eq!(tpl.els.len(), 3);
    for e in &tpl.els {
        match e.eid {
            EId::CONST_HANDLE => (),
            EId::CONST_SPACEID => (),
            EId::CONST_REAL => (),
            EId::CONST_NEXT => (),
            EId::CONST_CURSPACE => (),
            EId::CONST_CURSPACE_SIZE => (),
            _ => unreachable!("{}", e),
        }
    }
}

fn model_op_tpl(tpl: &Sla) -> PcodeOp {
    for e in &tpl.els {
        match e.eid {
            EId::VARNODE_TPL => {
                model_varnode_tpl(e);
            }
            EId::NULL => (),
            _ => unreachable!("{}", e),
        }
    }
    let Attribute::Pcode(pcode) = tpl.attrs[&AId::CODE] else {
        unreachable!("{}", tpl);
    };
    pcode
}

fn model_constructor(constructor: &Sla) -> Constructor {
    let operands = constructor
        .els
        .iter()
        .filter(|e| e.eid == EId::OPER)
        .map(|e| SymIdx(e.get_id()))
        .collect();

    let prints = constructor
        .els
        .iter()
        .filter_map(|e| {
            Some(match e.eid {
                EId::OPPRINT => Print::OpPrint(e.get_id()),
                EId::PRINT => Print::Print(e.get_str(AId::PIECE)),
                _ => return None,
            })
        })
        .collect();

    let construct_tpl = constructor
        .get_el(EId::CONSTRUCT_TPL)
        .els
        .iter()
        .filter_map(|e| match e.eid {
            EId::NULL => None,
            EId::OP_TPL => Some(model_op_tpl(e)),
            EId::HANDLE_TPL => None,
            _ => unreachable!(),
        })
        .collect();

    Constructor {
        operands,
        prints,
        construct_tpl,
    }
}

// DECISION

fn model_pair(pair: &Sla) -> Option<Mask> {
    let insn_pat = pair.get_el(EId::INSTRUCT_PAT);
    let pat_block = insn_pat.get_el(EId::PAT_BLOCK);

    let nonzero = pat_block.get_int(AId::NONZERO);
    if nonzero == 0 {
        return None;
    }

    let mask_word = pat_block.get_el(EId::MASK_WORD);

    Some(Mask {
        id: pair.get_id(),
        off: pat_block.get_int(AId::OFF),
        nonzero,
        mask: mask_word.get_int(AId::MASK),
        val: mask_word.get_int(AId::VAL),
    })
}

fn model_decision(decision: &Sla) -> Option<Decision> {
    let masks = decision
        .els
        .iter()
        .filter(|e| e.eid == EId::PAIR)
        .map(model_pair)
        .collect::<Option<Vec<_>>>()?;

    let size: u8 = decision.get_int(AId::SIZE);

    if size != 0 {
        let options: Vec<_> = decision
            .els
            .iter()
            .filter(|e| e.eid == EId::DECISION)
            .map(|e| model_decision(e).expect("non-root decisions must have mask"))
            .collect();

        assert_eq!(options.len(), 1 << size);
        Some(Decision::Bits {
            start: decision.get_int(AId::STARTBIT),
            size,
            options,
        })
    } else {
        Some(Decision::Masks(masks))
    }
}

// SYMS

fn model_tokenfield(tokenfield: &Sla) -> TokenField {
    TokenField {
        startbit: tokenfield.get_int(AId::STARTBIT),
        endbit: tokenfield.get_int(AId::ENDBIT),
        startbyte: tokenfield.get_int(AId::STARTBYTE),
        endbyte: tokenfield.get_int(AId::ENDBYTE),
        shift: tokenfield.get_int(AId::SHIFT),
    }
}

fn parse_exprs<const C: usize>(expr: &Sla, args: &mut Vec<(u8, u8, u8)>) -> [Box<Expr>; C] {
    expr.els
        .iter()
        .map(|e| match e.eid {
            EId::OPERAND_EXP => {
                let idx = args.len().try_into().unwrap();
                args.push((
                    e.get_int(AId::TABLE),
                    e.get_int(AId::CT),
                    e.get_int(AId::INDEX),
                ));
                Box::new(Expr::Operand(idx))
            }
            EId::INTB => Box::new(Expr::Const(e.get_int(AId::VAL))),
            EId::END_EXP => Box::new(Expr::InsnEnd),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn model_operand(op: &Sla) -> Sym {
    let mut args = Vec::new();
    let expr = if op.attrs.contains_key(&AId::SUBSYM) {
        assert_eq!(op.els.len(), 1);
        OpExpr::Subsym(SymIdx(op.get_int(AId::SUBSYM)))
    } else {
        assert_eq!(op.els.len(), 2);
        let e = &op.els[1];
        match e.eid {
            EId::TOKENFIELD => OpExpr::Tok(model_tokenfield(e)),
            EId::PLUS_EXP => {
                let [a, b] = parse_exprs::<2>(e, &mut args);
                OpExpr::Expr(Expr::Plus(a, b))
            }
            EId::LSHIFT_EXP => {
                let [a, b] = parse_exprs::<2>(e, &mut args);
                OpExpr::Expr(Expr::Lshift(a, b))
            }
            EId::MINUS_EXP => {
                let [a] = parse_exprs::<1>(e, &mut args);
                OpExpr::Expr(Expr::Minus(a))
            }
            _ => unreachable!(),
        }
    };

    Sym::Op(Operand {
        off: op.get_int(AId::OFF),
        expr,
        args,
    })
}

fn model_subtable(subtable: &Sla) -> Sym {
    let decision = model_decision(subtable.get_el(EId::DECISION));
    let constructors = subtable
        .els
        .iter()
        .filter(|e| e.eid == EId::CONSTRUCTOR)
        .map(model_constructor)
        .collect();

    Sym::Subtable(Subtable {
        constructors,
        decision,
    })
}

fn model_varlist(varlist: &Sla) -> Sym {
    let tokenfield = model_tokenfield(varlist.get_el(EId::TOKENFIELD));
    let vars = varlist
        .els
        .iter()
        .filter_map(|e| {
            Some(match e.eid {
                EId::VAR => Some(SymIdx(e.get_id())),
                EId::NULL => None,
                _ => return None,
            })
        })
        .collect();

    Sym::Varlist(Varlist { tokenfield, vars })
}

// SYMBOL TABLE

fn model_symtab(symtab: &Sla) -> SymbolTable {
    let mut syms = Vec::new();
    let mut sym_names = Vec::new();

    for el in &symtab.els {
        let id: usize = el.get_id();
        let sym = match el.eid {
            EId::SUBTABLE_SYM_HEAD
            | EId::START_SYM_HEAD
            | EId::END_SYM_HEAD
            | EId::NEXT2_SYM_HEAD
            | EId::VARNODE_SYM_HEAD
            | EId::VALUE_SYM_HEAD
            | EId::VARLIST_SYM_HEAD
            | EId::OPERAND_SYM_HEAD
            | EId::USEROP_HEAD => {
                let name = el.get_str(AId::NAME);
                assert_eq!(id, sym_names.len());
                sym_names.push(name);
                continue;
            }
            EId::SUBTABLE_SYM => model_subtable(el),
            EId::OPERAND_SYM => model_operand(el),
            EId::VARLIST_SYM => model_varlist(el),
            EId::VARNODE_SYM => Sym::Varnode,
            _ => continue,
        };

        syms.resize_with(id + 1, || Sym::Unknown);
        syms[id] = sym;
    }

    SymbolTable { syms, sym_names }
}

impl SymbolTable {
    pub(crate) fn get_sym(&self, idx: SymIdx) -> &Sym {
        &self.syms[idx.0 as usize]
    }
}

impl Sleigh {
    pub(crate) fn new(sleigh: Sla) -> Self {
        let symtab = model_symtab(sleigh.get_el(EId::SYMBOL_TABLE));
        Sleigh { symtab }
    }
}
