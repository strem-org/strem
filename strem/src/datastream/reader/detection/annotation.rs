use std::collections::HashMap;

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use super::Dimensions;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Translation {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BoundingBox {
    pub dimensions: Dimensions,
    pub translation: Translation,
}

impl BoundingBox {
    pub fn min(&self) -> (f64, f64) {
        let x = self.translation.x - (self.dimensions.width / 2.0);
        let y = self.translation.y - (self.dimensions.height / 2.0);

        (x, y)
    }

    pub fn max(&self) -> (f64, f64) {
        let x = self.translation.x + (self.dimensions.width / 2.0);
        let y = self.translation.y + (self.dimensions.height / 2.0);

        (x, y)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Annotation {
    pub category: usize,
    pub score: f64,
    pub bbox: BoundingBox,
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<usize, Vec<Annotation>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct AnnotationVisitor;

    impl<'de> Visitor<'de> for AnnotationVisitor {
        type Value = HashMap<usize, Vec<Annotation>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of annotations")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut map = HashMap::new();

            while let Some(annotation) = seq.next_element::<Annotation>()? {
                map.entry(annotation.category)
                    .or_insert_with(Vec::new)
                    .push(annotation);
            }

            Ok(map)
        }
    }

    deserializer.deserialize_seq(AnnotationVisitor)
}
