
#[macro_use] extern crate enum_primitive;
use enum_primitive::FromPrimitive;

use bytes::{Bytes, Buf};
mod tiff;


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

    let ifd_offset = res.slice(4..8).get_u16();

    let header = tiff::Header {
        byte_order: tiff::ByteOrder::from_u16(order.get_u16()).unwrap(),
        tiff_version: tiff_version,
        ifd_offset: ifd_offset
    };

    println!("{:?}", header);
}