use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait StatementNode {
    fn statement_node();
}

trait ExpressionNode {
    fn expression_node();
}

struct Statement {}
struct Expression {}

impl StatementNode for Statement {
    fn statement_node() {
        todo!()
    }
}

impl ExpressionNode for Expression {
    fn expression_node() {
        todo!()
    }
}

struct Program {
    statements: Vec<Statement>,
}

fn takes_node(node: impl Node) -> String {
    println!("{}", node.token_literal());
    node.token_literal()
}

fn main() {
    let expression = Expression {};
}
