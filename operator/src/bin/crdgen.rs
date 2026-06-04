use kube::CustomResourceExt;
use operator::Desktop;
use std::{error::Error, fs::File, io::Write};

pub fn main() -> Result<(), Box<dyn Error>> {
    let yaml = serde_yaml_neo::to_string(&Desktop::crd()).unwrap();
    let path = "crds/desktop-crd.yaml";
    let mut file = File::create(path).unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    println!("Generated file {}", path);
    Ok(())
}
