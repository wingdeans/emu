use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum GBLoadError {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

// #[derive(Debug, Serialize)]
// pub struct Mapping {
// pub(crate) src: u32,
// pub(crate) srclen: u32,
// pub(crate) dst: u32,
// pub(crate) dstlen: u32,
// }

pub(crate) struct GBLoader(pub(crate) Vec<u8>);

impl GBLoader {
    pub(crate) fn new(path: &str) -> Result<Self, GBLoadError> {
        Ok(GBLoader(std::fs::read(path)?))
    }

    // pub(crate) fn mappings(&self) -> Result<Vec<Mapping>, GBLoadError> {
    // let mbc = self.0[0x147];
    // match mbc {
    // 1 => {
    // let full_bank_cnt = self.0.len() / 0x4000;
    // let last_bank_size = self.0.len() % 0x4000;
    // assert_eq!(0, last_bank_size);
    //
    // let bank_cnt = full_bank_cnt + (last_bank_size == 0) as usize;
    //
    // let mut mappings = Vec::with_capacity(bank_cnt);
    // mappings.push(Mapping {
    // src: 0,
    // srclen: 0x4000,
    // dst: 0,
    // dstlen: 0x4000,
    // });
    // for i in 1..full_bank_cnt as u32 {
    // mappings.push(Mapping {
    // src: i * 0x4000,
    // srclen: 0x4000,
    // dst: 0x4000 + i * 0x0100_0000,
    // dstlen: 0x4000,
    // });
    // }
    // Ok(mappings)
    // }
    // _ => todo!(),
    // }
    // }
}
