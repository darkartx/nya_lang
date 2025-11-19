use std::{fmt::Write};

use crate::ast::{self, *};

use super::*;

#[derive(Default)]
struct TestPrinter {
    buffer: String,
}

impl ast::ExpressionVisitor for TestPrinter {
    fn visit_identifier(&mut self, identifier: &ast::Identifier) {
        write!(self.buffer, "{}", identifier.name()).unwrap();
    }
    
    fn visit_literal(&mut self, literal: &ast::Literal) {
        match literal {
            ast::Literal::Int(value) => write!(self.buffer, "(int {})", value).unwrap(),
            ast::Literal::Str(value) => write!(self.buffer, "(str \"{}\")", value).unwrap(),
            ast::Literal::Bool(value) => write!(self.buffer, "({})", value).unwrap(),
            ast::Literal::Float(value) => write!(self.buffer, "(float {})", value).unwrap(),
        }
    }
}

impl ast::StatementVisitor for TestPrinter {
    fn visit_ast(&mut self, ast: &Ast) {
        for stmt in ast.statements() {
            stmt.accept(self);
            write!(self.buffer, "\n").unwrap();
        }
    }

    fn visit_let(&mut self, let_expr: &ast::Let) {
        write!(self.buffer, "(let ").unwrap();
        let_expr.identifier().accept(self);
        if let Some(expr) = let_expr.expression() {
            write!(self.buffer, " = ").unwrap();
            expr.accept(self);
        }
        write!(self.buffer, ")").unwrap();
    }
}

#[test]
fn test_parse_let_statement() {
    let test_cases = vec![
        ("let a = 5;", "(let a = (int 5))\n"),
        ("let name = \"orc\\u{2764}\";", "(let name = (str \"orc❤\"))\n"),
        ("let alive = true;", "(let alive = (true))\n"),
//         ("let z = x + y;", "(let z = (x + y))\n"),
//         ("let dmg = (a + b * c) / 2 - crit;", "(let dmg = (((a + (b * c)) / 2) - crit)\n"),
//         ("let msg = f\"HP: {hp}\";", "(let msg = (fstr \"HP: {hp}\"))\n"),
//         ("let arr = [1, 2, 3, 4];", "(let arr = ([1, 2, 3, 4]))\n"),
//         ("let obj = { x: 1, y: 2 };", "(let obj = (obj {x: (1), y: (2)}))\n"),
//         ("let player = new Player();", "TODO"),
//         ("let hp = player.hp;", "TODO"),
//         ("let name = player.getName();", "TODO"),
//         ("let val = obj.key.subkey.method();", "TODO"),
//         ("let data = await loadData();", "TODO"),
//         ("let token = await user.getToken();", "TODO"),
//         ("let result = compute(1, 2, 3);", "TODO"),
//         ("let status = check(player.hp, 10);", "TODO"),
//         ("let ok = a > 10;", "TODO"),
//         ("let fail = a == b && c != d;", "TODO"),
//         (r#"
// if (x > 10) {
//     let y = x * 2;
// }"#,
// "TODO"
//         ),
//         (r#"
// let player = {
//     name: getName(),
//     hp: stats.base + stats.bonus,
//     pos: [x, y, z],
// };"#,
// "TODO"
//         ),
        ("let   x    =      10    ;", "(let x = (int 10))\n"),
        ("let здоровье = 100;", "(let здоровье = (int 100))\n"),
    ];

    for tc in test_cases {
        let lexer = Lexer::new(tc.0.to_string());
        let parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();
        let mut test_printer = TestPrinter::default();
        test_printer.visit_ast(&ast);

        assert_eq!(tc.1, test_printer.buffer);
    }
}
