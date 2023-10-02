use std::collections::HashMap;

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub supercategory: Option<String>,
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Category>, D::Error>
where
    D: Deserializer<'de>,
{
    struct CategoryVisitor;

    impl<'de> Visitor<'de> for CategoryVisitor {
        type Value = HashMap<String, Category>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of categories")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut map = HashMap::new();

            while let Some(category) = seq.next_element::<Category>()? {
                map.insert(category.name.clone(), category);
            }

            Ok(map)
        }
    }

    deserializer.deserialize_seq(CategoryVisitor)
}
