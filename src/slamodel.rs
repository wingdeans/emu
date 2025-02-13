use crate::pcodeop::PcodeOp;
use crate::slaformat::{AId, EId};
use crate::slaparser::Sla;

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
    Operand, // TODO
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

fn model_op_tpl(tpl: &Sla) -> PcodeOp {
    tpl.get_int::<u8>(AId::CODE).into()
}

fn model_construct_tpl(tpl: &Sla) -> Vec<PcodeOp> {
    tpl.els
        .iter()
        .filter_map(|e| match e.eid {
            EId::NULL => None,
            EId::OP_TPL => Some(model_op_tpl(e)),
            EId::HANDLE_TPL => None,
            _ => unreachable!(),
        })
        .collect()
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

    Constructor {
        operands,
        prints,
        construct_tpl: Vec::new(), /* TODO */
    }
}

// DECISION

fn model_pair(pair: &Sla) -> Option<Mask> {
    let insn_pat = &pair[EId::INSTRUCT_PAT];
    let pat_block = &insn_pat[EId::PAT_BLOCK];

    let nonzero = pat_block.get_int(AId::NONZERO);
    if nonzero == 0 {
        return None;
    }

    let mask_word = &pat_block[EId::MASK_WORD];

    Some(Mask {
        id: pair.get_id(),
        off: pat_block.get_int(AId::OFF),
        nonzero,
        mask: mask_word.get_int(AId::MASK),
        val: mask_word.get_int(AId::VAL),
    })
}

fn model_decision(decision: &Sla) -> Option<Decision> {
    let mut masks = Vec::new();
    let mut options = Vec::new();

    for e in &decision.els {
        match e.eid {
            EId::PAIR => {
                if let Some(mask) = model_pair(e) {
                    masks.push(mask)
                } else {
                    return None;
                }
            }
            EId::DECISION => {
                options.push(model_decision(e).expect("non-root decisions must have mask"))
            }
            _ => (),
        }
    }

    let size: u8 = decision.get_int(AId::SIZE);

    if size != 0 {
        assert_eq!(options.len(), 1 << size);
        Some(Decision::Bits {
            start: decision.get_int(AId::STARTBIT),
            size: decision.get_int(AId::SIZE),
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

fn parse_exprs<const C: usize>(expr: &Sla) -> [Box<Expr>; C] {
    expr.els
        .iter()
        .map(|e| match e.eid {
            EId::OPERAND_EXP => Box::new(Expr::Operand),
            EId::INTB => Box::new(Expr::Const(e.get_int(AId::VAL))),
            EId::END_EXP => Box::new(Expr::InsnEnd),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn model_operand(op: &Sla) -> Sym {
    let mut expr = None;
    for e in &op.els {
        match e.eid {
            EId::OPERAND_EXP => (),
            EId::TOKENFIELD => (),
            EId::PLUS_EXP => {
                let [a, b] = parse_exprs::<2>(e);
                expr = Some(Expr::Plus(a, b))
            }
            EId::LSHIFT_EXP => {
                let [a, b] = parse_exprs::<2>(e);
                expr = Some(Expr::Lshift(a, b))
            }
            EId::MINUS_EXP => {
                let [a] = parse_exprs::<1>(e);
                expr = Some(Expr::Minus(a))
            }
            _ => unreachable!(),
        }
    }

    let expr = if op.attrs.contains_key(&AId::SUBSYM) {
        OpExpr::Subsym(SymIdx(op.get_int(AId::SUBSYM)))
    } else if let Some(expr) = expr {
        OpExpr::Expr(expr)
    } else {
        OpExpr::Tok(model_tokenfield(&op[EId::TOKENFIELD]))
    };

    Sym::Op(Operand {
        off: op.get_int(AId::OFF),
        expr,
    })
}

fn model_subtable(subtable: &Sla) -> Sym {
    let decision = model_decision(&subtable[EId::DECISION]);
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
    let mut tokenfield = model_tokenfield(&varlist[EId::TOKENFIELD]);
    let mut vars = varlist
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

    for (i, el) in symtab.els.iter().enumerate() {
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

impl Sleigh {
    pub(crate) fn new(sleigh: Sla) -> Self {
        let symtab = model_symtab(&sleigh[EId::SYMBOL_TABLE]);
        Sleigh { symtab }
    }
}
