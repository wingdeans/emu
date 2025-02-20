mod gbloader;
mod pcode;
mod sleigh;

use crate::gbloader::GBLoader;
use crate::pcode::Pcode;
use crate::sleigh::Sleigh;

use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let sleigh = Sleigh::new("sm83.sla");

    let loader = GBLoader::new("../../roms/roms/porklike-gb.gb")?;
    let _mappings = loader.mappings()?;

    let mut pcode_buf = Vec::new();
    let mut pc = 0x100;

    #[derive(Debug)]
    enum Branchpoint {
        Call(u64, u64),
        Branch(u64),
    }
    let mut branch_stack = Vec::new();

    let mut calls: BTreeMap<u64, bool> = BTreeMap::new();
    let mut seen: BTreeMap<u64, (u32, u32)> = BTreeMap::new();

    'outer: loop {
        let (a, b) = sleigh.disassemble(&loader.0[pc as usize..], pc);
        println!("0x{:x} {} {}", pc, a, b);
        let pcode_start = pcode_buf.len();
        pc += sleigh.pcode(&loader.0[pc as usize..], pc, &mut pcode_buf) as u64;

        let old_pc = pc;
        let mut done = false;
        for pcode in &pcode_buf[pcode_start..] {
            match pcode {
                Pcode::Branch(target) => pc = target.offset,
                Pcode::CBranch(target, _) => {
                    branch_stack.push(Branchpoint::Branch(target.offset));
                }
                Pcode::Call(target) => {
                    if let Some(true) = calls.get(&target.offset) {
                    } else {
                        branch_stack.push(Branchpoint::Call(target.offset, pc));
                        pc = target.offset;
                    }
                }
                Pcode::Return => match branch_stack.pop() {
                    Some(Branchpoint::Call(target, addr)) => {
                        calls.insert(target, true);
                        pc = addr
                    }
                    Some(Branchpoint::Branch(addr)) => {
                        for bp in branch_stack.iter_mut().rev() {
                            if let Branchpoint::Call(target, addr) = bp {
                                calls.insert(*target, true);
                            }
                        }
                        pc = addr;
                    }
                    None => (),
                },
                Pcode::BranchInd(_) => done = true,
                _ => (),
            }
        }
        for pcode in &pcode_buf[pcode_start..] {
            println!("\t{:x?}", pcode);
        }

        if pc != old_pc || done {
            loop {
                if seen.insert(pc, (0, 0)).is_some() || done {
                    match branch_stack.pop() {
                        Some(Branchpoint::Call(_, addr)) => pc = addr,
                        Some(Branchpoint::Call(_, addr)) => (),
                        Some(Branchpoint::Branch(addr)) => pc = addr,
                        None => break 'outer,
                    }
                    done = false;
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}
