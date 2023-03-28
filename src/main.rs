use color_eyre::eyre::Context;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct TSMConfig {
    windows: HashMap<String, HashMap<String, TSMTab>>,
    name: String,
    tabsNumber: u32,
    date: u64,
    tag: String,
    sessionStartTime: String,
}

// #[derive(Deserialize, Debug)]
// #[serde(untagged)]
// enum TSMWindow {
//     // #[serde(rename = "name")]
//     TSMTab(HashMap<String, TSMTab>),
// }

#[derive(Debug, Deserialize)]
struct TSMTab {
    id: f32,
    url: String,
    title: String,
    #[serde(rename(deserialize = "favIconUrl"))]
    fav_icon_url: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // Read STG JSON file as input
    let text = read_input()?;

    let result = serde_json::from_str::<Vec<TSMConfig>>(&text).unwrap();
    
    for session in result {
        println!("{:#?}", session);
    }
    // result.
    // println!("{:#?}", result);
    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input_file = "src/demo_data/tsm.json";
    let input = std::fs::read_to_string(input_file).wrap_err(format!("Reading {}", input_file))?;
    Ok(input)
}
