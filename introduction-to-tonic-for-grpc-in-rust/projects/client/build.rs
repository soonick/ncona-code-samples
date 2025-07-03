use std::{
    fs,
    path::Path
};

fn collect_protos(dir: &Path) -> Vec<String> {
    let mut protos = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            protos.extend(collect_protos(&path));
        } else if path.extension().unwrap() == "proto" {
            protos.push(path.to_str().unwrap().to_string());
        }
    }
    protos
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new("../protos");
    let protos = collect_protos(&root);
    tonic_build::configure().build_server(false).compile_protos(&protos, &[root])?;
    Ok(())
}
