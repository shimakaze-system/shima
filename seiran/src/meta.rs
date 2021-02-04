use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    cmp,
    collections::HashMap,
    fmt::Display,
    ops::{Deref, Sub},
    str::FromStr,
};

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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub name: String,
    pub media_link: String,
    pub id: String,
    pub md5_hash: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub size: u32,
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
        let items: HashMap<String, Meta> = input.items.into_iter().map(|meta| (meta.id.clone(), meta)).collect();
        Self {
            items,
            update_at: chrono::offset::Local::now().to_rfc3339().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "ListObjects")]
pub struct MetaTable {
    items: HashMap<String, Meta>,
    update_at: String,
}

impl Sub for MetaTable {
    type Output = Vec<Meta>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.items
            .into_iter()
            .filter_map(|(id, meta)| (!rhs.items.contains_key(&id)).then_some(meta))
            .collect()
    }
}

pub async fn fetch(uri: String) -> Result<MetaTable> {
    Ok(reqwest::get(uri.deref()).await?.json().await?)
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
