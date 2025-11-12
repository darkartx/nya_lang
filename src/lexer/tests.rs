use crate::token::*;

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
        // let a = 10;
        make_token(Let, "let", LiteralValue::None, Position::new(1, 2, 1)),
        make_token(Identifier, "a", LiteralValue::new_str("a"), Position::new(5, 2, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(7, 2, 7)),
        make_token(IntNumber, "10", LiteralValue::IntNumber(10), Position::new(9, 2, 9)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(11, 2, 11)),

        // let b = 3.14;
        make_token(Let, "let", LiteralValue::None, Position::new(13, 3, 1)),
        make_token(Identifier, "b", LiteralValue::new_str("b"), Position::new(17, 3, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(19, 3, 7)),
        make_token(FloatNumber, "3.14", LiteralValue::FloatNumber(3.14), Position::new(21, 3, 9)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(25, 3, 13)),

        // let c = a + b * 2;
        make_token(Let, "let", LiteralValue::None, Position::new(27, 4, 1)),
        make_token(Identifier, "c", LiteralValue::new_str("c"), Position::new(31, 4, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(33, 4, 7)),
        make_token(Identifier, "a", LiteralValue::new_str("a"), Position::new(35, 4, 9)),
        make_token(Plus, "+", LiteralValue::None, Position::new(37, 4, 11)),
        make_token(Identifier, "b", LiteralValue::new_str("b"), Position::new(39, 4, 13)),
        make_token(Mult, "*", LiteralValue::None, Position::new(41, 4, 15)),
        make_token(IntNumber, "2", LiteralValue::IntNumber(2), Position::new(43, 4, 17)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(44, 4, 18)),

        // let name = "Alice";
        make_token(Let, "let", LiteralValue::None, Position::new(47, 6, 1)),
        make_token(Identifier, "name", LiteralValue::new_str("name"), Position::new(51, 6, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(56, 6, 10)),
        make_token(String, "\"Alice\"", LiteralValue::new_str("Alice"), Position::new(58, 6, 12)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(65, 6, 19)),

        // let greeting = "Hello, " + name;
        make_token(Let, "let", LiteralValue::None, Position::new(67, 7, 1)),
        make_token(Identifier, "greeting", LiteralValue::new_str("greeting"), Position::new(71, 7, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(80, 7, 14)),
        make_token(String, "\"Hello, \"", LiteralValue::new_str("Hello, "), Position::new(82, 7, 16)),
        make_token(Plus, "+", LiteralValue::None, Position::new(92, 7, 26)),
        make_token(Identifier, "name", LiteralValue::new_str("name"), Position::new(94, 7, 28)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(98, 7, 32)),

        // let ok = true;
        make_token(Let, "let", LiteralValue::None, Position::new(101, 9, 1)),
        make_token(Identifier, "ok", LiteralValue::new_str("ok"), Position::new(105, 9, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(108, 9, 8)),
        make_token(True, "true", LiteralValue::Bool(true), Position::new(110, 9, 10)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(114, 9, 14)),

        // let nope = false;
        make_token(Let, "let", LiteralValue::None, Position::new(116, 10, 1)),
        make_token(Identifier, "nope", LiteralValue::new_str("nope"), Position::new(120, 10, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(125, 10, 10)),
        make_token(False, "false", LiteralValue::Bool(false), Position::new(127, 10, 12)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(132, 10, 17)),

        // let nothing = null;
        make_token(Let, "let", LiteralValue::None, Position::new(134, 11, 1)),
        make_token(Identifier, "nothing", LiteralValue::new_str("nothing"), Position::new(138, 11, 5)),
        make_token(Assign, "=", LiteralValue::None, Position::new(146, 11, 13)),
        make_token(Null, "null", LiteralValue::Null, Position::new(148, 11, 15)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(152, 11, 19)),

        // print(a, b, c, greeting, ok, nope, nothing);
        make_token(Identifier, "print", LiteralValue::new_str("print"), Position::new(155, 13, 1)),
        make_token(Lparen, "(", LiteralValue::None, Position::new(160, 13, 6)),
        make_token(Identifier, "a", LiteralValue::new_str("a"), Position::new(161, 13, 7)),
        make_token(Comma, ",", LiteralValue::None, Position::new(162, 13, 8)),
        make_token(Identifier, "b", LiteralValue::new_str("b"), Position::new(164, 13, 10)),
        make_token(Comma, ",", LiteralValue::None, Position::new(165, 13, 11)),
        make_token(Identifier, "c", LiteralValue::new_str("c"), Position::new(167, 13, 13)),
        make_token(Comma, ",", LiteralValue::None, Position::new(168, 13, 14)),
        make_token(Identifier, "greeting", LiteralValue::new_str("greeting"), Position::new(170, 13, 16)),
        make_token(Comma, ",", LiteralValue::None, Position::new(178, 13, 24)),
        make_token(Identifier, "ok", LiteralValue::new_str("ok"), Position::new(180, 13, 26)),
        make_token(Comma, ",", LiteralValue::None, Position::new(182, 13, 28)),
        make_token(Identifier, "nope", LiteralValue::new_str("nope"), Position::new(184, 13, 30)),
        make_token(Comma, ",", LiteralValue::None, Position::new(188, 13, 34)),
        make_token(Identifier, "nothing", LiteralValue::new_str("nothing"), Position::new(190, 13, 36)),
        make_token(Rparen, ")", LiteralValue::None, Position::new(197, 13, 43)),
        make_token(Semicolon, ";", LiteralValue::None, Position::new(198, 13, 44)),

        // EOF
        make_token(Eof, "", LiteralValue::None, Position::new(200, 14, 1))
    ];

    let lexer = Lexer::new(input.to_string());
    let mut tokens = lexer.tokens();

    for tc in test_cases {
        let token = tokens.next().unwrap().unwrap();

        assert_eq!(tc, token);
    }
}

#[test]
fn test_string_char_escaping() {
    let input = "\"asdasd\\\"asda
sd\\\"sda\\nsd\"";
    let lexer = Lexer::new(input.to_string());
    let mut tokens = lexer.tokens();

    let expected = make_token(
        TokenType::String,
        "\"asdasd\\\"asda\nsd\\\"sda\\nsd\"",
        LiteralValue::Str("asdasd\"asda\nsd\"sda\nsd".to_string()),
        Position::new(0, 1, 1)
    );

    assert_eq!(tokens.next().unwrap().unwrap(), expected)
}

fn make_token(token_type: TokenType, lexeme: &str, literal_value: LiteralValue, position: Position) -> Token {
    Token::new_with_position(token_type, lexeme.into(), literal_value, position)
}
