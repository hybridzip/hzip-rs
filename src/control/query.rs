use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum QueryCtl {
    Archive = 0x0,
    CheckIfFileExists = 0x1,
    ListFileSystem = 0x2,
    PiggyBack = 0x3,
    DeleteFile = 0x4,
    DeleteModel = 0x5,
    GetMemUsage = 0x6,
    Target = 0x7,
}
