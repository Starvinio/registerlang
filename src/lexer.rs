use crate::{LangError, LangToken, TokenType};

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
    src: Box<str>,

    // Current position (byte index) in the source
    // TODO: Update this to a Span 
    ptr: usize
}

impl Lexer {

    /// Init that loads [`Box<str>`] into [`src`](Self::src) 
    /// and initializes ['ptr'](Self::ptr) to 0
    pub fn init(src: Box<str>) -> Self {
        Self {
            src,
            ptr:0
        }
    }

    /// Advances source ptr by one and returns previous char as u8
    fn advance(&mut self) -> u8 {
        self.ptr += 1;
        return self.src.as_bytes()[self.ptr-1]
    } 

    /// Shortcut function to determine if given byte is a digit
    fn is_digit(&self, char:u8) -> bool {
        matches!(char, b'0'..=b'9') 
    }

    /// Consumes and concatenates digits and ',' starting from [`start_ptr`]
    /// And returns [`LangToken`] with type [`TokenType::Num`]
    ///
    /// Returns [`LangError`] if invalid Syntax
    fn number(&mut self, start_ptr: usize) -> Result<LangToken, LangError> {

        // All digits before '.'
        while let Some(c) = self.peek() {
            if !self.is_digit(c) {
                break
            }
            self.advance();
        }

        if matches!(self.peek(), Some(b'.')) && matches!(self.peek_next(), Some(c) if self.is_digit(c)) {
            self.advance();
            // All digits after '.'
            while matches!(self.src.as_bytes()[self.ptr], b'0'..=b'9') {
                self.advance();
            }
        }
        Ok(LangToken { ttype: TokenType::Num, tptr: start_ptr as u32 })
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

        // End of file reached, output EOF
        if self.is_at_end() {return Ok(self.make_token(TokenType::EOF))} 
        
        let c = self.advance();
        let res = match c {
            // Numbers
            b'0'..=b'9' => return self.number(self.ptr-1),

            // Arithmetic operators
            b'+' => self.make_token(TokenType::Plus),
            b'-' => self.make_token(TokenType::Minus),
            b'*' => self.make_token(TokenType::Star),
            b'/' => self.make_token(TokenType::Slash),

            // Boolean Operators
            b'!' => self.two_char_token('=', TokenType::BangEq, TokenType::Bang),
            b'<' => self.two_char_token('=', TokenType::Lthen, TokenType::LthenEq),
            b'>' => self.two_char_token('=', TokenType::Gthen, TokenType::GthenEq),
            b'=' => self.two_char_token('=', TokenType::EqEq, TokenType::Eq),

            // Grouping
            b'(' => self.make_token(TokenType::LParen),
            b')' => self.make_token(TokenType::RParen),
            t => return Err(LangError::compile(self.ptr as u32, format!("Invalid character: '{}'", t as char)))
        };
        return Ok(res)
    }

    pub fn token_stream(&mut self) -> Result<Vec<LangToken>, LangError> {
        let mut tokens = Vec::new();
        while self.ptr < self.src.len() && !self.is_at_end() {
            tokens.push(self.emit_token()?)
        }
        Ok(tokens)
    }

    /// Utility function used to ouput either a or b
    /// Depending on whether the next character matches the expected
    fn two_char_token(&mut self, expected: char, matched:TokenType, single:TokenType) -> LangToken {
        if self.match_current(expected) {
            self.make_token(matched)
        } else {
            self.make_token(single)
        }

    }

    /// Outputs a LangToken with type of ttype param.
    /// Token source pointer is inferred via Lexer.ptr
    fn make_token(&self, ttype: TokenType) -> LangToken {
        // We subtract from ptr because it will already
        // be on the next token
        LangToken { ttype, tptr: (self.ptr-1) as u32 }
    }
    
    /// returns true if current is EOF char
    fn is_at_end(&self) -> bool {
        return self.ptr+1 > self.src.len() || self.src.as_bytes()[self.ptr] == b'\0'
    }
    
    /// Outputs true and advances only if current char matches expected
    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() || 
            self.src.as_bytes()[self.ptr] != expected as u8 {
            return false
        }
        self.ptr+=1;
        true
    }

    /// Returns u8 char of current pointer position
    fn peek(&self) -> Option<u8> {
        return self.src.as_bytes().get(self.ptr).copied()
    }

    /// Returns u8 char of (current pointer position + 1)
    fn peek_next(&self) -> Option<u8> {
        return self.src.as_bytes().get(self.ptr + 1).copied()
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
                b'\n' | b'\r' => self.ptr+=1,
                // \xOB -> Vertical Tab \xOC -> Form Feed
                b' ' | b'\t' | b'\x0B' | b'\x0C' => self.ptr+=1,
                _ => return
            }
        }
        
    }
}
