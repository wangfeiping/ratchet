use std::io::Read;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub url: String,
}

pub fn get_services() -> Vec<Service> {
    // Open file handle
    let mut file = std::fs::File::open("./ratchet.yaml").unwrap();

    // Read the data into a String
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // Deserialize
    let services: Vec<Service> = serde_yaml::from_str(&buf).unwrap();

    println!("deserialized: {}", services.len());

    services
}
