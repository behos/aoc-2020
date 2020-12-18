use anyhow::{Context, Result};
use aoc_2020::read_entries;

#[derive(Debug)]
enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Paren(Box<Expression>),
    Digit(usize),
}

impl Expression {
    fn parse(expression: &[char], op: Option<char>) -> Result<Self> {
        // println!("Parsing: {}", expression.iter().collect::<String>());
        let parsed = match expression {
            [c] => Expression::Digit(
                c.to_digit(10).context("Should be a digit.")? as usize,
            ),
            ['(', inner @ .., ')'] if balanced_parens(inner) => {
                Expression::Paren(Box::new(Expression::parse(&inner, op)?))
            }
            _ => {
                let opi = find_last_op(&expression, op)
                    .or_else(|| find_last_op(&expression, None))
                    .context("Failed to find an operator!")?;
                let left = Box::new(Expression::parse(&expression[..opi], op)?);
                let right =
                    Box::new(Expression::parse(&expression[opi + 1..], op)?);
                if expression[opi] == '*' {
                    Expression::Mul(left, right)
                } else {
                    Expression::Add(left, right)
                }
            }
        };
        Ok(parsed)
    }

    fn evaluate(&self) -> usize {
        match self {
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Mul(left, right) => left.evaluate() * right.evaluate(),
            Expression::Paren(inner) => inner.evaluate(),
            Expression::Digit(val) => *val,
        }
    }
}

fn balanced_parens(chars: &[char]) -> bool {
    let mut parens = 0;
    for &c in chars {
        if c == '(' {
            parens += 1
        }
        if c == ')' {
            parens -= 1
        }
        if parens < 0 {
            return false;
        }
    }
    parens == 0
}

fn find_last_op(chars: &[char], op: Option<char>) -> Option<usize> {
    let mut parens = 0;
    for i in (0..chars.len()).rev() {
        match chars[i] {
            '(' => parens += 1,
            ')' => parens -= 1,
            c @ '*' | c @ '+'
                if (op.is_none() || Some(c) == op) && parens == 0 =>
            {
                return Some(i)
            }
            _ => {}
        }
    }
    None
}

fn main() {
    let expressions = read_entries::<String>("./data/day-18.txt")
        .map(|e| e.chars().filter(|c| c != &' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let sum: usize = expressions
        .iter()
        .map(|e| Expression::parse(e, None).unwrap().evaluate())
        .sum();

    println!("Sum is {}", sum);

    let sum: usize = expressions
        .iter()
        .map(|e| Expression::parse(e, Some('*')).unwrap().evaluate())
        .sum();

    println!("Sum with addition precedence is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Expression::parse(
                &"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                    .chars()
                    .filter(|c| c != &' ')
                    .collect::<Vec<_>>(),
                None
            )
            .unwrap()
            .evaluate(),
            13632
        )
    }

    #[test]
    fn examples_advanced() {
        assert_eq!(
            Expression::parse(
                &"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                    .chars()
                    .filter(|c| c != &' ')
                    .collect::<Vec<_>>(),
                Some('*')
            )
            .unwrap()
            .evaluate(),
            23340
        )
    }
}
