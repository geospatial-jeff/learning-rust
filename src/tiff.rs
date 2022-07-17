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


enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug)]
    pub enum ByteOrder {
        LittleEndian = 0x4949,
        BigEndian = 0x4d4d,
    }
}


#[derive(Debug)]
pub struct Header {
    pub byte_order: ByteOrder,
    pub tiff_version: u8,
    pub ifd_offset: u16,
}