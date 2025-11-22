use std::{fmt::Write};

use crate::ast::{self, *};

use super::*;

#[derive(Default)]
struct TestPrinter {
    buffer: String,
}

impl ast::ExpressionVisitor for TestPrinter {
    fn visit_identifier(&mut self, identifier: &ast::Identifier) {
        write!(self.buffer, "{}", identifier.to_string()).unwrap();
    }
    
    fn visit_literal(&mut self, literal: &ast::Literal) {
        match literal {
            ast::Literal::Int(value) => write!(self.buffer, "{}", value).unwrap(),
            ast::Literal::Str(value) => write!(self.buffer, "\"{}\"", value).unwrap(),
            ast::Literal::Bool(value) => write!(self.buffer, "{}", value).unwrap(),
            ast::Literal::Float(value) => write!(self.buffer, "{}", value).unwrap(),
        }
    }

    fn visit_binary(&mut self, binary: &Binary) {
        write!(self.buffer, "(").unwrap();

        match binary.op {
            ast::BinaryOp::And => write!(self.buffer, "&& ").unwrap(),
            ast::BinaryOp::Div => write!(self.buffer, "/ ").unwrap(),
            ast::BinaryOp::Eq => write!(self.buffer, "== ").unwrap(),
            ast::BinaryOp::Gt => write!(self.buffer, "> ").unwrap(),
            ast::BinaryOp::Gte => write!(self.buffer, ">= ").unwrap(),
            ast::BinaryOp::Lt => write!(self.buffer, "< ").unwrap(),
            ast::BinaryOp::Lte => write!(self.buffer, "<= ").unwrap(),
            ast::BinaryOp::Minus => write!(self.buffer, "- ").unwrap(),
            ast::BinaryOp::Mod => write!(self.buffer, "% ").unwrap(),
            ast::BinaryOp::Mult => write!(self.buffer, "* ").unwrap(),
            ast::BinaryOp::Neq => write!(self.buffer, "!= ").unwrap(),
            ast::BinaryOp::Or => write!(self.buffer, "|| ").unwrap(),
            ast::BinaryOp::Plus => write!(self.buffer, "+ ").unwrap(),
        }

        binary.left.accept(self);
        write!(self.buffer, " ").unwrap();

        binary.right.accept(self);
        write!(self.buffer, ")").unwrap();
    }

    fn visit_unary(&mut self, unary: &Unary) {
        write!(self.buffer, "(").unwrap();

        match unary.op {
            ast::UnaryOp::Minus => write!(self.buffer, "- ").unwrap(),
            ast::UnaryOp::Not => write!(self.buffer, "! ").unwrap(),
        }

        unary.right.accept(self);
        write!(self.buffer, ")").unwrap();
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
        let_expr.identifier.accept(self);
        if let Some(expr) = &let_expr.expression {
            write!(self.buffer, " = ").unwrap();
            expr.accept(self);
        }
        write!(self.buffer, ")").unwrap();
    }
    
    fn visit_return(&mut self, return_statement: &Return) {
        write!(self.buffer, "(return").unwrap();

        if let Some(expr) = &return_statement.expression {
            write!(self.buffer, " ").unwrap();
            expr.accept(self);
        }

        write!(self.buffer, ")").unwrap();
    }
    
    fn visit_expr(&mut self, expr: &Expr) {
        expr.expression.accept(self);
    }
}

#[test]
fn test_parse_let_statement() {
    let test_cases = vec![
        ("let a = 5;", "(let a = 5)\n"),
        ("let name = \"orc\\u{2764}\";", "(let name = \"orc❤\")\n"),
        ("let alive = true;", "(let alive = true)\n"),
        ("let z = x + y;", "(let z = (+ x y))\n"),
        ("let dmg = (a + b * c) / 2 - crit;", "(let dmg = (- (/ (+ a (* b c)) 2) crit))\n"),
        ("let neg = -100.5", "(let neg = (- 100.5))\n"),
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
        ("let   x    =      10    ;", "(let x = 10)\n"),
        ("let здоровье = 100;", "(let здоровье = 100)\n"),
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

#[test]
fn test_parse_return_statement() {
    let test_cases = vec![
        ("return", "(return)\n"),
        ("return 42", "(return 42)\n"),
        ("return 123;", "(return 123)\n"),
        ("return a + b", "(return (+ a b))\n"),
        ("return -42", "(return (- 42))\n"),
        // ("return player.hp", "TODO\n"),
        (
            r#"
return (
  1 + 2
)
"#,
            "(return (+ 1 2))\n",
        ),
//         (
//             r#"
// return 
// x
// "#,
//             "(return)\nx\n",
//         ),
//         (
//             r#"
// return;
// x; // unreachable
// "#,
//             "(return)\nx\n",
//         ),
//         (
//             r#"
// fn f() {
//   return 10
// }
// "#,
//             "TODO",
//         ),
//         (
//             r#"
// if (x > 0)
//     return x
// }
// "#,
//             "TODO",
//         ),
//         (
//             r#"
// if (ok)
//     return 1
// else
//     return 2
// "#,
//             "TODO",
//         ),
//         (
//             r#"
// while (true) {
//   return x
// }
// "#,
//             "TODO",
//         ),
//         (
//             r#"
// return {
//   x: 1,
//   y: 2
// }
// "#,
//             "TODO",
//         ),
        // ("return [1, 2, 3]", "TODO"),
        // ("return (x) => x * 2", "TODO"),
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

#[test]
fn test_parse_expression_statement() {
    let test_cases = vec![
        ("1", "1\n"),
        ("x", "x\n"),
        ("\"hello\"", "\"hello\"\n"),
        // ("[1, 2, 3]", "TODO"),
        // ("foo()", "TODO"),
        // ("foo(1, 2, x)", "TODO"),
        // ("foo(bar(1))", "TODO"),
        // ("player.move(10)", "TODO"),
        // ("foo().bar().baz()", "TODO"),
        // ("x = 10", "TODO"),
        // ("player.hp = 100", "TODO"),
        // ("x = y + z * 3", "TODO"),
        // ("config = loadConfig()", "TODO"),
        // ("foo(1) + bar(2)", "TODO"),
        // ("flag || doStuff()", "TODO"),
        ("!ready", "(! ready)\n"),
        ("-count", "(- count)\n"),
        ("(1 + 2) * 3", "(* (+ 1 2) 3)\n"),
//         (
//             r#"
// foo()
// bar()
// "#,
//             "TODO",
//         ),
//         (
//             r#"
// {
//   foo()
//   bar()
// }"#,
//             "TODO",
//         ),
//         (
//             r#"
// foo();
// bar();
// "#,
//             "TODO",
//         ),
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
