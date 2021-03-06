use logos_derive::Logos;
use tests::assert_lex;

mod crunch {
    use super::*;

    #[derive(Logos, Debug, Clone, Copy, PartialEq)]
    enum Token {
        #[end]
        End,
        #[error]
        Error,
        #[token = "else"]
        Else,
        #[token = "exposed"]
        Exposed,
        #[regex = "[^ \t\n\r\"\'!@#$%\\^&*()-+=,.<>/?;:\\[\\]{}\\\\|`~]+"]
        Ident,
    }

    #[test]
    fn crunch() {
        assert_lex(
            "exposed_function",
            &[
                (Token::Ident, "exposed_function", 0..16),
            ],
        );
    }
}

mod numbers {
    use super::*;

    #[derive(Logos, Debug, Clone, Copy, PartialEq)]
    enum Token {
        #[end]
        End,
        #[error]
        Error,
        #[regex = r"[0-9][0-9_]*"]
        LiteralUnsignedNumber,
        #[regex = r"[0-9][0-9_]*\.[0-9][0-9_]*[TGMKkmupfa]"]
        LiteralRealNumberDotScaleChar,
        #[regex = r"[0-9][0-9_]*\.[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*"]
        LiteralRealNumberDotExp,
        #[regex = r"[0-9][0-9_]*[TGMKkmupfa]"]
        LiteralRealNumberScaleChar,
        #[regex = r"[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*"]
        LiteralRealNumberExp,
        #[regex = r"[0-9][0-9_]*\.[0-9][0-9_]*"]
        LiteralRealNumberDot,
    }

    #[test]
    fn numbers() {
        assert_lex(
            "42.42 42 777777K 90e+8 42.42m 77.77e-29",
            &[
                (Token::LiteralRealNumberDot, "42.42", 0..5),
                (Token::LiteralUnsignedNumber, "42", 6..8),
                (Token::LiteralRealNumberScaleChar, "777777K", 9..16),
                (Token::LiteralRealNumberExp, "90e+8", 17..22),
                (Token::LiteralRealNumberDotScaleChar, "42.42m", 23..29),
                (Token::LiteralRealNumberDotExp, "77.77e-29", 30..39),
            ]
        )
    }
}

mod benches {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Logos)]
    pub enum Token {
        #[error]
        InvalidToken,

        #[end]
        EndOfProgram,

        #[regex = "[a-zA-Z_$][a-zA-Z0-9_$]*"]
        Identifier,

        #[regex = r#""([^"\\]|\\t|\\u|\\n|\\")*""#]
        String,

        #[token = "private"]
        Private,

        #[token = "primitive"]
        Primitive,

        #[token = "protected"]
        Protected,

        #[token = "in"]
        In,

        #[token = "instanceof"]
        Instanceof,

        #[token = "."]
        Accessor,

        #[token = "..."]
        Ellipsis,

        #[token = "("]
        ParenOpen,

        #[token = ")"]
        ParenClose,

        #[token = "{"]
        BraceOpen,

        #[token = "}"]
        BraceClose,

        #[token = "+"]
        OpAddition,

        #[token = "++"]
        OpIncrement,

        #[token = "="]
        OpAssign,

        #[token = "=="]
        OpEquality,

        #[token = "==="]
        OpStrictEquality,

        #[token = "=>"]
        FatArrow,
    }

    #[test]
    fn idents() {
        static IDENTIFIERS: &str = "It was the year when they finally immanentized the Eschaton";

        assert_lex(
            IDENTIFIERS,
            &[
                (Token::Identifier, "It", 0..2),
                (Token::Identifier, "was", 3..6),
                (Token::Identifier, "the", 7..10),
                (Token::Identifier, "year", 11..15),
                (Token::Identifier, "when", 16..20),
                (Token::Identifier, "they", 21..25),
                (Token::Identifier, "finally", 26..33),
                (Token::Identifier, "immanentized", 34..46),
                (Token::Identifier, "the", 47..50),
                (Token::Identifier, "Eschaton", 51..59),
            ]
        )
    }

    #[test]
    fn keywords_and_punctators() {
        static SOURCE: &str = "foobar(protected primitive private instanceof in) { + ++ = == === => }";

        assert_lex(
            SOURCE,
            &[
                (Token::Identifier, "foobar", 0..6),
                (Token::ParenOpen, "(", 6..7),
                (Token::Protected, "protected", 7..16),
                (Token::Primitive, "primitive", 17..26),
                (Token::Private, "private", 27..34),
                (Token::Instanceof, "instanceof", 35..45),
                (Token::In, "in", 46..48),
                (Token::ParenClose, ")", 48..49),
                (Token::BraceOpen, "{", 50..51),
                (Token::OpAddition, "+", 52..53),
                (Token::OpIncrement, "++", 54..56),
                (Token::OpAssign, "=", 57..58),
                (Token::OpEquality, "==", 59..61),
                (Token::OpStrictEquality, "===", 62..65),
                (Token::FatArrow, "=>", 66..68),
                (Token::BraceClose, "}", 69..70),
            ]
        )
    }

    #[test]
    fn strings() {
        static STRINGS: &str = r#""tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree.""#;

        assert_lex(
            STRINGS,
            &[
                (Token::String, r#""tree""#, 0..6),
                (Token::String, r#""to""#, 7..11),
                (Token::String, r#""a""#, 12..15),
                (Token::String, r#""graph""#, 16..23),
                (Token::String, r#""that can""#, 24..34),
                (Token::String, r#""more adequately represent""#, 35..62),
                (Token::String, r#""loops and arbitrary state jumps""#, 63..96),
                (Token::String, r#""with\"\"\"out""#, 97..112),
                (Token::String, r#""the\n\n\n\n\n""#, 113..128),
                (Token::String, r#""expl\"\"\"osive""#, 129..146),
                (Token::String, r#""nature\"""#, 147..157),
                (Token::String, r#""of trying to build up all possible permutations in a tree.""#, 157..217),
            ]
        )
    }
}