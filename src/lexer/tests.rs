use crate::{
    token::*,
    span::*
};

use super::*;

use TokenType::*;

#[test]
fn test_next_token() {
    let input = r#"
let a = 10;
let b = 3.14;
let c = a + b * 2;

let name = "Alice";
let greeting = "Hello, " + name;

let ok = true;
let nope = false;
let nothing = null;

print(a, b, c, greeting, ok, nope, nothing);
"#;
    let test_cases = vec![
        make_token(NewLine, "\n", Pos::new(0, 1, 1), 1),

        // let a = 10;
        make_token(Let, "let", Pos::new(1, 2, 1), 3),
        make_token(Identifier, "a", Pos::new(5, 2, 5), 1),
        make_token(Assign, "=", Pos::new(7, 2, 7), 1),
        make_token(IntNumber, "10", Pos::new(9, 2, 9), 2),
        make_token(Semicolon, ";", Pos::new(11, 2, 11), 1),
        make_token(NewLine, "\n", Pos::new(12, 2, 12), 1),

        // let b = 3.14;
        make_token(Let, "let", Pos::new(13, 3, 1), 3),
        make_token(Identifier, "b", Pos::new(17, 3, 5), 1),
        make_token(Assign, "=", Pos::new(19, 3, 7), 1),
        make_token(FloatNumber, "3.14", Pos::new(21, 3, 9), 4),
        make_token(Semicolon, ";", Pos::new(25, 3, 13), 1),
        make_token(NewLine, "\n", Pos::new(26, 3, 14), 1),

        // let c = a + b * 2;
        make_token(Let, "let", Pos::new(27, 4, 1), 3),
        make_token(Identifier, "c", Pos::new(31, 4, 5), 1),
        make_token(Assign, "=", Pos::new(33, 4, 7), 1),
        make_token(Identifier, "a", Pos::new(35, 4, 9), 1),
        make_token(Plus, "+", Pos::new(37, 4, 11), 1),
        make_token(Identifier, "b", Pos::new(39, 4, 13), 1),
        make_token(Mult, "*", Pos::new(41, 4, 15), 1),
        make_token(IntNumber, "2", Pos::new(43, 4, 17), 1),
        make_token(Semicolon, ";", Pos::new(44, 4, 18), 1),
        make_token(NewLine, "\n", Pos::new(45, 4, 19), 1),
        make_token(NewLine, "\n", Pos::new(46, 5, 1), 1),

        // let name = "Alice";
        make_token(Let, "let", Pos::new(47, 6, 1), 3),
        make_token(Identifier, "name", Pos::new(51, 6, 5), 4),
        make_token(Assign, "=", Pos::new(56, 6, 10), 1),
        make_token(String, "\"Alice\"", Pos::new(58, 6, 12), 7),
        make_token(Semicolon, ";", Pos::new(65, 6, 19), 1),
        make_token(NewLine, "\n", Pos::new(66, 6, 20), 1),

        // let greeting = "Hello, " + name;
        make_token(Let, "let", Pos::new(67, 7, 1), 3),
        make_token(Identifier, "greeting", Pos::new(71, 7, 5), 8),
        make_token(Assign, "=", Pos::new(80, 7, 14), 1),
        make_token(String, "\"Hello, \"", Pos::new(82, 7, 16), 9),
        make_token(Plus, "+", Pos::new(92, 7, 26), 1),
        make_token(Identifier, "name", Pos::new(94, 7, 28), 4),
        make_token(Semicolon, ";", Pos::new(98, 7, 32), 1),
        make_token(NewLine, "\n", Pos::new(99, 7, 33), 1),
        make_token(NewLine, "\n", Pos::new(100, 8, 1), 1),

        // let ok = true;
        make_token(Let, "let", Pos::new(101, 9, 1), 3),
        make_token(Identifier, "ok", Pos::new(105, 9, 5), 2),
        make_token(Assign, "=", Pos::new(108, 9, 8), 1),
        make_token(True, "true", Pos::new(110, 9, 10), 4),
        make_token(Semicolon, ";", Pos::new(114, 9, 14), 1),
        make_token(NewLine, "\n", Pos::new(115, 9, 15), 1),

        // let nope = false;
        make_token(Let, "let", Pos::new(116, 10, 1), 3),
        make_token(Identifier, "nope", Pos::new(120, 10, 5), 4),
        make_token(Assign, "=", Pos::new(125, 10, 10), 1),
        make_token(False, "false", Pos::new(127, 10, 12), 5),
        make_token(Semicolon, ";", Pos::new(132, 10, 17), 1),
        make_token(NewLine, "\n", Pos::new(133, 10, 18), 1),

        // let nothing = null;
        make_token(Let, "let", Pos::new(134, 11, 1), 3),
        make_token(Identifier, "nothing", Pos::new(138, 11, 5), 7),
        make_token(Assign, "=", Pos::new(146, 11, 13), 1),
        make_token(Null, "null", Pos::new(148, 11, 15), 4),
        make_token(Semicolon, ";", Pos::new(152, 11, 19), 1),
        make_token(NewLine, "\n", Pos::new(153, 11, 20), 1),
        make_token(NewLine, "\n", Pos::new(154, 12, 1), 1),

        // print(a, b, c, greeting, ok, nope, nothing);
        make_token(Identifier, "print", Pos::new(155, 13, 1), 5),
        make_token(Lparen, "(", Pos::new(160, 13, 6), 1),
        make_token(Identifier, "a", Pos::new(161, 13, 7), 1),
        make_token(Comma, ",", Pos::new(162, 13, 8), 1),
        make_token(Identifier, "b", Pos::new(164, 13, 10), 1),
        make_token(Comma, ",", Pos::new(165, 13, 11), 1),
        make_token(Identifier, "c", Pos::new(167, 13, 13), 1),
        make_token(Comma, ",", Pos::new(168, 13, 14), 1),
        make_token(Identifier, "greeting", Pos::new(170, 13, 16), 8),
        make_token(Comma, ",", Pos::new(178, 13, 24), 1),
        make_token(Identifier, "ok", Pos::new(180, 13, 26), 2),
        make_token(Comma, ",", Pos::new(182, 13, 28), 1),
        make_token(Identifier, "nope", Pos::new(184, 13, 30), 4),
        make_token(Comma, ",", Pos::new(188, 13, 34), 1),
        make_token(Identifier, "nothing", Pos::new(190, 13, 36), 7),
        make_token(Rparen, ")", Pos::new(197, 13, 43), 1),
        make_token(Semicolon, ";", Pos::new(198, 13, 44), 1),
        make_token(NewLine, "\n", Pos::new(199, 13, 45), 1),

        // EOF
        make_token(Eof, "", Pos::new(200, 14, 1), 0),
    ];

    let lexer = Lexer::new(input.to_string());
    let mut tokens = lexer.tokens();

    for tc in test_cases {
        let token = tokens.next_token().unwrap();

        assert_eq!(tc, token);
    }
}

#[test]
fn test_numbers() {
    let cases = vec![
        ("0", vec![make_token(IntNumber, "0", Pos::new(0, 1, 1), 1)]),
        ("1", vec![make_token(IntNumber, "1", Pos::new(0, 1, 1), 1)]),
        ("42", vec![make_token(IntNumber, "42", Pos::new(0, 1, 1), 2)]),
        ("999999", vec![make_token(IntNumber, "999999", Pos::new(0, 1, 1), 6)]),

        ("-1", vec![make_token(Minus, "-", Pos::new(0, 1, 1), 1), make_token(IntNumber, "1", Pos::new(1, 1, 2), 1)]),
        ("-42", vec![make_token(Minus, "-", Pos::new(0, 1, 1), 1), make_token(IntNumber, "42", Pos::new(1, 1, 2), 2)]),

        ("1_000", vec![make_token(IntNumber, "1_000", Pos::new(0, 1, 1), 5)]),
        ("10_20_30", vec![make_token(IntNumber, "10_20_30", Pos::new(0, 1, 1), 8)]),
        ("0_1_2", vec![make_token(IntNumber, "0_1_2", Pos::new(0, 1, 1), 5)]),

        // Is not number
        ("__123", vec![make_token(Identifier, "__123", Pos::new(0, 1, 1), 5)]),

        ("0x0", vec![make_token(IntNumber, "0x0", Pos::new(0, 1, 1), 3)]),
        ("0xFF", vec![make_token(IntNumber, "0xFF", Pos::new(0, 1, 1), 4)]),
        ("0xdeadBEEF", vec![make_token(IntNumber, "0xdeadBEEF", Pos::new(0, 1, 1), 10)]),
        ("0xF_F", vec![make_token(IntNumber, "0xF_F", Pos::new(0, 1, 1), 5)]),

        ("0b0", vec![make_token(IntNumber, "0b0", Pos::new(0, 1, 1), 3)]),
        ("0b1010", vec![make_token(IntNumber, "0b1010", Pos::new(0, 1, 1), 6)]),
        ("0b10_11", vec![make_token(IntNumber, "0b10_11", Pos::new(0, 1, 1), 7)]),

        ("0o77", vec![make_token(IntNumber, "0o77", Pos::new(0, 1, 1), 4)]),
        ("0o1_2_3", vec![make_token(IntNumber, "0o1_2_3", Pos::new(0, 1, 1), 7)]),

        ("0.0", vec![make_token(FloatNumber, "0.0", Pos::new(0, 1, 1), 3)]),
        ("1.0", vec![make_token(FloatNumber, "1.0", Pos::new(0, 1, 1), 3)]),
        ("42.5", vec![make_token(FloatNumber, "42.5", Pos::new(0, 1, 1), 4)]),
        ("0.001", vec![make_token(FloatNumber, "0.001", Pos::new(0, 1, 1), 5)]),

        ("5.", vec![make_token(FloatNumber, "5.", Pos::new(0, 1, 1), 2)]),
        ("42.", vec![make_token(FloatNumber, "42.", Pos::new(0, 1, 1), 3)]),

        ("1_000.5", vec![make_token(FloatNumber, "1_000.5", Pos::new(0, 1, 1), 7)]),
        ("0.1_2_3", vec![make_token(FloatNumber, "0.1_2_3", Pos::new(0, 1, 1), 7)]),
        ("1_2_3.4_5", vec![make_token(FloatNumber, "1_2_3.4_5", Pos::new(0, 1, 1), 9)]),

        ("1e3", vec![make_token(FloatNumber, "1e3", Pos::new(0, 1, 1), 3)]),
        ("1E3", vec![make_token(FloatNumber, "1E3", Pos::new(0, 1, 1), 3)]),
        ("1.5e2", vec![make_token(FloatNumber, "1.5e2", Pos::new(0, 1, 1), 5)]),
        ("1e-3", vec![make_token(FloatNumber, "1e-3", Pos::new(0, 1, 1), 4)]),
        ("1e+3", vec![make_token(FloatNumber, "1e+3", Pos::new(0, 1, 1), 4)]),

        ("42.max", vec![
            make_token(IntNumber, "42", Pos::new(0, 1, 1), 2),
            make_token(Dot, ".", Pos::new(2, 1, 3), 1),
            make_token(Identifier, "max", Pos::new(3, 1, 4), 3)
        ]),
        ("10._bar", vec![
            make_token(IntNumber, "10", Pos::new(0, 1, 1), 2),
            make_token(Dot, ".", Pos::new(2, 1, 3), 1),
            make_token(Identifier, "_bar", Pos::new(3, 1, 4), 4)
        ]),

        ("1..5", vec![
            make_token(IntNumber, "1", Pos::new(0, 1, 1), 1),
            make_token(Range, "..", Pos::new(1, 1, 2), 2),
            make_token(IntNumber, "5", Pos::new(3, 1, 4), 1)
        ]),

        ("42..foo", vec![
            make_token(IntNumber, "42", Pos::new(0, 1, 1), 2),
            make_token(Range, "..", Pos::new(2, 1, 3), 2),
            make_token(Identifier, "foo", Pos::new(4, 1, 5), 3)
        ]),

        ("1.2.max", vec![
            make_token(FloatNumber, "1.2", Pos::new(0, 1, 1), 3),
            make_token(Dot, ".", Pos::new(3, 1, 4), 1),
            make_token(Identifier, "max", Pos::new(4, 1, 5), 3)
        ]),

        ("123_456_789.123_456_789", vec![make_token(FloatNumber, "123_456_789.123_456_789", Pos::new(0, 1, 1), 23)]),
        ("0b1010_1010_1010_1010", vec![make_token(IntNumber, "0b1010_1010_1010_1010", Pos::new(0, 1, 1), 21)]),
        ("123______________456", vec![make_token(IntNumber, "123______________456", Pos::new(0, 1, 1), 20)]),
    ];

    for tc in cases {
        let lexer = Lexer::new(tc.0.to_string());
        let mut tokens = lexer.tokens();

        for nt in tc.1 {
            let token = tokens.next_token().unwrap();

            assert_eq!(nt, token)
        }
    }
}

#[test]
fn test_ops() {
    let input = "+- == != &&|| > >= < <= */%";

    let test_cases = vec![
        // Plus
        make_token(Plus, "+", Pos::new(0, 1, 1), 1),
        // Minus
        make_token(Minus, "-", Pos::new(1, 1, 2), 1),
        // Eq
        make_token(Eq, "==", Pos::new(3, 1, 4), 2),
        // Neq
        make_token(Neq, "!=", Pos::new(6, 1, 7), 2),
        // And
        make_token(And, "&&", Pos::new(9, 1, 10), 2),
        // Or
        make_token(Or, "||", Pos::new(11, 1, 12), 2),
        // Gt
        make_token(Gt, ">", Pos::new(14, 1, 15), 1),
        // Gte
        make_token(Gte, ">=", Pos::new(16, 1, 17), 2),
        // Lt
        make_token(Lt, "<", Pos::new(19, 1, 20), 1),
        // Lte
        make_token(Lte, "<=", Pos::new(21, 1, 22), 2),
        // Mult
        make_token(Mult, "*", Pos::new(24, 1, 25), 1),
        // Div
        make_token(Div, "/", Pos::new(25, 1, 26), 1),
        // Mod
        make_token(Mod, "%", Pos::new(26, 1, 27), 1),
        // EOF
        make_token(Eof, "", Pos::new(27, 1, 28), 0),
    ];

    let lexer = Lexer::new(input.to_string());
    let mut tokens = lexer.tokens();

    for tc in test_cases {
        let token = tokens.next_token().unwrap();

        assert_eq!(tc, token);
    }
}

#[test]
fn test_keywords() {
    let test_cases = [
        ("let", Let),
        ("const", Const),
        ("fn", Fn),
        ("async", Async),
        ("await", Await),
        ("return", Return),
        ("if", If),
        ("else", Else),
        ("for", For),
        ("while", While),
        ("break", Break),
        ("continue", Continue),
        ("class", Class),
        ("static", Static),
        ("import", Import),
        ("from", From),
        ("export", Export),
        ("try", Try),
        ("catch", Catch),
        ("finally", Finally),
        ("true", True),
        ("false", False),
        ("null", Null),
    ];

    for tc in test_cases {
        let lexer = Lexer::new(tc.0.to_string());
        let mut tokens = lexer.tokens();
        let token = tokens.next_token().unwrap();

        assert_eq!(tc.1, token.token_type);
    }
}

fn make_token(token_type: TokenType, lexeme: &str, start_pos: Pos, length: usize) -> Token {
    Token::new_with_span(token_type, lexeme.into(), Span::new(start_pos, length))
}
