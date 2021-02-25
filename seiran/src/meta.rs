use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Deserializer, Serialize};
use std::{borrow::Cow, cmp, fmt::Display, io, io::Write, ops::Sub, str::FromStr};

fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub name: String,
    pub media_link: String,
    pub id: String,
    pub md5_hash: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub size: u32,
}

impl Meta {
    pub fn name(&self) -> String {
        self.name.split('/').last().unwrap_or_default().to_owned()
    }
}

impl cmp::PartialEq for Meta {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl cmp::Eq for Meta {}

#[derive(Deserialize)]
struct ListObjects {
    items: Vec<Meta>,
}

impl From<ListObjects> for MetaTable {
    fn from(input: ListObjects) -> MetaTable {
        Self {
            items: input.items,
            update_at: chrono::offset::Local::now().to_rfc3339(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "ListObjects")]
pub struct MetaTable {
    items: Vec<Meta>,
    update_at: String,
}

impl Default for MetaTable {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            update_at: chrono::offset::Local::now().to_rfc3339(),
        }
    }
}

impl Sub for MetaTable {
    type Output = Vec<Meta>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.items
            .into_iter()
            .filter_map(|meta| (!rhs.items.contains(&meta)).then_some(meta))
            .collect()
    }
}

pub async fn fetch<'a>(uri: &str) -> Result<Cow<'a, MetaTable>> {
    print!("Fetch meta...");
    io::stdout().flush().unwrap();
    let res = reqwest::get(uri).await?.json().await?;
    println!("{}", "OK".green());
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sub() -> anyhow::Result<()> {
        let meta1 = Meta {
            name: "ccc".into(),
            id: "1".into(),
            media_link: "aaa".into(),
            md5_hash: "aaa".into(),
            size: 3333,
        };
        let meta2 = Meta {
            name: "ccc2".into(),
            id: "2".into(),
            media_link: "aaa".into(),
            md5_hash: "aaa".into(),
            size: 3333,
        };
        let table1: MetaTable = ListObjects { items: vec![meta1] }.into();
        let table2: MetaTable = ListObjects { items: vec![meta2] }.into();
        let sub = table1 - table2;
        assert_eq!("1", sub.first().unwrap().id);
        Ok(())
    }
}
