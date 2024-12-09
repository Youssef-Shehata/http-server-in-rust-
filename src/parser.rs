use std::{io::Read, str::FromStr};

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{line_ending, not_line_ending},
    combinator::map_res,
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

use crate::{
    http::Headers,
    http::{HttpMethod, HttpRequest, HttpVersion},
};

pub fn parse_request(request: &[u8]) -> IResult<&[u8], HttpRequest> {
    let (request, (method, _, path, _, version, _)) = tuple((
        parse_method,
        tag(" "),
        parse_path,
        tag(" "),
        parse_version,
        line_ending,
    ))(request)?;

    let (mut request, headers) = parse_headers(request)?;

    //in parse_headers the many0 parser takes the two registered nurses at the end now
    //let (request, _) = line_ending(request)?;

    let mut body = Vec::new();
    if let Some(content_length) = headers.get("Content-length") {
        body.resize(content_length.parse().unwrap(), 0);
        let _ = request.read_exact(&mut body);
    } else {
        //TODO: chunked-transfer , 100-continue
        body = request.to_vec();
    };

    Ok((
        &[],
        HttpRequest::new(method, path.to_string(), version, headers, body),
    ))
}

fn parse_method(input: &[u8]) -> IResult<&[u8], HttpMethod> {
    map_res(
        take_while1(|c: u8| c.is_ascii_alphabetic()),
        |method: &[u8]| {
            let method_str = std::str::from_utf8(method).unwrap();
            HttpMethod::from_str(method_str).or_else(|_| Err("Invalid HTTP method"))
        },
    )(input)
}

fn parse_path(input: &[u8]) -> IResult<&[u8], &str> {
    map_res(take_while1(|c: u8| c != b' '), std::str::from_utf8)(input)
}

fn parse_version(input: &[u8]) -> IResult<&[u8], HttpVersion> {
    map_res(take_until("\r\n"), |version: &[u8]| {
        let version_str = std::str::from_utf8(version).unwrap();
        HttpVersion::from_str(version_str).ok_or_else(|| "Invalid HTTP version")
    })(input)
}

fn parse_headers(input: &[u8]) -> IResult<&[u8], Headers> {
    let (input, header_lines) = many0(terminated(not_line_ending, line_ending))(input)?;

    let mut headers = Headers::new();
    for line in header_lines {
        if let Ok(header_str) = std::str::from_utf8(line) {
            if let Some((key, value)) = header_str.split_once(':') {
                headers.add(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    Ok((input, headers))
}
