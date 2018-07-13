mod parser;

use nom::types::CompleteStr;
use failure::Error;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Value(String),
}

impl Expr {
    pub fn calc<S: AsRef<str>>(&self, values: &[S]) -> bool {
        match self {
            Expr::Or(a, b) => a.calc(values) || b.calc(values),
            Expr::And(a, b) => a.calc(values) && b.calc(values),
            Expr::Not(e) => !e.calc(values),
            Expr::Value(v) => values.iter().find(|item| item.as_ref() == v.as_str()).is_some(),
        }
    }
}

#[derive(Default)]
pub struct Ops {
    pub lp: String,
    pub rp: String,
    pub or: String,
    pub and: String,
    pub not: String,
}

pub trait ExprOps {
    fn lp(&self) -> &str;
    fn rp(&self) -> &str;
    fn or(&self) -> &str;
    fn and(&self) -> &str;
    fn not(&self) -> &str;
}

impl ExprOps for Ops {
    fn lp(&self) -> &str {
        self.lp.as_str()
    }

    fn rp(&self) -> &str {
        self.rp.as_str()
    }

    fn or(&self) -> &str {
        self.or.as_str()
    }

    fn and(&self) -> &str {
        self.and.as_str()
    }

    fn not(&self) -> &str {
        self.not.as_str()
    }
}

pub fn parse<S: AsRef<str>>(input: S, ops: &dyn ExprOps) -> Result<Expr, Error> {
    let parsed = parser::parse_expr(CompleteStr(input.as_ref()), ops)
        .expect(&format!("Error occurred while parsing expression `{}`", input.as_ref()));
    match parsed {
        (CompleteStr(""), expr) => Ok(expr),
        (CompleteStr(s), _) => Err(format_err!("Can't parse expression`{}`", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Expr::*;

    #[test]
    fn expr_parse() {
        let ops = Ops {
            lp: "(".to_string(),
            rp: ")".to_string(),
            or: ",".to_string(),
            and: "+".to_string(),
            not: "^".to_string(),
        };

        let expr = "abc";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(Value("abc".to_string()), parsed);

        let values = ["abc"];
        assert!(parsed.calc(&values));

        let values = ["a"];
        assert!(!parsed.calc(&values));


        let expr = "(abc)";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(Value("abc".to_string()), parsed);


        let expr = "^abc";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(Not(Box::new(Value("abc".to_string()))), parsed);

        let values = ["abc"];
        assert!(!parsed.calc(&values));

        let values = [""];
        assert!(parsed.calc(&values));


        let expr = "abc,de";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(
            Or(
                Box::new(Value("abc".to_string())),
                Box::new(Value("de".to_string()))
            ),
            parsed
        );

        let values = ["abc", "de"];
        assert!(parsed.calc(&values));

        let values = ["abc", ""];
        assert!(parsed.calc(&values));

        let values = ["", "de"];
        assert!(parsed.calc(&values));

        let values = ["", ""];
        assert!(!parsed.calc(&values));


        let expr = "abc+de";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(
            And(
                Box::new(Value("abc".to_string())),
                Box::new(Value("de".to_string()))
            ),
            parsed
        );

        let values = ["abc", "de"];
        assert!(parsed.calc(&values));

        let values = ["abc", ""];
        assert!(!parsed.calc(&values));

        let values = ["", "de"];
        assert!(!parsed.calc(&values));

        let values = ["", ""];
        assert!(!parsed.calc(&values));


        let expr = "(ab + (c, d) + e)";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(
            And(
                Box::new(And(
                    Box::new(Value("ab".to_string())),
                    Box::new(Or(
                        Box::new(Value("c".to_string())),
                        Box::new(Value("d".to_string()))
                    ))
                )),
                Box::new(Value("e".to_string()))
            ),
            parsed
        );

        let values = ["ab", "c", "d", "e"];
        assert!(parsed.calc(&values));

        let values = ["ab", "c", "e"];
        assert!(parsed.calc(&values));

        let values = ["ab", "d", "e"];
        assert!(parsed.calc(&values));

        let values = ["ab", "c", "d"];
        assert!(!parsed.calc(&values));

        let values = ["c", "d", "e"];
        assert!(!parsed.calc(&values));


        let expr = "^(a,b+^c)";
        let parsed = parse(expr, &ops).unwrap();
        assert_eq!(
            Not(
                Box::new(Or(
                    Box::new(Value("a".to_string())),
                    Box::new(And(
                        Box::new(Value("b".to_string())),
                        Box::new(Not(
                            Box::new(Value("c".to_string()))
                        ))
                    ))
                ))
            ),
            parsed
        );

        let values = ["a", "b", "c"];
        assert!(!parsed.calc(&values));

        let values = ["a"];
        assert!(!parsed.calc(&values));

        let values = ["b"];
        assert!(!parsed.calc(&values));

        let values = ["c"];
        assert!(parsed.calc(&values));


        let expr = "^(bad,";
        let parsed = parse(expr, &ops);
        assert!(parsed.is_err());
    }
}