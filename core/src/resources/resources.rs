use std::borrow::Borrow;
use std::collections::HashMap;
use json::JsonValue;
use crate::world::pixel::PixelType;

const JSON: &str = include_str!("../../../target/output.json");

pub struct ResourceManager {
    resources: HashMap<String, HashMap<String, Box<dyn JsonResource>>>,
}

pub trait JsonResource {}

impl ResourceManager {
    pub fn new() -> Self {
        let mut resources: HashMap<String, HashMap<String, Box<dyn JsonResource>>> = HashMap::new();

        for value in json::parse(JSON).expect("No JSON bundled!").members() {
            if value.is_empty() {
                continue;
            }

            let json_type = value["type"].as_str().expect(
                format!("No type for top-level JSON element {}", value).as_str()).to_string();
            let name = value["name"].as_str().expect(
                format!("No name for top-level JSON element {}", value).as_str()).to_string();

            let mut index = 0;
            if resources.contains_key(&json_type) {
                index = resources[&json_type].len();
            }

            match resources.get_mut(&json_type) {
                Some(map) => {
                    map.insert(name, ResourceManager::create_type(&json_type, &value, index));
                }
                None => {
                    resources.insert(json_type.clone(), HashMap::from([(name, ResourceManager::create_type(
                        &json_type, &value, index))]));
                }
            }
        }

        return ResourceManager {
            resources
        };
    }

    pub fn get_type<T: JsonResource>(&self, resource_type: &str, name: &str) -> &T {
        unsafe {
            return &*(self.resources[resource_type][name].borrow() as *const dyn JsonResource as *const T);
        }
    }

    fn create_type(value_type: &str, values: &JsonValue, index: usize) -> Box<dyn JsonResource> {
        match value_type {
            "pixel" => Box::new(PixelType::new(values, index)),
            _ => panic!("Unrecognized JsonResource type {}", value_type)
        }
    }
}