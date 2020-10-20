use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum StreamCtl {
    Encode = 0x0,
    Decode = 0x1,
}

#[derive(FromPrimitive)]
pub enum EncodeCtl {
    Stream = 0x0,
    Model = 0x1,
    Archive = 0x2,
    Destination = 0x3,
    Algorithm = 0x4,
    Piggyback = 0x5,
}

#[derive(FromPrimitive)]
pub enum DecodeCtl {
    Stream = 0x0,
    Model = 0x1,
    Archive = 0x2,
    Source = 0x3,
    Algorithm = 0x4,
    Piggyback = 0x5,
    ModelStream = 0x6,
    BlobStream = 0x7,
}

