use abi::{Param, ABI};
use ethereum_types::U256;

fn main() {
    let params = vec![Param::String32 {
        name: "coinIds".to_owned(),
        value: "ethereum".to_owned(),
    }, Param::String32 {
        name: "coinVs_currencies".to_owned(),
        value: "usd".to_owned(),
    }];
    let res: Vec<U256> = ABI::new(params).encode().unwrap();
    println!("{:#?}", res);
    let de= ABI::decode(&res, true).unwrap();
    println!("{:?}", de);
    println!("{:?}", de.get("coinIds").unwrap());
}
