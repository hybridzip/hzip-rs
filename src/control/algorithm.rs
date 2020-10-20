use num_derive::FromPrimitive;

#[derive(Debug, Clone, FromPrimitive, PartialEq)]
pub enum Algorithm {
    Undefined = 0x0,
    Victini = 0x1
}