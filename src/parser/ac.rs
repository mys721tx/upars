use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, satisfy, space1};
use nom::combinator::{map_res, opt, recognize, value};
use nom::error::ParseError;
use nom::multi::many_m_n;
use nom::sequence::{terminated, tuple};
use nom::IResult;
use nom::{AsChar, InputIter, InputLength, Slice};

fn accession(i: &str) -> IResult<&str, &str> {
    alt((
        recognize(tuple((
            satisfy(|x| matches!(x, 'A'..='N' | 'R'..='Z')),
            satisfy(char::is_numeric),
            satisfy(char::is_uppercase),
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
            alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
            satisfy(char::is_numeric),
            opt(tuple((
                satisfy(char::is_uppercase),
                alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
                alt((satisfy(char::is_numeric), satisfy(char::is_uppercase))),
                satisfy(char::is_numeric),
            ))),
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

#[cfg(test)]
mod tests {
    use super::*;

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
        // Leaving the invalid suffix processing to ac_line
        // assert!(accession("A0AAA000000").is_err());
        // assert!(accession("A0AAA0A000A").is_err());
        assert!(accession("O0AAAA").is_err());
    }
}
