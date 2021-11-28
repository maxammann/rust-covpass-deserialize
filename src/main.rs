use flate2::read::ZlibDecoder;
use std::io::Read;
use coset::CborSerializable;

fn main() {
    let img = image::open("/home/max/Downloads/vac.png").unwrap();

    // Use default decoder
    let decoder = bardecoder::default_decoder();

    let results = decoder.decode(&img);
    let result = results.first();
    let str_data = result.unwrap().as_ref().unwrap();
    let base45_str = str_data.replace("HC1:", "");
    println!("{}", str_data);
    let compressed = base45::decode(base45_str.as_str()).unwrap();

    let mut d = ZlibDecoder::new(compressed.as_slice());
    let mut data = Vec::new();
    d.read_to_end(&mut data).unwrap();

    let sign1 = serde_cose::from_slice(&data).unwrap();

    let payload = sign1.payload;
    println!("{:?}", sign1.signature);

    let recovered_object = sk_cbor::reader::read(payload.as_ref()).unwrap();
    println!("Deserializes to {:?}", recovered_object);
}
