mod decode;

use crate::decode::decode;

fn main() {
    let mut buf = [0, 0];
    for i in 0..=255 {
        buf[0] = i;
        if let Some(out) = decode(&buf) {
            println!("{:x} {}", i, out);

            let vec = out.pcode();
            for v in vec {
                println!("\t{:?}", v);
            }
        } else {
            // println!("{:x}", i);
        }
        /*
        for j in 0..=255 {
            buf[1] = j;
            println!("{:?}", decode(&buf));
        }*/
    }
}
