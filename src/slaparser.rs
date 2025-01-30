use crate::slaformat::{AId, EId};
use crate::slareader::{Attribute, SlaReader, Tag};

pub(crate) struct SlaParser<'a> {
    reader: &'a mut SlaReader<'a>,
    level: u32
}

impl<'a> SlaParser<'a> {
    pub(crate) fn new(reader: &'a mut SlaReader<'a>) -> Self {
        SlaParser { reader, level: 0 }
    }
}

#[derive(Debug)]
pub(crate) enum SlaItem<'a> {
    Elem(EId),
    Attr(AId, Attribute<'a>),
}

impl<'a> SlaParser<'a> {
    pub fn skip_elem(&mut self) {
        let mut level = 1;
        for tag in &mut *self.reader {
            match tag {
                Tag::ElStart(_) => level += 1,
                Tag::ElEnd => {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    pub fn enter(&mut self) {
        self.level += 1
    }
}

impl<'a> Iterator for SlaParser<'a> {
    type Item = SlaItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.next().unwrap() {
                Tag::ElStart(id) => return Some(SlaItem::Elem(id)),
                Tag::Attr(id, attr) => return Some(SlaItem::Attr(id, attr)),
                Tag::ElEnd => {
                    if self.level == 0 { return None }
                    self.level -= 1;
                }
            }
        }
    }
}
