#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(clippy::identity_op)]
#![allow(clippy::double_parens)]
#![allow(clippy::ptr_arg)]
pub(crate) struct Pcode();
pub(crate) struct Insn(Sym0);
pub(crate) fn decode(buf: &[u8]) -> Option<Insn> {
    Sym0::decode(buf).map(Insn)
}
impl Insn {
    pub(crate) fn pcode(&self, vec: &mut Vec<Pcode>) {
        self.0.pcode(vec)
    }
}
impl std::fmt::Display for Insn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

enum Sym0 {
    Variant0(Op60, Op61),
    Variant1(Op62, Op63),
    Variant2(Op64, Op65),
    Variant3(Op67, Op66),
    Variant4(Op68, Op69),
    Variant5(Op70, Op71),
    Variant6(Op72, Op73),
    Variant7(Op74, Op75),
    Variant8(Op77, Op76),
    Variant9(Op78, Op79),
    Variant10(Op81, Op80),
    Variant11(Op82, Op83),
    Variant12(Op85, Op84),
    Variant13(Op86, Op87),
    Variant14(Op88, Op89),
    Variant15(Op91, Op90),
    Variant16(Op93, Op92),
    Variant17(Op95, Op94),
    Variant18(Op97, Op96),
    Variant19(Op98),
    Variant20(Op99),
    Variant21(Op100),
    Variant22(Op101),
    Variant23(Op102),
    Variant24(Op103),
    Variant25(Op104),
    Variant26(Op105),
    Variant27(Op106),
    Variant28(Op107),
    Variant29(Op108),
    Variant30(Op109),
    Variant31(Op110),
    Variant32(Op111),
    Variant33(Op112),
    Variant34(Op113),
    Variant35(Op114),
    Variant36(Op115),
    Variant37(Op116),
    Variant38(Op117),
    Variant39(Op118),
    Variant40(Op119),
    Variant41(Op120),
    Variant42(Op121),
    Variant43(Op122),
    Variant44(Op123),
    Variant45(Op124),
    Variant46(Op125),
    Variant47(),
    Variant48(),
    Variant49(),
    Variant50(),
    Variant51(Op126),
    Variant52(Op127),
    Variant53(Op128),
    Variant54(Op129),
    Variant55(Op130),
    Variant56(Op131),
    Variant57(Op132),
    Variant58(Op133),
    Variant59(Op134),
    Variant60(Op135),
    Variant61(Op136),
    Variant62(Op137),
    Variant63(Op138),
    Variant64(Op139),
    Variant65(Op140),
    Variant66(Op141),
    Variant67(Op142, Op143),
    Variant68(Op145, Op144),
    Variant69(Op146, Op147),
    Variant70(Op149, Op148),
    Variant71(Op150, Op151),
    Variant72(Op153, Op152),
    Variant73(Op154),
    Variant74(Op155),
    Variant75(Op156),
    Variant76(Op157),
    Variant77(),
    Variant78(),
    Variant79(Op158, Op159),
    Variant80(Op160, Op161),
    Variant81(Op162, Op163),
    Variant82(Op164),
    Variant83(Op165),
    Variant84(),
    Variant85(),
    Variant86(),
    Variant87(),
    Variant88(),
    Variant89(),
    Variant90(),
    Variant91(),
    Variant92(),
    Variant93(Op166, Op167),
    Variant94(Op169, Op168),
    Variant95(Op170, Op171),
    Variant96(Op172, Op173, Op174),
    Variant97(Op175, Op176, Op178, Op177),
    Variant98(Op179),
    Variant99(Op180),
    Variant100(Op182, Op181),
    Variant101(Op183, Op184),
    Variant102(Op185),
    Variant103(Op186),
}
impl Sym0 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 6u8) & 3u8 {
            0u8 => {
                match (buf[0usize] >> 0u8) & 7u8 {
                    0u8 => {
                        match (buf[0usize] >> 5u8) & 1u8 {
                            0u8 => {
                                match (buf[0usize] >> 3u8) & 3u8 {
                                    0u8 => {
                                        if (buf[0usize] & 255u8 == 0u8) {
                                            Some(Self::Variant90())
                                        } else {
                                            None
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 8u8) {
                                            Some(
                                                Self::Variant94(Op169::decode(buf)?, Op168::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 16u8) {
                                            Some(Self::Variant85())
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 24u8) {
                                            Some(Self::Variant75(Op156::decode(buf)?))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 231u8 == 32u8) {
                                    Some(
                                        Self::Variant80(Op160::decode(buf)?, Op161::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    1u8 => {
                        match (buf[0usize] >> 3u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 207u8 == 1u8) {
                                    Some(
                                        Self::Variant93(Op166::decode(buf)?, Op167::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 207u8 == 9u8) {
                                    Some(
                                        Self::Variant100(Op182::decode(buf)?, Op181::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    2u8 => {
                        match (buf[0usize] >> 3u8) & 7u8 {
                            0u8 => {
                                if (buf[0usize] & 255u8 == 2u8) {
                                    Some(
                                        Self::Variant15(Op91::decode(buf)?, Op90::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 10u8) {
                                    Some(Self::Variant5(Op70::decode(buf)?, Op71::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            2u8 => {
                                if (buf[0usize] & 255u8 == 18u8) {
                                    Some(
                                        Self::Variant16(Op93::decode(buf)?, Op92::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            3u8 => {
                                if (buf[0usize] & 255u8 == 26u8) {
                                    Some(Self::Variant6(Op72::decode(buf)?, Op73::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            4u8 => {
                                if (buf[0usize] & 255u8 == 34u8) {
                                    Some(
                                        Self::Variant17(Op95::decode(buf)?, Op94::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            5u8 => {
                                if (buf[0usize] & 255u8 == 42u8) {
                                    Some(
                                        Self::Variant13(Op86::decode(buf)?, Op87::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            6u8 => {
                                if (buf[0usize] & 255u8 == 50u8) {
                                    Some(
                                        Self::Variant18(Op97::decode(buf)?, Op96::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            7u8 => {
                                if (buf[0usize] & 255u8 == 58u8) {
                                    Some(
                                        Self::Variant14(Op88::decode(buf)?, Op89::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    3u8 => {
                        match (buf[0usize] >> 3u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 207u8 == 3u8) {
                                    Some(Self::Variant102(Op185::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 207u8 == 11u8) {
                                    Some(Self::Variant103(Op186::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    4u8 => {
                        if (buf[0usize] & 255u8 == 52u8) {
                            Some(Self::Variant44(Op123::decode(buf)?))
                        } else if (buf[0usize] & 199u8 == 4u8) {
                            Some(Self::Variant43(Op122::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    5u8 => {
                        if (buf[0usize] & 255u8 == 53u8) {
                            Some(Self::Variant46(Op125::decode(buf)?))
                        } else if (buf[0usize] & 199u8 == 5u8) {
                            Some(Self::Variant45(Op124::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    6u8 => {
                        if (buf[0usize] & 255u8 == 54u8) {
                            Some(Self::Variant4(Op68::decode(buf)?, Op69::decode(buf)?))
                        } else if (buf[0usize] & 199u8 == 6u8) {
                            Some(Self::Variant1(Op62::decode(buf)?, Op63::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    7u8 => {
                        match (buf[0usize] >> 3u8) & 7u8 {
                            0u8 => {
                                if (buf[0usize] & 255u8 == 7u8) {
                                    Some(Self::Variant47())
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 15u8) {
                                    Some(Self::Variant49())
                                } else {
                                    None
                                }
                            }
                            2u8 => {
                                if (buf[0usize] & 255u8 == 23u8) {
                                    Some(Self::Variant48())
                                } else {
                                    None
                                }
                            }
                            3u8 => {
                                if (buf[0usize] & 255u8 == 31u8) {
                                    Some(Self::Variant50())
                                } else {
                                    None
                                }
                            }
                            4u8 => {
                                if (buf[0usize] & 255u8 == 39u8) {
                                    Some(Self::Variant91())
                                } else {
                                    None
                                }
                            }
                            5u8 => {
                                if (buf[0usize] & 255u8 == 47u8) {
                                    Some(Self::Variant92())
                                } else {
                                    None
                                }
                            }
                            6u8 => {
                                if (buf[0usize] & 255u8 == 55u8) {
                                    Some(Self::Variant89())
                                } else {
                                    None
                                }
                            }
                            7u8 => {
                                if (buf[0usize] & 255u8 == 63u8) {
                                    Some(Self::Variant88())
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            1u8 => {
                if (buf[0usize] & 255u8 == 118u8) {
                    Some(Self::Variant84())
                } else if (buf[0usize] & 199u8 == 70u8) {
                    Some(Self::Variant2(Op64::decode(buf)?, Op65::decode(buf)?))
                } else if (buf[0usize] & 248u8 == 112u8) {
                    Some(Self::Variant3(Op67::decode(buf)?, Op66::decode(buf)?))
                } else if (buf[0usize] & 192u8 == 64u8) {
                    Some(Self::Variant0(Op60::decode(buf)?, Op61::decode(buf)?))
                } else {
                    None
                }
            }
            2u8 => {
                match (buf[0usize] >> 3u8) & 7u8 {
                    0u8 => {
                        if (buf[0usize] & 255u8 == 134u8) {
                            Some(Self::Variant21(Op100::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 128u8) {
                            Some(Self::Variant19(Op98::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    1u8 => {
                        if (buf[0usize] & 255u8 == 142u8) {
                            Some(Self::Variant24(Op103::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 136u8) {
                            Some(Self::Variant22(Op101::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    2u8 => {
                        if (buf[0usize] & 255u8 == 150u8) {
                            Some(Self::Variant27(Op106::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 144u8) {
                            Some(Self::Variant25(Op104::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    3u8 => {
                        if (buf[0usize] & 255u8 == 158u8) {
                            Some(Self::Variant30(Op109::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 152u8) {
                            Some(Self::Variant28(Op107::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    4u8 => {
                        if (buf[0usize] & 255u8 == 166u8) {
                            Some(Self::Variant33(Op112::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 160u8) {
                            Some(Self::Variant31(Op110::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    5u8 => {
                        if (buf[0usize] & 255u8 == 174u8) {
                            Some(Self::Variant36(Op115::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 168u8) {
                            Some(Self::Variant34(Op113::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    6u8 => {
                        if (buf[0usize] & 255u8 == 182u8) {
                            Some(Self::Variant39(Op118::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 176u8) {
                            Some(Self::Variant37(Op116::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    7u8 => {
                        if (buf[0usize] & 255u8 == 190u8) {
                            Some(Self::Variant42(Op121::decode(buf)?))
                        } else if (buf[0usize] & 248u8 == 184u8) {
                            Some(Self::Variant40(Op119::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    _ => unreachable!(),
                }
            }
            3u8 => {
                match (buf[0usize] >> 0u8) & 7u8 {
                    0u8 => {
                        match (buf[0usize] >> 5u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 231u8 == 192u8) {
                                    Some(Self::Variant82(Op164::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                match (buf[0usize] >> 3u8) & 3u8 {
                                    0u8 => {
                                        if (buf[0usize] & 255u8 == 224u8) {
                                            Some(
                                                Self::Variant10(Op81::decode(buf)?, Op80::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 232u8) {
                                            Some(
                                                Self::Variant101(Op183::decode(buf)?, Op184::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 240u8) {
                                            Some(Self::Variant9(Op78::decode(buf)?, Op79::decode(buf)?))
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 248u8)
                                            && (buf[1usize] & 128u8 == 128u8)
                                        {
                                            Some(
                                                Self::Variant97(
                                                    Op175::decode(buf)?,
                                                    Op176::decode(buf)?,
                                                    Op178::decode(buf)?,
                                                    Op177::decode(buf)?,
                                                ),
                                            )
                                        } else if (buf[0usize] & 255u8 == 248u8) {
                                            Some(
                                                Self::Variant96(
                                                    Op172::decode(buf)?,
                                                    Op173::decode(buf)?,
                                                    Op174::decode(buf)?,
                                                ),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    1u8 => {
                        match (buf[0usize] >> 3u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 207u8 == 193u8) {
                                    Some(Self::Variant99(Op180::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                match (buf[0usize] >> 4u8) & 3u8 {
                                    0u8 => {
                                        if (buf[0usize] & 255u8 == 201u8) {
                                            Some(Self::Variant77())
                                        } else {
                                            None
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 217u8) {
                                            Some(Self::Variant78())
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 233u8) {
                                            Some(Self::Variant74(Op155::decode(buf)?))
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 249u8) {
                                            Some(
                                                Self::Variant95(Op170::decode(buf)?, Op171::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    2u8 => {
                        match (buf[0usize] >> 5u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 231u8 == 194u8) {
                                    Some(
                                        Self::Variant79(Op158::decode(buf)?, Op159::decode(buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                match (buf[0usize] >> 3u8) & 3u8 {
                                    0u8 => {
                                        if (buf[0usize] & 255u8 == 226u8) {
                                            Some(Self::Variant8(Op77::decode(buf)?, Op76::decode(buf)?))
                                        } else {
                                            None
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 234u8) {
                                            Some(
                                                Self::Variant12(Op85::decode(buf)?, Op84::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 242u8) {
                                            Some(Self::Variant7(Op74::decode(buf)?, Op75::decode(buf)?))
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 250u8) {
                                            Some(
                                                Self::Variant11(Op82::decode(buf)?, Op83::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    3u8 => {
                        match (buf[0usize] >> 3u8) & 3u8 {
                            0u8 => {
                                if (buf[0usize] & 255u8 == 195u8) {
                                    Some(Self::Variant73(Op154::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                match (buf[1usize] >> 6u8) & 3u8 {
                                    0u8 => {
                                        match (buf[1usize] >> 3u8) & 7u8 {
                                            0u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 6u8)
                                                {
                                                    Some(Self::Variant52(Op127::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 0u8)
                                                {
                                                    Some(Self::Variant51(Op126::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            1u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 14u8)
                                                {
                                                    Some(Self::Variant54(Op129::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 8u8)
                                                {
                                                    Some(Self::Variant53(Op128::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            2u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 22u8)
                                                {
                                                    Some(Self::Variant56(Op131::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 16u8)
                                                {
                                                    Some(Self::Variant55(Op130::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            3u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 30u8)
                                                {
                                                    Some(Self::Variant58(Op133::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 24u8)
                                                {
                                                    Some(Self::Variant57(Op132::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            4u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 38u8)
                                                {
                                                    Some(Self::Variant60(Op135::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 32u8)
                                                {
                                                    Some(Self::Variant59(Op134::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            5u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 46u8)
                                                {
                                                    Some(Self::Variant62(Op137::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 40u8)
                                                {
                                                    Some(Self::Variant61(Op136::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            6u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 54u8)
                                                {
                                                    Some(Self::Variant64(Op139::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 48u8)
                                                {
                                                    Some(Self::Variant63(Op138::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            7u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 62u8)
                                                {
                                                    Some(Self::Variant66(Op141::decode(buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 56u8)
                                                {
                                                    Some(Self::Variant65(Op140::decode(buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 199u8 == 70u8)
                                        {
                                            Some(
                                                Self::Variant68(Op145::decode(buf)?, Op144::decode(buf)?),
                                            )
                                        } else if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 192u8 == 64u8)
                                        {
                                            Some(
                                                Self::Variant67(Op142::decode(buf)?, Op143::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 199u8 == 134u8)
                                        {
                                            Some(
                                                Self::Variant70(Op149::decode(buf)?, Op148::decode(buf)?),
                                            )
                                        } else if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 192u8 == 128u8)
                                        {
                                            Some(
                                                Self::Variant69(Op146::decode(buf)?, Op147::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 199u8 == 198u8)
                                        {
                                            Some(
                                                Self::Variant72(Op153::decode(buf)?, Op152::decode(buf)?),
                                            )
                                        } else if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 192u8 == 192u8)
                                        {
                                            Some(
                                                Self::Variant71(Op150::decode(buf)?, Op151::decode(buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            2u8 => {
                                if (buf[0usize] & 255u8 == 243u8) {
                                    Some(Self::Variant86())
                                } else {
                                    None
                                }
                            }
                            3u8 => {
                                if (buf[0usize] & 255u8 == 251u8) {
                                    Some(Self::Variant87())
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    4u8 => {
                        if (buf[0usize] & 231u8 == 196u8) {
                            Some(
                                Self::Variant81(Op162::decode(buf)?, Op163::decode(buf)?),
                            )
                        } else {
                            None
                        }
                    }
                    5u8 => {
                        match (buf[0usize] >> 3u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 207u8 == 197u8) {
                                    Some(Self::Variant98(Op179::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 205u8) {
                                    Some(Self::Variant76(Op157::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    6u8 => {
                        match (buf[0usize] >> 3u8) & 7u8 {
                            0u8 => {
                                if (buf[0usize] & 255u8 == 198u8) {
                                    Some(Self::Variant20(Op99::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 206u8) {
                                    Some(Self::Variant23(Op102::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            2u8 => {
                                if (buf[0usize] & 255u8 == 214u8) {
                                    Some(Self::Variant26(Op105::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            3u8 => {
                                if (buf[0usize] & 255u8 == 222u8) {
                                    Some(Self::Variant29(Op108::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            4u8 => {
                                if (buf[0usize] & 255u8 == 230u8) {
                                    Some(Self::Variant32(Op111::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            5u8 => {
                                if (buf[0usize] & 255u8 == 238u8) {
                                    Some(Self::Variant35(Op114::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            6u8 => {
                                if (buf[0usize] & 255u8 == 246u8) {
                                    Some(Self::Variant38(Op117::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            7u8 => {
                                if (buf[0usize] & 255u8 == 254u8) {
                                    Some(Self::Variant41(Op120::decode(buf)?))
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    7u8 => {
                        if (buf[0usize] & 199u8 == 199u8) {
                            Some(Self::Variant83(Op165::decode(buf)?))
                        } else {
                            None
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0, op1) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant1(op0, op1) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant2(op0, op1) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
            }
            Self::Variant3(op0, op1) => {
                println!("\t{}", "Unk");
            }
            Self::Variant4(op0, op1) => {
                println!("\t{}", "Unk");
            }
            Self::Variant5(op0, op1) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
            }
            Self::Variant6(op0, op1) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(4), size: Real(2) })"
                );
            }
            Self::Variant7(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant8(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
            }
            Self::Variant9(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant10(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
            }
            Self::Variant11(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant12(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
            }
            Self::Variant13(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant14(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant15(op0, op1) => {
                println!("\t{}", "Unk");
            }
            Self::Variant16(op0, op1) => {
                println!("\t{}", "Unk");
            }
            Self::Variant17(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
            }
            Self::Variant18(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
            }
            Self::Variant19(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(4480), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(4736), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(4992), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(4864), size: Real(1) }, Varnode { space: Unk, offset: Real(4736), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(5120), size: Real(1) }, Varnode { space: Unk, offset: Real(4864), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4992), size: Real(1) }, Varnode { space: Unk, offset: Real(5120), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5248), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(5376), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5248), size: Real(1) }, Varnode { space: Unk, offset: Real(5376), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(6144), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5504), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5632), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(5760), size: Real(1) }, Varnode { space: Unk, offset: Real(5504), size: Real(1) }, Varnode { space: Unk, offset: Real(5632), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5888), size: Real(1) }, Varnode { space: Unk, offset: Real(5760), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(6016), size: Real(1) }, Varnode { space: Unk, offset: Real(5888), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(6272), size: Real(1) }, Varnode { space: Unk, offset: Real(6016), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6144), size: Real(1) }, Varnode { space: Unk, offset: Real(6272), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(6528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(6400), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(6656), size: Real(1) }, Varnode { space: Unk, offset: Real(6400), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6528), size: Real(1) }, Varnode { space: Unk, offset: Real(6656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4736), size: Real(1) })"
                );
            }
            Self::Variant20(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(4480), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(4736), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(4992), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(4864), size: Real(1) }, Varnode { space: Unk, offset: Real(4736), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(5120), size: Real(1) }, Varnode { space: Unk, offset: Real(4864), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4992), size: Real(1) }, Varnode { space: Unk, offset: Real(5120), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5248), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(5376), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5248), size: Real(1) }, Varnode { space: Unk, offset: Real(5376), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(6144), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5504), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5632), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(5760), size: Real(1) }, Varnode { space: Unk, offset: Real(5504), size: Real(1) }, Varnode { space: Unk, offset: Real(5632), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5888), size: Real(1) }, Varnode { space: Unk, offset: Real(5760), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(6016), size: Real(1) }, Varnode { space: Unk, offset: Real(5888), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(6272), size: Real(1) }, Varnode { space: Unk, offset: Real(6016), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6144), size: Real(1) }, Varnode { space: Unk, offset: Real(6272), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(6528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(6400), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(6656), size: Real(1) }, Varnode { space: Unk, offset: Real(6400), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6528), size: Real(1) }, Varnode { space: Unk, offset: Real(6656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4736), size: Real(1) })"
                );
            }
            Self::Variant21(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(4480), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(4736), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(4992), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(4864), size: Real(1) }, Varnode { space: Unk, offset: Real(4736), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(5120), size: Real(1) }, Varnode { space: Unk, offset: Real(4864), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4992), size: Real(1) }, Varnode { space: Unk, offset: Real(5120), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5248), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(5376), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5248), size: Real(1) }, Varnode { space: Unk, offset: Real(5376), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(6144), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5504), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5632), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(5760), size: Real(1) }, Varnode { space: Unk, offset: Real(5504), size: Real(1) }, Varnode { space: Unk, offset: Real(5632), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(5888), size: Real(1) }, Varnode { space: Unk, offset: Real(5760), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(6016), size: Real(1) }, Varnode { space: Unk, offset: Real(5888), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(6272), size: Real(1) }, Varnode { space: Unk, offset: Real(6016), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6144), size: Real(1) }, Varnode { space: Unk, offset: Real(6272), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(6528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(6400), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(6656), size: Real(1) }, Varnode { space: Unk, offset: Real(6400), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6528), size: Real(1) }, Varnode { space: Unk, offset: Real(6656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4736), size: Real(1) })"
                );
            }
            Self::Variant22(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(6784), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(6912), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(7168), size: Real(1) }, Varnode { space: Unk, offset: Real(6912), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(7296), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(7552), size: Real(1) }, Varnode { space: Unk, offset: Real(7296), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(7808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(7680), size: Real(1) }, Varnode { space: Unk, offset: Real(7552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(7936), size: Real(1) }, Varnode { space: Unk, offset: Real(7680), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7808), size: Real(1) }, Varnode { space: Unk, offset: Real(7936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(8192), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(8064), size: Real(1) }, Varnode { space: Unk, offset: Real(8192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(9088), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8320), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8448), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(8576), size: Real(1) }, Varnode { space: Unk, offset: Real(8320), size: Real(1) }, Varnode { space: Unk, offset: Real(8448), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(8704), size: Real(1) }, Varnode { space: Unk, offset: Real(8576), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8832), size: Real(1) }, Varnode { space: Unk, offset: Real(8704), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(8960), size: Real(1) }, Varnode { space: Unk, offset: Real(8832), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(9216), size: Real(1) }, Varnode { space: Unk, offset: Real(8960), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(9088), size: Real(1) }, Varnode { space: Unk, offset: Real(9216), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(9600), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(9344), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(9472), size: Real(1) }, Varnode { space: Unk, offset: Real(9344), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(9728), size: Real(1) }, Varnode { space: Unk, offset: Real(9472), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(9600), size: Real(1) }, Varnode { space: Unk, offset: Real(9728), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(7552), size: Real(1) })"
                );
            }
            Self::Variant23(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(6784), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(6912), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(7168), size: Real(1) }, Varnode { space: Unk, offset: Real(6912), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(7296), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(7552), size: Real(1) }, Varnode { space: Unk, offset: Real(7296), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(7808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(7680), size: Real(1) }, Varnode { space: Unk, offset: Real(7552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(7936), size: Real(1) }, Varnode { space: Unk, offset: Real(7680), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7808), size: Real(1) }, Varnode { space: Unk, offset: Real(7936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(8192), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(8064), size: Real(1) }, Varnode { space: Unk, offset: Real(8192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(9088), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8320), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8448), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(8576), size: Real(1) }, Varnode { space: Unk, offset: Real(8320), size: Real(1) }, Varnode { space: Unk, offset: Real(8448), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(8704), size: Real(1) }, Varnode { space: Unk, offset: Real(8576), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8832), size: Real(1) }, Varnode { space: Unk, offset: Real(8704), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(8960), size: Real(1) }, Varnode { space: Unk, offset: Real(8832), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(9216), size: Real(1) }, Varnode { space: Unk, offset: Real(8960), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(9088), size: Real(1) }, Varnode { space: Unk, offset: Real(9216), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(9600), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(9344), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(9472), size: Real(1) }, Varnode { space: Unk, offset: Real(9344), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(9728), size: Real(1) }, Varnode { space: Unk, offset: Real(9472), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(9600), size: Real(1) }, Varnode { space: Unk, offset: Real(9728), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(7552), size: Real(1) })"
                );
            }
            Self::Variant24(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(6784), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(6912), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(7168), size: Real(1) }, Varnode { space: Unk, offset: Real(6912), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(7296), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(7552), size: Real(1) }, Varnode { space: Unk, offset: Real(7296), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(7808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(7680), size: Real(1) }, Varnode { space: Unk, offset: Real(7552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(7936), size: Real(1) }, Varnode { space: Unk, offset: Real(7680), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7808), size: Real(1) }, Varnode { space: Unk, offset: Real(7936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(8192), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(8064), size: Real(1) }, Varnode { space: Unk, offset: Real(8192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(9088), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8320), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8448), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(8576), size: Real(1) }, Varnode { space: Unk, offset: Real(8320), size: Real(1) }, Varnode { space: Unk, offset: Real(8448), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(8704), size: Real(1) }, Varnode { space: Unk, offset: Real(8576), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(8832), size: Real(1) }, Varnode { space: Unk, offset: Real(8704), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(8960), size: Real(1) }, Varnode { space: Unk, offset: Real(8832), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(9216), size: Real(1) }, Varnode { space: Unk, offset: Real(8960), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(9088), size: Real(1) }, Varnode { space: Unk, offset: Real(9216), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(9600), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(9344), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(9472), size: Real(1) }, Varnode { space: Unk, offset: Real(9344), size: Real(1) }, Varnode { space: Unk, offset: Real(7168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(9728), size: Real(1) }, Varnode { space: Unk, offset: Real(9472), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(9600), size: Real(1) }, Varnode { space: Unk, offset: Real(9728), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(7552), size: Real(1) })"
                );
            }
            Self::Variant25(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(9856), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10112), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10368), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(10240), size: Real(1) }, Varnode { space: Unk, offset: Real(10112), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(10496), size: Real(1) }, Varnode { space: Unk, offset: Real(10240), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(10368), size: Real(1) }, Varnode { space: Unk, offset: Real(10496), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10624), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(10752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(10624), size: Real(1) }, Varnode { space: Unk, offset: Real(10752), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11520), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10880), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11008), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(11136), size: Real(1) }, Varnode { space: Unk, offset: Real(10880), size: Real(1) }, Varnode { space: Unk, offset: Real(11008), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11264), size: Real(1) }, Varnode { space: Unk, offset: Real(11136), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(11392), size: Real(1) }, Varnode { space: Unk, offset: Real(11264), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(11648), size: Real(1) }, Varnode { space: Unk, offset: Real(11392), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(11520), size: Real(1) }, Varnode { space: Unk, offset: Real(11648), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11904), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(11776), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(12032), size: Real(1) }, Varnode { space: Unk, offset: Real(11776), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(11904), size: Real(1) }, Varnode { space: Unk, offset: Real(12032), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(10112), size: Real(1) })"
                );
            }
            Self::Variant26(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(9856), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10112), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10368), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(10240), size: Real(1) }, Varnode { space: Unk, offset: Real(10112), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(10496), size: Real(1) }, Varnode { space: Unk, offset: Real(10240), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(10368), size: Real(1) }, Varnode { space: Unk, offset: Real(10496), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10624), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(10752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(10624), size: Real(1) }, Varnode { space: Unk, offset: Real(10752), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11520), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10880), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11008), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(11136), size: Real(1) }, Varnode { space: Unk, offset: Real(10880), size: Real(1) }, Varnode { space: Unk, offset: Real(11008), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11264), size: Real(1) }, Varnode { space: Unk, offset: Real(11136), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(11392), size: Real(1) }, Varnode { space: Unk, offset: Real(11264), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(11648), size: Real(1) }, Varnode { space: Unk, offset: Real(11392), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(11520), size: Real(1) }, Varnode { space: Unk, offset: Real(11648), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11904), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(11776), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(12032), size: Real(1) }, Varnode { space: Unk, offset: Real(11776), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(11904), size: Real(1) }, Varnode { space: Unk, offset: Real(12032), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(10112), size: Real(1) })"
                );
            }
            Self::Variant27(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(9856), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10112), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10368), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(10240), size: Real(1) }, Varnode { space: Unk, offset: Real(10112), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(10496), size: Real(1) }, Varnode { space: Unk, offset: Real(10240), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(10368), size: Real(1) }, Varnode { space: Unk, offset: Real(10496), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10624), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(10752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(10624), size: Real(1) }, Varnode { space: Unk, offset: Real(10752), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11520), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(10880), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11008), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(11136), size: Real(1) }, Varnode { space: Unk, offset: Real(10880), size: Real(1) }, Varnode { space: Unk, offset: Real(11008), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11264), size: Real(1) }, Varnode { space: Unk, offset: Real(11136), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(11392), size: Real(1) }, Varnode { space: Unk, offset: Real(11264), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(11648), size: Real(1) }, Varnode { space: Unk, offset: Real(11392), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(11520), size: Real(1) }, Varnode { space: Unk, offset: Real(11648), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(11904), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(11776), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(9856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(12032), size: Real(1) }, Varnode { space: Unk, offset: Real(11776), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(11904), size: Real(1) }, Varnode { space: Unk, offset: Real(12032), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(10112), size: Real(1) })"
                );
            }
            Self::Variant28(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(12288), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(12544), size: Real(1) }, Varnode { space: Unk, offset: Real(12288), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(12672), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(12928), size: Real(1) }, Varnode { space: Unk, offset: Real(12672), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13184), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(13056), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(13312), size: Real(1) }, Varnode { space: Unk, offset: Real(13056), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(13184), size: Real(1) }, Varnode { space: Unk, offset: Real(13312), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13440), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(13568), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(13440), size: Real(1) }, Varnode { space: Unk, offset: Real(13568), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14464), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13696), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13824), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(13952), size: Real(1) }, Varnode { space: Unk, offset: Real(13696), size: Real(1) }, Varnode { space: Unk, offset: Real(13824), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(14080), size: Real(1) }, Varnode { space: Unk, offset: Real(13952), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14208), size: Real(1) }, Varnode { space: Unk, offset: Real(14080), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(14336), size: Real(1) }, Varnode { space: Unk, offset: Real(14208), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(14592), size: Real(1) }, Varnode { space: Unk, offset: Real(14336), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(14464), size: Real(1) }, Varnode { space: Unk, offset: Real(14592), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14976), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(14720), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(14848), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(14720), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(15104), size: Real(1) }, Varnode { space: Unk, offset: Real(14848), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(14976), size: Real(1) }, Varnode { space: Unk, offset: Real(15104), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(12928), size: Real(1) })"
                );
            }
            Self::Variant29(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(12288), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(12544), size: Real(1) }, Varnode { space: Unk, offset: Real(12288), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(12672), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(12928), size: Real(1) }, Varnode { space: Unk, offset: Real(12672), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13184), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(13056), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(13312), size: Real(1) }, Varnode { space: Unk, offset: Real(13056), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(13184), size: Real(1) }, Varnode { space: Unk, offset: Real(13312), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13440), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(13568), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(13440), size: Real(1) }, Varnode { space: Unk, offset: Real(13568), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14464), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13696), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13824), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(13952), size: Real(1) }, Varnode { space: Unk, offset: Real(13696), size: Real(1) }, Varnode { space: Unk, offset: Real(13824), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(14080), size: Real(1) }, Varnode { space: Unk, offset: Real(13952), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14208), size: Real(1) }, Varnode { space: Unk, offset: Real(14080), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(14336), size: Real(1) }, Varnode { space: Unk, offset: Real(14208), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(14592), size: Real(1) }, Varnode { space: Unk, offset: Real(14336), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(14464), size: Real(1) }, Varnode { space: Unk, offset: Real(14592), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14976), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(14720), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(14848), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(14720), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(15104), size: Real(1) }, Varnode { space: Unk, offset: Real(14848), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(14976), size: Real(1) }, Varnode { space: Unk, offset: Real(15104), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(12928), size: Real(1) })"
                );
            }
            Self::Variant30(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(12288), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(12544), size: Real(1) }, Varnode { space: Unk, offset: Real(12288), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(12672), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(12928), size: Real(1) }, Varnode { space: Unk, offset: Real(12672), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13184), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(13056), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(13312), size: Real(1) }, Varnode { space: Unk, offset: Real(13056), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(13184), size: Real(1) }, Varnode { space: Unk, offset: Real(13312), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13440), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(13568), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(13440), size: Real(1) }, Varnode { space: Unk, offset: Real(13568), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14464), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13696), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(13824), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(13952), size: Real(1) }, Varnode { space: Unk, offset: Real(13696), size: Real(1) }, Varnode { space: Unk, offset: Real(13824), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(14080), size: Real(1) }, Varnode { space: Unk, offset: Real(13952), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14208), size: Real(1) }, Varnode { space: Unk, offset: Real(14080), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(14336), size: Real(1) }, Varnode { space: Unk, offset: Real(14208), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(14592), size: Real(1) }, Varnode { space: Unk, offset: Real(14336), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(14464), size: Real(1) }, Varnode { space: Unk, offset: Real(14592), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(14976), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(14720), size: Real(1) }, Varnode { space: Unk, offset: Real(12160), size: Real(1) }, Varnode { space: Unk, offset: Real(12544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(14848), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(14720), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(15104), size: Real(1) }, Varnode { space: Unk, offset: Real(14848), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(14976), size: Real(1) }, Varnode { space: Unk, offset: Real(15104), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(12928), size: Real(1) })"
                );
            }
            Self::Variant31(op0) => {
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(15616), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(15488), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(15744), size: Real(1) }, Varnode { space: Unk, offset: Real(15488), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(15616), size: Real(1) }, Varnode { space: Unk, offset: Real(15744), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(15872), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16000), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(15872), size: Real(1) }, Varnode { space: Unk, offset: Real(16000), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(16128), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16256), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(16128), size: Real(1) }, Varnode { space: Unk, offset: Real(16256), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(16384), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(16384), size: Real(1) }, Varnode { space: Unk, offset: Real(16512), size: Real(1) })"
                );
            }
            Self::Variant32(op0) => {
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(15616), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(15488), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(15744), size: Real(1) }, Varnode { space: Unk, offset: Real(15488), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(15616), size: Real(1) }, Varnode { space: Unk, offset: Real(15744), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(15872), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16000), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(15872), size: Real(1) }, Varnode { space: Unk, offset: Real(16000), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(16128), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16256), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(16128), size: Real(1) }, Varnode { space: Unk, offset: Real(16256), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(16384), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(16384), size: Real(1) }, Varnode { space: Unk, offset: Real(16512), size: Real(1) })"
                );
            }
            Self::Variant33(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(45056), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(45056), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(15616), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(15488), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(15744), size: Real(1) }, Varnode { space: Unk, offset: Real(15488), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(15616), size: Real(1) }, Varnode { space: Unk, offset: Real(15744), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(15872), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16000), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(15872), size: Real(1) }, Varnode { space: Unk, offset: Real(16000), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(16128), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16256), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(16128), size: Real(1) }, Varnode { space: Unk, offset: Real(16256), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(16384), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(16512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(16384), size: Real(1) }, Varnode { space: Unk, offset: Real(16512), size: Real(1) })"
                );
            }
            Self::Variant34(op0) => {
                println!(
                    "\t{}",
                    "IntXor(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(16896), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17152), size: Real(1) }, Varnode { space: Unk, offset: Real(16896), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17024), size: Real(1) }, Varnode { space: Unk, offset: Real(17152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17280), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17408), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17280), size: Real(1) }, Varnode { space: Unk, offset: Real(17408), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17536), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17664), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17536), size: Real(1) }, Varnode { space: Unk, offset: Real(17664), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17792), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17920), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17792), size: Real(1) }, Varnode { space: Unk, offset: Real(17920), size: Real(1) })"
                );
            }
            Self::Variant35(op0) => {
                println!(
                    "\t{}",
                    "IntXor(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(16896), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17152), size: Real(1) }, Varnode { space: Unk, offset: Real(16896), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17024), size: Real(1) }, Varnode { space: Unk, offset: Real(17152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17280), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17408), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17280), size: Real(1) }, Varnode { space: Unk, offset: Real(17408), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17536), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17664), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17536), size: Real(1) }, Varnode { space: Unk, offset: Real(17664), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17792), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17920), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17792), size: Real(1) }, Varnode { space: Unk, offset: Real(17920), size: Real(1) })"
                );
            }
            Self::Variant36(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(45312), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntXor(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(45312), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(16896), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17152), size: Real(1) }, Varnode { space: Unk, offset: Real(16896), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17024), size: Real(1) }, Varnode { space: Unk, offset: Real(17152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17280), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17408), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17280), size: Real(1) }, Varnode { space: Unk, offset: Real(17408), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17536), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17664), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17536), size: Real(1) }, Varnode { space: Unk, offset: Real(17664), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(17792), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(17920), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(17792), size: Real(1) }, Varnode { space: Unk, offset: Real(17920), size: Real(1) })"
                );
            }
            Self::Variant37(op0) => {
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18432), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(18304), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(18560), size: Real(1) }, Varnode { space: Unk, offset: Real(18304), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18432), size: Real(1) }, Varnode { space: Unk, offset: Real(18560), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18688), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(18816), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18688), size: Real(1) }, Varnode { space: Unk, offset: Real(18816), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18944), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(19072), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18944), size: Real(1) }, Varnode { space: Unk, offset: Real(19072), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(19200), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(19328), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(19200), size: Real(1) }, Varnode { space: Unk, offset: Real(19328), size: Real(1) })"
                );
            }
            Self::Variant38(op0) => {
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18432), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(18304), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(18560), size: Real(1) }, Varnode { space: Unk, offset: Real(18304), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18432), size: Real(1) }, Varnode { space: Unk, offset: Real(18560), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18688), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(18816), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18688), size: Real(1) }, Varnode { space: Unk, offset: Real(18816), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18944), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(19072), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18944), size: Real(1) }, Varnode { space: Unk, offset: Real(19072), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(19200), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(19328), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(19200), size: Real(1) }, Varnode { space: Unk, offset: Real(19328), size: Real(1) })"
                );
            }
            Self::Variant39(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(45568), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(45568), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18432), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(18304), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(18560), size: Real(1) }, Varnode { space: Unk, offset: Real(18304), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18432), size: Real(1) }, Varnode { space: Unk, offset: Real(18560), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18688), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(18816), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18688), size: Real(1) }, Varnode { space: Unk, offset: Real(18816), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(18944), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(19072), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18944), size: Real(1) }, Varnode { space: Unk, offset: Real(19072), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(19200), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(19328), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(19200), size: Real(1) }, Varnode { space: Unk, offset: Real(19328), size: Real(1) })"
                );
            }
            Self::Variant40(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(19456), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(19712), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(19968), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(19840), size: Real(1) }, Varnode { space: Unk, offset: Real(19712), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(20096), size: Real(1) }, Varnode { space: Unk, offset: Real(19840), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(19968), size: Real(1) }, Varnode { space: Unk, offset: Real(20096), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20224), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(20352), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(20224), size: Real(1) }, Varnode { space: Unk, offset: Real(20352), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(21120), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20480), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20608), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(20736), size: Real(1) }, Varnode { space: Unk, offset: Real(20480), size: Real(1) }, Varnode { space: Unk, offset: Real(20608), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20864), size: Real(1) }, Varnode { space: Unk, offset: Real(20736), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(20992), size: Real(1) }, Varnode { space: Unk, offset: Real(20864), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(21248), size: Real(1) }, Varnode { space: Unk, offset: Real(20992), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(21120), size: Real(1) }, Varnode { space: Unk, offset: Real(21248), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(21504), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(21376), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(21632), size: Real(1) }, Varnode { space: Unk, offset: Real(21376), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(21504), size: Real(1) }, Varnode { space: Unk, offset: Real(21632), size: Real(1) })"
                );
            }
            Self::Variant41(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(19456), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(19712), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(19968), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(19840), size: Real(1) }, Varnode { space: Unk, offset: Real(19712), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(20096), size: Real(1) }, Varnode { space: Unk, offset: Real(19840), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(19968), size: Real(1) }, Varnode { space: Unk, offset: Real(20096), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20224), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(20352), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(20224), size: Real(1) }, Varnode { space: Unk, offset: Real(20352), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(21120), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20480), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20608), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(20736), size: Real(1) }, Varnode { space: Unk, offset: Real(20480), size: Real(1) }, Varnode { space: Unk, offset: Real(20608), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20864), size: Real(1) }, Varnode { space: Unk, offset: Real(20736), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(20992), size: Real(1) }, Varnode { space: Unk, offset: Real(20864), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(21248), size: Real(1) }, Varnode { space: Unk, offset: Real(20992), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(21120), size: Real(1) }, Varnode { space: Unk, offset: Real(21248), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(21504), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(21376), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(21632), size: Real(1) }, Varnode { space: Unk, offset: Real(21376), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(21504), size: Real(1) }, Varnode { space: Unk, offset: Real(21632), size: Real(1) })"
                );
            }
            Self::Variant42(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(19456), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(19712), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(19968), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(19840), size: Real(1) }, Varnode { space: Unk, offset: Real(19712), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(20096), size: Real(1) }, Varnode { space: Unk, offset: Real(19840), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(19968), size: Real(1) }, Varnode { space: Unk, offset: Real(20096), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20224), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(20352), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(20224), size: Real(1) }, Varnode { space: Unk, offset: Real(20352), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(21120), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20480), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20608), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(20736), size: Real(1) }, Varnode { space: Unk, offset: Real(20480), size: Real(1) }, Varnode { space: Unk, offset: Real(20608), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(20864), size: Real(1) }, Varnode { space: Unk, offset: Real(20736), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(20992), size: Real(1) }, Varnode { space: Unk, offset: Real(20864), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(21248), size: Real(1) }, Varnode { space: Unk, offset: Real(20992), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(21120), size: Real(1) }, Varnode { space: Unk, offset: Real(21248), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(21504), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLess(Varnode { space: Unk, offset: Real(21376), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(19456), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(21632), size: Real(1) }, Varnode { space: Unk, offset: Real(21376), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(21504), size: Real(1) }, Varnode { space: Unk, offset: Real(21632), size: Real(1) })"
                );
            }
            Self::Variant43(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(21760), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(22016), size: Real(1) }, Varnode { space: Unk, offset: Real(21760), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(22272), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(22144), size: Real(1) }, Varnode { space: Unk, offset: Real(22016), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(22400), size: Real(1) }, Varnode { space: Unk, offset: Real(22144), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(22272), size: Real(1) }, Varnode { space: Unk, offset: Real(22400), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(22528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(22656), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(22528), size: Real(1) }, Varnode { space: Unk, offset: Real(22656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(23040), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(22784), size: Real(1) }, Varnode { space: Unk, offset: Real(21760), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(22912), size: Real(1) }, Varnode { space: Unk, offset: Real(22784), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(23168), size: Real(1) }, Varnode { space: Unk, offset: Real(22912), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(23040), size: Real(1) }, Varnode { space: Unk, offset: Real(23168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(22016), size: Real(1) })"
                );
            }
            Self::Variant44(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(46080), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(21760), size: Real(1) }, Varnode { space: Unk, offset: Real(46080), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(22016), size: Real(1) }, Varnode { space: Unk, offset: Real(21760), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(22272), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(22144), size: Real(1) }, Varnode { space: Unk, offset: Real(22016), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(22400), size: Real(1) }, Varnode { space: Unk, offset: Real(22144), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(22272), size: Real(1) }, Varnode { space: Unk, offset: Real(22400), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(22528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(22656), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(22528), size: Real(1) }, Varnode { space: Unk, offset: Real(22656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(23040), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(22784), size: Real(1) }, Varnode { space: Unk, offset: Real(21760), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(22912), size: Real(1) }, Varnode { space: Unk, offset: Real(22784), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(23168), size: Real(1) }, Varnode { space: Unk, offset: Real(22912), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(23040), size: Real(1) }, Varnode { space: Unk, offset: Real(23168), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(46080), size: Real(1) }, Varnode { space: Unk, offset: Real(22016), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant45(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(23296), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(23552), size: Real(1) }, Varnode { space: Unk, offset: Real(23296), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(23808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(23680), size: Real(1) }, Varnode { space: Unk, offset: Real(23552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(23936), size: Real(1) }, Varnode { space: Unk, offset: Real(23680), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(23808), size: Real(1) }, Varnode { space: Unk, offset: Real(23936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(24064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(24192), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(24064), size: Real(1) }, Varnode { space: Unk, offset: Real(24192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(24576), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(24320), size: Real(1) }, Varnode { space: Unk, offset: Real(23296), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(24448), size: Real(1) }, Varnode { space: Unk, offset: Real(24320), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(24704), size: Real(1) }, Varnode { space: Unk, offset: Real(24448), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(24576), size: Real(1) }, Varnode { space: Unk, offset: Real(24704), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(23552), size: Real(1) })"
                );
            }
            Self::Variant46(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(46336), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(23296), size: Real(1) }, Varnode { space: Unk, offset: Real(46336), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(23552), size: Real(1) }, Varnode { space: Unk, offset: Real(23296), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(23808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(23680), size: Real(1) }, Varnode { space: Unk, offset: Real(23552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(23936), size: Real(1) }, Varnode { space: Unk, offset: Real(23680), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(23808), size: Real(1) }, Varnode { space: Unk, offset: Real(23936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(24064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(24192), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(24064), size: Real(1) }, Varnode { space: Unk, offset: Real(24192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(24576), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(24320), size: Real(1) }, Varnode { space: Unk, offset: Real(23296), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(24448), size: Real(1) }, Varnode { space: Unk, offset: Real(24320), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(24704), size: Real(1) }, Varnode { space: Unk, offset: Real(24448), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(24576), size: Real(1) }, Varnode { space: Unk, offset: Real(24704), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(46336), size: Real(1) }, Varnode { space: Unk, offset: Real(23552), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant47() => {
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(46592), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(46720), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(46720), size: Real(1) }, Varnode { space: Unk, offset: Real(46592), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(46976), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(47104), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(46976), size: Real(1) }, Varnode { space: Unk, offset: Real(47104), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(47232), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(47360), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(47232), size: Real(1) }, Varnode { space: Unk, offset: Real(47360), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(47488), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(47616), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(47488), size: Real(1) }, Varnode { space: Unk, offset: Real(47616), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(47744), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(47872), size: Real(1) }, Varnode { space: Unk, offset: Real(46592), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(47744), size: Real(1) }, Varnode { space: Unk, offset: Real(47872), size: Real(1) })"
                );
            }
            Self::Variant48() => {
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(48128), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(48256), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(48384), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(48512), size: Real(1) }, Varnode { space: Unk, offset: Real(48384), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(48256), size: Real(1) }, Varnode { space: Unk, offset: Real(48512), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(48768), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(48896), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(48768), size: Real(1) }, Varnode { space: Unk, offset: Real(48896), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(49024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(49152), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(49024), size: Real(1) }, Varnode { space: Unk, offset: Real(49152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(49280), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(49408), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(49280), size: Real(1) }, Varnode { space: Unk, offset: Real(49408), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(49536), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(49664), size: Real(1) }, Varnode { space: Unk, offset: Real(48128), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(49536), size: Real(1) }, Varnode { space: Unk, offset: Real(49664), size: Real(1) })"
                );
            }
            Self::Variant49() => {
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(49920), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(50048), size: Real(1) }, Varnode { space: Unk, offset: Real(49920), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(50176), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(50048), size: Real(1) }, Varnode { space: Unk, offset: Real(50176), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(50432), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(50560), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(50432), size: Real(1) }, Varnode { space: Unk, offset: Real(50560), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(50688), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(50816), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(50688), size: Real(1) }, Varnode { space: Unk, offset: Real(50816), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(50944), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(51072), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(50944), size: Real(1) }, Varnode { space: Unk, offset: Real(51072), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(51200), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(51328), size: Real(1) }, Varnode { space: Unk, offset: Real(49920), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(51200), size: Real(1) }, Varnode { space: Unk, offset: Real(51328), size: Real(1) })"
                );
            }
            Self::Variant50() => {
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(51584), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(51712), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(51840), size: Real(1) }, Varnode { space: Unk, offset: Real(51712), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(51968), size: Real(1) }, Varnode { space: Unk, offset: Real(51840), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(52096), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(51968), size: Real(1) }, Varnode { space: Unk, offset: Real(52096), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(52352), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(52480), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(52352), size: Real(1) }, Varnode { space: Unk, offset: Real(52480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(52608), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(52736), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(52608), size: Real(1) }, Varnode { space: Unk, offset: Real(52736), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(52864), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(52992), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(52864), size: Real(1) }, Varnode { space: Unk, offset: Real(52992), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(53120), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(53248), size: Real(1) }, Varnode { space: Unk, offset: Real(51584), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(53120), size: Real(1) }, Varnode { space: Unk, offset: Real(53248), size: Real(1) })"
                );
            }
            Self::Variant51(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(24832), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(25088), size: Real(1) }, Varnode { space: Unk, offset: Real(24832), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(25216), size: Real(1) }, Varnode { space: Unk, offset: Real(24832), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(25472), size: Real(1) }, Varnode { space: Unk, offset: Real(25216), size: Real(1) }, Varnode { space: Unk, offset: Real(25088), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(25728), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(25600), size: Real(1) }, Varnode { space: Unk, offset: Real(25472), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(25856), size: Real(1) }, Varnode { space: Unk, offset: Real(25600), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(25728), size: Real(1) }, Varnode { space: Unk, offset: Real(25856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(25984), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(26112), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(25984), size: Real(1) }, Varnode { space: Unk, offset: Real(26112), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(26240), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(26368), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(26240), size: Real(1) }, Varnode { space: Unk, offset: Real(26368), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(26496), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(26624), size: Real(1) }, Varnode { space: Unk, offset: Real(25088), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(26496), size: Real(1) }, Varnode { space: Unk, offset: Real(26624), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(25472), size: Real(1) })"
                );
            }
            Self::Variant52(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(53504), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(24832), size: Real(1) }, Varnode { space: Unk, offset: Real(53504), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(25088), size: Real(1) }, Varnode { space: Unk, offset: Real(24832), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(25216), size: Real(1) }, Varnode { space: Unk, offset: Real(24832), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(25472), size: Real(1) }, Varnode { space: Unk, offset: Real(25216), size: Real(1) }, Varnode { space: Unk, offset: Real(25088), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(25728), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(25600), size: Real(1) }, Varnode { space: Unk, offset: Real(25472), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(25856), size: Real(1) }, Varnode { space: Unk, offset: Real(25600), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(25728), size: Real(1) }, Varnode { space: Unk, offset: Real(25856), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(25984), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(26112), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(25984), size: Real(1) }, Varnode { space: Unk, offset: Real(26112), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(26240), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(26368), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(26240), size: Real(1) }, Varnode { space: Unk, offset: Real(26368), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(26496), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(26624), size: Real(1) }, Varnode { space: Unk, offset: Real(25088), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(26496), size: Real(1) }, Varnode { space: Unk, offset: Real(26624), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(53504), size: Real(1) }, Varnode { space: Unk, offset: Real(25472), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant53(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(26752), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(27008), size: Real(1) }, Varnode { space: Unk, offset: Real(26752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(27136), size: Real(1) }, Varnode { space: Unk, offset: Real(27008), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(27264), size: Real(1) }, Varnode { space: Unk, offset: Real(26752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(27520), size: Real(1) }, Varnode { space: Unk, offset: Real(27136), size: Real(1) }, Varnode { space: Unk, offset: Real(27264), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(27776), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(27648), size: Real(1) }, Varnode { space: Unk, offset: Real(27520), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(27904), size: Real(1) }, Varnode { space: Unk, offset: Real(27648), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(27776), size: Real(1) }, Varnode { space: Unk, offset: Real(27904), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(28032), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(28160), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(28032), size: Real(1) }, Varnode { space: Unk, offset: Real(28160), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(28288), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(28416), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(28288), size: Real(1) }, Varnode { space: Unk, offset: Real(28416), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(28544), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(28672), size: Real(1) }, Varnode { space: Unk, offset: Real(27008), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(28544), size: Real(1) }, Varnode { space: Unk, offset: Real(28672), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(27520), size: Real(1) })"
                );
            }
            Self::Variant54(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(53760), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(26752), size: Real(1) }, Varnode { space: Unk, offset: Real(53760), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(27008), size: Real(1) }, Varnode { space: Unk, offset: Real(26752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(27136), size: Real(1) }, Varnode { space: Unk, offset: Real(27008), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(27264), size: Real(1) }, Varnode { space: Unk, offset: Real(26752), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(27520), size: Real(1) }, Varnode { space: Unk, offset: Real(27136), size: Real(1) }, Varnode { space: Unk, offset: Real(27264), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(27776), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(27648), size: Real(1) }, Varnode { space: Unk, offset: Real(27520), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(27904), size: Real(1) }, Varnode { space: Unk, offset: Real(27648), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(27776), size: Real(1) }, Varnode { space: Unk, offset: Real(27904), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(28032), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(28160), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(28032), size: Real(1) }, Varnode { space: Unk, offset: Real(28160), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(28288), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(28416), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(28288), size: Real(1) }, Varnode { space: Unk, offset: Real(28416), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(28544), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(28672), size: Real(1) }, Varnode { space: Unk, offset: Real(27008), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(28544), size: Real(1) }, Varnode { space: Unk, offset: Real(28672), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(53760), size: Real(1) }, Varnode { space: Unk, offset: Real(27520), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant55(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(28800), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(29056), size: Real(1) }, Varnode { space: Unk, offset: Real(28800), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(29184), size: Real(1) }, Varnode { space: Unk, offset: Real(28800), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(29312), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(29440), size: Real(1) }, Varnode { space: Unk, offset: Real(29312), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(29696), size: Real(1) }, Varnode { space: Unk, offset: Real(29184), size: Real(1) }, Varnode { space: Unk, offset: Real(29440), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(29952), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(29824), size: Real(1) }, Varnode { space: Unk, offset: Real(29696), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30080), size: Real(1) }, Varnode { space: Unk, offset: Real(29824), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(29952), size: Real(1) }, Varnode { space: Unk, offset: Real(30080), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(30208), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30336), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(30208), size: Real(1) }, Varnode { space: Unk, offset: Real(30336), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(30464), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30592), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(30464), size: Real(1) }, Varnode { space: Unk, offset: Real(30592), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(30720), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30848), size: Real(1) }, Varnode { space: Unk, offset: Real(29056), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(30720), size: Real(1) }, Varnode { space: Unk, offset: Real(30848), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(29696), size: Real(1) })"
                );
            }
            Self::Variant56(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(54016), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(28800), size: Real(1) }, Varnode { space: Unk, offset: Real(54016), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(29056), size: Real(1) }, Varnode { space: Unk, offset: Real(28800), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(29184), size: Real(1) }, Varnode { space: Unk, offset: Real(28800), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(29312), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(29440), size: Real(1) }, Varnode { space: Unk, offset: Real(29312), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(29696), size: Real(1) }, Varnode { space: Unk, offset: Real(29184), size: Real(1) }, Varnode { space: Unk, offset: Real(29440), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(29952), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(29824), size: Real(1) }, Varnode { space: Unk, offset: Real(29696), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30080), size: Real(1) }, Varnode { space: Unk, offset: Real(29824), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(29952), size: Real(1) }, Varnode { space: Unk, offset: Real(30080), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(30208), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30336), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(30208), size: Real(1) }, Varnode { space: Unk, offset: Real(30336), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(30464), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30592), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(30464), size: Real(1) }, Varnode { space: Unk, offset: Real(30592), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(30720), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(30848), size: Real(1) }, Varnode { space: Unk, offset: Real(29056), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(30720), size: Real(1) }, Varnode { space: Unk, offset: Real(30848), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(54016), size: Real(1) }, Varnode { space: Unk, offset: Real(29696), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant57(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(30976), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(31232), size: Real(1) }, Varnode { space: Unk, offset: Real(30976), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(31360), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(31488), size: Real(1) }, Varnode { space: Unk, offset: Real(31360), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(31616), size: Real(1) }, Varnode { space: Unk, offset: Real(31488), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(31744), size: Real(1) }, Varnode { space: Unk, offset: Real(30976), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(32000), size: Real(1) }, Varnode { space: Unk, offset: Real(31616), size: Real(1) }, Varnode { space: Unk, offset: Real(31744), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(32256), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(32128), size: Real(1) }, Varnode { space: Unk, offset: Real(32000), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(32384), size: Real(1) }, Varnode { space: Unk, offset: Real(32128), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(32256), size: Real(1) }, Varnode { space: Unk, offset: Real(32384), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(32512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(32640), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(32512), size: Real(1) }, Varnode { space: Unk, offset: Real(32640), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(32768), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(32896), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(32768), size: Real(1) }, Varnode { space: Unk, offset: Real(32896), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(33024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(33152), size: Real(1) }, Varnode { space: Unk, offset: Real(31232), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(33024), size: Real(1) }, Varnode { space: Unk, offset: Real(33152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(32000), size: Real(1) })"
                );
            }
            Self::Variant58(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(54272), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(30976), size: Real(1) }, Varnode { space: Unk, offset: Real(54272), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(31232), size: Real(1) }, Varnode { space: Unk, offset: Real(30976), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(31360), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(31488), size: Real(1) }, Varnode { space: Unk, offset: Real(31360), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(31616), size: Real(1) }, Varnode { space: Unk, offset: Real(31488), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(31744), size: Real(1) }, Varnode { space: Unk, offset: Real(30976), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(32000), size: Real(1) }, Varnode { space: Unk, offset: Real(31616), size: Real(1) }, Varnode { space: Unk, offset: Real(31744), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(32256), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(32128), size: Real(1) }, Varnode { space: Unk, offset: Real(32000), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(32384), size: Real(1) }, Varnode { space: Unk, offset: Real(32128), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(32256), size: Real(1) }, Varnode { space: Unk, offset: Real(32384), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(32512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(32640), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(32512), size: Real(1) }, Varnode { space: Unk, offset: Real(32640), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(32768), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(32896), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(32768), size: Real(1) }, Varnode { space: Unk, offset: Real(32896), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(33024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(33152), size: Real(1) }, Varnode { space: Unk, offset: Real(31232), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(33024), size: Real(1) }, Varnode { space: Unk, offset: Real(33152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(54272), size: Real(1) }, Varnode { space: Unk, offset: Real(32000), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant59(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(33280), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(33536), size: Real(1) }, Varnode { space: Unk, offset: Real(33280), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(33792), size: Real(1) }, Varnode { space: Unk, offset: Real(33280), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34048), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(33920), size: Real(1) }, Varnode { space: Unk, offset: Real(33792), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34176), size: Real(1) }, Varnode { space: Unk, offset: Real(33920), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34048), size: Real(1) }, Varnode { space: Unk, offset: Real(34176), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34304), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34432), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34304), size: Real(1) }, Varnode { space: Unk, offset: Real(34432), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34560), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34688), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34560), size: Real(1) }, Varnode { space: Unk, offset: Real(34688), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34816), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34944), size: Real(1) }, Varnode { space: Unk, offset: Real(33536), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34816), size: Real(1) }, Varnode { space: Unk, offset: Real(34944), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(33792), size: Real(1) })"
                );
            }
            Self::Variant60(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(54528), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(33280), size: Real(1) }, Varnode { space: Unk, offset: Real(54528), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(33536), size: Real(1) }, Varnode { space: Unk, offset: Real(33280), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(33792), size: Real(1) }, Varnode { space: Unk, offset: Real(33280), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34048), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(33920), size: Real(1) }, Varnode { space: Unk, offset: Real(33792), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34176), size: Real(1) }, Varnode { space: Unk, offset: Real(33920), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34048), size: Real(1) }, Varnode { space: Unk, offset: Real(34176), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34304), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34432), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34304), size: Real(1) }, Varnode { space: Unk, offset: Real(34432), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34560), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34688), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34560), size: Real(1) }, Varnode { space: Unk, offset: Real(34688), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(34816), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(34944), size: Real(1) }, Varnode { space: Unk, offset: Real(33536), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(34816), size: Real(1) }, Varnode { space: Unk, offset: Real(34944), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(54528), size: Real(1) }, Varnode { space: Unk, offset: Real(33792), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant61(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(35072), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(35328), size: Real(1) }, Varnode { space: Unk, offset: Real(35072), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSright(Varnode { space: Unk, offset: Real(35584), size: Real(1) }, Varnode { space: Unk, offset: Real(35072), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(35840), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(35712), size: Real(1) }, Varnode { space: Unk, offset: Real(35584), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(35968), size: Real(1) }, Varnode { space: Unk, offset: Real(35712), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(35840), size: Real(1) }, Varnode { space: Unk, offset: Real(35968), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36096), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(36224), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(36096), size: Real(1) }, Varnode { space: Unk, offset: Real(36224), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36352), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(36480), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(36352), size: Real(1) }, Varnode { space: Unk, offset: Real(36480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36608), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(36736), size: Real(1) }, Varnode { space: Unk, offset: Real(35328), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(36608), size: Real(1) }, Varnode { space: Unk, offset: Real(36736), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(35584), size: Real(1) })"
                );
            }
            Self::Variant62(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(54784), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(35072), size: Real(1) }, Varnode { space: Unk, offset: Real(54784), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(35328), size: Real(1) }, Varnode { space: Unk, offset: Real(35072), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntSright(Varnode { space: Unk, offset: Real(35584), size: Real(1) }, Varnode { space: Unk, offset: Real(35072), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(35840), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(35712), size: Real(1) }, Varnode { space: Unk, offset: Real(35584), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(35968), size: Real(1) }, Varnode { space: Unk, offset: Real(35712), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(35840), size: Real(1) }, Varnode { space: Unk, offset: Real(35968), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36096), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(36224), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(36096), size: Real(1) }, Varnode { space: Unk, offset: Real(36224), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36352), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(36480), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(36352), size: Real(1) }, Varnode { space: Unk, offset: Real(36480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36608), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(36736), size: Real(1) }, Varnode { space: Unk, offset: Real(35328), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(36608), size: Real(1) }, Varnode { space: Unk, offset: Real(36736), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(54784), size: Real(1) }, Varnode { space: Unk, offset: Real(35584), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant63(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(36864), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36992), size: Real(1) }, Varnode { space: Unk, offset: Real(36864), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(37120), size: Real(1) }, Varnode { space: Unk, offset: Real(36992), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(37248), size: Real(1) }, Varnode { space: Unk, offset: Real(36864), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(37504), size: Real(1) }, Varnode { space: Unk, offset: Real(37120), size: Real(1) }, Varnode { space: Unk, offset: Real(37248), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(37760), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(37632), size: Real(1) }, Varnode { space: Unk, offset: Real(37504), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(37888), size: Real(1) }, Varnode { space: Unk, offset: Real(37632), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(37760), size: Real(1) }, Varnode { space: Unk, offset: Real(37888), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(38016), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(38144), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(38016), size: Real(1) }, Varnode { space: Unk, offset: Real(38144), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(38272), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(38400), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(38272), size: Real(1) }, Varnode { space: Unk, offset: Real(38400), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(38528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(38656), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(38528), size: Real(1) }, Varnode { space: Unk, offset: Real(38656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(37504), size: Real(1) })"
                );
            }
            Self::Variant64(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(55040), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(36864), size: Real(1) }, Varnode { space: Unk, offset: Real(55040), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(36992), size: Real(1) }, Varnode { space: Unk, offset: Real(36864), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(37120), size: Real(1) }, Varnode { space: Unk, offset: Real(36992), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(37248), size: Real(1) }, Varnode { space: Unk, offset: Real(36864), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(37504), size: Real(1) }, Varnode { space: Unk, offset: Real(37120), size: Real(1) }, Varnode { space: Unk, offset: Real(37248), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(37760), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(37632), size: Real(1) }, Varnode { space: Unk, offset: Real(37504), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(37888), size: Real(1) }, Varnode { space: Unk, offset: Real(37632), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(37760), size: Real(1) }, Varnode { space: Unk, offset: Real(37888), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(38016), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(38144), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(38016), size: Real(1) }, Varnode { space: Unk, offset: Real(38144), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(38272), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(38400), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(38272), size: Real(1) }, Varnode { space: Unk, offset: Real(38400), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(38528), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(38656), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(38528), size: Real(1) }, Varnode { space: Unk, offset: Real(38656), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(55040), size: Real(1) }, Varnode { space: Unk, offset: Real(37504), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant65(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(38784), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(39040), size: Real(1) }, Varnode { space: Unk, offset: Real(38784), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(39296), size: Real(1) }, Varnode { space: Unk, offset: Real(38784), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(39552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(39424), size: Real(1) }, Varnode { space: Unk, offset: Real(39296), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(39680), size: Real(1) }, Varnode { space: Unk, offset: Real(39424), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(39552), size: Real(1) }, Varnode { space: Unk, offset: Real(39680), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(39808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(39936), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(39808), size: Real(1) }, Varnode { space: Unk, offset: Real(39936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(40064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(40192), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(40064), size: Real(1) }, Varnode { space: Unk, offset: Real(40192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(40320), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(40448), size: Real(1) }, Varnode { space: Unk, offset: Real(39040), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(40320), size: Real(1) }, Varnode { space: Unk, offset: Real(40448), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(39296), size: Real(1) })"
                );
            }
            Self::Variant66(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(55296), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(38784), size: Real(1) }, Varnode { space: Unk, offset: Real(55296), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(39040), size: Real(1) }, Varnode { space: Unk, offset: Real(38784), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(39296), size: Real(1) }, Varnode { space: Unk, offset: Real(38784), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(39552), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(39424), size: Real(1) }, Varnode { space: Unk, offset: Real(39296), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(39680), size: Real(1) }, Varnode { space: Unk, offset: Real(39424), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(39552), size: Real(1) }, Varnode { space: Unk, offset: Real(39680), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(39808), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(39936), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(39808), size: Real(1) }, Varnode { space: Unk, offset: Real(39936), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(40064), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(40192), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(40064), size: Real(1) }, Varnode { space: Unk, offset: Real(40192), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(40320), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(40448), size: Real(1) }, Varnode { space: Unk, offset: Real(39040), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(40320), size: Real(1) }, Varnode { space: Unk, offset: Real(40448), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(55296), size: Real(1) }, Varnode { space: Unk, offset: Real(39296), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant67(op0, op1) => {
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(40832), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41088), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(40832), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41344), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(41216), size: Real(1) }, Varnode { space: Unk, offset: Real(41088), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(41472), size: Real(1) }, Varnode { space: Unk, offset: Real(41216), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(41344), size: Real(1) }, Varnode { space: Unk, offset: Real(41472), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41600), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(41728), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(41600), size: Real(1) }, Varnode { space: Unk, offset: Real(41728), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41856), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(41984), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(41856), size: Real(1) }, Varnode { space: Unk, offset: Real(41984), size: Real(1) })"
                );
            }
            Self::Variant68(op0, op1) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(55552), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(40832), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41088), size: Real(1) }, Varnode { space: Unk, offset: Real(55552), size: Real(1) }, Varnode { space: Unk, offset: Real(40832), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41344), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(41216), size: Real(1) }, Varnode { space: Unk, offset: Real(41088), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(41472), size: Real(1) }, Varnode { space: Unk, offset: Real(41216), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(41344), size: Real(1) }, Varnode { space: Unk, offset: Real(41472), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41600), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(41728), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(41600), size: Real(1) }, Varnode { space: Unk, offset: Real(41728), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(41856), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(41984), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(41856), size: Real(1) }, Varnode { space: Unk, offset: Real(41984), size: Real(1) })"
                );
            }
            Self::Variant69(op0, op1) => {
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(42240), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntNegate(Varnode { space: Unk, offset: Real(42496), size: Real(1) }, Varnode { space: Unk, offset: Real(42240), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(42496), size: Real(1) })"
                );
            }
            Self::Variant70(op0, op1) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(55808), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(42240), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntNegate(Varnode { space: Unk, offset: Real(42496), size: Real(1) }, Varnode { space: Unk, offset: Real(42240), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(55808), size: Real(1) }, Varnode { space: Unk, offset: Real(55808), size: Real(1) }, Varnode { space: Unk, offset: Real(42496), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant71(op0, op1) => {
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(43136), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(43136), size: Real(1) })"
                );
            }
            Self::Variant72(op0, op1) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(56064), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(43136), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(56064), size: Real(1) }, Varnode { space: Unk, offset: Real(56064), size: Real(1) }, Varnode { space: Unk, offset: Real(43136), size: Real(1) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant73(op0) => {
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant74(op0) => {
                println!("\t{}", "Unk");
            }
            Self::Variant75(op0) => {
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant76(op0) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant77() => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(8), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(10), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant78() => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(8), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(10), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant79(op0, op1) => {
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant80(op0, op1) => {
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant81(op0, op1) => {
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "BoolNegate(Varnode { space: Unk, offset: Real(56320), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant82(op0) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "BoolNegate(Varnode { space: Unk, offset: Real(56448), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Real(8), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(10), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant83(op0) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
                println!("\t{}", "Unk");
            }
            Self::Variant84() => {
                println!("\t{}", "Unk");
            }
            Self::Variant85() => {
                println!("\t{}", "Unk");
            }
            Self::Variant86() => {
                println!("\t{}", "Unk");
            }
            Self::Variant87() => {
                println!("\t{}", "Unk");
            }
            Self::Variant88() => {
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(56832), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(56960), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(56832), size: Real(1) }, Varnode { space: Unk, offset: Real(56960), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(57088), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(57216), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(57088), size: Real(1) }, Varnode { space: Unk, offset: Real(57216), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(57728), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(57344), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(57472), size: Real(1) }, Varnode { space: Unk, offset: Real(57344), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "BoolNegate(Varnode { space: Unk, offset: Real(57600), size: Real(1) }, Varnode { space: Unk, offset: Real(57472), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(57856), size: Real(1) }, Varnode { space: Unk, offset: Real(57600), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(57728), size: Real(1) }, Varnode { space: Unk, offset: Real(57856), size: Real(1) })"
                );
            }
            Self::Variant89() => {
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(57984), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(58112), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(57984), size: Real(1) }, Varnode { space: Unk, offset: Real(58112), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(58240), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(58368), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(58240), size: Real(1) }, Varnode { space: Unk, offset: Real(58368), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(58496), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(58624), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(58496), size: Real(1) }, Varnode { space: Unk, offset: Real(58624), size: Real(1) })"
                );
            }
            Self::Variant90() => {}
            Self::Variant91() => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(58752), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(58880), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(59008), size: Real(1) }, Varnode { space: Unk, offset: Real(58880), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(59136), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(59264), size: Real(1) }, Varnode { space: Unk, offset: Real(59136), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(59392), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(59520), size: Real(1) }, Varnode { space: Unk, offset: Real(59392), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(59904), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(58752), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(60160), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(58752), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(60416), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntEqual(Varnode { space: Unk, offset: Real(60288), size: Real(1) }, Varnode { space: Unk, offset: Real(59904), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(60544), size: Real(1) }, Varnode { space: Unk, offset: Real(60288), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(60416), size: Real(1) }, Varnode { space: Unk, offset: Real(60544), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(60672), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(60800), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(60672), size: Real(1) }, Varnode { space: Unk, offset: Real(60800), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(61824), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(60928), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(61056), size: Real(1) }, Varnode { space: Unk, offset: Real(60928), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(61184), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(61312), size: Real(1) }, Varnode { space: Unk, offset: Real(61184), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "BoolNegate(Varnode { space: Unk, offset: Real(61440), size: Real(1) }, Varnode { space: Unk, offset: Real(61312), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "BoolAnd(Varnode { space: Unk, offset: Real(61568), size: Real(1) }, Varnode { space: Unk, offset: Real(61440), size: Real(1) }, Varnode { space: Unk, offset: Real(60160), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "BoolOr(Varnode { space: Unk, offset: Real(61696), size: Real(1) }, Varnode { space: Unk, offset: Real(61056), size: Real(1) }, Varnode { space: Unk, offset: Real(61568), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(61952), size: Real(1) }, Varnode { space: Unk, offset: Real(61696), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(61824), size: Real(1) }, Varnode { space: Unk, offset: Real(61952), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(59904), size: Real(1) })"
                );
            }
            Self::Variant92() => {
                println!(
                    "\t{}",
                    "IntNegate(Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(62208), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(62336), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(62208), size: Real(1) }, Varnode { space: Unk, offset: Real(62336), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(62464), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(62592), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(62464), size: Real(1) }, Varnode { space: Unk, offset: Real(62592), size: Real(1) })"
                );
            }
            Self::Variant93(op0, op1) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
            }
            Self::Variant94(op0, op1) => {
                println!("\t{}", "Unk");
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(10), size: Real(2) })"
                );
            }
            Self::Variant95(op0, op1) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
            }
            Self::Variant96(op0, op1, op2) => {
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(62848), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(62976), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(63104), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(62976), size: Real(1) }, Varnode { space: Unk, offset: Real(63104), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(63232), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(63360), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(63232), size: Real(1) }, Varnode { space: Unk, offset: Real(63360), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(64128), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(63488), size: Real(1) }, Varnode { space: Unk, offset: Real(10), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(63616), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(63744), size: Real(1) }, Varnode { space: Unk, offset: Real(63488), size: Real(1) }, Varnode { space: Unk, offset: Real(63616), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(63872), size: Real(1) }, Varnode { space: Unk, offset: Real(63744), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(64000), size: Real(1) }, Varnode { space: Unk, offset: Real(63872), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(64256), size: Real(1) }, Varnode { space: Unk, offset: Real(64000), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(64128), size: Real(1) }, Varnode { space: Unk, offset: Real(64256), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(64512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(64384), size: Real(1) }, Varnode { space: Unk, offset: Real(10), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(64640), size: Real(1) }, Varnode { space: Unk, offset: Real(64384), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(64512), size: Real(1) }, Varnode { space: Unk, offset: Real(64640), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(62848), size: Real(2) })"
                );
            }
            Self::Variant97(op0, op1, op2, op3) => {
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(64896), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(65024), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(65152), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(65024), size: Real(1) }, Varnode { space: Unk, offset: Real(65152), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(65280), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(65408), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(65280), size: Real(1) }, Varnode { space: Unk, offset: Real(65408), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(66176), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(65536), size: Real(1) }, Varnode { space: Unk, offset: Real(10), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(65664), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(65792), size: Real(1) }, Varnode { space: Unk, offset: Real(65536), size: Real(1) }, Varnode { space: Unk, offset: Real(65664), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(65920), size: Real(1) }, Varnode { space: Unk, offset: Real(65792), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(66048), size: Real(1) }, Varnode { space: Unk, offset: Real(65920), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(66304), size: Real(1) }, Varnode { space: Unk, offset: Real(66048), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(66176), size: Real(1) }, Varnode { space: Unk, offset: Real(66304), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(66560), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(66432), size: Real(1) }, Varnode { space: Unk, offset: Real(10), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(66688), size: Real(1) }, Varnode { space: Unk, offset: Real(66432), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(66560), size: Real(1) }, Varnode { space: Unk, offset: Real(66688), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(64896), size: Real(2) })"
                );
            }
            Self::Variant98(op0) => {
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
                println!("\t{}", "Unk");
            }
            Self::Variant99(op0) => {
                println!(
                    "\t{}",
                    "Load(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Real(8) }, Varnode { space: Unk, offset: Real(10), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(2) })"
                );
            }
            Self::Variant100(op0, op1) => {
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(66944), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(67072), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(67200), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(67072), size: Real(1) }, Varnode { space: Unk, offset: Real(67200), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(67968), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(67328), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(4095), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(67456), size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(4095), size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(67584), size: Real(2) }, Varnode { space: Unk, offset: Real(67328), size: Real(2) }, Varnode { space: Unk, offset: Real(67456), size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(67712), size: Real(2) }, Varnode { space: Unk, offset: Real(67584), size: Real(2) }, Varnode { space: Unk, offset: Real(4096), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(67840), size: Real(1) }, Varnode { space: Unk, offset: Real(67712), size: Real(2) }, Varnode { space: Unk, offset: Real(0), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(68096), size: Real(1) }, Varnode { space: Unk, offset: Real(67840), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(67968), size: Real(1) }, Varnode { space: Unk, offset: Real(68096), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(68352), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(68224), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Unk })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(68480), size: Real(1) }, Varnode { space: Unk, offset: Real(68224), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(68352), size: Real(1) }, Varnode { space: Unk, offset: Real(68480), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(66944), size: Real(2) })"
                );
            }
            Self::Variant101(op0, op1) => {
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(68736), size: Real(2) }, Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(68864), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551487), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(68992), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(68864), size: Real(1) }, Varnode { space: Unk, offset: Real(68992), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(69120), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551551), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(69248), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(6), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(69120), size: Real(1) }, Varnode { space: Unk, offset: Real(69248), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(70016), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551583), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(69376), size: Real(1) }, Varnode { space: Unk, offset: Real(10), size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(69504), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) }, Varnode { space: Unk, offset: Real(15), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(69632), size: Real(1) }, Varnode { space: Unk, offset: Real(69376), size: Real(1) }, Varnode { space: Unk, offset: Real(69504), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(69760), size: Real(1) }, Varnode { space: Unk, offset: Real(69632), size: Real(1) }, Varnode { space: Unk, offset: Real(16), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntNotEqual(Varnode { space: Unk, offset: Real(69888), size: Real(1) }, Varnode { space: Unk, offset: Real(69760), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(70144), size: Real(1) }, Varnode { space: Unk, offset: Real(69888), size: Real(1) }, Varnode { space: Unk, offset: Real(5), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(70016), size: Real(1) }, Varnode { space: Unk, offset: Real(70144), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(70400), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(18446744073709551599), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntCarry(Varnode { space: Unk, offset: Real(70272), size: Real(1) }, Varnode { space: Unk, offset: Real(10), size: Real(1) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntLeft(Varnode { space: Unk, offset: Real(70528), size: Real(1) }, Varnode { space: Unk, offset: Real(70272), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(70400), size: Real(1) }, Varnode { space: Unk, offset: Real(70528), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(10), size: Real(2) }, Varnode { space: Unk, offset: Real(68736), size: Real(2) })"
                );
            }
            Self::Variant102(op0) => {
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Unk })"
                );
            }
            Self::Variant103(op0) => {
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Unk, size: Unk }, Varnode { space: Unk, offset: Real(1), size: Unk })"
                );
            }
        }
    }
}
impl std::fmt::Display for Sym0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant1(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant2(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op1.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant3(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op1.write(f)?;
                write!(f, "),")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant4(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, "),")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant5(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op1.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant6(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op1.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant7(op0, op1) => {
                write!(f, "LDH")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant8(op0, op1) => {
                write!(f, "LDH")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant9(op0, op1) => {
                write!(f, "LDH")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant10(op0, op1) => {
                write!(f, "LDH")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant11(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant12(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant13(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant14(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant15(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op1.write(f)?;
                write!(f, "),")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant16(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op1.write(f)?;
                write!(f, "),")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant17(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant18(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant19(op0) => {
                write!(f, "ADD")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant20(op0) => {
                write!(f, "ADD")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant21(op0) => {
                write!(f, "ADD")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant22(op0) => {
                write!(f, "ADC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant23(op0) => {
                write!(f, "ADC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant24(op0) => {
                write!(f, "ADC")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant25(op0) => {
                write!(f, "SUB")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant26(op0) => {
                write!(f, "SUB")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant27(op0) => {
                write!(f, "SUB")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant28(op0) => {
                write!(f, "SBC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant29(op0) => {
                write!(f, "SBC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant30(op0) => {
                write!(f, "SBC")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant31(op0) => {
                write!(f, "AND")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant32(op0) => {
                write!(f, "AND")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant33(op0) => {
                write!(f, "AND")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant34(op0) => {
                write!(f, "XOR")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant35(op0) => {
                write!(f, "XOR")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant36(op0) => {
                write!(f, "XOR")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant37(op0) => {
                write!(f, "OR")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant38(op0) => {
                write!(f, "OR")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant39(op0) => {
                write!(f, "OR")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant40(op0) => {
                write!(f, "CP")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant41(op0) => {
                write!(f, "CP")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant42(op0) => {
                write!(f, "CP")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant43(op0) => {
                write!(f, "INC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant44(op0) => {
                write!(f, "INC")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant45(op0) => {
                write!(f, "DEC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant46(op0) => {
                write!(f, "DEC")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant47() => {
                write!(f, "RLCA")?;
                Ok(())
            }
            Self::Variant48() => {
                write!(f, "RLA")?;
                Ok(())
            }
            Self::Variant49() => {
                write!(f, "RRCA")?;
                Ok(())
            }
            Self::Variant50() => {
                write!(f, "RRA")?;
                Ok(())
            }
            Self::Variant51(op0) => {
                write!(f, "RLC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant52(op0) => {
                write!(f, "RLC")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant53(op0) => {
                write!(f, "RRC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant54(op0) => {
                write!(f, "RRC")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant55(op0) => {
                write!(f, "RL")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant56(op0) => {
                write!(f, "RL")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant57(op0) => {
                write!(f, "RR")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant58(op0) => {
                write!(f, "RR")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant59(op0) => {
                write!(f, "SLA")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant60(op0) => {
                write!(f, "SLA")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant61(op0) => {
                write!(f, "SRA")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant62(op0) => {
                write!(f, "SRA")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant63(op0) => {
                write!(f, "SWAP")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant64(op0) => {
                write!(f, "SWAP")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant65(op0) => {
                write!(f, "SRL")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant66(op0) => {
                write!(f, "SRL")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant67(op0, op1) => {
                write!(f, "BIT")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant68(op0, op1) => {
                write!(f, "BIT")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant69(op0, op1) => {
                write!(f, "RES")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant70(op0, op1) => {
                write!(f, "RES")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant71(op0, op1) => {
                write!(f, "SET")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant72(op0, op1) => {
                write!(f, "SET")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
            Self::Variant73(op0) => {
                write!(f, "JP")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant74(op0) => {
                write!(f, "JP")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant75(op0) => {
                write!(f, "JR")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant76(op0) => {
                write!(f, "CALL")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant77() => {
                write!(f, "RET")?;
                Ok(())
            }
            Self::Variant78() => {
                write!(f, "RETI")?;
                Ok(())
            }
            Self::Variant79(op0, op1) => {
                write!(f, "JP")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant80(op0, op1) => {
                write!(f, "JR")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant81(op0, op1) => {
                write!(f, "CALL")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant82(op0) => {
                write!(f, "RET")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant83(op0) => {
                write!(f, "RST")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant84() => {
                write!(f, "HALT")?;
                Ok(())
            }
            Self::Variant85() => {
                write!(f, "STOP")?;
                Ok(())
            }
            Self::Variant86() => {
                write!(f, "DI")?;
                Ok(())
            }
            Self::Variant87() => {
                write!(f, "EI")?;
                Ok(())
            }
            Self::Variant88() => {
                write!(f, "CCF")?;
                Ok(())
            }
            Self::Variant89() => {
                write!(f, "SCF")?;
                Ok(())
            }
            Self::Variant90() => {
                write!(f, "NOP")?;
                Ok(())
            }
            Self::Variant91() => {
                write!(f, "DAA")?;
                Ok(())
            }
            Self::Variant92() => {
                write!(f, "CPL")?;
                Ok(())
            }
            Self::Variant93(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant94(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant95(op0, op1) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant96(op0, op1, op2) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, "+")?;
                op2.write(f)?;
                Ok(())
            }
            Self::Variant97(op0, op1, op2, op3) => {
                write!(f, "LD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, "-")?;
                op3.write(f, op2.0 as u64)?;
                Ok(())
            }
            Self::Variant98(op0) => {
                write!(f, "PUSH")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant99(op0) => {
                write!(f, "POP")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant100(op0, op1) => {
                write!(f, "ADD")?;
                write!(f, " ")?;
                op1.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant101(op0, op1) => {
                write!(f, "ADD")?;
                write!(f, " ")?;
                op0.write(f)?;
                write!(f, ",")?;
                write!(f, " ")?;
                op1.write(f)?;
                Ok(())
            }
            Self::Variant102(op0) => {
                write!(f, "INC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
            Self::Variant103(op0) => {
                write!(f, "DEC")?;
                write!(f, " ")?;
                op0.write(f)?;
                Ok(())
            }
        }
    }
}

struct Sym4();
impl Sym4 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "F")
    }
}

struct Sym5();
impl Sym5 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "A")
    }
}

struct Sym6();
impl Sym6 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "C")
    }
}

struct Sym7();
impl Sym7 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "B")
    }
}

struct Sym8();
impl Sym8 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "E")
    }
}

struct Sym9();
impl Sym9 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "D")
    }
}

struct Sym10();
impl Sym10 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "L")
    }
}

struct Sym11();
impl Sym11 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "H")
    }
}

struct Sym12();
impl Sym12 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AF")
    }
}

struct Sym13();
impl Sym13 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BC")
    }
}

struct Sym14();
impl Sym14 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DE")
    }
}

struct Sym15();
impl Sym15 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HL")
    }
}

struct Sym16();
impl Sym16 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PC")
    }
}

struct Sym17();
impl Sym17 {
    fn decode(_: &[u8]) -> Option<Self> {
        Some(Self())
    }
}
impl std::fmt::Display for Sym17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SP")
    }
}

enum Sym21 {
    Variant0(Sym7),
    Variant1(Sym6),
    Variant2(Sym9),
    Variant3(Sym8),
    Variant4(Sym11),
    Variant5(Sym10),
    Variant7(Sym5),
}
impl Sym21 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 0u8 & 7u8) {
            0u8 => Some(Self::Variant0(Sym7::decode(buf)?)),
            1u8 => Some(Self::Variant1(Sym6::decode(buf)?)),
            2u8 => Some(Self::Variant2(Sym9::decode(buf)?)),
            3u8 => Some(Self::Variant3(Sym8::decode(buf)?)),
            4u8 => Some(Self::Variant4(Sym11::decode(buf)?)),
            5u8 => Some(Self::Variant5(Sym10::decode(buf)?)),
            7u8 => Some(Self::Variant7(Sym5::decode(buf)?)),
            _ => None,
        }
    }
}
impl std::fmt::Display for Sym21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
            Self::Variant2(x) => x.fmt(f),
            Self::Variant3(x) => x.fmt(f),
            Self::Variant4(x) => x.fmt(f),
            Self::Variant5(x) => x.fmt(f),
            Self::Variant7(x) => x.fmt(f),
        }
    }
}

enum Sym24 {
    Variant0(Sym7),
    Variant1(Sym6),
    Variant2(Sym9),
    Variant3(Sym8),
    Variant4(Sym11),
    Variant5(Sym10),
    Variant7(Sym5),
}
impl Sym24 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 3u8 & 7u8) {
            0u8 => Some(Self::Variant0(Sym7::decode(buf)?)),
            1u8 => Some(Self::Variant1(Sym6::decode(buf)?)),
            2u8 => Some(Self::Variant2(Sym9::decode(buf)?)),
            3u8 => Some(Self::Variant3(Sym8::decode(buf)?)),
            4u8 => Some(Self::Variant4(Sym11::decode(buf)?)),
            5u8 => Some(Self::Variant5(Sym10::decode(buf)?)),
            7u8 => Some(Self::Variant7(Sym5::decode(buf)?)),
            _ => None,
        }
    }
}
impl std::fmt::Display for Sym24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
            Self::Variant2(x) => x.fmt(f),
            Self::Variant3(x) => x.fmt(f),
            Self::Variant4(x) => x.fmt(f),
            Self::Variant5(x) => x.fmt(f),
            Self::Variant7(x) => x.fmt(f),
        }
    }
}

enum Sym26 {
    Variant0(Sym13),
    Variant1(Sym14),
    Variant2(Sym15),
    Variant3(Sym12),
}
impl Sym26 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 4u8 & 3u8) {
            0u8 => Some(Self::Variant0(Sym13::decode(buf)?)),
            1u8 => Some(Self::Variant1(Sym14::decode(buf)?)),
            2u8 => Some(Self::Variant2(Sym15::decode(buf)?)),
            3u8 => Some(Self::Variant3(Sym12::decode(buf)?)),
            _ => None,
        }
    }
}
impl std::fmt::Display for Sym26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
            Self::Variant2(x) => x.fmt(f),
            Self::Variant3(x) => x.fmt(f),
        }
    }
}

enum Sym27 {
    Variant0(Sym13),
    Variant1(Sym14),
    Variant2(Sym15),
    Variant3(Sym17),
}
impl Sym27 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 4u8 & 3u8) {
            0u8 => Some(Self::Variant0(Sym13::decode(buf)?)),
            1u8 => Some(Self::Variant1(Sym14::decode(buf)?)),
            2u8 => Some(Self::Variant2(Sym15::decode(buf)?)),
            3u8 => Some(Self::Variant3(Sym17::decode(buf)?)),
            _ => None,
        }
    }
}
impl std::fmt::Display for Sym27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
            Self::Variant2(x) => x.fmt(f),
            Self::Variant3(x) => x.fmt(f),
        }
    }
}

enum Sym28 {
    Variant0(Sym13),
    Variant1(Sym14),
    Variant2(Sym15),
    Variant3(Sym17),
}
impl Sym28 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 4u8 & 3u8) {
            0u8 => Some(Self::Variant0(Sym13::decode(buf)?)),
            1u8 => Some(Self::Variant1(Sym14::decode(buf)?)),
            2u8 => Some(Self::Variant2(Sym15::decode(buf)?)),
            3u8 => Some(Self::Variant3(Sym17::decode(buf)?)),
            _ => None,
        }
    }
}
impl std::fmt::Display for Sym28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
            Self::Variant2(x) => x.fmt(f),
            Self::Variant3(x) => x.fmt(f),
        }
    }
}

enum Sym35 {
    Variant0(),
    Variant1(),
    Variant2(),
    Variant3(),
}
impl Sym35 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 3u8) & 3u8 {
            0u8 => {
                if (buf[0usize] & 24u8 == 0u8) { Some(Self::Variant0()) } else { None }
            }
            1u8 => {
                if (buf[0usize] & 24u8 == 8u8) { Some(Self::Variant1()) } else { None }
            }
            2u8 => {
                if (buf[0usize] & 24u8 == 16u8) { Some(Self::Variant2()) } else { None }
            }
            3u8 => {
                if (buf[0usize] & 24u8 == 24u8) { Some(Self::Variant3()) } else { None }
            }
            _ => unreachable!(),
        }
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0() => {
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "BoolNegate(Varnode { space: Unk, offset: Real(256), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) })"
                );
            }
            Self::Variant1() => {
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(512), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(7), size: Real(4) })"
                );
            }
            Self::Variant2() => {
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(640), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(768), size: Real(1) }, Varnode { space: Unk, offset: Real(640), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "BoolNegate(Varnode { space: Unk, offset: Real(1024), size: Real(1) }, Varnode { space: Unk, offset: Real(768), size: Real(1) })"
                );
            }
            Self::Variant3() => {
                println!(
                    "\t{}",
                    "IntRight(Varnode { space: Unk, offset: Real(1152), size: Real(1) }, Varnode { space: Unk, offset: Real(0), size: Real(1) }, Varnode { space: Unk, offset: Real(4), size: Real(4) })"
                );
                println!(
                    "\t{}",
                    "IntAnd(Varnode { space: Unk, offset: Real(1408), size: Real(1) }, Varnode { space: Unk, offset: Real(1152), size: Real(1) }, Varnode { space: Unk, offset: Real(1), size: Real(1) })"
                );
            }
        }
    }
}
impl std::fmt::Display for Sym35 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0() => {
                write!(f, "NZ")?;
                Ok(())
            }
            Self::Variant1() => {
                write!(f, "Z")?;
                Ok(())
            }
            Self::Variant2() => {
                write!(f, "NC")?;
                Ok(())
            }
            Self::Variant3() => {
                write!(f, "C")?;
                Ok(())
            }
        }
    }
}

enum Sym36 {
    Variant0(Op37),
}
impl Sym36 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op37::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {}
        }
    }
}
impl std::fmt::Display for Sym36 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                op0.write(f)?;
                Ok(())
            }
        }
    }
}

struct Op37(u8);
impl Op37 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

enum Sym38 {
    Variant0(Op39),
}
impl Sym38 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op39::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {}
        }
    }
}
impl std::fmt::Display for Sym38 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

struct Op39(u8);
impl Op39 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

enum Sym40 {
    Variant0(Op41),
}
impl Sym40 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op41::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {}
        }
    }
}
impl std::fmt::Display for Sym40 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

struct Op41(u8);
impl Op41 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

enum Sym42 {
    Variant0(Op44, Op43),
}
impl Sym42 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op44::decode(buf)?, Op43::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0, op1) => {}
        }
    }
}
impl std::fmt::Display for Sym42 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0, op1) => {
                op1.write(f, op0.0 as u64)?;
                Ok(())
            }
        }
    }
}

struct Op43();
impl Op43 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self())
    }
    fn write(&self, f: &mut std::fmt::Formatter, arg0: u64) -> std::fmt::Result {
        write!(f, "{}", (0 + arg0))
    }
}

struct Op44(u8);
impl Op44 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

enum Sym45 {
    Variant0(Op47, Op46),
}
impl Sym45 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op47::decode(buf)?, Op46::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0, op1) => {}
        }
    }
}
impl std::fmt::Display for Sym45 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0, op1) => {
                op1.write(f, op0.0 as u64)?;
                Ok(())
            }
        }
    }
}

struct Op46();
impl Op46 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self())
    }
    fn write(&self, f: &mut std::fmt::Formatter, arg0: u64) -> std::fmt::Result {
        write!(f, "{}", (arg0 << 3u64))
    }
}

struct Op47(u8);
impl Op47 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

enum Sym48 {
    Variant0(Op49),
}
impl Sym48 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op49::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(2176), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntAdd(Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(1), size: Real(2) })"
                );
            }
        }
    }
}
impl std::fmt::Display for Sym48 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, "+)")?;
                Ok(())
            }
        }
    }
}

struct Op49(Sym15);
impl Op49 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum Sym50 {
    Variant0(Op51),
}
impl Sym50 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op51::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {
                println!(
                    "\t{}",
                    "Copy(Varnode { space: Unk, offset: Real(2560), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) })"
                );
                println!(
                    "\t{}",
                    "IntSub(Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(6), size: Real(2) }, Varnode { space: Unk, offset: Real(1), size: Real(2) })"
                );
            }
        }
    }
}
impl std::fmt::Display for Sym50 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, "-)")?;
                Ok(())
            }
        }
    }
}

struct Op51(Sym15);
impl Op51 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum Sym52 {
    Variant0(Op53),
}
impl Sym52 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op53::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {
                println!(
                    "\t{}",
                    "IntZext(Varnode { space: Unk, offset: Real(2944), size: Real(2) }, Varnode { space: Unk, offset: Real(2), size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(3200), size: Real(2) }, Varnode { space: Unk, offset: Real(65280), size: Real(2) }, Varnode { space: Unk, offset: Real(2944), size: Real(2) })"
                );
            }
        }
    }
}
impl std::fmt::Display for Sym52 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

struct Op53(Sym6);
impl Op53 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym6::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum Sym54 {
    Variant0(Op55),
}
impl Sym54 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op55::decode(buf)?))
    }
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        match self {
            Self::Variant0(op0) => {
                println!(
                    "\t{}",
                    "IntZext(Varnode { space: Unk, offset: Real(3584), size: Real(2) }, Varnode { space: Unk, offset: Unk, size: Real(1) })"
                );
                println!(
                    "\t{}",
                    "IntOr(Varnode { space: Unk, offset: Real(3840), size: Real(2) }, Varnode { space: Unk, offset: Real(65280), size: Real(2) }, Varnode { space: Unk, offset: Real(3584), size: Real(2) })"
                );
            }
        }
    }
}
impl std::fmt::Display for Sym54 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                write!(f, "(")?;
                op0.write(f)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

struct Op55(u8);
impl Op55 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op60(Sym24);
impl Op60 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op61(Sym21);
impl Op61 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op62(Sym24);
impl Op62 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op63(u8);
impl Op63 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op64(Sym24);
impl Op64 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op65(Sym15);
impl Op65 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op66(Sym15);
impl Op66 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op67(Sym21);
impl Op67 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op68(Sym15);
impl Op68 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op69(u8);
impl Op69 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op70(Sym5);
impl Op70 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op71(Sym13);
impl Op71 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym13::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op72(Sym5);
impl Op72 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op73(Sym14);
impl Op73 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym14::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op74(Sym5);
impl Op74 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op75(Sym52);
impl Op75 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym52::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op76(Sym52);
impl Op76 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym52::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op77(Sym5);
impl Op77 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op78(Sym5);
impl Op78 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op79(Sym54);
impl Op79 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym54::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op80(Sym54);
impl Op80 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym54::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op81(Sym5);
impl Op81 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op82(Sym5);
impl Op82 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op83(Sym38);
impl Op83 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym38::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op84(Sym38);
impl Op84 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym38::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op85(Sym5);
impl Op85 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op86(Sym5);
impl Op86 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op87(Sym48);
impl Op87 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym48::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op88(Sym5);
impl Op88 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op89(Sym50);
impl Op89 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym50::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op90(Sym13);
impl Op90 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym13::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op91(Sym5);
impl Op91 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op92(Sym14);
impl Op92 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym14::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op93(Sym5);
impl Op93 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op94(Sym48);
impl Op94 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym48::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op95(Sym5);
impl Op95 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op96(Sym50);
impl Op96 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym50::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op97(Sym5);
impl Op97 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op98(Sym21);
impl Op98 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op99(u8);
impl Op99 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op100(Sym15);
impl Op100 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op101(Sym21);
impl Op101 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op102(u8);
impl Op102 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op103(Sym15);
impl Op103 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op104(Sym21);
impl Op104 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op105(u8);
impl Op105 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op106(Sym15);
impl Op106 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op107(Sym21);
impl Op107 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op108(u8);
impl Op108 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op109(Sym15);
impl Op109 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op110(Sym21);
impl Op110 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op111(u8);
impl Op111 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op112(Sym15);
impl Op112 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op113(Sym21);
impl Op113 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op114(u8);
impl Op114 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op115(Sym15);
impl Op115 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op116(Sym21);
impl Op116 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op117(u8);
impl Op117 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op118(Sym15);
impl Op118 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op119(Sym21);
impl Op119 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op120(u8);
impl Op120 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op121(Sym15);
impl Op121 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op122(Sym24);
impl Op122 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op123(Sym15);
impl Op123 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op124(Sym24);
impl Op124 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op125(Sym15);
impl Op125 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op126(Sym21);
impl Op126 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op127(Sym15);
impl Op127 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op128(Sym21);
impl Op128 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op129(Sym15);
impl Op129 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op130(Sym21);
impl Op130 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op131(Sym15);
impl Op131 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op132(Sym21);
impl Op132 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op133(Sym15);
impl Op133 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op134(Sym21);
impl Op134 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op135(Sym15);
impl Op135 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op136(Sym21);
impl Op136 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op137(Sym15);
impl Op137 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op138(Sym21);
impl Op138 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op139(Sym15);
impl Op139 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op140(Sym21);
impl Op140 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op141(Sym15);
impl Op141 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op142(u8);
impl Op142 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op143(Sym21);
impl Op143 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op144(u8);
impl Op144 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op145(Sym15);
impl Op145 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op146(u8);
impl Op146 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op147(Sym21);
impl Op147 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op148(u8);
impl Op148 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op149(Sym15);
impl Op149 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op150(u8);
impl Op150 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op151(Sym21);
impl Op151 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op152(u8);
impl Op152 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op153(Sym15);
impl Op153 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op154(Sym36);
impl Op154 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op155(Sym15);
impl Op155 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op156(Sym42);
impl Op156 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym42::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op157(Sym36);
impl Op157 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op158(Sym35);
impl Op158 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op159(Sym36);
impl Op159 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op160(Sym35);
impl Op160 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op161(Sym42);
impl Op161 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym42::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op162(Sym35);
impl Op162 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op163(Sym36);
impl Op163 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op164(Sym35);
impl Op164 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op165(Sym45);
impl Op165 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym45::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op166(Sym27);
impl Op166 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym27::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op167(u8);
impl Op167 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op168(Sym40);
impl Op168 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym40::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op169(Sym17);
impl Op169 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op170(Sym17);
impl Op170 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op171(Sym15);
impl Op171 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op172(Sym15);
impl Op172 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op173(Sym17);
impl Op173 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op174(u8);
impl Op174 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op175(Sym15);
impl Op175 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op176(Sym17);
impl Op176 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op177();
impl Op177 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self())
    }
    fn write(&self, f: &mut std::fmt::Formatter, arg0: u64) -> std::fmt::Result {
        write!(f, "{}", (- (arg0 as i64)))
    }
}

struct Op178(u8);
impl Op178 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op179(Sym26);
impl Op179 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym26::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op180(Sym26);
impl Op180 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym26::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op181(Sym15);
impl Op181 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op182(Sym28);
impl Op182 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym28::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op183(Sym17);
impl Op183 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op184(u8);
impl Op184 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8)))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

struct Op185(Sym28);
impl Op185 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym28::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Op186(Sym28);
impl Op186 {
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym28::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

