use std::io::Read;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub url: String,
}

pub fn get_services() -> Vec<Service> {
    load_services("./ratchet.yaml")
}

fn load_services(yaml: &str) -> Vec<Service> {
    // Open file handle
    let mut file = std::fs::File::open(yaml).unwrap();

    // Read the data into a String
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // Deserialize
    let services: Vec<Service> = serde_yaml::from_str(&buf).unwrap();

    println!("deserialized: {}", services.len());

    services
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_yaml_load() {
        let services = super::load_services("../ratchet.yaml");

        assert_eq!(services.len(), 3);
    }
}
