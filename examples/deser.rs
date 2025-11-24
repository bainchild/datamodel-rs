use datamodel_rs::rbx_dom_to_tree;
fn main() {
    let binding = std::env::args().skip(1).collect::<Vec<String>>();
    let path = binding.first().unwrap();
    println!("opening {}", path);
    let input = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let dom = rbx_binary::from_reader(input).unwrap();
    let tree = rbx_dom_to_tree(dom);
    println!("all good");
}
