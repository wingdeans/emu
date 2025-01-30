mod slaformat;
mod slaparser;
mod slareader;

use crate::slaformat::{AId, EId};
use crate::slareader::{SlaBuf, Tag};
use crate::slareader::Attribute::{Int, Str, Uint};
use crate::slaparser::SlaParser;
use crate::slaparser::SlaItem::{Elem, Attr};

fn parse_constructor(parser: &mut SlaParser) {
    while let Some(item) = parser.next() {
        match item {
            Elem(EId::OPER | EId::PRINT | EId::OPPRINT | EId::CONSTRUCT_TPL) => {
                parser.skip_elem()
            }
            Attr(_, _) => (),
            _ => unreachable!("unknown constructor item: {:?}", item),
        }
    }
}

#[derive(Debug)]
struct Mask {
    id: u16,
    off: u8,
    nonzero: u8,
    mask: u64,
    val: u64
}

#[derive(Debug)]
enum Decision {
    Bits(u8, u8, Vec<Decision>),
    Masks(Vec<Mask>)
}

fn parse_pair(parser: &mut SlaParser) -> Mask {
    let mut id = 0;
    let (mut off, mut nonzero) = (0, 0);
    let (mut mask, mut val) = (0, 0);
    while let Some(item) = parser.next() {
        match item {
            // pair
            Attr(AId::ID, Int(aid)) => id = aid,
            // pair -> instruct pat
            Elem(EId::INSTRUCT_PAT) => parser.enter(),
            // pair -> instruct pat -> pat block
            Elem(EId::PAT_BLOCK) => parser.enter(),
            Attr(AId::OFF, Int(aoff)) => off = aoff,
            Attr(AId::NONZERO, Int(anonzero)) => nonzero = anonzero,
            // pair -> instruct pat -> pat block -> mask word
            Elem(EId::MASK_WORD) => parser.enter(),
            Attr(AId::MASK, Uint(amask)) => mask = amask,
            Attr(AId::VAL, Uint(aval)) => val = aval,
            // end
            Attr(_, _) => (),
            _ => unreachable!("unknown pair item: {:?}", item)
        }
    }
    let id = id.try_into().unwrap();
    let off = off.try_into().unwrap();
    let nonzero = nonzero.try_into().unwrap();
    Mask { id, off, nonzero, mask, val }
}

fn parse_decision(parser: &mut SlaParser) -> Decision {
    let (mut start, mut size) = (0, 0);
    let mut masks = Vec::new();
    let mut options = Vec::new();
    while let Some(item) = parser.next() {
        match item {
            Elem(EId::PAIR) => masks.push(parse_pair(parser)),
            Elem(EId::DECISION) => options.push(parse_decision(parser)),
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
        Decision::Bits(start, size, options)
    } else {
        Decision::Masks(masks)
    }
}

#[derive(Debug)]
struct Subtable {
    decision: Decision
}

fn parse_subtable(parser: &mut SlaParser) -> (u16, Subtable) {
    let mut id = 0;
    let mut decision = None;
    while let Some(item) = parser.next() {
        match item {
            Elem(EId::CONSTRUCTOR) => parse_constructor(parser),
            Elem(EId::DECISION) => decision = Some(parse_decision(parser)),
            Attr(AId::ID, Uint(aid)) => id = aid,
            Attr(_, _) => (),
            _ => unreachable!("unknown subtable item: {:?}", item),
        }
    }

    let id = id.try_into().unwrap();
    (id, Subtable { decision: decision.unwrap() })
}

fn parse_head<'a>(parser: &mut SlaParser<'a>) -> (u16, &'a str) {
    let mut name = None;
    let mut id = 0;
    while let Some(item) = parser.next() {
        match item {
            Attr(AId::NAME, Str(aname)) => name = Some(aname),
            Attr(AId::ID, Uint(aid)) => id = aid,
            Attr(AId::SCOPE, Uint(_)) => (),
            _ => unreachable!("unknown head item: {:?}", item)
        }
    }
    (id.try_into().unwrap(), name.unwrap())
}

struct SymbolTable {
    instruction: Subtable
}

fn parse_symbol_table(parser: &mut SlaParser) -> SymbolTable {
    let mut heads = Vec::new();
    let mut subtables = Vec::new();
    while let Some(item) = parser.next() {
        match item {
            Elem(EId::SUBTABLE_SYM_HEAD) => heads.push(parse_head(parser)),
            Elem(EId::SUBTABLE_SYM) => subtables.push(parse_subtable(parser)),
            Elem(_) => parser.skip_elem(),
            Attr(_, _) => (),
            _ => unreachable!("unknown symbol table item: {:?}", item),
        }
    }

    let instruction_id = heads.iter().find(|(_, name)|name == &"instruction").unwrap().0;
    let instruction_idx = subtables.iter().position(|(id, _)| *id == instruction_id).unwrap();
    let instruction = subtables.swap_remove(instruction_idx).1;

    SymbolTable { instruction }
}

/*
fn disassemble(decision: &Decision, byte: u8) {
    match &decision {
        Decision::Bits(start, size, options) => {
            println!("{:x} {} {}", byte, start, size);
            let idx = (byte >> 8 - (start + size)) & ((1 << size) - 1);
            disassemble(&options[idx as usize], byte)
        },
        Decision::Masks(masks) => {
            for Mask { id, off, nonzero, mask, val } in masks {
                assert_eq!(*off, 0);
                for i in 1..=*nonzero {
                    let shift = 32 - 8 * i;
                    if (mask >> shift) as u8 & byte == (val >> shift) as u8 {
                        println!("{:x} got: {}", byte, id)
                    }
                }
            }
        }
    }
}
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = SlaBuf::new("sm83.sla")?;
    let mut reader = buf.into_iter();
    let mut parser = SlaParser::new(&mut reader);

    let mut symtab = None;

    if let Elem(EId::SLEIGH) = parser.next().unwrap() {
        while let Some(item) = parser.next() {
            match item {
                Elem(EId::SOURCEFILES | EId::SPACES) => parser.skip_elem(),
                Elem(EId::SYMBOL_TABLE) => symtab = Some(parse_symbol_table(&mut parser)),
                Attr(_, _) => (),
                _ => unreachable!("unknown sleigh item: {:?}", item),
            }
        }
    } else {
        unreachable!()
    }

    let symtab = symtab.unwrap();

    Ok(())
}
