
#[macro_use] extern crate enum_primitive;
use enum_primitive::FromPrimitive;

use bytes::{Bytes, Buf};
mod tiff;
use std::mem::size_of;


#[tokio::main]
async fn main() {
    // Create a client.
    let client = reqwest::Client::new();
    
    // Read the header.
    let url = "https://naipeuwest.blob.core.windows.net/naip/v002/tx/2020/tx_060cm_2020/26097/m_2609719_se_14_060_20201217.tif";
    let res = client.get(url).header("Range", "bytes=0-65536").send().await.unwrap().bytes().await.unwrap();
    let mut order: Bytes = res.slice(0..2);

    let tiff_version = res.slice(2..4).get_u8();
    if tiff_version != 42 {
        panic!("not a tiff")
    }

    let ifd_offset = res.slice(4..8).get_u32_le();

    let header = tiff::Header {
        byte_order: tiff::ByteOrder::from_u16(order.get_u16_le()).unwrap(),
        tiff_version: tiff_version,
        ifd_offset: ifd_offset
    };
    println!("{:?}", header);
    
    let mut next_ifd_offset = ifd_offset as usize;

    while next_ifd_offset != 0 {
        // First 2 bytes contain number of tags in the IFD.
        let tag_count = res.slice(next_ifd_offset..next_ifd_offset+2).get_u16_le();
        
        // Read each tag
        for idx in 0..tag_count {
            // Tags are always 12 bytes each.
            let tag_start = next_ifd_offset + 2 + (12 * idx) as usize;
            
            // First 2 bytes contain tag code.
            let tag_code = res.slice(tag_start..tag_start+2).get_u16_le();
            
            // Bytes 2-4 contain the tag's field type.
            let tag_field_type = res.slice(tag_start+2..tag_start+4).get_u16_le();
            let tag_type = tiff::TagType::from_u16(tag_field_type).unwrap();
            let tag_size = tiff::tag_size(&tag_type);

            // Bytes 4-8 contain the number of values in the tag.
            let tag_count = res.slice(tag_start+4..tag_start+8).get_u32_le();

            // Bytes 8-12 contain the tag value if it fits, otherwise an offset
            // to where the tag value is stored.
            let tag_value_size = tag_count * tag_size;

            if tag_value_size <= 4 {
                let tag_value = res.slice(tag_start+8..tag_start+12);
            } else {
                // println!("Skipping long tag {:?}", tag_code)
            }
        }
        // Last 4 bytes of IFD contains offset to the next IFD
        let start = next_ifd_offset + 2 + (tag_count * 12) as usize;
        next_ifd_offset = res.slice(start..start+4).get_u32_le() as usize;
    }

}
