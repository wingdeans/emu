#[derive(Debug)]
pub(crate) struct Pcode();
#[derive(Debug)]
pub(crate) struct Insn(Sym0);
pub(crate) fn decode(buf: &[u8]) -> Option<Insn> {
    Sym0::decode(buf).map(|st| Insn(st))
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

#[derive(Debug)]
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
    #[allow(unused_parens)]
    #[allow(unused_variables)]
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
                                                Self::Variant94(Op169::decode(&buf)?, Op168::decode(&buf)?),
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
                                            Some(Self::Variant75(Op156::decode(&buf)?))
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
                                        Self::Variant80(Op160::decode(&buf)?, Op161::decode(&buf)?),
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
                                        Self::Variant93(Op166::decode(&buf)?, Op167::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 207u8 == 9u8) {
                                    Some(
                                        Self::Variant100(Op182::decode(&buf)?, Op181::decode(&buf)?),
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
                                        Self::Variant15(Op91::decode(&buf)?, Op90::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 10u8) {
                                    Some(
                                        Self::Variant5(Op70::decode(&buf)?, Op71::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            2u8 => {
                                if (buf[0usize] & 255u8 == 18u8) {
                                    Some(
                                        Self::Variant16(Op93::decode(&buf)?, Op92::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            3u8 => {
                                if (buf[0usize] & 255u8 == 26u8) {
                                    Some(
                                        Self::Variant6(Op72::decode(&buf)?, Op73::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            4u8 => {
                                if (buf[0usize] & 255u8 == 34u8) {
                                    Some(
                                        Self::Variant17(Op95::decode(&buf)?, Op94::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            5u8 => {
                                if (buf[0usize] & 255u8 == 42u8) {
                                    Some(
                                        Self::Variant13(Op86::decode(&buf)?, Op87::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            6u8 => {
                                if (buf[0usize] & 255u8 == 50u8) {
                                    Some(
                                        Self::Variant18(Op97::decode(&buf)?, Op96::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            7u8 => {
                                if (buf[0usize] & 255u8 == 58u8) {
                                    Some(
                                        Self::Variant14(Op88::decode(&buf)?, Op89::decode(&buf)?),
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
                                    Some(Self::Variant102(Op185::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 207u8 == 11u8) {
                                    Some(Self::Variant103(Op186::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    4u8 => {
                        if (buf[0usize] & 255u8 == 52u8) {
                            Some(Self::Variant44(Op123::decode(&buf)?))
                        } else if (buf[0usize] & 199u8 == 4u8) {
                            Some(Self::Variant43(Op122::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    5u8 => {
                        if (buf[0usize] & 255u8 == 53u8) {
                            Some(Self::Variant46(Op125::decode(&buf)?))
                        } else if (buf[0usize] & 199u8 == 5u8) {
                            Some(Self::Variant45(Op124::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    6u8 => {
                        if (buf[0usize] & 255u8 == 54u8) {
                            Some(
                                Self::Variant4(Op68::decode(&buf)?, Op69::decode(&buf)?),
                            )
                        } else if (buf[0usize] & 199u8 == 6u8) {
                            Some(
                                Self::Variant1(Op62::decode(&buf)?, Op63::decode(&buf)?),
                            )
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
                    Some(Self::Variant2(Op64::decode(&buf)?, Op65::decode(&buf)?))
                } else if (buf[0usize] & 248u8 == 112u8) {
                    Some(Self::Variant3(Op67::decode(&buf)?, Op66::decode(&buf)?))
                } else if (buf[0usize] & 192u8 == 64u8) {
                    Some(Self::Variant0(Op60::decode(&buf)?, Op61::decode(&buf)?))
                } else {
                    None
                }
            }
            2u8 => {
                match (buf[0usize] >> 3u8) & 7u8 {
                    0u8 => {
                        if (buf[0usize] & 255u8 == 134u8) {
                            Some(Self::Variant21(Op100::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 128u8) {
                            Some(Self::Variant19(Op98::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    1u8 => {
                        if (buf[0usize] & 255u8 == 142u8) {
                            Some(Self::Variant24(Op103::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 136u8) {
                            Some(Self::Variant22(Op101::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    2u8 => {
                        if (buf[0usize] & 255u8 == 150u8) {
                            Some(Self::Variant27(Op106::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 144u8) {
                            Some(Self::Variant25(Op104::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    3u8 => {
                        if (buf[0usize] & 255u8 == 158u8) {
                            Some(Self::Variant30(Op109::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 152u8) {
                            Some(Self::Variant28(Op107::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    4u8 => {
                        if (buf[0usize] & 255u8 == 166u8) {
                            Some(Self::Variant33(Op112::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 160u8) {
                            Some(Self::Variant31(Op110::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    5u8 => {
                        if (buf[0usize] & 255u8 == 174u8) {
                            Some(Self::Variant36(Op115::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 168u8) {
                            Some(Self::Variant34(Op113::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    6u8 => {
                        if (buf[0usize] & 255u8 == 182u8) {
                            Some(Self::Variant39(Op118::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 176u8) {
                            Some(Self::Variant37(Op116::decode(&buf)?))
                        } else {
                            None
                        }
                    }
                    7u8 => {
                        if (buf[0usize] & 255u8 == 190u8) {
                            Some(Self::Variant42(Op121::decode(&buf)?))
                        } else if (buf[0usize] & 248u8 == 184u8) {
                            Some(Self::Variant40(Op119::decode(&buf)?))
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
                                    Some(Self::Variant82(Op164::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                match (buf[0usize] >> 3u8) & 3u8 {
                                    0u8 => {
                                        if (buf[0usize] & 255u8 == 224u8) {
                                            Some(
                                                Self::Variant10(Op81::decode(&buf)?, Op80::decode(&buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 232u8) {
                                            Some(
                                                Self::Variant101(Op183::decode(&buf)?, Op184::decode(&buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 240u8) {
                                            Some(
                                                Self::Variant9(Op78::decode(&buf)?, Op79::decode(&buf)?),
                                            )
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
                                                    Op175::decode(&buf)?,
                                                    Op176::decode(&buf)?,
                                                    Op178::decode(&buf)?,
                                                    Op177::decode(&buf)?,
                                                ),
                                            )
                                        } else if (buf[0usize] & 255u8 == 248u8) {
                                            Some(
                                                Self::Variant96(
                                                    Op172::decode(&buf)?,
                                                    Op173::decode(&buf)?,
                                                    Op174::decode(&buf)?,
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
                                    Some(Self::Variant99(Op180::decode(&buf)?))
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
                                            Some(Self::Variant74(Op155::decode(&buf)?))
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 249u8) {
                                            Some(
                                                Self::Variant95(Op170::decode(&buf)?, Op171::decode(&buf)?),
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
                                        Self::Variant79(Op158::decode(&buf)?, Op159::decode(&buf)?),
                                    )
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                match (buf[0usize] >> 3u8) & 3u8 {
                                    0u8 => {
                                        if (buf[0usize] & 255u8 == 226u8) {
                                            Some(
                                                Self::Variant8(Op77::decode(&buf)?, Op76::decode(&buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    1u8 => {
                                        if (buf[0usize] & 255u8 == 234u8) {
                                            Some(
                                                Self::Variant12(Op85::decode(&buf)?, Op84::decode(&buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    2u8 => {
                                        if (buf[0usize] & 255u8 == 242u8) {
                                            Some(
                                                Self::Variant7(Op74::decode(&buf)?, Op75::decode(&buf)?),
                                            )
                                        } else {
                                            None
                                        }
                                    }
                                    3u8 => {
                                        if (buf[0usize] & 255u8 == 250u8) {
                                            Some(
                                                Self::Variant11(Op82::decode(&buf)?, Op83::decode(&buf)?),
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
                                    Some(Self::Variant73(Op154::decode(&buf)?))
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
                                                    Some(Self::Variant52(Op127::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 0u8)
                                                {
                                                    Some(Self::Variant51(Op126::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            1u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 14u8)
                                                {
                                                    Some(Self::Variant54(Op129::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 8u8)
                                                {
                                                    Some(Self::Variant53(Op128::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            2u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 22u8)
                                                {
                                                    Some(Self::Variant56(Op131::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 16u8)
                                                {
                                                    Some(Self::Variant55(Op130::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            3u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 30u8)
                                                {
                                                    Some(Self::Variant58(Op133::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 24u8)
                                                {
                                                    Some(Self::Variant57(Op132::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            4u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 38u8)
                                                {
                                                    Some(Self::Variant60(Op135::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 32u8)
                                                {
                                                    Some(Self::Variant59(Op134::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            5u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 46u8)
                                                {
                                                    Some(Self::Variant62(Op137::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 40u8)
                                                {
                                                    Some(Self::Variant61(Op136::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            6u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 54u8)
                                                {
                                                    Some(Self::Variant64(Op139::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 48u8)
                                                {
                                                    Some(Self::Variant63(Op138::decode(&buf)?))
                                                } else {
                                                    None
                                                }
                                            }
                                            7u8 => {
                                                if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 255u8 == 62u8)
                                                {
                                                    Some(Self::Variant66(Op141::decode(&buf)?))
                                                } else if (buf[0usize] & 255u8 == 203u8)
                                                    && (buf[1usize] & 248u8 == 56u8)
                                                {
                                                    Some(Self::Variant65(Op140::decode(&buf)?))
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
                                                Self::Variant68(Op145::decode(&buf)?, Op144::decode(&buf)?),
                                            )
                                        } else if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 192u8 == 64u8)
                                        {
                                            Some(
                                                Self::Variant67(Op142::decode(&buf)?, Op143::decode(&buf)?),
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
                                                Self::Variant70(Op149::decode(&buf)?, Op148::decode(&buf)?),
                                            )
                                        } else if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 192u8 == 128u8)
                                        {
                                            Some(
                                                Self::Variant69(Op146::decode(&buf)?, Op147::decode(&buf)?),
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
                                                Self::Variant72(Op153::decode(&buf)?, Op152::decode(&buf)?),
                                            )
                                        } else if (buf[0usize] & 255u8 == 203u8)
                                            && (buf[1usize] & 192u8 == 192u8)
                                        {
                                            Some(
                                                Self::Variant71(Op150::decode(&buf)?, Op151::decode(&buf)?),
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
                                Self::Variant81(Op162::decode(&buf)?, Op163::decode(&buf)?),
                            )
                        } else {
                            None
                        }
                    }
                    5u8 => {
                        match (buf[0usize] >> 3u8) & 1u8 {
                            0u8 => {
                                if (buf[0usize] & 207u8 == 197u8) {
                                    Some(Self::Variant98(Op179::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 205u8) {
                                    Some(Self::Variant76(Op157::decode(&buf)?))
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
                                    Some(Self::Variant20(Op99::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            1u8 => {
                                if (buf[0usize] & 255u8 == 206u8) {
                                    Some(Self::Variant23(Op102::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            2u8 => {
                                if (buf[0usize] & 255u8 == 214u8) {
                                    Some(Self::Variant26(Op105::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            3u8 => {
                                if (buf[0usize] & 255u8 == 222u8) {
                                    Some(Self::Variant29(Op108::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            4u8 => {
                                if (buf[0usize] & 255u8 == 230u8) {
                                    Some(Self::Variant32(Op111::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            5u8 => {
                                if (buf[0usize] & 255u8 == 238u8) {
                                    Some(Self::Variant35(Op114::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            6u8 => {
                                if (buf[0usize] & 255u8 == 246u8) {
                                    Some(Self::Variant38(Op117::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            7u8 => {
                                if (buf[0usize] & 255u8 == 254u8) {
                                    Some(Self::Variant41(Op120::decode(&buf)?))
                                } else {
                                    None
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    7u8 => {
                        if (buf[0usize] & 199u8 == 199u8) {
                            Some(Self::Variant83(Op165::decode(&buf)?))
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
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!(
            "    {}", match self { Self::Variant0(op0, op1) => "COPY",
            Self::Variant1(op0, op1) => "COPY", Self::Variant2(op0, op1) => "LOAD",
            Self::Variant3(op0, op1) => "STORE", Self::Variant4(op0, op1) => "STORE",
            Self::Variant5(op0, op1) => "LOAD", Self::Variant6(op0, op1) => "LOAD",
            Self::Variant7(op0, op1) => "MULTIEQUAL, COPY", Self::Variant8(op0, op1) =>
            "MULTIEQUAL, COPY", Self::Variant9(op0, op1) => "MULTIEQUAL, COPY",
            Self::Variant10(op0, op1) => "MULTIEQUAL, COPY", Self::Variant11(op0, op1) =>
            "MULTIEQUAL, COPY", Self::Variant12(op0, op1) => "MULTIEQUAL, COPY",
            Self::Variant13(op0, op1) => "MULTIEQUAL, COPY", Self::Variant14(op0, op1) =>
            "MULTIEQUAL, COPY", Self::Variant15(op0, op1) => "STORE",
            Self::Variant16(op0, op1) => "STORE", Self::Variant17(op0, op1) =>
            "MULTIEQUAL, COPY", Self::Variant18(op0, op1) => "MULTIEQUAL, COPY",
            Self::Variant19(op0) =>
            "COPY, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant20(op0) =>
            "COPY, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant21(op0) =>
            "LOAD, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant22(op0) =>
            "COPY, INT_RIGHT, INT_AND, INT_ADD, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant23(op0) =>
            "COPY, INT_RIGHT, INT_AND, INT_ADD, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant24(op0) =>
            "LOAD, INT_RIGHT, INT_AND, INT_ADD, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant25(op0) =>
            "COPY, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_LESS, INT_LEFT, INT_OR, COPY",
            Self::Variant26(op0) =>
            "COPY, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_LESS, INT_LEFT, INT_OR, COPY",
            Self::Variant27(op0) =>
            "LOAD, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_LESS, INT_LEFT, INT_OR, COPY",
            Self::Variant28(op0) =>
            "COPY, INT_RIGHT, INT_AND, INT_SUB, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_ADD, INT_LESS, INT_LEFT, INT_OR, COPY",
            Self::Variant29(op0) =>
            "COPY, INT_RIGHT, INT_AND, INT_SUB, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_ADD, INT_LESS, INT_LEFT, INT_OR, COPY",
            Self::Variant30(op0) =>
            "LOAD, INT_RIGHT, INT_AND, INT_SUB, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_ADD, INT_LESS, INT_LEFT, INT_OR, COPY",
            Self::Variant31(op0) =>
            "INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant32(op0) =>
            "INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant33(op0) =>
            "LOAD, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant34(op0) =>
            "INT_XOR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant35(op0) =>
            "INT_XOR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant36(op0) =>
            "LOAD, INT_XOR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant37(op0) =>
            "INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant38(op0) =>
            "INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant39(op0) =>
            "LOAD, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant40(op0) =>
            "COPY, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_LESS, INT_LEFT, INT_OR",
            Self::Variant41(op0) =>
            "COPY, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_LESS, INT_LEFT, INT_OR",
            Self::Variant42(op0) =>
            "LOAD, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_SUB, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_LESS, INT_LEFT, INT_OR",
            Self::Variant43(op0) =>
            "COPY, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, COPY",
            Self::Variant44(op0) =>
            "LOAD, COPY, INT_ADD, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant45(op0) =>
            "COPY, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, COPY",
            Self::Variant46(op0) =>
            "LOAD, COPY, INT_SUB, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant47() =>
            "INT_RIGHT, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant48() =>
            "INT_RIGHT, INT_LEFT, INT_RIGHT, INT_AND, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant49() =>
            "INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant50() =>
            "INT_AND, INT_RIGHT, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant51(op0) =>
            "COPY, INT_RIGHT, INT_LEFT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant52(op0) =>
            "LOAD, COPY, INT_RIGHT, INT_LEFT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant53(op0) =>
            "COPY, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant54(op0) =>
            "LOAD, COPY, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant55(op0) =>
            "COPY, INT_RIGHT, INT_LEFT, INT_RIGHT, INT_AND, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant56(op0) =>
            "LOAD, COPY, INT_RIGHT, INT_LEFT, INT_RIGHT, INT_AND, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant57(op0) =>
            "COPY, INT_AND, INT_RIGHT, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant58(op0) =>
            "LOAD, COPY, INT_AND, INT_RIGHT, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant59(op0) =>
            "COPY, INT_RIGHT, INT_LEFT, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant60(op0) =>
            "LOAD, COPY, INT_RIGHT, INT_LEFT, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant61(op0) =>
            "COPY, INT_AND, INT_SRIGHT, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant62(op0) =>
            "LOAD, COPY, INT_AND, INT_SRIGHT, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant63(op0) =>
            "COPY, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant64(op0) =>
            "LOAD, COPY, INT_AND, INT_LEFT, INT_RIGHT, INT_OR, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant65(op0) =>
            "COPY, INT_AND, INT_RIGHT, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY",
            Self::Variant66(op0) =>
            "LOAD, COPY, INT_AND, INT_RIGHT, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, COPY, STORE",
            Self::Variant67(op0, op1) =>
            "INT_LEFT, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant68(op0, op1) =>
            "LOAD, INT_LEFT, INT_AND, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant69(op0, op1) => "INT_LEFT, INT_NEGATE, INT_AND",
            Self::Variant70(op0, op1) => "LOAD, INT_LEFT, INT_NEGATE, INT_AND, STORE",
            Self::Variant71(op0, op1) => "INT_LEFT, INT_OR", Self::Variant72(op0, op1) =>
            "LOAD, INT_LEFT, INT_OR, STORE", Self::Variant73(op0) =>
            "MULTIEQUAL, BRANCH", Self::Variant74(op0) => "BRANCHIND",
            Self::Variant75(op0) => "MULTIEQUAL, BRANCH", Self::Variant76(op0) =>
            "MULTIEQUAL, INT_SUB, STORE, CALL", Self::Variant77() =>
            "LOAD, INT_ADD, RETURN", Self::Variant78() =>
            "LOAD, INT_ADD, CALLOTHER, RETURN", Self::Variant79(op0, op1) =>
            "MULTIEQUAL, MULTIEQUAL, CBRANCH", Self::Variant80(op0, op1) =>
            "MULTIEQUAL, MULTIEQUAL, CBRANCH", Self::Variant81(op0, op1) =>
            "MULTIEQUAL, MULTIEQUAL, BOOL_NEGATE, CBRANCH, INT_SUB, STORE, CALL",
            Self::Variant82(op0) =>
            "MULTIEQUAL, BOOL_NEGATE, CBRANCH, LOAD, INT_ADD, RETURN",
            Self::Variant83(op0) => "MULTIEQUAL, INT_SUB, STORE, CALL", Self::Variant84()
            => "CALLOTHER", Self::Variant85() => "CALLOTHER", Self::Variant86() =>
            "CALLOTHER", Self::Variant87() => "CALLOTHER", Self::Variant88() =>
            "INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_RIGHT, INT_AND, BOOL_NEGATE, INT_LEFT, INT_OR",
            Self::Variant89() =>
            "INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant90() => "", Self::Variant91() =>
            "COPY, INT_RIGHT, INT_AND, INT_RIGHT, INT_AND, INT_RIGHT, INT_AND, CALLOTHER, INT_ADD, INT_CARRY, INT_AND, INT_EQUAL, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_RIGHT, INT_AND, INT_RIGHT, INT_AND, BOOL_NEGATE, BOOL_AND, BOOL_OR, INT_LEFT, INT_OR, COPY",
            Self::Variant92() =>
            "INT_NEGATE, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR",
            Self::Variant93(op0, op1) => "COPY", Self::Variant94(op0, op1) =>
            "MULTIEQUAL, COPY", Self::Variant95(op0, op1) => "COPY", Self::Variant96(op0,
            op1, op2) =>
            "INT_ADD, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant97(op0, op1, op2, op3) =>
            "INT_ADD, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant98(op0) => "INT_SUB, STORE", Self::Variant99(op0) =>
            "LOAD, INT_ADD", Self::Variant100(op0, op1) =>
            "INT_ADD, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant101(op0, op1) =>
            "INT_ADD, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_LEFT, INT_OR, INT_AND, INT_AND, INT_AND, INT_ADD, INT_AND, INT_NOTEQUAL, INT_LEFT, INT_OR, INT_AND, INT_CARRY, INT_LEFT, INT_OR, COPY",
            Self::Variant102(op0) => "INT_ADD", Self::Variant103(op0) => "INT_SUB", }
        )
    }
}
impl std::fmt::Display for Sym0 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym4();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym5();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym6();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym7();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym8();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym9();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym10();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym11();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym12();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym13();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym14();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym15();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym16();
#[allow(dead_code)]
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

#[derive(Debug)]
#[allow(dead_code)]
struct Sym17();
#[allow(dead_code)]
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

#[derive(Debug)]
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
        match (buf[0usize] >> 0u8 & 7u8).into() {
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

#[derive(Debug)]
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
        match (buf[0usize] >> 3u8 & 7u8).into() {
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

#[derive(Debug)]
enum Sym26 {
    Variant0(Sym13),
    Variant1(Sym14),
    Variant2(Sym15),
    Variant3(Sym12),
}
impl Sym26 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 4u8 & 3u8).into() {
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

#[derive(Debug)]
enum Sym27 {
    Variant0(Sym13),
    Variant1(Sym14),
    Variant2(Sym15),
    Variant3(Sym17),
}
impl Sym27 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 4u8 & 3u8).into() {
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

#[derive(Debug)]
enum Sym28 {
    Variant0(Sym13),
    Variant1(Sym14),
    Variant2(Sym15),
    Variant3(Sym17),
}
impl Sym28 {
    fn decode(buf: &[u8]) -> Option<Self> {
        match (buf[0usize] >> 4u8 & 3u8).into() {
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

#[derive(Debug)]
enum Sym35 {
    Variant0(),
    Variant1(),
    Variant2(),
    Variant3(),
}
impl Sym35 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!(
            "    {}", match self { Self::Variant0() => "INT_RIGHT, BOOL_NEGATE",
            Self::Variant1() => "INT_RIGHT", Self::Variant2() =>
            "INT_RIGHT, INT_AND, BOOL_NEGATE", Self::Variant3() => "INT_RIGHT, INT_AND",
            }
        )
    }
}
impl std::fmt::Display for Sym35 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
enum Sym36 {
    Variant0(Op37),
}
impl Sym36 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op37::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "", })
    }
}
impl std::fmt::Display for Sym36 {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0) => {
                op0.write(f)?;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
struct Op37(u8);
impl Op37 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
enum Sym38 {
    Variant0(Op39),
}
impl Sym38 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op39::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "", })
    }
}
impl std::fmt::Display for Sym38 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
struct Op39(u8);
impl Op39 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
enum Sym40 {
    Variant0(Op41),
}
impl Sym40 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op41::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "", })
    }
}
impl std::fmt::Display for Sym40 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
struct Op41(u8);
impl Op41 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
enum Sym42 {
    Variant0(Op44, Op43),
}
impl Sym42 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op44::decode(&buf)?, Op43::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0, op1) => "", })
    }
}
impl std::fmt::Display for Sym42 {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0, op1) => {
                op1.write(f, op0.0 as u64)?;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
struct Op43();
impl Op43 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self())
    }
    fn write(&self, f: &mut std::fmt::Formatter, arg0: u64) -> std::fmt::Result {
        write!(f, "{}", (0 + arg0))
    }
}

#[derive(Debug)]
struct Op44(u8);
impl Op44 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
enum Sym45 {
    Variant0(Op47, Op46),
}
impl Sym45 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op47::decode(&buf)?, Op46::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0, op1) => "", })
    }
}
impl std::fmt::Display for Sym45 {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0(op0, op1) => {
                op1.write(f, op0.0 as u64)?;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
struct Op46();
impl Op46 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self())
    }
    fn write(&self, f: &mut std::fmt::Formatter, arg0: u64) -> std::fmt::Result {
        write!(f, "{}", (arg0 << 3u64))
    }
}

#[derive(Debug)]
struct Op47(u8);
impl Op47 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
enum Sym48 {
    Variant0(Op49),
}
impl Sym48 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op49::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "COPY, INT_ADD", })
    }
}
impl std::fmt::Display for Sym48 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
struct Op49(Sym15);
impl Op49 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum Sym50 {
    Variant0(Op51),
}
impl Sym50 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op51::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "COPY, INT_SUB", })
    }
}
impl std::fmt::Display for Sym50 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
struct Op51(Sym15);
impl Op51 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum Sym52 {
    Variant0(Op53),
}
impl Sym52 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op53::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "INT_ZEXT, INT_OR", })
    }
}
impl std::fmt::Display for Sym52 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
struct Op53(Sym6);
impl Op53 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym6::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum Sym54 {
    Variant0(Op55),
}
impl Sym54 {
    #[allow(unused_parens)]
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self::Variant0(Op55::decode(&buf)?))
    }
    #[allow(unused_variables)]
    #[allow(dead_code)]
    fn pcode(&self, vec: &mut Vec<Pcode>) {
        println!("    {}", match self { Self::Variant0(op0) => "INT_ZEXT, INT_OR", })
    }
}
impl std::fmt::Display for Sym54 {
    #[allow(unused_variables)]
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

#[derive(Debug)]
struct Op55(u8);
impl Op55 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[0usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op60(Sym24);
impl Op60 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op61(Sym21);
impl Op61 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op62(Sym24);
impl Op62 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op63(u8);
impl Op63 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op64(Sym24);
impl Op64 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op65(Sym15);
impl Op65 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op66(Sym15);
impl Op66 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op67(Sym21);
impl Op67 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op68(Sym15);
impl Op68 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op69(u8);
impl Op69 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op70(Sym5);
impl Op70 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op71(Sym13);
impl Op71 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym13::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op72(Sym5);
impl Op72 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op73(Sym14);
impl Op73 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym14::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op74(Sym5);
impl Op74 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op75(Sym52);
impl Op75 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym52::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op76(Sym52);
impl Op76 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym52::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op77(Sym5);
impl Op77 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op78(Sym5);
impl Op78 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op79(Sym54);
impl Op79 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym54::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op80(Sym54);
impl Op80 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym54::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op81(Sym5);
impl Op81 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op82(Sym5);
impl Op82 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op83(Sym38);
impl Op83 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym38::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op84(Sym38);
impl Op84 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym38::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op85(Sym5);
impl Op85 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op86(Sym5);
impl Op86 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op87(Sym48);
impl Op87 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym48::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op88(Sym5);
impl Op88 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op89(Sym50);
impl Op89 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym50::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op90(Sym13);
impl Op90 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym13::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op91(Sym5);
impl Op91 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op92(Sym14);
impl Op92 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym14::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op93(Sym5);
impl Op93 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op94(Sym48);
impl Op94 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym48::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op95(Sym5);
impl Op95 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op96(Sym50);
impl Op96 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym50::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op97(Sym5);
impl Op97 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym5::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op98(Sym21);
impl Op98 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op99(u8);
impl Op99 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op100(Sym15);
impl Op100 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op101(Sym21);
impl Op101 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op102(u8);
impl Op102 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op103(Sym15);
impl Op103 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op104(Sym21);
impl Op104 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op105(u8);
impl Op105 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op106(Sym15);
impl Op106 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op107(Sym21);
impl Op107 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op108(u8);
impl Op108 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op109(Sym15);
impl Op109 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op110(Sym21);
impl Op110 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op111(u8);
impl Op111 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op112(Sym15);
impl Op112 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op113(Sym21);
impl Op113 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op114(u8);
impl Op114 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op115(Sym15);
impl Op115 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op116(Sym21);
impl Op116 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op117(u8);
impl Op117 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op118(Sym15);
impl Op118 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op119(Sym21);
impl Op119 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op120(u8);
impl Op120 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op121(Sym15);
impl Op121 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op122(Sym24);
impl Op122 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op123(Sym15);
impl Op123 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op124(Sym24);
impl Op124 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym24::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op125(Sym15);
impl Op125 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op126(Sym21);
impl Op126 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op127(Sym15);
impl Op127 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op128(Sym21);
impl Op128 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op129(Sym15);
impl Op129 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op130(Sym21);
impl Op130 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op131(Sym15);
impl Op131 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op132(Sym21);
impl Op132 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op133(Sym15);
impl Op133 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op134(Sym21);
impl Op134 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op135(Sym15);
impl Op135 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op136(Sym21);
impl Op136 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op137(Sym15);
impl Op137 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op138(Sym21);
impl Op138 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op139(Sym15);
impl Op139 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op140(Sym21);
impl Op140 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op141(Sym15);
impl Op141 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op142(u8);
impl Op142 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op143(Sym21);
impl Op143 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op144(u8);
impl Op144 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op145(Sym15);
impl Op145 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op146(u8);
impl Op146 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op147(Sym21);
impl Op147 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op148(u8);
impl Op148 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op149(Sym15);
impl Op149 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op150(u8);
impl Op150 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op151(Sym21);
impl Op151 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym21::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op152(u8);
impl Op152 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 3u8 & 7u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op153(Sym15);
impl Op153 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op154(Sym36);
impl Op154 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op155(Sym15);
impl Op155 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op156(Sym42);
impl Op156 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym42::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op157(Sym36);
impl Op157 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op158(Sym35);
impl Op158 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op159(Sym36);
impl Op159 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op160(Sym35);
impl Op160 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op161(Sym42);
impl Op161 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym42::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op162(Sym35);
impl Op162 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op163(Sym36);
impl Op163 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym36::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op164(Sym35);
impl Op164 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym35::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op165(Sym45);
impl Op165 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym45::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op166(Sym27);
impl Op166 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym27::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op167(u8);
impl Op167 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op168(Sym40);
impl Op168 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym40::decode(&buf[1usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op169(Sym17);
impl Op169 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op170(Sym17);
impl Op170 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op171(Sym15);
impl Op171 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op172(Sym15);
impl Op172 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op173(Sym17);
impl Op173 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op174(u8);
impl Op174 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op175(Sym15);
impl Op175 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op176(Sym17);
impl Op176 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op177();
impl Op177 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self())
    }
    fn write(&self, f: &mut std::fmt::Formatter, arg0: u64) -> std::fmt::Result {
        write!(f, "{}", (- (arg0 as i64)))
    }
}

#[derive(Debug)]
struct Op178(u8);
impl Op178 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op179(Sym26);
impl Op179 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym26::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op180(Sym26);
impl Op180 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym26::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op181(Sym15);
impl Op181 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym15::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op182(Sym28);
impl Op182 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym28::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op183(Sym17);
impl Op183 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym17::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op184(u8);
impl Op184 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self((buf[1usize] >> 0u8 & 255u8).into()))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Debug)]
struct Op185(Sym28);
impl Op185 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym28::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Op186(Sym28);
impl Op186 {
    #[allow(unused_variables)]
    fn decode(buf: &[u8]) -> Option<Self> {
        Some(Self(Sym28::decode(&buf[0usize..])?))
    }
    fn write(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

