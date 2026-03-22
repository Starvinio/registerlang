use crate::{LangError, LangToken, TokenType};

/// Byte-based lexical scanner
/// 
/// Consumes a source string and produces a stream
/// of [`LangToken`] via [`emit_token`](Self::emit_token)
///
/// # Design Choices:
/// - Operates on raw bytes for performance
/// - Assumes ASCII-oriented output
/// - Does not handle unicode
pub struct Scanner {

    /// Source code being scanned
    /// Stored as [`Box<>`] to:
    /// - Display immutability
    /// - Avoid structure lifetime
    src: Box<str>,

    // Current position (byte index) in the source
    // TODO: Update this to a Span 
    ptr: usize
}

impl Scanner {

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

    /// Consumes and concatenates digits and ',' starting from [`start_ptr`]
    /// And returns [`LangToken`] with type [`TokenType::Num`]
    ///
    /// Returns [`LangError`] if invalid Syntax
    fn number(&mut self, start_ptr: usize) -> LangToken {
        while matches!(self.src.as_bytes()[self.ptr], b'0'..=b'9') {
            
        }

        LangToken { ttype: TokenType::Num(1.0), tptr: start_ptr as u32 }
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
    /// Since the scanner operates on raw `u8` bytes (ASCII-oriented),
    /// any unsupported or non-ASCII input will result in an error.
    pub fn emit_token(&mut self) -> Result<LangToken, LangError> {
        self.skip_whitespace();

        if self.is_at_end() {return Ok(self.make_token(TokenType::EOF))} 
        
        let c = self.advance();
        let res = match c {
            b'0'..=b'9' => self.number(self.ptr-1),
            b'+' => self.make_token(TokenType::Plus),
            b'-' => self.make_token(TokenType::Minus),
            b'*' => self.make_token(TokenType::Star),
            b'/' => self.make_token(TokenType::Slash),
            b'<' => self.make_token(TokenType::Lthen),
            b'>' => self.make_token(TokenType::Gthen),
            b'(' => self.make_token(TokenType::LParen),
            b')' => self.make_token(TokenType::RParen),
            b'=' => {
                if self.match_current('=') {
                    self.make_token(TokenType::EqEquals)
                } else {
                    self.make_token(TokenType::Equals)
                }
            }
            t => return Err(LangError::compile(self.ptr as u32, format!("Unexpected Token: '{}'", t as char)))
        };
        return Ok(res)
    }

    /// Outputs a LangToken with type of ttype param.
    /// Token source pointer is inferred via Scanner.ptr
    fn make_token(&self, ttype: TokenType) -> LangToken {
        LangToken { ttype, tptr: self.ptr as u32 }
    }
    
    /// returns true if current is EOF char
    fn is_at_end(&self) -> bool {
        return self.src.as_bytes()[self.ptr] == b'\0'
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
    fn peek(&self) -> u8 {
        return self.src.as_bytes()[self.ptr]
    }

    /// Returns u8 char of (current pointer position + 1)
    fn peek_next(&self) -> u8 {
        return self.src.as_bytes()[self.ptr+1]
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
        loop {
            let c = self.peek();
            match c {
                b'\n' | b'\r' => todo!("Add sum farkin newline detector"),
                // \xOB -> Vertical Tab \xOC -> Form Feed
                b' ' | b'\t' | b'\x0B' | b'\x0C' => self.ptr+=1,
                _ => return
            }
        }
    }
}
