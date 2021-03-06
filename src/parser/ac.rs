use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, satisfy, space0, space1};
use nom::combinator::recognize;
use nom::multi::{many1, many_m_n};
use nom::sequence::{delimited, tuple};
use nom::IResult;

fn accession(i: &str) -> IResult<&str, &str> {
    alt((
        recognize(tuple((
            satisfy(|x| matches!(x, 'A'..='N' | 'R'..='Z')),
            satisfy(char::is_numeric),
            many_m_n(
                1,
                2,
                tuple((
                    satisfy(char::is_uppercase),
                    alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
                    alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
                    satisfy(char::is_numeric),
                )),
            ),
        ))),
        recognize(tuple((
            satisfy(|x| matches!(x, 'O'..='Q')),
            satisfy(char::is_numeric),
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
            satisfy(char::is_numeric),
        ))),
    ))(i)
}

pub fn ac_line(i: &str) -> IResult<&str, Vec<&str>> {
    let (l, r) = many1(delimited(
        tuple((tag("AC"), space1)),
        many_m_n(1, 8, delimited(space0, accession, tag(";"))),
        newline,
    ))(i)?;

    Ok((l, r.concat()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::combinator::all_consuming;

    #[test]
    fn accession_test() {
        assert_eq!(accession("P12345"), Ok(("", "P12345")));
        assert_eq!(accession("Q1AAA9"), Ok(("", "Q1AAA9")));
        assert_eq!(accession("O456A1"), Ok(("", "O456A1")));
        assert_eq!(accession("P4A123"), Ok(("", "P4A123")));
        assert_eq!(accession("A0A022YWF9"), Ok(("", "A0A022YWF9")));
        assert!(accession("10AAA0").is_err());
        assert!(accession("AAAAA0").is_err());
        assert!(accession("A0AAAA").is_err());
        assert!(all_consuming(accession)("A0AAA000000").is_err());
        assert!(all_consuming(accession)("A0AAA0A000A").is_err());
        assert!(accession("O0AAAA").is_err());
    }

    #[test]
    fn ac_line_test() {
        assert_eq!(
            ac_line(
                "AC   P00321;
"
            ),
            Ok(("", vec!("P00321")))
        );
        assert_eq!(
            ac_line(
                "AC   Q16653; O00713; O00714; O00715; Q13054; Q13055; Q14855; Q92891;
AC   Q92892; Q92893; Q92894; Q92895; Q93053; Q96KU9; Q96KV0; Q96KV1;
AC   Q99605;
"
            ),
            Ok((
                "",
                vec!(
                    "Q16653", "O00713", "O00714", "O00715", "Q13054", "Q13055", "Q14855", "Q92891",
                    "Q92892", "Q92893", "Q92894", "Q92895", "Q93053", "Q96KU9", "Q96KV0", "Q96KV1",
                    "Q99605"
                )
            ))
        );
        assert!(ac_line(
            "AC   A0AAA000000;
"
        )
        .is_err());
        assert!(ac_line(
            "AC   A0AAA0A000A;
"
        )
        .is_err());
    }
}
