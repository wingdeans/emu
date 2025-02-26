#![feature(btree_cursors)]

mod gbloader;
mod pcode;
mod sleigh;

use crate::gbloader::GBLoader;
use crate::pcode::Pcode;
use crate::sleigh::Sleigh;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Bound;

use serde::Serialize;

#[derive(Debug)]
struct Insn {
    len: u8,
    pcodes: Vec<Pcode>,
}

#[derive(Debug, PartialEq)]
enum Branch {
    Jmp(u64),
    Pop,
    Fallthru,
}

#[derive(Debug, Serialize)]
struct Output {
    insns: Vec<u64>,
    data: Vec<(u64, u64)>,
}

fn disassemble_from(pc: u64, sleigh: &Sleigh, loader: &GBLoader, insns: &mut BTreeMap<u64, Insn>) {
    let mut pc = pc;
    let mut cursor = insns.upper_bound_mut(Bound::Included(&pc));
    // pc of (potential) next instruction
    let mut end_pc = match cursor.peek_next() {
        Some((addr, _)) => *addr,
        None => u64::MAX,
    };

    if let Some(prev) = cursor.peek_prev() {
        if *prev.0 == pc {
            return;
        }
    }

    let mut branch_stack = Vec::new();

    'outer: loop {
        let (a, b) = sleigh.disassemble(&loader.0[pc as usize..], pc);
        println!("0x{:x} {} {}", pc, a, b);

        let (pcodes, len) = sleigh.pcode(&loader.0[pc as usize..], pc);

        let mut branch = Branch::Fallthru;
        for pcode in &pcodes {
            println!("\t{:x?}", pcode);

            assert_eq!(branch, Branch::Fallthru);
            branch = match pcode {
                Pcode::Call(target) => {
                    if target.offset != 0xFF80 {
                        branch_stack.push(target.offset);
                    }
                    Branch::Fallthru
                }
                Pcode::Branch(target) => Branch::Jmp(target.offset),
                Pcode::CBranch(target, _) => {
                    branch_stack.push(target.offset);
                    Branch::Fallthru
                }
                Pcode::Return => {
                    if let Some(addr) = branch_stack.pop() {
                        Branch::Jmp(addr)
                    } else {
                        break 'outer;
                    }
                }
                Pcode::BranchInd(_) => Branch::Pop,
                _ => Branch::Fallthru,
            }
        }

        cursor
            .insert_before(
                pc,
                Insn {
                    len: len.try_into().unwrap(),
                    pcodes,
                },
            )
            .unwrap();

        loop {
            match branch {
                Branch::Jmp(jmp) => {
                    cursor = insns.upper_bound_mut(Bound::Included(&jmp));
                    if let Some(prev) = cursor.peek_prev() {
                        if *prev.0 == jmp {
                            branch = Branch::Pop;
                            continue;
                        }
                    }
                    pc = jmp;

                    end_pc = match cursor.peek_next() {
                        Some((addr, _)) => *addr,
                        None => u64::MAX,
                    };
                    break;
                }
                Branch::Pop => {
                    if let Some(addr) = branch_stack.pop() {
                        branch = Branch::Jmp(addr);
                    } else {
                        break 'outer;
                    }
                }
                Branch::Fallthru => {
                    pc += len as u64;
                    if pc >= end_pc {
                        branch = Branch::Jmp(pc);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let rom_path = &args[1];

    let sleigh = Sleigh::new("sm83.sla");

    let loader = GBLoader::new(rom_path)?;
    let mut insns = BTreeMap::new();
    disassemble_from(0x100, &sleigh, &loader, &mut insns);

    // restarts/interrupts
    for pc in (0..=0x60).step_by(8) {
        if loader.0[pc as usize] != 0xFF {
            disassemble_from(pc, &sleigh, &loader, &mut insns);
        }
    }

    let mut data = Vec::new();

    let mut data_start = 0;
    let mut cnt = 0;
    for (i, b) in loader.0.iter().enumerate() {
        if *b == 0xFF {
            cnt += 1;
            if cnt == 4 {
                data_start = i - 3;
            }
        } else {
            if cnt >= 4 {
                data.push((data_start as u64, i as u64));
            }
            cnt = 0;
        }
    }
    if cnt != 0 {
        data.push((data_start as u64, loader.0.len() as u64));
    }

    let out_writer = BufWriter::new(File::create(rom_path.to_string() + ".json")?);
    let output = Output {
        insns: insns.keys().copied().collect(),
        data,
    };
    serde_json::to_writer(out_writer, &output)?;

    Ok(())
}
