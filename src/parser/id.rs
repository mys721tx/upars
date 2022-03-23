use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, satisfy, space1};
use nom::combinator::recognize;
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::IResult;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(PartialEq, Debug, EnumString)]
pub enum EntryStatus {
    Reviewed,
    Unreviewed,
}

#[derive(PartialEq, Debug)]
pub struct IdLine<'a> {
    name: &'a str,
    status: EntryStatus,
    length: u64,
}

fn entry_name(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        many_m_n(
            1,
            10,
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
        ),
        char('_'),
        many_m_n(
            1,
            5,
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
        ),
    )))(i)
}

fn status(i: &str) -> IResult<&str, EntryStatus> {
    let (l, r) = alt((tag("Unreviewed"), tag("Reviewed")))(i)?;
    Ok((l, EntryStatus::from_str(r).unwrap()))
}

fn length(i: &str) -> IResult<&str, u64> {
    let (l, (d, _, _)) = tuple((digit1, space1, tag("AA")))(i)?;
    Ok((l, u64::from_str(d).unwrap()))
}

pub fn id_line(i: &str) -> IResult<&str, IdLine> {
    let (l, (_, _, name, _, status, _, _, length, _)) = tuple((
        tag("ID"),
        space1,
        entry_name,
        space1,
        status,
        char(';'),
        space1,
        length,
        char('.'),
    ))(i)?;
    Ok((
        l,
        IdLine {
            name,
            status,
            length,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_status_from_str_test() {
        assert_eq!(
            EntryStatus::from_str("Reviewed").unwrap(),
            EntryStatus::Reviewed
        );
        assert_eq!(
            EntryStatus::from_str("Unreviewed").unwrap(),
            EntryStatus::Unreviewed
        );
    }
    #[test]
    fn entry_name_test() {
        assert_eq!(entry_name("CYC_BOVIN"), Ok(("", "CYC_BOVIN")));
        assert_eq!(entry_name("GIA2_GIALA"), Ok(("", "GIA2_GIALA")));
        assert!(entry_name("CYC").is_err());
        assert!(entry_name("CYC__").is_err());
        assert!(entry_name("CYC__BOVIN").is_err());
    }
    #[test]
    fn status_test() {
        assert_eq!(status("Reviewed"), Ok(("", EntryStatus::Reviewed)));
        assert_eq!(status("Unreviewed"), Ok(("", EntryStatus::Unreviewed)));
        assert!(status("UnReviewed").is_err());
        assert!(status("viewed").is_err());
    }
    #[test]
    fn length_test() {
        assert_eq!(length("104 AA"), Ok(("", 104)));
        assert!(length("104AA").is_err());
        assert!(length("104 B").is_err());
    }
    #[test]
    fn id_line_test() {
        assert_eq!(
            id_line("ID   CYC_BOVIN               Reviewed;         104 AA."),
            Ok((
                "",
                IdLine {
                    name: "CYC_BOVIN",
                    status: EntryStatus::Reviewed,
                    length: 104
                }
            ))
        );
        assert_eq!(
            id_line("ID   GIA2_GIALA              Reviewed;         296 AA."),
            Ok((
                "",
                IdLine {
                    name: "GIA2_GIALA",
                    status: EntryStatus::Reviewed,
                    length: 296
                }
            ))
        );
        assert_eq!(
            id_line("ID   Q5JU06_HUMAN            Unreviewed;       268 AA."),
            Ok((
                "",
                IdLine {
                    name: "Q5JU06_HUMAN",
                    status: EntryStatus::Unreviewed,
                    length: 268
                }
            ))
        );
    }
}
