use std::{fmt::Write};

use crate::ast::*;

use super::*;

#[derive(Default)]
struct TestPrinter {
    buffer: String,
}

impl ExpressionVisitor for TestPrinter {
    fn visit_identifier(&mut self, identifier: &Identifier) {
        write!(self.buffer, "{}", identifier.to_string()).unwrap();
    }
    
    fn visit_literal(&mut self, literal: &Literal) {
        match literal {
            Literal::Int(value) => write!(self.buffer, "{}", value).unwrap(),
            Literal::Str(value) => write!(self.buffer, "\"{}\"", value).unwrap(),
            Literal::Bool(value) => write!(self.buffer, "{}", value).unwrap(),
            Literal::Float(value) => write!(self.buffer, "{}", value).unwrap(),
        }
    }

    fn visit_binary(&mut self, binary: &Binary) {
        use BinaryOp::*;

        write!(self.buffer, "(").unwrap();

        match binary.op {
            And => write!(self.buffer, "&& ").unwrap(),
            Div => write!(self.buffer, "/ ").unwrap(),
            Eq => write!(self.buffer, "== ").unwrap(),
            Gt => write!(self.buffer, "> ").unwrap(),
            Gte => write!(self.buffer, ">= ").unwrap(),
            Lt => write!(self.buffer, "< ").unwrap(),
            Lte => write!(self.buffer, "<= ").unwrap(),
            Minus => write!(self.buffer, "- ").unwrap(),
            Mod => write!(self.buffer, "% ").unwrap(),
            Mult => write!(self.buffer, "* ").unwrap(),
            Neq => write!(self.buffer, "!= ").unwrap(),
            Or => write!(self.buffer, "|| ").unwrap(),
            Plus => write!(self.buffer, "+ ").unwrap(),
            Assign => write!(self.buffer, "= ").unwrap(),
            AssignPlus => write!(self.buffer, "+= ").unwrap(),
            AssignMinus => write!(self.buffer, "-= ").unwrap(),
            AssignMult => write!(self.buffer, "*= ").unwrap(),
            AssignDiv => write!(self.buffer, "/= ").unwrap(),
            AssignMod => write!(self.buffer, "%= ").unwrap(),
            AssignBitAnd => write!(self.buffer, "&= ").unwrap(),
            AssignBitOr => write!(self.buffer, "|= ").unwrap(),
            AssignBitXor => write!(self.buffer, "^= ").unwrap(),
            AssignShiftLeft => write!(self.buffer, "<<= ").unwrap(),
            AssignShiftRight => write!(self.buffer, ">>= ").unwrap(),
            BitOr => write!(self.buffer, "| ").unwrap(),
            BitAnd => write!(self.buffer, "& ").unwrap(),
            BitXor => write!(self.buffer, "^ ").unwrap(),
            ShiftLeft => write!(self.buffer, "<< ").unwrap(),
            ShiftRight => write!(self.buffer, ">> ").unwrap(),
        }

        binary.left.accept(self);
        write!(self.buffer, " ").unwrap();

        binary.right.accept(self);
        write!(self.buffer, ")").unwrap();
    }

    fn visit_unary(&mut self, unary: &Unary) {
        use UnaryOp::*;

        write!(self.buffer, "(").unwrap();

        match unary.op {
            Minus => write!(self.buffer, "- ").unwrap(),
            Not => write!(self.buffer, "! ").unwrap(),
            BitNot => write!(self.buffer, "~ ").unwrap(),
        }

        unary.right.accept(self);
        write!(self.buffer, ")").unwrap();
    }
    
    fn visit_if(&mut self, if_expr: &If) {
        write!(self.buffer, "(if ").unwrap();
        if_expr.condition.accept(self);
        write!(self.buffer, ")\n").unwrap();
        if_expr.consequence.accept(self);

        if let Some(alternarive) = &if_expr.alternative {
            write!(self.buffer, "\n(else)\n").unwrap();
            alternarive.accept(self);
            write!(self.buffer, "\n").unwrap();
        }

        write!(self.buffer, "(endif)").unwrap();
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
    
    fn visit_block(&mut self, block: &Block) {
        write!(self.buffer, "(block)\n").unwrap();
        for statement in &block.statements {
            statement.accept(self);
            write!(self.buffer, "\n").unwrap();
        }
        write!(self.buffer, "(end block)").unwrap();
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
        (
            r#"
return 
x
"#,
            "(return x)\n",
        ),
        (
            r#"
return;
x; // unreachable
"#,
            "(return)\nx\n",
        ),
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
        ("1.3e-1", "0.13\n"),
        // ("true != false", "(!= true false)\n"),
        ("a + b - c", "(+ a (- b c))\n"),
        ("a * b / c % d", "(* a (/ b (% c d)))\n"),
        ("-a + !b - ~c", "(+ (- a) (- (! b) (~ c)))\n"),
        ("(a + b) * d", "(* (+ a b) d)\n"),
        (
            r#"
(
    a + b
)
*
89
"#,
            "(* (+ a b) 89)\n",
        ),
        ("a | b ^ c", "(| a (^ b c))\n"),
        ("a == b", "(== a b)\n"),
        ("a != b", "(!= a b)\n"),
        ("a < b", "(< a b)\n"),
        ("a > b", "(> a b)\n"),
        ("a <= b", "(<= a b)\n"),
        ("a >= b", "(>= a b)\n"),
        ("a = b", "(= a b)\n"),
        ("a += b", "(+= a b)\n"),
        ("a -= b", "(-= a b)\n"),
        ("a *= b", "(*= a b)\n"),
        ("a /= b", "(/= a b)\n"),
        ("a %= b", "(%= a b)\n"),
        ("a |= b", "(|= a b)\n"),
        ("a &= b", "(&= a b)\n"),
        ("a >>= b", "(>>= a b)\n"),
        ("a <<= b", "(<<= a b)\n"),
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
        ("!!ready", "(! (! ready))\n"),
        ("-count", "(- count)\n"),
        ("~mask", "(~ mask)\n"),
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

#[test]
fn test_parse_block() {
    let test_cases = vec![
        ("{}", "(block)\n(end block)\n"),
        (
            r#"
{
    a + b;
}"#,
            r#"(block)
(+ a b)
(end block)
"#
        ),
        (
            r#"
{
    { a + b }
}"#,
            r#"(block)
(block)
(+ a b)
(end block)
(end block)
"#
        ),
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
fn test_if_block() {
    let test_cases = vec![
        ("if (true) {} else {}", "(if true)\n(block)\n(end block)\n(else)\n(block)\n(end block)\n(endif)\n"),
        (
            r#"
if (a) {
    true
} else {
    false
}"#,
        r#"(if a)
(block)
true
(end block)
(else)
(block)
false
(end block)
(endif)
"#
        ),
        (
            r#"
if (if (b) true; else false;) {
    true
} else {
    false
}"#,
        r#"(if (if b)
true
(else)
false
(endif))
(block)
true
(end block)
(else)
(block)
false
(end block)
(endif)
"#
        ),
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
