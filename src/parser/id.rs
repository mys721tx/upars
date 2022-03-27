use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, satisfy, space1};
use nom::combinator::{map_res, recognize, value};
use nom::error::ParseError;
use nom::multi::many_m_n;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;
use nom::{AsChar, InputIter, InputLength, Slice};
use std::ops::RangeFrom;
use strum_macros::EnumString;

#[derive(PartialEq, Debug, EnumString, Clone)]
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

fn many_m_n_alphanumeric<I, E>(m: usize, n: usize) -> impl FnMut(I) -> IResult<I, Vec<char>, E>
where
    I: Clone + InputLength + InputIter + Slice<RangeFrom<usize>>,
    E: ParseError<I>,
    <I as InputIter>::Item: AsChar,
{
    many_m_n(
        m,
        n,
        alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
    )
}

fn entry_name(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        many_m_n_alphanumeric(1, 10),
        char('_'),
        many_m_n_alphanumeric(1, 5),
    )))(i)
}

fn status(i: &str) -> IResult<&str, EntryStatus> {
    alt((
        value(EntryStatus::Unreviewed, tag("Unreviewed")),
        value(EntryStatus::Reviewed, tag("Reviewed")),
    ))(i)
}

fn length(i: &str) -> IResult<&str, u64> {
    terminated(
        map_res(digit1, str::parse::<u64>),
        tuple((space1, tag("AA"))),
    )(i)
}

pub fn id_line(i: &str) -> IResult<&str, IdLine> {
    let (l, (name, status, length)) = delimited(
        tuple((tag("ID"), space1)),
        tuple((
            terminated(entry_name, space1),
            terminated(status, char(';')),
            delimited(space1, length, char('.')),
        )),
        newline,
    )(i)?;

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
    use std::str::FromStr;

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
            id_line(
                "ID   CYC_BOVIN               Reviewed;         104 AA.
"
            ),
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
            id_line(
                "ID   GIA2_GIALA              Reviewed;         296 AA.
"
            ),
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
            id_line(
                "ID   Q5JU06_HUMAN            Unreviewed;       268 AA.
"
            ),
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
