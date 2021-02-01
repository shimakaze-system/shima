use serde::{Deserialize, Deserializer, Serialize};
use std::{borrow::Cow, cmp, collections::HashMap, fmt::Display, ops::Sub, str::FromStr};

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
pub struct Meta<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    media_link: Cow<'a, str>,
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow)]
    md5_hash: Cow<'a, str>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    size: u32,
}

impl<'a> cmp::PartialEq for Meta<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> cmp::Eq for Meta<'a> {}

#[derive(Deserialize)]
struct ListObjects<'a> {
    #[serde(borrow)]
    items: Vec<Meta<'a>>,
}

impl<'a> From<ListObjects<'a>> for MetaTable<'a> {
    fn from(input: ListObjects<'a>) -> MetaTable<'a> {
        let items: HashMap<Cow<'a, str>, Meta<'a>> =
            input.items.into_iter().map(|meta| (meta.id.clone(), meta)).collect();
        Self {
            items,
            update_at: chrono::offset::Local::now().to_rfc3339().into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "ListObjects")]
pub struct MetaTable<'a> {
    items: HashMap<Cow<'a, str>, Meta<'a>>,
    #[serde(borrow)]
    update_at: Cow<'a, str>,
}

impl<'a> Sub for MetaTable<'a> {
    type Output = Vec<Meta<'a>>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.items
            .into_iter()
            .filter_map(|(id, meta)| (!rhs.items.contains_key(&id)).then_some(meta))
            .collect()
    }
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
