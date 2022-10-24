use abi::{Param, ABI};
use primitive_types::U256;

fn main() {
    // http://coingecko.com/en/api/simple/price?ids=ethereum&vs_currencies=usd
    let params = vec![
        Param::String32 {
            name: "coinIds".to_owned(),
            value: "ethereum".to_owned(),
        },
        Param::String32 {
            name: "coinVs_currencies".to_owned(),
            value: "usd".to_owned(),
        },
    ];
    let res: Vec<U256> = ABI::new(params).encode().unwrap();
    let de = ABI::decode(&res, true).unwrap();
    println!("{:?}", de);

    let h = res
        .iter()
        .map(|x| format!("{:#x}", x).trim_start_matches("0x").to_owned())
        .collect::<String>();
    println!("0x{}", h);
    assert_eq!(h, "3173730000000000000000000000000000000000000000000000000000000000636f696e49647300000000000000000000000000000000000000000000000000657468657265756d000000000000000000000000000000000000000000000000636f696e56735f63757272656e636965730000000000000000000000000000007573640000000000000000000000000000000000000000000000000000000000");
}
