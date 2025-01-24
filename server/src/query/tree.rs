pub enum Expression {
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    ExclusiveOr(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}