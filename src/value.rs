use std::fmt::Display;

pub enum ValueType {
    Str,
    Num,
    Bool,
    Null,
    Lst,
    Obj,
}

pub struct Value<'a> {
    inp: &'a str,
    typ: ValueType,
    pos: usize,
    len: usize,
}

impl<'a> Value<'a> {
    pub fn from_args(inp: &'a str, typ: ValueType, pos: usize, len: usize) -> Self {
        Value { inp, typ, pos, len }
    }

    pub fn query(&self, query: String) -> Self {
        todo!()
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = self.inp[self.pos..self.pos + self.len].to_string();
        writeln!(f, "{}", o)
    }
}