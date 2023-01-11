use std::fmt::Debug;
use std::str::FromStr;
use regex::Regex;

use crate::AocDay;

pub struct Parser {
    pattern: Regex,
}

impl Parser {

    pub fn new(pattern: &str) -> Parser {
        Parser { pattern: Regex::new(pattern).unwrap() }
    }

    pub fn parse<A>(&self, input: &str) -> Option<A> where A: FromStr, <A as FromStr>::Err: Debug{
        self.pattern.captures(input).map(|captures| {
            captures.get(1).unwrap().as_str().parse::<A>().unwrap()
        })
    }

    pub fn parse2<A, B>(&self, input: &str) -> Option<(A, B)>
        where A: FromStr + Debug, <A as FromStr>::Err: Debug,
              B: FromStr + Debug, <B as FromStr>::Err: Debug {

        self.pattern.captures(input).map(|captures| {
            let a: Result<A, _> = captures.get(1).unwrap().as_str().parse::<A>();
            let b: Result<B, _> = captures.get(2).unwrap().as_str().parse::<B>();
            (a.unwrap(), b.unwrap())
        })
    }

    pub fn parse3<A, B, C>(&self, input: &str) -> Option<(A, B, C)>
        where A: FromStr + Debug, <A as FromStr>::Err: Debug,
              B: FromStr + Debug, <B as FromStr>::Err: Debug,
              C: FromStr + Debug, <C as FromStr>::Err: Debug {

        self.pattern.captures(input).map(|captures| {
            let a: Result<A, _> = captures.get(1).unwrap().as_str().parse::<A>();
            let b: Result<B, _> = captures.get(2).unwrap().as_str().parse::<B>();
            let c: Result<C, _> = captures.get(3).unwrap().as_str().parse::<C>();
            (a.unwrap(), b.unwrap(), c.unwrap())
        })
    }

    pub fn parse4<A, B, C, D>(&self, input: &str) -> Option<(A, B, C, D)>
        where A: FromStr + Debug, <A as FromStr>::Err: Debug,
              B: FromStr + Debug, <B as FromStr>::Err: Debug,
              C: FromStr + Debug, <C as FromStr>::Err: Debug,
              D: FromStr + Debug, <D as FromStr>::Err: Debug {

        self.pattern.captures(input).map(|captures| {
            let a: Result<A, _> = captures.get(1).unwrap().as_str().parse::<A>();
            let b: Result<B, _> = captures.get(2).unwrap().as_str().parse::<B>();
            let c: Result<C, _> = captures.get(3).unwrap().as_str().parse::<C>();
            let d: Result<D, _> = captures.get(4).unwrap().as_str().parse::<D>();
            (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap())
        })
    }

}
