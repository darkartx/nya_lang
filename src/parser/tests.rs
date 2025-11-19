use std::{fmt::Write};

use crate::ast::*;

use super::*;

#[derive(Default)]
struct TestPrinter {
    buffer: String,
}

impl ExpressionVisitor for TestPrinter {
    fn visit_identifier(&mut self, identifier: &Identifier) {
        write!(self.buffer, "{}", identifier.name()).unwrap();
    }
    
    fn visit_value(&mut self, value: &Value) {
        match value {
            Value::Int(value) => write!(self.buffer, "(int {})", value).unwrap()
        }
    }
}

impl StatementVisitor for TestPrinter {
    fn visit_ast(&mut self, ast: &Ast) {
        for stmt in ast.statements() {
            stmt.accept(self);
            write!(self.buffer, "\n").unwrap();
        }
    }

    fn visit_let(&mut self, let_expr: &Let) {
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
        ("let a = 5;", "(let a = (int 5))\n")
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
