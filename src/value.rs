use std::collections::HashMap;

pub enum ValueKind {
    String,
    Number,
    List,
    Object,
}

pub struct Value<'a> {
    input: &'a str,
    kind: ValueKind,
    line: usize,
    row: usize,
}
