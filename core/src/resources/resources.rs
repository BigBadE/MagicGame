use std::collections::HashMap;
use json::JsonValue;
use crate::world::pixel::PixelType;

pub struct ResourceManager {

}

pub trait JsonResource {

}

impl ResourceManager {
    pub fn new(json_string: &str) -> Self {
        let mut resources: Vec<Box<dyn JsonResource>> = Vec::new();

        let mut counts = HashMap::new();

        for value in json::parse(json_string) {
            if value.is_empty() {
                continue
            }

            let json_type = value["type"].as_str().expect("No name for top-level JSON element");
            resources.push(ResourceManager::create_type(
                json_type, &value, counts[json_type]));

            if counts.contains_key(json_type) {
                counts[json_type] += 1;
            } else {
                counts.insert(json_type, 1);
            }
        }

        return ResourceManager {

        };
    }

    fn create_type(value_type: &str, values: &JsonValue, index: u32) -> Box<dyn JsonResource> {
        match value_type {
            "pixel" => Box::new(PixelType::new(values, index)),
            _ => panic!("Unrecognized JsonResource type {}", value_type)
        }
    }
}