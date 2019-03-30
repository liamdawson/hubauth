use std::path::Path;

#[path = "doc/mod.rs"]
mod doc;

fn main() {
    let out_dir = "target/";
    let man_dir = Path::new(&out_dir).join("man");
    std::fs::create_dir_all(&man_dir).unwrap();

    for (name, content) in doc::man::generate() {
        println!("{:?}", &man_dir);
        std::fs::write(&man_dir.join(name), content).unwrap();
    }
}
