mod tokenizer;
pub mod parser_types;
use crate::parser::parser_types::*;
use crate::parser::tokenizer::Tokenizer;

#[derive(Debug)]
pub struct Parser<'a,I>
where I: Iterator<Item = char>,
{
    tokenizer: Tokenizer<'a,I>,
    current: Option<Token>,
    prev: Option<Token>,
}
/*
` for chars
' mutability op (lifetime if needed goes before, & goes after)
left (\: type:default:argname, ::arg2 -> {}) right
\ type:default:argname, ::arg -> rettype {}
then => else !> and match # only

struct:name:<T> [']{
  name:type:default;
}
trait:name:[<T>] [']{
  name:type;
}
enum:name:[<T>] [']{
  name:type;
}

Impl
<T>:[type,names]:structname [']{
  name = value;
}

Scope
[type] [']{
    [type][:]varname = value;
    <- varname;
}
[type] {
    [type]:varname2 = varname;
    <- varname;
    [type]:varname = value;
}
[type] '{
    [type][:]varname = value;
    varname
}

Match
val [type] [']{
    Pattern[,][cond] -> val+2;
    !> val-2;
}

Sets [']{ val1, val2, val3 }
Hashmap [']{ key1: val1, key2: val2, key3: val3 }
Arrays ['][val1, val2, val3]

\& makes it so that you can have multiple mutable refs?
but dereference becomes the function defined and returns an option?
You cant read or write to the value if you dont own it except by using this if defined?
Is defined at use site of mutable types?
Possibly mutable structs define a signature for it?
Will be used instead of unsafe?

I want to have raw pointer writing for embedded
and IO and whatnot passed in via the node definition
and then you can define mutable IO and monadic pure IO and pass them in.
But im not sure how this is going to work completely

<@ is value to stream/actor
@ is open/run stream/actor on node
@@ is same but on current node
@> is value from stream/actor
@>> untilcond, fallback TTL(int)
>>> while ->
>>| continue
>>! break

:name = 5;
`int:name2 = 6;

"#!" "#@" <- node config enclosers
doubles as shebang for interpreted mode
*/
impl<'a,I> Parser<'a,I>
where I: Iterator<Item = char>,
{
    pub fn new_tokenizer(input:I) -> Tokenizer<'a,I> {
        let settings  = Box::new(TokenizerSettings {
            blockcomstart: "#^",
            blockcomend: "#$",
            linecom: "#",
            ops: &[
                "=", "+", "-", "/", "%", "//", "|", "^", "++",
                ">>", "<<", "!", "||", "&&", "..",
                "!=", "==", "<=", ">=",
                "-=", "+=", "*=", "/=", "&=", "|=", "%=", "//=", "^=", "++=",
                "\\", "\\:", "...", "->", "<-", ">>=", "|>", "<|", "?",
                "'", "&", "*", "\\&",
                "=>", "!>",
                ">>>", ">>|", ">>!",
                "<@", "@", "@@", "@>", "@>>",
                ":", ".", ",", ";",
            ],
            enclosers: &[("(", ")"), ("[", "]"), ("{", "}"), ("<", ">"), ("#<", ">"), ("#@", "@#")],
            charop: "`",
            templop: "\"",
            interstart: "$[",
            interend: "]",
            escape_char: '\\',
            capture_comments: false,
        });
        Tokenizer::new(input, Box::leak(settings))
    }
    pub fn new(input:I) -> Parser<'a,I> {
        let mut p = Parser {
            tokenizer:Self::new_tokenizer(input),
            current: None,
            prev: None,
        };
        // NOTE: populate first value before returning new parser
        p.eat();
        p
    }
    fn at(&self) -> Option<Token> {
        self.current.clone()
    }
    fn eat(&mut self) -> Option<Token> {
        let out = self.current.clone();
        self.prev = out.clone();
        self.current = self.tokenizer.next();
        out
    }
    fn skip(&mut self) {
        self.eat();
    }
    fn prev(&self) -> Option<Token> {
        self.prev.clone()
    }
    fn not_eof(&self) -> bool {
        ! matches!(self.at(), Some(Token::Eof) | None)
    }

    pub fn parse_program(&mut self) -> ParseResult {
        let mut program = Vec::new();
        while self.not_eof() {
            program.push(self.parse_stmt()?.into());
        }
        Ok(Stmt::Module { body: program, ttype: Lexeme::Module})
    }

    pub fn parse_stmt(&mut self) -> ParseResult {
        self.parse_expr()
    }
    pub fn parse_expr(&mut self) -> ParseResult {
        self.parse_binary_expr()
    }
    pub fn parse_binary_expr(&mut self) -> ParseResult {
        self.parse_additive_expr()
    }
    pub fn parse_additive_expr(&mut self) -> ParseResult {
        let mut left = self.parse_multiplicative_expr()?;
        while let Some(Token::Op(coin)) = self.at() {
            let coin = coin.clone();
            if !matches!(coin.val.as_str(), "+" | "-") {
                break;
            }
            self.skip();
            let ttype = match coin.val.as_str() {
                "+" => Lexeme::Add,
                _ => Lexeme::Sub,
            };
            let right = self.parse_multiplicative_expr()?;
            left = Stmt::BinaryExpr{ ttype,coin,l:left.into(),r:right.into()};
        }
        Ok(left)
    }
    pub fn parse_multiplicative_expr(&mut self) -> ParseResult {
        let mut left = self.parse_primary_expr()?;
        while let Some(Token::Op(coin)) = self.at() {
            let coin = coin.clone();
            if !matches!(coin.val.as_str(), "*" | "/" | "%") {
                break;
            }
            self.skip();
            let ttype = match coin.val.as_str() {
                "*" => Lexeme::Mult,
                "/" => Lexeme::Div,
                _ => Lexeme::Mod,
            };
            let right = self.parse_primary_expr()?;
            left = Stmt::BinaryExpr{ ttype,coin,l:left.into(),r:right.into()};
        }
        Ok(left)
    }
    pub fn parse_primary_expr(&mut self) -> ParseResult {
        match self.at() {
            Some(Token::Identifier(_)) => self.parse_ident(),
            Some(Token::Numeric(_)) => self.parse_numeric(),
            Some(Token::Op(coin)) if coin.val.as_str() == "(" => {
                let coin = coin.clone();
                self.skip();
                let val = self.parse_expr();
                match self.eat() {
                    Some(Token::Op(c)) if c.val.as_str() == ")" => {},
                    _ => {
                        return Err(ParseError::UnmatchedEncloser(Token::Op(coin)))
                    },
                }
                val
            },
            _ => Err(ParseError::InvalidExpression(self.at().unwrap_or(Token::Eof).clone())),
        }
    }
    pub fn parse_ident(&mut self) -> ParseResult {
        let Some(Token::Identifier(coin)) = self.eat() else { return Err(ParseError::InvalidIdent(self.prev().unwrap_or(Token::Eof))) };
        Ok(Stmt::Identifier{ ttype:Lexeme::Ident,coin:coin.clone(),val:coin.val.clone().into()})
    }
    pub fn parse_numeric(&mut self) -> ParseResult {
        // coin is coin.val (which is a string) and coin.pos
        // check if the string parses to float it or hex
        let Some(Token::Numeric(coin)) = self.eat() else { return Err(ParseError::InvalidNumber(self.prev().unwrap_or(Token::Eof))) };
        let value = &coin.val; // Assuming `coin.val` is the string representation of the number.
        if let Ok(val) = value.parse::<u64>() {
            Ok(Stmt::IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val})
        } else if let Ok(val) = value.parse::<f64>() {
            Ok(Stmt::FloatLiteral{ ttype:Lexeme::Float,coin:coin.clone(),val})
        } else if let Some(stripped) = value.strip_prefix("0x") {
            let Ok(val) = u64::from_str_radix(stripped, 16) else {
                return Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
            };
            Ok(Stmt::IntLiteral{ ttype:Lexeme::Int,coin:coin.clone(),val})
        } else {
            Err(ParseError::InvalidNumber(Token::Numeric(coin.clone())))
        }
    }
}
