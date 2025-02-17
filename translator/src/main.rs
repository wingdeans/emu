mod sleigh;

use crate::sleigh::{Sla, Sleigh};

fn main() {
    let sleigh = Sleigh::new("sm83.sla");
    let sla = Sla::new("sm83.sla");
}
