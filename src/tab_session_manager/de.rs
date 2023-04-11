use serde::{de::Visitor, Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct TSMObject<T> {
    pub id: String,
    pub value: T,
}

#[derive(Debug, Deserialize)]
pub struct TSMConfig(Vec<TSMSession>);

impl IntoIterator for TSMConfig {
    type Item = TSMSession;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// #[derive(Debug, Deserialize)]
#[derive(Debug, Default)]
pub struct TSMSession {
    // windows: Vec<HashMap<String, TSMWindow>>,
    pub windows: Vec<TSMObject<TSMWindow>>,
    pub name: String,
    // #[serde(rename(deserialize = "tabsNumber"))]
    pub tabs_number: usize,
    pub date: u64,
    pub tag: String,

    // #[serde(rename(deserialize = "sessionStartTime"))]
    session_start_time: String,
}

impl TSMSession {
    pub fn check_valid_tabs(&self) -> bool {
        let tabs_number_counted = self
            .windows
            .iter()
            .fold(0, |acc, window| acc + window.value.tabs.len());

        if tabs_number_counted == self.tabs_number {
            return true;
        }

        false
    }
}

impl<'de> Deserialize<'de> for TSMSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(TSMSessionVisitor {})
    }
}

struct TSMSessionVisitor {}

impl<'de> Visitor<'de> for TSMSessionVisitor {
    type Value = TSMSession;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize session")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut session = TSMSession {
            ..Default::default()
        };

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "windows" => {
                    let windows_map = map.next_value::<HashMap<String, TSMWindow>>()?;
                    let windows: Vec<TSMObject<TSMWindow>> = windows_map
                        .iter()
                        .map(|(window_id, window_map)| TSMObject {
                            id: window_id.clone(),
                            value: window_map.clone(),
                        })
                        .collect();

                    session.windows = windows;
                }
                "tabsNumber" => session.tabs_number = map.next_value()?,
                "name" => session.name = map.next_value()?,
                "date" => session.date = map.next_value()?,
                "tag" => session.tag = map.next_value()?,
                "sessionStartTime" => session.session_start_time = map.next_value()?,
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        Ok(session)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TSMWindow {
    pub tabs: Vec<TSMObject<TSMTab>>,
}

impl<'de> Deserialize<'de> for TSMWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(TSMWindowVisitor {})
    }
}

struct TSMWindowVisitor {}

impl<'de> Visitor<'de> for TSMWindowVisitor {
    type Value = TSMWindow;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize session")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut window = TSMWindow {
            ..Default::default()
        };

        let mut tabs_vec:Vec<TSMObject<TSMTab>> = Vec::new();

        while let Some(key) = map.next_key::<String>()? {
            tabs_vec.push(TSMObject { id: key, value: map.next_value::<TSMTab>()? });
        }

        window.tabs = tabs_vec;

        Ok(window)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TSMTab {
    pub url: String,
    pub title: String,
    #[serde(rename(deserialize = "favIconUrl"))]
    pub fav_icon_url: String,
}