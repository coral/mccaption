use crate::TimeCodeFormat;
use std::fs::read_to_string;
use std::path::Path;
use thiserror::Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    character::complete::{char, digit1, line_ending, not_line_ending, one_of, tab},
    combinator::{map, map_opt, map_res, recognize},
    multi::many0,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Header {
    pub format: String,
    pub uuid: String,
    pub creation_program: String,
    pub creation_date: String,
    pub creation_time: String,
    pub timecode_format: TimeCodeFormat,
}

impl Header {
    fn parse(input: &str) -> IResult<&str, Header> {
        let (input, format) = terminated(Self::parse_format, line_ending)(input)?;
        let (input, _) = Self::skip_comments(input)?; // Skip comments
        let (input, uuid) = terminated(Self::parse_uuid, line_ending)(input)?;
        let (input, creation_program) =
            terminated(Self::parse_creation_program, line_ending)(input)?;
        let (input, creation_date) = terminated(Self::parse_creation_date, line_ending)(input)?;
        let (input, creation_time) = terminated(Self::parse_creation_time, line_ending)(input)?;
        let (input, timecode_format) = terminated(Self::parse_time_code, line_ending)(input)?;

        Ok((
            input,
            Header {
                format,
                uuid,
                creation_program,
                creation_date,
                creation_time,
                timecode_format,
            },
        ))
    }

    fn parse_format(input: &str) -> IResult<&str, String> {
        let (input, _) = tag("File Format=")(input)?;
        map(not_line_ending, |s: &str| s.to_string())(input)
    }

    fn parse_uuid(input: &str) -> IResult<&str, String> {
        let (input, _) = tag("UUID=")(input)?;
        map(not_line_ending, |s: &str| s.to_string())(input)
    }

    fn parse_creation_program(input: &str) -> IResult<&str, String> {
        let (input, _) = tag("Creation Program=")(input)?;
        map(not_line_ending, |s: &str| s.to_string())(input)
    }

    fn parse_creation_date(input: &str) -> IResult<&str, String> {
        let (input, _) = tag("Creation Date=")(input)?;
        map(not_line_ending, |s: &str| s.to_string())(input)
    }

    fn parse_creation_time(input: &str) -> IResult<&str, String> {
        let (input, _) = tag("Creation Time=")(input)?;
        map(not_line_ending, |s: &str| s.to_string())(input)
    }

    fn parse_time_code(input: &str) -> IResult<&str, TimeCodeFormat> {
        let (input, _) = tag("Time Code Rate=")(input)?;
        let (remaining_input, TimeCodeFormat) = map_opt(
            take_while1(|c: char| c.is_numeric() || c == 'D' || c == 'F'),
            TimeCodeFormat::from_str,
        )(input)?;
        let (remaining_input, _) = line_ending(remaining_input)?;
        Ok((remaining_input, TimeCodeFormat))
    }

    fn skip_comments(input: &str) -> IResult<&str, ()> {
        let comment_line = recognize(tuple((char('/'), many1(char('/')), not_line_ending)));

        let skip_element = alt((
            map(comment_line, |_| ()), // match comment and discard
            map(line_ending, |_| ()),  // match newline and discard
        ));

        let (input, _) = many0(skip_element)(input)?;
        Ok((input, ()))
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MCC {
    header: Header,
}

impl MCC {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = read_to_string(path)?;
        let (mut input, v) = Self::from_str(&mut file)?;

        let (_, m) = Self::parse_line(&mut input)?;

        dbg!(m);

        Ok(v)
    }

    pub fn from_str(input: &str) -> IResult<&str, MCC> {
        let (input, header) = Header::parse(input)?;

        Ok((input, MCC { header }))
    }

    fn parse_time_code(input: &str) -> IResult<&str, (u32, u32, u32, u32)> {
        tuple((
            Self::parse_time,
            preceded(tag(":"), Self::parse_time),
            preceded(tag(":"), Self::parse_time),
            preceded(tag(":"), Self::parse_time),
        ))(input)
    }

    fn parse_time(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }

    fn ascii_to_hex(c: char) -> Vec<u8> {
        match c {
            'G' => vec![0xFA, 0x00, 0x00],
            'H' => vec![0xFA, 0x00, 0x00, 0xFA, 0x00, 0x00],
            'I' => vec![0xFA, 0x00, 0x00, 0xFA, 0x00, 0x00, 0xFA, 0x00, 0x00],
            'J' => vec![0xFA, 0x00, 0x00].repeat(4),
            'K' => vec![0xFA, 0x00, 0x00].repeat(5),
            'L' => vec![0xFA, 0x00, 0x00].repeat(6),
            'M' => vec![0xFA, 0x00, 0x00].repeat(7),
            'N' => vec![0xFA, 0x00, 0x00].repeat(8),
            'O' => vec![0xFA, 0x00, 0x00].repeat(9),
            'P' => vec![0xFB, 0x80, 0x80],
            'Q' => vec![0xFC, 0x80, 0x80],
            'R' => vec![0xFD, 0x80, 0x80],
            'S' => vec![0x96, 0x69],
            'T' => vec![0x61, 0x01],
            'U' => vec![0xE1, 0x00, 0x00, 0x00],
            'Z' => vec![0x00],
            _ => vec![],
        }
    }

    fn parse_line(input: &str) -> IResult<&str, ((u32, u32, u32, u32), Vec<u8>)> {
        dbg!("k");
        dbg!(&input);
        tuple((Self::parse_time_code, preceded(tab, Self::combined_parser)))(input)
    }

    fn special_char_parser(input: &str) -> IResult<&str, Vec<u8>> {
        map(one_of("GHIJKLMNOPQRSTUVWXYZ"), Self::ascii_to_hex)(input)
    }

    fn hex_byte_parser(input: &str) -> IResult<&str, Vec<u8>> {
        map_res(take(2usize), Self::parse_hex_byte)(input)
    }

    fn parse_hex_byte(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
        u8::from_str_radix(input, 16).map(|byte| vec![byte])
    }

    fn combined_parser(input: &str) -> IResult<&str, Vec<u8>> {
        let mut result = vec![];
        let (input, parsed_values) =
            many0(alt((Self::special_char_parser, Self::hex_byte_parser)))(input)?;
        for v in parsed_values {
            result.extend(v);
        }
        Ok((input, result))
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("file error")]
    Disconnect(#[from] std::io::Error),
    #[error("parsing error: {0}")]
    ParseError(String),
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => Error::ParseError(format!("{:?}", e)),
            nom::Err::Incomplete(_) => Error::ParseError("incomplete input".to_string()),
        }
    }
}
