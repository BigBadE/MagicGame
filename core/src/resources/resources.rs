use json::JsonValue;
use crate::world::pixel::PixelType;

pub struct ResourceManager {

}

pub trait JsonResource {

}

impl ResourceManager {
    pub fn new(json_string: &str) -> Self {
        let mut resources: Vec<Box<dyn JsonResource>> = Vec::new();

        for value in json::parse(json_string) {
            if value.is_empty() {
                continue
            }

            resources.push(ResourceManager::create_type(
                value["name"].as_str().expect("No name for top-level JSON element"), &value))
        }

        return ResourceManager {

        };
    }

    fn create_type(name: &str, values: &JsonValue) -> Box<dyn JsonResource> {
        match name {
            _ => panic!("Unrecognized JSON type {}", name)
        }
    }
}