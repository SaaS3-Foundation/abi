use abi::{Param, ABI};
use primitive_types::U256;

fn main() {
    let matchs = vec![
        ("Qatar", "Ecuador"),
        ("England", "IR Iran"),
        ("Senegal", "Netherlands"),
        ("USA", "Wales"),
        ("Argentina", "Saudi Arabia"),
        ("Denmark", "Tunisia"),
        ("Mexico", "Poland"),
        ("France", "Australia"),
        ("Morocco", "Croatia"),
        ("Germany", "Japan"),
        ("Spain", "Costa Rica"),
        ("Belgium", "Canada"),
        ("Switzerland", "Cameroon"),
        ("Uruguay", "Korea Republic"),
        ("Portugal", "Ghana"),
        ("Brazil", "Serbia"),
        ("Wales", "IR Iran"),
        ("Qatar", "Senegal"),
        ("Netherlands", "Ecuador"),
        ("England", "USA"),
        ("Tunisia", "Australia"),
        ("Poland", "Saudi Arabia"),
        ("France", "Denmark"),
        ("Argentina", "Mexico"),
        ("Japan", "Costa Rica"),
        ("Belgium", "Morocco"),
        ("Croatia", "Canada"),
        ("Spain", "Germany"),
        ("Cameroon", "Serbia"),
        ("Korea Republic", "Ghana"),
        ("Brazil", "Switzerland"),
        ("Portugal", "Uruguay"),
        ("Netherlands", "Qatar"),
        ("Ecuador", "Senegal"),
        ("IR Iran", "USA"),
        ("Wales", "England"),
        ("Australia", "Denmark"),
        ("Tunisia", "France"),
        ("Saudi Arabia", "Mexico"),
        ("Poland", "Argentina"),
        ("Croatia", "Belgium"),
        ("Canada", "Morocco"),
        ("Costa Rica", "Germany"),
        ("Japan", "Spain"),
        ("Ghana", "Uruguay"),
        ("Korea Republic", "Portugal"),
        ("Cameroon", "Brazil"),
        ("Serbia", "Switzerland"),
        ("1. FC Köln", "TSG 1899 Hoffenheim"),
    ];
    // assert_eq!(matchs.len(), 48);
    for (pos, v) in matchs.iter().enumerate() {
        let params = vec![
            Param::String32 {
                name: "home".to_owned(),
                value: v.0.to_string().to_owned(),
            },
            Param::String32 {
                name: "guest".to_owned(),
                value: v.1.to_string().to_owned(),
            },
        ];
        let res: Vec<U256> = ABI::new(params).encode().unwrap();

        let h = res
            .iter()
            .map(|x| format!("{:#x}", x).trim_start_matches("0x").to_owned())
            .collect::<String>();
        println!("game_parameters[{}] = bytes(\"0x{}\");", pos + 1, h);
    }
}
