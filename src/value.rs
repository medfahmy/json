use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Literal(&'a str),
    List {
        slice: &'a str,
        items: Vec<Value<'a>>,
    },
    Obj {
        slice: &'a str,
        map: HashMap<&'a str, Value<'a>>,
    },
}

use Value::*;

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Literal(s) => s,
            List { slice, .. } => slice,
            Obj { slice, .. } => slice,
        };

        writeln!(f, "{}", output)
    }
}
