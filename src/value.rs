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
}
