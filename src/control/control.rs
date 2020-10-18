use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum CommonCtl {
    Success = 0x0,
    PiggyBack = 0x1,
    Error = 0xff
}