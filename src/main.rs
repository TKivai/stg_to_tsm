use color_eyre::{eyre::eyre, eyre::Context};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct TSMConfig {
    windows: HashMap<String, HashMap<String, TSMTab>>,
    name: String,

    #[serde(rename(deserialize = "tabsNumber"))]
    tabs_number: usize,

    date: u64,
    tag: String,

    #[serde(rename(deserialize = "sessionStartTime"))]
    session_start_time: String,
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

    let result = serde_json::from_str::<Vec<TSMConfig>>(&text)?;

    for session in result {
        for window in session.windows.values() {
            match is_valid_tabs_number(window, session.tabs_number) {
                Ok(valid) => {
                    if valid {
                        println!("Valid window")
                    } else {
                        println!("Inalid window")
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
    }

    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input_file = "src/demo_data/tsm.json";
    let input =
        std::fs::read_to_string(input_file).wrap_err(format!("Error reading {}", input_file))?;
    Ok(input)
}

fn is_valid_tabs_number(
    window: &HashMap<String, TSMTab>,
    tabs_number_in_file: usize,
) -> color_eyre::Result<bool> {
    let tabs = window.values().collect::<Vec<_>>().len();
    if tabs == tabs_number_in_file {
        Ok(true)
    } else {
        Err(eyre!("Incorrect number of tabs"))
    }
}
