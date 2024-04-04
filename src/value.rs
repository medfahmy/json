use std::fmt::Display;
use std::collections::HashMap;

pub enum Value<'a> {
    Null,
    Str(&'a str),
    Num(&'a str),
    Bool(&'a str),
    List { slice: &'a str, items: Vec<&'a str> },
    Obj { slice: &'a str, map: HashMap<&'a str, &'a str> },
}

use Value::*;

impl<'a> Value<'a> {
    pub fn query(&self, query: String) -> Self {
        todo!()
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Null => "null",
            Str(s) => s,
            Num(s) => s,
            Bool(s) => s,
            List { slice, .. } => slice,
            Obj { slice, .. } => slice,
        };

        writeln!(f, "{}", output)
    }
}
