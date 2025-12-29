use crate::token::{self, Token, TokenKind};
use crate::utils::{Error, ErrorKind, Location, Result};

struct Parser<'a> {
    tokens: Vec<Token>,
    loc: Location,
    file: &'a str,
}

impl<'a> Parser<'a> {
    fn next(&mut self) -> Result<Token> {
        if let Some(token) = self.tokens.pop() {
            self.loc = token.loc;
            Ok(token)
        } else {
            Err(Error {
                loc: self.loc,
                file: &self.file,
                kind: ErrorKind::UnexpectedEnd,
            })
        }
    }

    fn expect(&mut self, kind: TokenKind, ekind: ErrorKind) -> Result<()> {
        let loc = self.loc;
        let file = self.file;
        let token = self.next()?;
        if token.kind != kind {
            Err(Error {
                loc: loc,
                file: &file,
                kind: ekind,
            })
        } else {
            Ok(())
        }
    }
}
