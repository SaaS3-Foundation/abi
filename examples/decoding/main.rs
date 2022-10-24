use abi::ABI;
use hex_literal::hex;
use primitive_types::U256;

fn main() {
    let data: Vec<U256> = vec![
        hex!("3162000000000000000000000000000000000000000000000000000000000000").into(),
        hex!("54657374427974657333324e616d650000000000000000000000000000000000").into(),
        hex!("536f6d6520627974657333322076616c75650000000000000000000000000000").into(),
    ];
    let res: ABI = ABI::decode(&data, true).unwrap();
    println!("{:#?}", res);
}
