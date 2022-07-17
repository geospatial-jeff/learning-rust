// // Base types of the TIFF format.
// pub type BYTE      = u8;
// pub type SHORT     = u16;
// pub type LONG      = u32;
// pub type ASCII     = String;
// pub type RATIONAL  = (u32, u32);
// pub type SBYTE     = i8;
// pub type SSHORT    = i16;
// pub type SLONG     = i32;
// pub type SRATIONAL = (i32, i32);
// pub type FLOAT     = f32;
// pub type DOUBLE    = f64;
use std::collections::HashMap;


enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    pub enum ByteOrder {
        LittleEndian = 0x4949,
        BigEndian = 0x4d4d,
    }
}


// https://github.com/georust/geotiff/blob/7dd9068b7aa9e21b7c50f737a765d9d6d089a79a/src/lowlevel.rs#L26-L62
enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug,PartialEq)]
    pub enum TagType {
        ByteTag           = 1,
        ASCIITag          = 2,
        ShortTag          = 3,
        LongTag           = 4,
        RationalTag       = 5,
        SignedByteTag     = 6,
        UndefinedTag      = 7,
        SignedShortTag    = 8,
        SignedLongTag     = 9,
        SignedRationalTag = 10,
        FloatTag          = 11,
        DoubleTag         = 12,
    }
}

pub fn tag_size(t: &TagType) -> u32 {
    match *t {
        TagType::ByteTag           => 1,
        TagType::ASCIITag          => 1,
        TagType::ShortTag          => 2,
        TagType::LongTag           => 4,
        TagType::RationalTag       => 8,
        TagType::SignedByteTag     => 1,
        TagType::UndefinedTag      => 1,
        TagType::SignedShortTag    => 2,
        TagType::SignedLongTag     => 2,
        TagType::SignedRationalTag => 8,
        TagType::FloatTag          => 4,
        TagType::DoubleTag         => 8,
        _                          => 0,
    }
}


#[derive(Debug)]
pub struct Header {
    pub byte_order: ByteOrder,
    pub tiff_version: u8,
    pub ifd_offset: u32,
}