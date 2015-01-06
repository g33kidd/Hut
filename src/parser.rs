use std::str::FromStr;
use regex::{Regex, Captures};

use abs::Expr::{self, Id, LitInt, Neg, Plus, Minus};
use abs::Stm::{self, Vardef, Assign};
use abs::Type;

#[derive(Show)]
pub struct Line<'a> {
    pub content: &'a str
}

struct ParseRule {
    name: String,
    regex: Regex,
}
pub struct Parser {
    rules: Vec<ParseRule>
}

impl Parser {

    pub fn new() -> Parser {
        let id = r"([:lower:][:alnum:]*)";
        let typ = r"([:upper:][:alnum:]*)";
        let litint = r"([:digit:]+)";
        let expr = r"(.*)";

        let parse_patterns = vec![
            ("Vardef", vec![id, r" :: ", typ]),
            ("Assign", vec![id, r" = ", expr]),

            ("Type", vec![typ]),

            ("Id", vec![id]),
            ("LitInt", vec![litint]),
            ("Plus", vec![expr, r" \+ ", expr]),
            ("Minus", vec![expr, r" - ", expr]),
            ("Neg", vec![r"-", expr]),
        ];

        let mut rules = vec![];
        for pp in parse_patterns.iter() {
            let (name, ref pattern_parts) = *pp;
            let mut regex_string = String::new();
            regex_string.push_str("^");
            for part in pattern_parts.iter() {
                regex_string.push_str(*part);
            }
            regex_string.push_str("$");
            let regex = Regex::new(regex_string.as_slice()).unwrap();
            rules.push(ParseRule {name: String::from_str(name), regex: regex});
        }
        return Parser {rules: rules};
    }

    pub fn parse(&self, s: Vec<Line>) -> Vec<Stm> {
        let mut res: Vec<Stm> = vec![];
        for line in s.iter() {
            let l = self.parse_stm((*line).content);
            res.push(l);
        }
        return res;
    }

    fn parse_stm(&self, s: &str) -> Stm {
        for rule in self.rules.iter() {
            if rule.regex.is_match(s) {
                let c = rule.regex.captures(s).expect("No captures");
                return match rule.name.as_slice() {
                    "Vardef" => self.vardef(c),
                    "Assign" => self.assign(c),
                    _ => panic!("Bad match: {}", rule.name)
                };
            }
        }
        panic!("No match: {}", s);
    }

    fn vardef(&self, cap: Captures) -> Stm {
        let e = self.parse_expr(cap.at(1).unwrap());
        let t = cap.at(2).and_then(FromStr::from_str).unwrap();
        return Vardef(e, Type(t));
    }

    fn assign(&self, cap: Captures) -> Stm {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Assign(e1, e2);
    }

    fn parse_expr(&self, s: &str) -> Expr {
        for rule in self.rules.iter() {
            if rule.regex.is_match(s) {
                let c = rule.regex.captures(s).expect("No captures");
                return match rule.name.as_slice() {
                    "Id" => self.id(c),
                    "LitInt" => self.litint(c),
                    "Neg" => self.neg(c),
                    "Plus" => self.plus(c),
                    "Minus" => self.minus(c),
                    _ => panic!("Bad match: {}", rule.name)
                };
            }
        }
        panic!("No match: {}", s);
    }

    fn id(&self, cap: Captures) -> Expr {
        let s = cap.at(1).and_then(FromStr::from_str).unwrap();
        return Id(s);
    }

    fn litint(&self, cap: Captures) -> Expr {
        let i = cap.at(1).and_then(FromStr::from_str).unwrap();
        return LitInt(i);
    }

    fn neg(&self, cap: Captures) -> Expr {
        let e = self.parse_expr(cap.at(1).unwrap());
        return Neg(box e);
    }

    fn plus(&self, cap: Captures) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Plus(box e1, box e2);
    }

    fn minus(&self, cap: Captures) -> Expr {
        let e1 = self.parse_expr(cap.at(1).unwrap());
        let e2 = self.parse_expr(cap.at(2).unwrap());
        return Minus(box e1, box e2);
    }
}