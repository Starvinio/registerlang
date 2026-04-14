use crate::{LangError, LangToken, Span, TokenType};
/// Byte-based lexer
///
/// Consumes a source string and produces a stream
/// of [`LangToken`] via [`emit_token`](Self::emit_token)
///
/// # Design Choices:
/// - Operates on raw bytes for performance
/// - Assumes ASCII-oriented output
/// - Does not handle unicode
pub struct Lexer {
    /// Source code being scanned
    /// Stored as [`Box<>`] to:
    /// - Display immutability
    /// - Avoid structure lifetime
    pub src: Box<str>,

    // Start of the current token
    start: usize,

    // Current position (byte index) in the source
    current: usize,
}

impl Lexer {
    /// Init that loads [`Box<str>`] into [`src`](Self::src)
    /// and initializes ['current'](Self::current) to 0
    pub fn init(src: Box<str>) -> Self {
        Self {
            src,
            start: 0,
            current: 0,
        }
    }

    /// Advances source current by one and returns previous char as u8
    fn advance(&mut self) -> u8 {
        self.current += 1;
        return self.src.as_bytes()[self.current - 1];
    }

    /// Shortcut function to determine if given byte is a digit
    /// Wrapped to avoid mismatch in definition
    fn is_digit(&self, char: u8) -> bool {
        matches!(char, b'0'..=b'9')
    }

    /// Used to determine the span of a literal
    /// Allows usage of letters, underscores and digits
    fn is_alphanumeric(&self, c: u8) -> bool {
        match c {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => true,
            _ => false,
        }
    }

    /// This function, as of now, is a VERY rough prototype
    /// Further development will add a proper Trie
    fn identifier(&mut self) -> LangToken {
        // Loop to grasp range of identifier
        while let Some(c) = self.peek() {
            if self.is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }
        let span = Span::start_end(self.start, self.current);
        LangToken::new(self.identifier_type(&span), span)
    }

    fn identifier_type(&self, span: &Span) -> TokenType {
        let ttype = match self.src.as_bytes()[span.start()] {
            b't' => self.check_keyword(self.current, 3, "rue", TokenType::True),
            b'f' => self.check_keyword(self.current, 4, "alse", TokenType::False),
            b'n' => self.check_keyword(self.current, 2, "il", TokenType::NIL),
            _ => TokenType::Identifier,
        };
        ttype
    }

    /// Matches remainder of literal to potential keyword
    /// Example (checking for keyword "true"):
    /// let type:TokenType = self.check_keyword(
    ///     span.start()+1,
    ///     span.len(),
    ///     "rue",
    ///     TokenType::True
    /// );
    fn check_keyword(
        &self,
        start: usize,
        len: usize,
        rest: &str,
        res_type: TokenType,
    ) -> TokenType {
        res_type
    }

    // Skips all digits until current is no longer a digit
    // Does NOT skip dots
    fn skip_digits(&mut self) {
        while let Some(c) = self.peek() {
            if !self.is_digit(c) {
                break;
            }
            self.advance();
        }
    }
    /// Consumes and concatenates digits and ',' starting from [`self.start`]
    /// And returns [`LangToken`] with type [`TokenType::Num`]
    ///
    /// Returns [`LangError`] if invalid Syntax
    fn number(&mut self) -> LangToken {
        // All digits before '.'
        self.skip_digits();

        if matches!(self.peek(), Some(b'.'))
            && matches!(self.peek_next(), Some(c) if self.is_digit(c))
        {
            self.advance(); // Skips the '.'
            self.skip_digits();
        }
        LangToken::new(TokenType::Num, Span::start_end(self.start, self.current))
    }

    /// Emits the next [`LangToken`] from the input stream.
    ///
    /// This is the core scanning function of the lexer. It:
    ///
    /// 1. Skips ASCII whitespace via [`skip_whitespace`]
    /// 2. Checks for end-of-input and emits [`TokenType::EOF`]
    /// 3. Consumes the next byte and matches it to a token
    ///
    /// Returns [`LangError`] if an unexpected byte is encountered.
    ///
    /// Since the lexer operates on raw `u8` bytes (ASCII-oriented),
    /// any unsupported or non-ASCII input will result in an error.
    pub fn emit_token(&mut self) -> Result<LangToken, LangError> {
        self.skip_whitespace();
        self.start = self.current;

        // End of file reached, output EOF
        if self.is_at_end() {
            return Ok(self.make_token(TokenType::EOF));
        }

        let c = self.advance();

        // Wrapper function used to avoid mismatch
        // Needs to be before identifier function
        // so that no name starts with number
        if self.is_digit(c) {
            return Ok(self.number());
        }

        // Usually allows letters, underscores AND digits
        // However, digits are already skipped by self.is_digit() call
        if self.is_alphanumeric(c) {
            return Ok(self.identifier());
        }

        let res = match c {
            // Arithmetic operators
            b'+' => self.make_token(TokenType::Plus),
            b'-' => self.make_token(TokenType::Minus),
            b'*' => self.make_token(TokenType::Star),
            b'/' => self.make_token(TokenType::Slash),

            // Boolean Operators
            b'!' => self.two_char_token('=', TokenType::BangEq, TokenType::Bang),
            b'<' => self.two_char_token('=', TokenType::LthenEq, TokenType::Lthen),
            b'>' => self.two_char_token('=', TokenType::GthenEq, TokenType::Gthen),
            b'=' => self.two_char_token('=', TokenType::EqEq, TokenType::Eq),

            // Grouping
            b'(' => self.make_token(TokenType::LParen),
            b')' => self.make_token(TokenType::RParen),
            c => {
                return Err(LangError::compile(
                    Span::start_len(self.current, 1),
                    format!("Invalid character: '{}'", c as char),
                ));
            }
        };
        return Ok(res);
    }

    /// Utility function used to ouput either a or b
    /// Depending on whether the next character matches the expected
    fn two_char_token(
        &mut self,
        expected: char,
        matched: TokenType,
        single: TokenType,
    ) -> LangToken {
        if self.match_current(expected) {
            self.make_token(matched)
        } else {
            self.make_token(single)
        }
    }

    /// Outputs a LangToken with type of ttype param.
    /// Token source pointer is inferred via Lexer.current
    fn make_token(&self, ttype: TokenType) -> LangToken {
        // We subtract from current because it will already
        // be on the next token
        LangToken::new(ttype, Span::start_end(self.start, self.current))
    }

    /// returns true if current is EOF char
    fn is_at_end(&self) -> bool {
        return self.current + 1 > self.src.len() || self.src.as_bytes()[self.current] == b'\0';
    }

    /// Outputs true and advances only if current char matches expected
    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.src.as_bytes()[self.current] != expected as u8 {
            return false;
        }
        self.current += 1;
        true
    }

    /// Returns u8 char of current pointer position
    fn peek(&self) -> Option<u8> {
        return self.src.as_bytes().get(self.current).copied();
    }

    /// Returns u8 char of (current pointer position + 1)
    fn peek_next(&self) -> Option<u8> {
        return self.src.as_bytes().get(self.current + 1).copied();
    }

    /// Skips only ASCII-native whitespace.
    ///
    /// This includes:
    /// - `' '` (space)
    /// - `'\t'` (tab)
    /// - `'\x0B'` (vertical tab)
    /// - `'\x0C'` (form feed)
    ///
    /// # Notes
    /// - Unicode whitespace is **not supported**
    /// - Encountering it will result in an *Invalid Token* error
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                // Might need to add NL Output soon
                b'\n' | b'\r' => self.current += 1,
                // \xOB -> Vertical Tab \xOC -> Form Feed
                b' ' | b'\t' | b'\x0B' | b'\x0C' => self.current += 1,
                _ => {
                    return;
                }
            }
        }
    }
}
