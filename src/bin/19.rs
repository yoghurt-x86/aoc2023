use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

use std::cmp;

extern crate combine;
use combine::{attempt, between};
use combine::{many1, Parser, sep_by, choice, many};
use combine::parser::char::{newline, spaces, char, digit, letter};


#[derive(Debug, Clone)]
pub enum Res {
    Goto(String),
    Reject,
    Accept,
}

#[derive(Debug, Clone, Copy)]
pub enum Cmp {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
pub enum Expr {
    If{ variable: char, cmp : Cmp, value: i32, result: Res },
    Result(Res),
}

#[derive(Debug, Clone, Copy)]
pub struct Params {
    x : i32,
    m : i32,
    a : i32,
    s : i32,
}

fn main() {
    let integer = || many1(digit()).map(|s : String| s.parse::<i32>().unwrap());
    let name = || many1(letter())
        .map(|name : String| name);
    let cmp = || choice((char('>').map(|_| Cmp::GreaterThan),char('<').map(|_| Cmp::LessThan)));
    let res = || choice(
        (char('A').map(|_| Res::Accept), 
         char('R').map(|_| Res::Reject),  
         name().map(|name| Res::Goto(name))
        ));
    let if_expr = || (letter(), cmp(), integer(), char(':'), res())
        .map(|(variable, cmp, value, _, result)| 
            Expr::If{ variable, cmp, value, result}
            );
    let expr = || attempt(if_expr())
        .or(res().map(|r| Expr::Result(r)));
    let expressions = || between(char('{'), char('}'), sep_by(expr(), char(',')));
    let rule = || name()
        .and(expressions())
        .skip(newline());


    let part_parameter = |c| char(c)
        .skip(char('='))
        .with(integer());

    let part_parameters = (
        part_parameter('x'),
        char(','),
        part_parameter('m'),
        char(','),
        part_parameter('a'),
        char(','),
        part_parameter('s'),
        ).map(|(x,_,m,_,a,_,s)| 
            Params{x, m, a, s}
        );

    let part = || between(char('{'), char('}'), part_parameters).skip(newline());
        
    let content = || many1(rule())
        .skip(spaces())
        .and(many(part()));

    let contents = fs::read_to_string("./19.input2")
        .expect("Should have been able to read the file");

    let blah : (Vec<(String, Vec<_>)>, Vec<_>)= content().parse(contents.as_str()).unwrap().0;

    println!("{:?}", blah);

    let map = blah.0.into_iter().collect::<HashMap<String,_>>();


    let evaluate = |exprs : &Vec<Expr>, part : &Params| {
        for e in exprs {
            match e {
                Expr::If { variable, cmp, value, result } => {
                    let v1 = match variable {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => panic!("shit"),
                    };

                    let eval = match cmp {
                        Cmp::LessThan => v1 < *value,
                        Cmp::GreaterThan => v1 > *value,
                    };

                    if eval {
                        return result.clone()
                    } else {
                        continue
                    }
                },
                Expr::Result(res) => return res.clone(),
            }
        }
        panic!("fuck!");
    };

    let mut sum = 0;
    for part in blah.1 {
        let in_ = map.get("in").unwrap();
        let mut res = evaluate(in_, &part);

        loop {
            match res {
                Res::Goto(hmm) => {
                    res = evaluate(map.get(&hmm).unwrap(), &part)
                },
                Res::Accept => break,
                Res::Reject => break,
            }
        }

        let part_sum = part.x + part.a + part.m + part.s;
        match res {
            Res::Accept => { sum = sum + part_sum },
            _ => {},
        }
    }

    println!("Ass:: {}", sum);

}

