# SaaS3 protocol ABI

This library allows to encode and decode different types of data
during interaction between SaaS3 protocol and Ethereum smart contracts

Parameters from contract event logs are consumed as `Vec<U256>`, which avoids reading
random raw bytes and provides guarantee of a proper data alignment on input.

Second parameter of decoding is `strict` flag, which defines whether decoding
could be done into extended types (`String32`,`Bool`,`Date`)
that are actually represented as `Bytes32` on a protocol level.

### decoding example
```
use abi::ABI;
use primitive_types::U256;
use hex_literal::hex;

fn main() {
    let data: Vec<U256> = vec![
        hex!("3162000000000000000000000000000000000000000000000000000000000000").into(),
        hex!("54657374427974657333324e616d650000000000000000000000000000000000").into(),
        hex!("536f6d6520627974657333322076616c75650000000000000000000000000000").into(),
    ];
    let res: ABI = ABI::decode(&data, true).unwrap();
    println!("{:#?}", res);
}
```

### encoding example
```
use abi::{ABI, Param};
use primitive_types::U256;

fn main() {
    let param = Param::String {
        name: "hello".to_owned(),
        value: "world".to_owned(),
    };
    let res: Vec<U256> = ABI::new(vec![param]).encode().unwrap();
    println!("{:#?}", res);
}
```
Please see more examples for each type of the parameter in unit tests.

### Contribute
Run `cargo +nightly  no-std-check --manifest-path Cargo.toml` when you introduce new deps


### License
MIT
