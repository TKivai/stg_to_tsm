use color_eyre::{eyre::eyre, eyre::Context};
use serde::{de::Visitor, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct TSMConfig(Vec<TSMSession>);

impl IntoIterator for TSMConfig {
    type Item = TSMSession;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    // type IntoIter = TSMConfigIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// struct TSMConfigIntoIterator {
//     config: TSMConfig,
//     index: usize,
// }

// impl Iterator for TSMConfigIntoIterator {
//     type Item = TSMSession;
//     fn next(&mut self) -> Option<TSMSession> {
//         self.index += 1;
//         let val = self.;
//         val
//     }
// }

#[derive(Debug, Deserialize)]
// #[derive(Debug, Default)]
struct TSMSession {
    // windows: Vec<TSMWindow>,
    windows: HashMap<String, TSMWindow>,
    name: String,

    #[serde(rename(deserialize = "tabsNumber"))]
    tabs_number: usize,

    date: u64,
    tag: String,

    #[serde(rename(deserialize = "sessionStartTime"))]
    session_start_time: String,
}

impl TSMSession {
    fn check_valid_tabs(&self) -> bool {
        let tabs_number_counted = self
            .windows
            .iter()
            .fold(0, |acc, window| acc + window.1.0.len());

        if tabs_number_counted == self.tabs_number {
            return true;
        }

        false
    }
}

// impl<'de> Deserialize<'de> for TSMSession {

//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_map(TSMSessionVisitor {})
//     }
// }

// struct TSMSessionVisitor{}

// impl<'de> Visitor<'de> for TSMSessionVisitor {
//     type Value = TSMSession;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("Could not deserialize session")
//         // self.expecting(formatter)
//     }

//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//     where
//         A: serde::de::MapAccess<'de>,
//     {
//         let mut session = TSMSession {
//             ..Default::default()
//         };

//         // let mut window_map = None;

//         // while let Some(key) = map.next_key::<String>()? {
//         //     // while let Some(value) = map.next_value::<String>()? {

//         //     // }

//         //     if key == "windows" {
//         //         window_map = Some(map.next_value()?)
//         //             .ok_or_else(|| serde::de::Error::missing_field("Windows"))?;
//         //         let mut value_vec = Vec::from(window_map);

//         //         // session.windows.push(&mut value_vec);
//         //     } else {
//         //         let _: serde::de::IgnoredAny = map.next_value()?;
//         //     }
//         // }

//         // let new_windows_struct = HashMap::<String, TSMWindow>::try_from(window_map);

//         Ok(session)
//     }

// }

#[derive(Debug, Deserialize)]
struct TSMWindow(HashMap<String, TSMTab>);

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

    let result = serde_json::from_str::<TSMConfig>(&text)?;

    for session in result {
        if session.check_valid_tabs() {
            println!("Valid window")
        } else {
            println!("Inalid window")
        }
    }
    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input_file = "src/demo_data/tsm.json";
    let input =
        std::fs::read_to_string(input_file).wrap_err(format!("Error reading {}", input_file))?;

    // Ignore BOM
    // let char1 = input.chars().next().unwrap();
    // if char1 == '\u{0306}' {
    //     input = input.strip_prefix('\u{FEFF}').unwrap().to_string()
    // }

    Ok(input)
}