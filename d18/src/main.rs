#[derive(Clone)]
enum Token {
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Number {val: usize}
}
fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = vec!();
    let mut chars = s.chars();
    let mut next = chars.next();
    loop {
        match next {
            Some(mut c) => {
                match c {
                    '(' => { tokens.push(Token::LeftParen{}); next = chars.next()},
                    ')' => { tokens.push(Token::RightParen{}); next = chars.next()},
                    '*' => { tokens.push(Token::Asterisk{}); next = chars.next()},
                    '+' => { tokens.push(Token::Plus{}); next = chars.next()},
                    ' ' => {next = chars.next()},
                    _   => {
                        let mut num: usize = 0;
                        loop {
                            match c.to_digit(10) {
                                Some(n) => {
                                    num = num * 10 + n as usize;
                                    next = chars.next();
                                    match next {
                                        Some(cc) => c = cc,
                                        None     => {
                                            tokens.push(Token::Number{val:num});
                                            return tokens
                                        }
                                    }
                                },
                                None    => {
                                    tokens.push(Token::Number{val:num});
                                    break
                                }
                            }    
                        }
                    }
                }
            },
            None => break
        }
    }
    tokens
}

struct TokenStream {
    tokens: Vec<Token>,
    next_ind: usize
}

impl TokenStream {
    fn new(tokens: &Vec<Token>) -> Self {
        TokenStream{
            tokens: (*tokens).clone(),
            next_ind: 0
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.next_ind >= self.tokens.len() {
            None
        } else {
            let t = self.tokens[self.next_ind].clone();
            self.next_ind += 1;
            Some(t)
        }
    }
}

#[derive(Clone)]
enum ArithmeticElement {
    Multiplication,
    Addition,
    Expression {
        expression: NumberExpression        
    }
}

impl ArithmeticElement {
    fn eval(&self, eager_multiplication: bool) -> usize {
        match self {
            ArithmeticElement::Expression{ expression } => if eager_multiplication { expression.eval_2() } else { expression.eval_1() },
            _ => panic!("Only Expression elements can be evaluated.")
        }
    }
}

#[derive(Clone)]
enum NumberExpression {
    SimpleNumber {val: usize},
    ArithmeticExpression {
        expression: Vec<ArithmeticElement>
    }
}

impl NumberExpression {
    fn eval_1(&self) -> usize {
        match self {
            NumberExpression::SimpleNumber { val } => *val,
            NumberExpression::ArithmeticExpression { expression } => {
                let mut val = 0;
                let mut prev_element: Option<ArithmeticElement> = None;
                for elem in expression {
                    match elem {
                        ArithmeticElement::Addition => prev_element = Some(ArithmeticElement::Addition),
                        ArithmeticElement::Multiplication => prev_element = Some(ArithmeticElement::Multiplication),
                        ArithmeticElement::Expression{ expression } => {
                            match prev_element {
                                Some(ArithmeticElement::Addition) => val += expression.eval_1(),
                                Some(ArithmeticElement::Multiplication) => val *= expression.eval_1(),
                                None => val = expression.eval_1(),
                                _ => panic!("Invalid sequence.")
                            }
                        }
                    }
                }
                val
            } 
        }
    }

    fn eval_2(&self) -> usize {
        match self {
            NumberExpression::SimpleNumber { val } => *val,
            NumberExpression::ArithmeticExpression { expression } => {
                let mut factors: Vec<Vec<ArithmeticElement>> = vec!();
                let mut factor: Vec<ArithmeticElement> = vec!();
                for elem in expression {
                    match elem {
                        ArithmeticElement::Multiplication => {
                            factors.push(factor);
                            factor = vec!();
                        },
                        ArithmeticElement::Addition => {}
                        _ => factor.push((*elem).clone())
                    }
                }
                if !factor.is_empty() { factors.push(factor) }
                factors.iter().map(
                    |f| f.iter().map(
                        |t| t.eval(true)).sum::<usize>()
                ).product()
            } 
        }
    }
}

fn parse_number_expression(token_stream: &mut TokenStream) -> ArithmeticElement {
    let mut expression: Vec<ArithmeticElement> = vec!();
    loop {
        match token_stream.next_token() {
            Some(token) => match token {
                Token::LeftParen => { 
                    expression.push(parse_number_expression(token_stream))
                },
                Token::RightParen => {
                    return ArithmeticElement::Expression {
                        expression: NumberExpression::ArithmeticExpression {expression}
                    }
                },
                Token::Asterisk => {
                    expression.push(ArithmeticElement::Multiplication)
                },
                Token::Plus => {
                    expression.push(ArithmeticElement::Addition)
                },
                Token::Number{ val } => { 
                    expression.push(
                        ArithmeticElement::Expression {
                            expression: NumberExpression::SimpleNumber{ val }
                        }
                    )
                }
            }
            None => break
        }
    }
    ArithmeticElement::Expression {
        expression: NumberExpression::ArithmeticExpression{ expression }
    }
}

fn part_1(expressions: &Vec<ArithmeticElement>) {
    let answer: usize = expressions.iter().map(|exp| exp.eval(false)).sum();
    println!("Answer part 1: {}", answer)
}

fn part_2(expressions: &Vec<ArithmeticElement>) {
    let answer: usize = expressions.iter().map(|exp| exp.eval(true)).sum();
    println!("Answer part 2: {}", answer)
}

fn main() {
    let token_vecs: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s|  tokenize(s)
    ).collect();

    let expressions: Vec<_> = token_vecs.iter().map(
        |tv| {
            let mut ts = TokenStream::new(&tv);
            parse_number_expression(&mut ts)
        }
    ).collect();

    part_1(&expressions);
    part_2(&expressions);
}
