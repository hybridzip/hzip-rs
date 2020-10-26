use num_derive::FromPrimitive;

#[derive(FromPrimitive, PartialEq)]
pub enum CommonCtl {
    Success = 0x0,
    Error = 0xff,
}

#[derive(FromPrimitive)]
pub enum ApiCtl {
    Stream = 0x0,
    Query = 0x1,
    HealthCheck = 0x2,
    Close = 0xff,
}
