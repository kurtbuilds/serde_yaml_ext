use serde_yaml::mapping::{Index, Keys};
use serde_yaml::Value;

pub trait ValueExt {
    fn remove(&mut self, key: &str) -> Option<Value>;
    fn insert(&mut self, key: impl Into<Value>, value: impl Into<Value>) -> Option<Value>;
    fn path_mut(&mut self, path: &str) -> &mut Value;
    fn path(&self, path: &str) -> &Self;
    fn get_path(&self, path: &str) -> Option<&Value>;
    fn get_path_mut(&mut self, path: &str) -> Option<&mut Value>;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value> + 'a>;
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Value> +'a>;
    fn into_iter(self) -> Box<dyn Iterator<Item=Value>>;
    fn keys(&self) -> serde_yaml::mapping::Keys;
}

impl ValueExt for Value {
    fn remove(&mut self, index: &str) -> Option<Self> {
        let mapping = self.as_mapping_mut()?;
        index.remove_from(mapping)
    }

    fn insert(&mut self, key: impl Into<Self>, value: impl Into<Self>) -> Option<Self> {
        let mapping = self.as_mapping_mut()?;
        mapping.insert(key.into(), value.into())
    }

    fn path_mut(&mut self, path: &str) -> &mut Self {
        let mut value = self;
        for part in path.split('.') {
            if value.get(part).is_some() {
                value = value.get_mut(part).unwrap();
                continue;
            }
            value = part.parse::<usize>().ok()
                .and_then(|idx| value.get_mut(idx))
                .expect("path not found (mut)");
        }
        value
    }

    fn path(&self, path: &str) -> &Self {
        let mut value = self;
        for part in path.split('.') {
            value = value.get(part)
                .or_else(|| {
                    let idx = part.parse::<usize>().ok()?;
                    value.get(idx)
                })
                .expect("path not found")
        }
        value
    }

    fn get_path(&self, path: &str) -> Option<&Self> {
        let mut value = self;
        for part in path.split('.') {
            value = value.get(part)
                .or_else(|| {
                    let idx = part.parse::<usize>().ok()?;
                    value.get(idx)
                })?;
        }
        Some(value)
    }

    fn get_path_mut(&mut self, path: &str) -> Option<&mut Self> {
        let mut value = self;
        for part in path.split('.') {
            if value.get(part).is_some() {
                value = value.get_mut(part).unwrap();
                continue;
            }
            value = part.parse::<usize>().ok()
                .and_then(|idx| value.get_mut(idx))?;
        }
        Some(value)
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Value> + 'a> {
        match self {
            Value::Sequence(seq) => Box::new(seq.iter()),
            Value::Mapping(map) => Box::new(map.values()),
            _ => panic!("Value must be map or seq to iterate"),
        }
    }

    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Self> + 'a> {
        match self {
            Value::Sequence(seq) => Box::new(seq.iter_mut()),
            Value::Mapping(map) => Box::new(map.values_mut()),
            _ => panic!("Value must be map or seq to iterate (mut)"),
        }
    }

    fn into_iter(self) -> Box<dyn Iterator<Item=Self>> {
        match self {
            Value::Sequence(seq) => Box::new(seq.into_iter()),
            Value::Mapping(map) => Box::new(map.into_values()),
            _ => panic!("Value must be map or seq to iterate (owned)"),
        }
    }

    fn keys(&self) -> Keys {
        match self {
            Value::Mapping(map) => map.keys(),
            _ => panic!("Value must be map to get keys"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
