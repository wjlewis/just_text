use super::common::Span;
use super::lexer::{Lexer, TokenKind};
use crate::lib::error::JustTextError;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub struct Note {
    paragraphs: Vec<Paragraph>,
}

#[derive(Debug, PartialEq)]
pub struct Paragraph {
    parts: Vec<TextElement>,
}

#[derive(Debug, PartialEq)]
pub enum TextElement {
    Text(Span),
    Link { title: Span, href: Span },
    Mono(Span),
    BlockMono(Span),
}

impl Note {
    pub fn resolve(&self, content: &str) -> String {
        let mut result = String::new();
        for paragraph in &self.paragraphs {
            result.push_str(&format!("<p>{}</p>", paragraph.resolve(content)));
        }
        result
    }
}

impl Paragraph {
    fn resolve(&self, content: &str) -> String {
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.resolve(content));
            result.push(' ');
        }
        result
    }
}

impl TextElement {
    fn resolve(&self, content: &str) -> String {
        match self {
            TextElement::Text((s, e)) => content[*s..*e].to_string(),
            TextElement::Link { title, href } => {
                let href = &content[href.0..href.1];
                let title = &content[title.0..title.1];
                format!("<a href=\"{}\">{}</a>", href, title)
            }
            TextElement::Mono((s, e)) => {
                let slice = &content[*s..*e].trim();
                format!("<span class=\"mono\">{}</span>", slice)
            }
            TextElement::BlockMono((s, e)) => {
                let slice = &content[*s..*e].trim();
                format!("<pre>{}</pre>", slice)
            }
        }
    }
}

pub fn parse(input: &str) -> Result<Note, JustTextError> {
    parse_note(&mut Lexer::new(input).peekable())
}

fn parse_note<'a>(tokens: &mut Peekable<Lexer<'a>>) -> Result<Note, JustTextError<'a>> {
    let mut paragraphs = Vec::new();

    while let Some(_) = tokens.peek() {
        match parse_paragraph(tokens) {
            Ok(p) => {
                paragraphs.push(p);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(Note { paragraphs })
}

fn parse_paragraph<'a>(tokens: &mut Peekable<Lexer<'a>>) -> Result<Paragraph, JustTextError<'a>> {
    let mut parts = Vec::new();

    while let Some(t) = tokens.peek() {
        // Todo: Is there a nicer way to handle this?
        if t.kind == TokenKind::Newline {
            tokens.next();
            if let Some(t) = tokens.peek() {
                if t.kind == TokenKind::Newline {
                    tokens.next();
                    return Ok(Paragraph { parts });
                }
            }
        }

        if tokens.peek().is_none() {
            return Ok(Paragraph { parts });
        }

        match parse_text_element(tokens) {
            Ok(te) => {
                parts.push(te);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(Paragraph { parts })
}

// We know that tokens aren't empty
fn parse_text_element<'a>(
    tokens: &mut Peekable<Lexer<'a>>,
) -> Result<TextElement, JustTextError<'a>> {
    use TokenKind::*;

    let next = tokens.peek().unwrap();

    match next.kind {
        Text => Ok(parse_text(tokens)),
        Backtick => parse_mono(tokens),
        TripleBacktick => parse_block_mono(tokens),
        LBracket => parse_link(tokens),
        // Todo: Improve error reporting
        _ => Err(JustTextError::new("Malformed note input")),
    }
}

fn parse_text<'a>(tokens: &mut Peekable<Lexer<'a>>) -> TextElement {
    use TokenKind::*;

    let (start, mut end) = tokens.next().unwrap().span;

    while let Some(t) = tokens.peek() {
        match t.kind {
            Backtick | TripleBacktick | LBracket | Newline => {
                break;
            }
            _ => {
                end = t.span.1;
                tokens.next();
            }
        }
    }

    TextElement::Text((start, end))
}

fn parse_mono<'a>(tokens: &mut Peekable<Lexer<'a>>) -> Result<TextElement, JustTextError<'a>> {
    use TokenKind::*;

    let start = tokens.next().unwrap().span.1;
    let mut end = start;

    loop {
        let next = tokens.next();
        if next.is_none() {
            return Err(JustTextError::new("Unterminated inline mono"));
        }
        let next = next.unwrap();

        match next.kind {
            Backtick => {
                break;
            }
            _ => {
                end = next.span.1;
            }
        }
    }

    Ok(TextElement::Mono((start, end)))
}

fn parse_block_mono<'a>(
    tokens: &mut Peekable<Lexer<'a>>,
) -> Result<TextElement, JustTextError<'a>> {
    use TokenKind::*;

    let start = tokens.next().unwrap().span.1;
    let mut end = start;

    loop {
        let next = tokens.next();
        if next.is_none() {
            return Err(JustTextError::new("Unterminated block mono"));
        }
        let next = next.unwrap();

        match next.kind {
            TripleBacktick => {
                break;
            }
            _ => {
                end = next.span.1;
            }
        }
    }

    Ok(TextElement::BlockMono((start, end)))
}

fn parse_link<'a>(tokens: &mut Peekable<Lexer<'a>>) -> Result<TextElement, JustTextError<'a>> {
    use TokenKind::*;

    // Consume "["
    tokens.next();

    // title
    let next = tokens.next();
    if next.is_none() {
        return Err(JustTextError::new("Incomplete link"));
    }
    let next = next.unwrap();
    if next.kind != Text {
        return Err(JustTextError::new(
            "Invalid link: expected text after \"[\"",
        ));
    }
    let title_span = next.span;

    // ]
    let next = tokens.next();
    if next.is_none() {
        return Err(JustTextError::new("Incomplete link"));
    }
    let next = next.unwrap();
    if next.kind != RBracket {
        return Err(JustTextError::new(
            "Invalid link: expected \"]\" after href",
        ));
    }

    // (
    let next = tokens.next();
    if next.is_none() {
        return Err(JustTextError::new("Incomplete link"));
    }
    let next = next.unwrap();
    if next.kind != LParen {
        return Err(JustTextError::new(
            "Invalid link: expected \"(\" after \"]\"",
        ));
    }

    // href
    let next = tokens.next();
    if next.is_none() {
        return Err(JustTextError::new("Incomplete link"));
    }
    let next = next.unwrap();
    if next.kind != Text {
        return Err(JustTextError::new(
            "Invalid link: expected text after \"(\"",
        ));
    }
    let href_span = next.span;

    // )
    let next = tokens.next();
    if next.is_none() {
        return Err(JustTextError::new("Incomplete link"));
    }
    let next = next.unwrap();
    if next.kind != RParen {
        return Err(JustTextError::new(
            "Invalid link: expected \")\" after title",
        ));
    }

    Ok(TextElement::Link {
        title: title_span,
        href: href_span,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_link1() {
        let input = "[a link](here)";
        //           01234567890123

        assert_eq!(
            parse_link(&mut Lexer::new(input).peekable()),
            Ok(TextElement::Link {
                title: (1, 7),
                href: (9, 13)
            })
        );
    }

    #[test]
    fn parse_mono1() {
        let input = "`Some monospace text`";
        //           012345678901234567890

        assert_eq!(
            parse_mono(&mut Lexer::new(input).peekable()),
            Ok(TextElement::Mono((1, 20)))
        );
    }

    #[test]
    fn parse_block_mono1() {
        let input = "```This is some block mono```";
        //           01234567890123456789012345678

        assert_eq!(
            parse_block_mono(&mut Lexer::new(input).peekable()),
            Ok(TextElement::Mono((3, 26)))
        );
    }

    #[test]
    fn parse_paragraph1() {
        let input = "This is a `paragraph` with [a link](here)";
        //           01234567890123456789012345678901234567890

        assert_eq!(
            parse_paragraph(&mut Lexer::new(input).peekable()),
            Ok(Paragraph {
                parts: vec![
                    TextElement::Text((0, 10)),
                    TextElement::Mono((11, 20)),
                    TextElement::Text((21, 27)),
                    TextElement::Link {
                        title: (28, 34),
                        href: (36, 40)
                    }
                ]
            })
        );
    }

    #[test]
    fn parse1() {
        let input = "This is a note

with a couple of lines";

        assert_eq!(
            parse(input),
            Ok(Note {
                paragraphs: vec![
                    Paragraph {
                        parts: vec![TextElement::Text((0, 14))]
                    },
                    Paragraph {
                        parts: vec![TextElement::Text((16, 38))]
                    }
                ]
            })
        );
    }
}
