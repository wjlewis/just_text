use super::common::Span;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    input_len: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars(),
            input_len: input.len(),
        }
    }

    fn eat_next(&mut self) -> Option<Token> {
        let start = self.current_pos();
        let next = self.munch();
        if next.is_none() {
            return None;
        }

        let next = next.unwrap();
        let kind = match next {
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '\n' => TokenKind::Newline,
            '`' => self.eat_backticks(start),
            _ => self.eat_text(),
        };
        let end = self.current_pos();
        Some(Token {
            kind,
            span: (start, end),
        })
    }

    fn eat_backticks(&mut self, first_tick_pos: usize) -> TokenKind {
        self.eat_while(|c| c == '`');
        let count = self.current_pos() - first_tick_pos;

        match count {
            1 => TokenKind::Backtick,
            3 => TokenKind::TripleBacktick,
            _ => TokenKind::Text,
        }
    }

    fn eat_text(&mut self) -> TokenKind {
        self.eat_while(|c| match c {
            '[' | ']' | '(' | ')' | '\n' | '`' => false,
            _ => true,
        });

        TokenKind::Text
    }

    fn current_pos(&self) -> usize {
        self.input_len - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn munch(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn eat_while<F>(&mut self, pred: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }
            self.munch();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_next()
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Text,
    Newline,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Backtick,
    TripleBacktick,
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    macro_rules! toks {
        ( $( ($kind:expr, $len:expr) ),* ) => {
            {
                let mut tokens = vec![];
                let mut end = 0;
                $(
                    end += $len;
                    tokens.push(Token {
                        kind: $kind,
                        span: (end - $len, end),
                    });
                )*
                tokens
            }
        };
    }

    #[test]
    fn lex1() {
        let input = "This is a test
with a `couple` of [lines](http://test.com)
```
And some preformatted
text
```";
        assert_eq!(
            Lexer::new(input).collect::<Vec<Token>>(),
            toks![
                (Text, 14),
                (Newline, 1),
                (Text, 7),
                (Backtick, 1),
                (Text, 6),
                (Backtick, 1),
                (Text, 4),
                (LBracket, 1),
                (Text, 5),
                (RBracket, 1),
                (LParen, 1),
                (Text, 15),
                (RParen, 1),
                (Newline, 1),
                (TripleBacktick, 3),
                (Newline, 1),
                (Text, 21),
                (Newline, 1),
                (Text, 4),
                (Newline, 1),
                (TripleBacktick, 3)
            ]
        );
    }
}
