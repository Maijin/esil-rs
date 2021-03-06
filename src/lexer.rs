use std::fmt::Debug;
use num::traits::Num;


const ESIL_INTERNAL_PREFIX: char = '$';

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Esil Opcodes
    EInterrupt,
    ECmp,
    ELt,
    EGt,
    EEq,
    EIf,
    ELsl,
    ELsr,
    ERor,
    ERol,
    EAnd,
    EOr,
    ENop,
    ENeg,
    EMul,
    EXor,
    EAdd,
    ESub,
    EDiv,
    EMod,
    EPoke(u8),
    EPeek(u8),
    EDump,
    EPop,
    ETodo,
    EGoto,
    EBreak,
    EClear,
    EDup,
    ETrap,
    // Esil Internal Vars
    IZero(u8),
    ICarry(u8),
    IParity(u8),
    IOverflow(u8),
    ISign(u8),
    IBorrow(u8),
    ISize(u8),
    IAddress(u8),
    // Esil Operands
    EConstant(u64),
    EIdentifier(String),
    // Invalid
    EInvalid,
    // Parser Instructions.
    PCopy(usize),
    PPop(usize),
    PSync,
}

pub trait Tokenize {
    type Token: Clone + Debug + PartialEq;
    fn tokenize<T: AsRef<str>>(esil: T) -> Vec<Self::Token>;
}

pub struct Tokenizer;

impl Tokenize for Tokenizer {
    type Token = Token;
    fn tokenize<T: AsRef<str>>(esil: T) -> Vec<Self::Token> {
        let mut tokens = Vec::new();
        for t in esil.as_ref().split(",").into_iter() {
            tokens.extend(
                match t {
                    "$" => vec![Token::EInterrupt],
                    "==" => vec![Token::ECmp],
                    "<" => vec![Token::ELt],
                    ">" => vec![Token::EGt],
                    "<=" => vec![Token::PCopy(2), Token::ELt, Token::PPop(2),
                    Token::ECmp, Token::EOr],
                    ">=" => vec![Token::PCopy(2), Token::EGt, Token::PPop(2),
                    Token::ECmp, Token::EOr],
                    "?{" => vec![Token::EIf],
                    "<<" => vec![Token::ELsl],
                    "<<=" => vec![Token::PCopy(2), Token::ELsl, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    ">>" => vec![Token::ELsr],
                    ">>=" => vec![Token::PCopy(2), Token::ELsr, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    ">>>" => vec![Token::ERor],
                    "<<<" => vec![Token::ERol],
                    "&" => vec![Token::EAnd],
                    "&=" => vec![Token::PCopy(2), Token::EAnd, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "}" => vec![Token::ENop],
                    "|" => vec![Token::EOr],
                    "|=" => vec![Token::PCopy(2), Token::EOr, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "!" => vec![Token::ENeg],
                    "!=" => vec![Token::PCopy(1), Token::ENeg, Token::EEq],
                    "=" => vec![Token::EEq],
                    "*" => vec![Token::EMul],
                    "*=" => vec![Token::PCopy(2), Token::EMul, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "^" => vec![Token::EXor],
                    "^=" => vec![Token::PCopy(2), Token::EXor, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "+" => vec![Token::EAdd],
                    "+=" => vec![Token::PCopy(2), Token::EAdd, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "++" => vec![Token::PPop(1), Token::EConstant(1), Token::EAdd],
                    "++=" => vec![Token::PCopy(1), Token::EConstant(1), Token::EAdd,
                    Token::PPop(1), Token::EEq],
                    "-" => vec![Token::ESub],
                    "-=" => vec![Token::PCopy(2), Token::ESub, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "--" => vec![Token::PPop(1), Token::EConstant(1), Token::ESub],
                    "--=" => vec![Token::PCopy(1), Token::EConstant(1), Token::ESub,
                    Token::PPop(1), Token::EEq],
                    "/" => vec![Token::EDiv],
                    "/=" => vec![Token::PCopy(2), Token::EDiv, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "%" => vec![Token::EMod],
                    "%=" => vec![Token::PCopy(2), Token::EMod, Token::PPop(1),
                    Token::EPop, Token::EEq],
                    "=[]" => vec![Token::EPoke(64)],
                    "=[1]" => vec![Token::EPoke(8)],
                    "=[2]" => vec![Token::EPoke(16)],
                    "=[4]" => vec![Token::EPoke(32)],
                    "=[8]" => vec![Token::EPoke(64)],
                    "|=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EOr,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "|=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EOr,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "|=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EOr,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "|=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EOr,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "|=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EOr,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "^=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EXor,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "^=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EXor,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "^=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EXor,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "^=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EXor,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "^=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EXor,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "&=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EAnd,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "&=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EAnd,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "&=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EAnd,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "&=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EAnd,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "&=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EAnd,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "+=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EAdd,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "+=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EAdd,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "+=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EAdd,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "+=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EAdd,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "+=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EAdd,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "-=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::ESub,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "-=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::ESub,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "-=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::ESub,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "-=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::ESub,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "-=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::ESub,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "%=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EMod,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "%=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EMod,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "%=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EMod,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "%=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EMod,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "%=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EMod,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "/=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EDiv,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "/=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EDiv,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "/=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EDiv,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "/=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EDiv,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "/=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EDiv,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "*=[]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EMul,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "*=[1]" => vec![Token::PCopy(1), Token::EPeek(8), Token::EMul,
                    Token::PPop(1), Token::EPop, Token::EPoke(8)],
                    "*=[2]" => vec![Token::PCopy(1), Token::EPeek(16), Token::EMul,
                    Token::PPop(1), Token::EPop, Token::EPoke(16)],
                    "*=[4]" => vec![Token::PCopy(1), Token::EPeek(32), Token::EMul,
                    Token::PPop(1), Token::EPop, Token::EPoke(32)],
                    "*=[8]" => vec![Token::PCopy(1), Token::EPeek(64), Token::EMul,
                    Token::PPop(1), Token::EPop, Token::EPoke(64)],
                    "++=[]" => vec![Token::PCopy(1), Token::EPeek(64),
                    Token::EConstant(1), Token::EAdd,
                    Token::PPop(1), Token::EPoke(64)],
                    "++=[1]" => vec![Token::PCopy(1), Token::EPeek(8),
                    Token::EConstant(1), Token::EAdd,
                    Token::PPop(1), Token::EPoke(8)],
                    "++=[2]" => vec![Token::PCopy(1), Token::EPeek(16),
                    Token::EConstant(1), Token::EAdd,
                    Token::PPop(1), Token::EPoke(16)],
                    "++=[4]" => vec![Token::PCopy(1), Token::EPeek(32),
                    Token::EConstant(1), Token::EAdd,
                    Token::PPop(1), Token::EPoke(32)],
                    "++=[8]" => vec![Token::PCopy(1), Token::EPeek(64),
                    Token::EConstant(1), Token::EAdd,
                    Token::PPop(1), Token::EPoke(64)],
                    "--=[]" => vec![Token::EConstant(1), Token::PPop(1),
                    Token::PCopy(1), Token::EPeek(64),
                    Token::ESub, Token::PPop(1),
                    Token::EPoke(64)],
                    "--=[1]" => vec![Token::EConstant(1), Token::PPop(1),
                    Token::PCopy(1), Token::EPeek(8),
                    Token::ESub, Token::PPop(1),
                    Token::EPoke(8)],
                    "--=[2]" => vec![Token::EConstant(1), Token::PPop(1),
                    Token::PCopy(1), Token::EPeek(16),
                    Token::ESub, Token::PPop(1),
                    Token::EPoke(16)],
                    "--=[4]" => vec![Token::EConstant(1), Token::PPop(1),
                    Token::PCopy(1), Token::EPeek(32),
                    Token::ESub, Token::PPop(1),
                    Token::EPoke(32)],
                    "--=[8]" => vec![Token::EConstant(1), Token::PPop(1),
                    Token::PCopy(1), Token::EPeek(64),
                    Token::ESub, Token::PPop(1),
                    Token::EPoke(64)],
                    "[]" => vec![Token::EPeek(64)],
                    "[*]" => vec![Token::EPeek(64)],
                    "=[*]" => vec![Token::EPoke(64)],
                    "[1]" => vec![Token::EPeek(8)],
                    "[2]" => vec![Token::EPeek(16)],
                    "[4]" => vec![Token::EPeek(32)],
                    "[8]" => vec![Token::EPeek(64)],
                    "STACK" => vec![Token::EDump],
                    "POP" => vec![Token::EPop],
                    "TODO" => vec![Token::ETodo],
                    "GOTO" => vec![Token::EGoto],
                    "BREAK" => vec![Token::EBreak],
                    "CLEAR" => vec![Token::EClear],
                    "DUP" => vec![Token::EDup],
                    "TRAP" => vec![Token::ETrap],
                    _   => {
                        // Handle internal vars
                        if Some(ESIL_INTERNAL_PREFIX) == t.chars().nth(0) {
                            let bit = if t.len() < 3 {
                                0
                            } else {
                                t[2..].parse::<u8>().unwrap_or(0)
                            };
                            match t.chars().nth(1).unwrap_or('\0') {
                                '$' => vec![Token::IAddress(bit)],
                                'z' => vec![Token::IZero(bit)],
                                'b' => vec![Token::IBorrow(bit)],
                                'c' => vec![Token::ICarry(bit)],
                                'p' => vec![Token::IParity(bit)],
                                'r' => vec![Token::ISize(bit)],
                                'o' => vec![Token::IOverflow(bit)],
                                's' => vec![Token::ISign(bit)],
                                _ => vec![Token::EInvalid],
                            }
                        } else if let Ok(v) = Num::from_str_radix(t.trim_left_matches("0x"), 16) {
                            vec![Token::EConstant(v)]
                        } else if let Ok(v) = t.parse::<u64>() {
                            vec![Token::EConstant(v)]
                        } else {
                            // Just returns it as an identifier. It is upto the
                            // parser to decide if it is a valid token.
                            vec![Token::EIdentifier(t.to_owned())]
                        }
                    }
                });
        }
        tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn esil_basic() {
        let op = vec![Token::EAdd];
        assert_eq!(op[0], Tokenizer::tokenize("+")[0]);
    }
}
