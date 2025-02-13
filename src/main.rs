mod codegen;
mod pcodeop;
mod slaformat;
mod slamodel;
mod slaparser;

use crate::slamodel::Sleigh;
use crate::slaparser::SlaBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = SlaBuf::new("sm83.sla")?;
    let sla = buf.parse();
    println!("{}", sla);
    // let sleigh = Sleigh::new(sla);
    // codegen::emit(sleigh)?;

    Ok(())
}
