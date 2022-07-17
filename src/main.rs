
use bytes::{Bytes, BytesMut, Buf, BufMut};


#[tokio::main]
async fn main() {
    // Create a client.
    let client = reqwest::Client::new();
    
    // Request header.
    let url = "https://naipeuwest.blob.core.windows.net/naip/v002/tx/2020/tx_060cm_2020/26097/m_2609719_se_14_060_20201217.tif";
    let res = client.get(url).header("Range", "bytes=0-65536").send().await.unwrap().bytes().await.unwrap();


    // Parse tiff header
    // Endianness
    let order = res.slice(0..2);
    let little_endian = Bytes::from(&b"II"[..]);
    let big_endian = Bytes::from(&b"MM"[..]);

    if order.eq(&little_endian) {
        println!("It's little endian!")
    } else if order.eq(&big_endian) {
        println!("It's big endian!")
    }

    // Magic number.
    let magic_number = res.slice(2..4).get_i8();

    if magic_number != 42 {
        panic!("not a tiff")
    };

}