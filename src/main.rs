mod codegen;
mod pcodeop;
mod slaformat;
mod slaparser;
mod slareader;

use crate::slareader::SlaBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buf = SlaBuf::new("sm83.sla")?;
    let sleigh = buf.parse();
    codegen::emit(sleigh)?;

    Ok(())
}
