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
        make_token(Let, "let", Pos::new(1, 2, 1), 3),
        make_token(Identifier, "a", Pos::new(5, 2, 5), 1),
        make_token(Assign, "=", Pos::new(7, 2, 7), 1),
        make_token(IntNumber, "10", Pos::new(9, 2, 9), 2),
        make_token(Semicolon, ";", Pos::new(11, 2, 11), 1),

        // let b = 3.14;
        make_token(Let, "let", Pos::new(13, 3, 1), 3),
        make_token(Identifier, "b", Pos::new(17, 3, 5), 1),
        make_token(Assign, "=", Pos::new(19, 3, 7), 1),
        make_token(FloatNumber, "3.14", Pos::new(21, 3, 9), 4),
        make_token(Semicolon, ";", Pos::new(25, 3, 13), 1),

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

        // let name = "Alice";
        make_token(Let, "let", Pos::new(47, 6, 1), 3),
        make_token(Identifier, "name", Pos::new(51, 6, 5), 4),
        make_token(Assign, "=", Pos::new(56, 6, 10), 1),
        make_token(String, "\"Alice\"", Pos::new(58, 6, 12), 7),
        make_token(Semicolon, ";", Pos::new(65, 6, 19), 1),

        // let greeting = "Hello, " + name;
        make_token(Let, "let", Pos::new(67, 7, 1), 3),
        make_token(Identifier, "greeting", Pos::new(71, 7, 5), 8),
        make_token(Assign, "=", Pos::new(80, 7, 14), 1),
        make_token(String, "\"Hello, \"", Pos::new(82, 7, 16), 9),
        make_token(Plus, "+", Pos::new(92, 7, 26), 1),
        make_token(Identifier, "name", Pos::new(94, 7, 28), 4),
        make_token(Semicolon, ";", Pos::new(98, 7, 32), 1),

        // let ok = true;
        make_token(Let, "let", Pos::new(101, 9, 1), 3),
        make_token(Identifier, "ok", Pos::new(105, 9, 5), 2),
        make_token(Assign, "=", Pos::new(108, 9, 8), 1),
        make_token(True, "true", Pos::new(110, 9, 10), 4),
        make_token(Semicolon, ";", Pos::new(114, 9, 14), 1),

        // let nope = false;
        make_token(Let, "let", Pos::new(116, 10, 1), 3),
        make_token(Identifier, "nope", Pos::new(120, 10, 5), 4),
        make_token(Assign, "=", Pos::new(125, 10, 10), 1),
        make_token(False, "false", Pos::new(127, 10, 12), 5),
        make_token(Semicolon, ";", Pos::new(132, 10, 17), 1),

        // let nothing = null;
        make_token(Let, "let", Pos::new(134, 11, 1), 3),
        make_token(Identifier, "nothing", Pos::new(138, 11, 5), 7),
        make_token(Assign, "=", Pos::new(146, 11, 13), 1),
        make_token(Null, "null", Pos::new(148, 11, 15), 4),
        make_token(Semicolon, ";", Pos::new(152, 11, 19), 1),

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

fn make_token(token_type: TokenType, lexeme: &str, start_pos: Pos, length: usize) -> Token {
    Token::new_with_span(token_type, lexeme.into(), Span::new(start_pos, length))
}
