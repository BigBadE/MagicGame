use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use json::JsonValue;
use crate::world::pixel::PixelType;

const JSON: &str = include_str!("../../../target/output.json");

pub struct ResourceManager {
    resources: HashMap<String, HashMap<String, Box<dyn JsonResource>>>,
}

pub trait JsonResource {
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut resources: HashMap<String, HashMap<String, Box<dyn JsonResource>>> = HashMap::new();

        for value in json::parse(JSON) {
            if value.is_empty() {
                continue;
            }

            let json_type = value["type"].as_str().expect("No name for top-level JSON element").to_string();
            let name = value["name"].as_str().expect("No name for top-level JSON element").to_string();

            if resources.contains_key(&json_type) {
                resources[&json_type].insert(name, ResourceManager::create_type(
                    &json_type, &value, resources[&json_type].len()));
            } else {
                resources.insert(json_type, HashMap::from([(name, ResourceManager::create_type(
                    &json_type, &value, 0))]));
            }
        }

        return ResourceManager {
            resources
        };
    }

    fn create_type(value_type: &str, values: &JsonValue, index: usize) -> Box<dyn JsonResource> {
        match value_type {
            "PixelType" => Box::new(PixelType::new(values, index)),
            _ => panic!("Unrecognized JsonResource type {}", value_type)
        }
    }
}