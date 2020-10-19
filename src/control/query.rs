use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum QueryCtl {
    Archive = 0x0,
    CheckIfFileExists = 0x1,
    GetAllFiles = 0x2,
    PiggyBack = 0x3,
}