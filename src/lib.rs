/// This enum represents a list of all valid tokens that procedural macros can parse.
/// This is an abstraction over the system that `proc_macro` uses in it's TokenTree.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// Represents an identifier.
    Ident(String),
    /// Represents a valid integer in normal or hexadecimal form.
    Integer(i128),
    /// Represents a valid float.
    Float(f64),
    /// Represents a byte character. `b'a'`
    ByteChar(char),
    /// Represents a normal character. `'a'`
    Char(char),
    /// Represents a byte string. `b"hello"`
    ByteString(String),
    /// Represents a normal string. `"hello"`
    String(String),
    /// Represents `+`.
    Plus,
    /// Represents `-`.
    Minus,
    /// Represents `/`.
    Slash,
    /// Represents `*`.
    Star,
    /// Represents `@`.
    At,
    /// Represents `&`.
    Ampersand,
    /// Represents `;`.
    Semi,
    /// Represents `:`.
    Colon,
    /// Represents `>` .
    GreaterThan,
    /// Represents `<`.
    LessThan,
    /// Reprsents `{`.
    OpenBrace,
    /// Represents `}`.
    CloseBrace,
    /// Represents `[`.
    OpenBracket,
    /// Represents `]`.
    CloseBracket,
    // Represents `(`.
    OpenParen,
    /// Represents `)`.
    CloseParen,
    /// Represents `,`.
    Comma,
    /// Represents `'`.
    SingleQuote,
    /// Represents `"`.
    DoubleQuote,
    /// Represents `!`.
    Bang,
    /// Represents `?`.
    Question,
    /// Represents `.`.
    Dot,
    /// Represents `~`.
    Tilde,
    /// Represents `%`.
    Percent,
    /// Represents `^`.
    Caret,
    /// Represents `|`.
    Pipe,
    /// Represents `#`.
    Hash,
    /// Represents `$`.
    Dollar,
    /// Represents `=`,
    Equal,
    /// Represents no token.
    None,
    /// Represents an uncategorizable literal.
    Literal(String),
}

/// This is an equivalent to the `Token` type with a span attached. Use `SpannedToken::span()` to retrieve it's span, and `SpannedToken::token()` to retreieve it's token.
/// Note that this type is read-only, you ideally should not mutate it.
#[derive(Clone, Debug)]
pub struct SpannedToken {
    token: Token,
    span: proc_macro2::Span,
}

impl SpannedToken {
    /// This function allows you to get the internal `Token` of a `SpannedToken`.
    /// ```
    /// let to_parse: proc_macro2::TokenStream = r#"
    ///      fn main() {
    ///          println!("Hello world!");
    ///      }
    ///  "#
    ///  .parse()
    ///  .expect("infallible");
    ///
    /// let mut stream: token_stream2::TokenStream = to_parse.into();
    /// assert!(stream.peek(2).unwrap().token() == &token_stream2::Token::OpenParen);
    /// assert!(stream.peek(3).unwrap().token() == &token_stream2::Token::CloseParen);
    /// ```
    pub fn token(&self) -> &Token {
        &self.token
    }
    /// This function allows you to get the span of a SpannedToken.
    /// An example was failed to be provided for this function, sorry.
    pub fn span(&self) -> &proc_macro2::Span {
        &self.span
    }
}

#[derive(Clone, Debug)]
pub struct TokenStream {
    tokens: Vec<SpannedToken>,
    iter_ptr: usize,
}

impl Iterator for TokenStream {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_ptr += 1;
        self.tokens.get(self.iter_ptr).cloned()
    }
}

impl TokenStream {
    /// This function allows you to peek ahead in the tokenstream's iterator.
    /// Useful for quickly parsing what's next.
    /// ```
    /// let to_parse: proc_macro2::TokenStream = r#"
    ///      fn main() {
    ///          println!("Hello world!");
    ///      }
    ///  "#
    ///  .parse()
    ///  .expect("infallible");
    ///
    /// let mut stream: token_stream2::TokenStream = to_parse.into();
    /// assert!(stream.peek(2).unwrap().token() == &token_stream2::Token::OpenParen);
    /// assert!(stream.peek(3).unwrap().token() == &token_stream2::Token::CloseParen);
    /// ```
    pub fn peek(&mut self, ahead: usize) -> Option<SpannedToken> {
        self.tokens.get(self.iter_ptr + ahead).cloned()
    }
}
impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(value: proc_macro2::TokenStream) -> Self {
        recursive_convert(value)
    }
}

// This function recursively transforms a `proc_macro2::TokenStream` into a `token_stream2::TokenStream`.
fn recursive_convert(tokens: proc_macro2::TokenStream) -> TokenStream {
    let mut tokens_output = vec![];
    let tokens = tokens.into_iter();

    for token in tokens {
        if let proc_macro2::TokenTree::Group(group) = token {
            tokens_output.push(SpannedToken {
                token: match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => Token::OpenParen,
                    proc_macro2::Delimiter::Brace => Token::OpenBrace,
                    proc_macro2::Delimiter::Bracket => Token::OpenBracket,
                    proc_macro2::Delimiter::None => Token::None,
                },
                span: group.span(),
            });
            tokens_output.extend(recursive_convert(group.stream()).tokens);
            tokens_output.push(SpannedToken {
                token: match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => Token::CloseParen,
                    proc_macro2::Delimiter::Brace => Token::CloseBrace,
                    proc_macro2::Delimiter::Bracket => Token::CloseBracket,
                    proc_macro2::Delimiter::None => Token::None,
                },
                span: group.span(),
            });
        } else {
            match token {
                proc_macro2::TokenTree::Group(..) => unreachable!(),
                proc_macro2::TokenTree::Ident(ident) => {
                    tokens_output.push(SpannedToken {
                        token: Token::Ident(ident.to_string()),
                        span: ident.span(),
                    });
                }
                proc_macro2::TokenTree::Punct(punct) => {
                    let tok = match punct.as_char() {
                        '+' => Token::Plus,
                        '-' => Token::Minus,
                        '>' => Token::GreaterThan,
                        '<' => Token::LessThan,
                        '@' => Token::At,
                        '/' => Token::Slash,
                        '*' => Token::Star,
                        '&' => Token::Ampersand,
                        ';' => Token::Semi,
                        ':' => Token::Colon,
                        '"' => Token::DoubleQuote,
                        '\'' => Token::SingleQuote,
                        '?' => Token::Question,
                        '!' => Token::Bang,
                        ',' => Token::Comma,
                        '.' => Token::Dot,
                        '~' => Token::Tilde,
                        '%' => Token::Percent,
                        '^' => Token::Caret,
                        '|' => Token::Pipe,
                        '#' => Token::Hash,
                        '$' => Token::Dollar,
                        '=' => Token::Equal,
                        _ => unreachable!(),
                    };
                    tokens_output.push(SpannedToken {
                        token: tok,
                        span: punct.span(),
                    });
                }
                proc_macro2::TokenTree::Literal(literal) => {
                    let mut panic = true;
                    if let Ok(int_value) = literal.to_string().parse::<i128>() {
                        panic = false;
                        tokens_output.push(SpannedToken {
                            token: Token::Integer(int_value),
                            span: literal.span(),
                        });
                    }

                    if let Ok(float_value) = literal.to_string().parse::<f64>() {
                        panic = false;
                        tokens_output.push(SpannedToken {
                            token: Token::Float(float_value),
                            span: literal.span(),
                        });
                    }
                    let str_value = literal.to_string();
                    if str_value.starts_with("0x") {
                        if let Ok(int_value) =
                            i128::from_str_radix(str_value.trim_start_matches("0x"), 16)
                        {
                            panic = false;
                            tokens_output.push(SpannedToken {
                                token: Token::Integer(int_value),
                                span: literal.span(),
                            });
                        }
                    }
                    if str_value.starts_with("b'") && str_value.ends_with('\'') {
                        panic = false;
                        let as_char = str_value
                            .trim_start_matches("b\'")
                            .trim_end_matches('\'')
                            .parse::<char>()
                            .expect("infallible - guaranteed to be a char");
                        tokens_output.push(SpannedToken {
                            token: Token::ByteChar(as_char),
                            span: literal.span(),
                        });
                    }
                    if str_value.starts_with('\'') && str_value.ends_with('\'') {
                        panic = false;
                        let as_char = str_value
                            .trim_matches('\'')
                            .trim()
                            .parse::<char>()
                            .expect("infallible - guaranteed to be a char");
                        tokens_output.push(SpannedToken {
                            token: Token::Char(as_char),
                            span: literal.span(),
                        });
                    }
                    if str_value.starts_with('"') && str_value.ends_with('"') {
                        panic = false;
                        tokens_output.push(SpannedToken {
                            token: Token::String(str_value.trim_matches('"').to_string()),
                            span: literal.span(),
                        });
                    }
                    if str_value.starts_with("b\"") && str_value.ends_with('"') {
                        panic = false;
                        tokens_output.push(SpannedToken {
                            token: Token::String(
                                str_value
                                    .trim_end_matches('"')
                                    .trim_start_matches("b\"")
                                    .to_string(),
                            ),
                            span: literal.span(),
                        });
                    }
                    if panic {
                        tokens_output.push(SpannedToken {
                            token: Token::Literal(str_value),
                            span: literal.span(),
                        });
                    }
                }
            }
        }
    }

    TokenStream {
        tokens: tokens_output,
        iter_ptr: 0,
    }
}
