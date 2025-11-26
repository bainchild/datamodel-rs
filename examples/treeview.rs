use datamodel_rs::{rbx_dom_to_tree, RootTrait};
use ego_tree::{Tree,NodeRef,NodeMut,NodeId};
use egui_ltreeview::{TreeView,TreeViewBuilder};
use eframe::egui;
fn main() {
    
    let binding = std::env::args().skip(1).collect::<Vec<String>>();
    let path = binding.first().unwrap();
    println!("opening {}", path);
    let input = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let dom = rbx_binary::from_reader(input).unwrap();
    let tree = rbx_dom_to_tree(dom);
    println!("all good");
    eframe::run_native("datamodel-rs treeview",eframe::NativeOptions::default(),Box::new(|cc| Ok(Box::new(ViewerApp(tree))))).unwrap();
}
fn recurse(build: &mut TreeViewBuilder<EgoTreeNodeId>, parent: NodeRef<Box<dyn RootTrait>>) {
    for child in parent.children() {
        let child2 = child.value();
        if build.dir(EgoTreeNodeId(child.id()),child2.as_instance().map(|x| x.name().clone()).unwrap_or(child2.classname())) {
            recurse(build,child);
        }
        build.close_dir();
    }
}
#[derive(Clone,PartialEq,Eq,Hash)]
struct EgoTreeNodeId(pub NodeId);
struct ViewerApp(pub Tree<Box<dyn RootTrait>>);
impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                TreeView::new("treeview".into()).show(ui,|build: &mut TreeViewBuilder<EgoTreeNodeId>| {
                    let root = self.0.root();
                    if build.dir(EgoTreeNodeId(root.id()),"DataModel".to_string()) {
                        for child in root.children() {
                            let child2 = child.value();
                            if build.dir(EgoTreeNodeId(child.id()),child2.as_instance().map(|x| x.name().clone()).unwrap_or(child2.classname())) {
                                recurse(build,child);
                            }
                            build.close_dir();
                        }
                    }
                });
            });
        });
    }
}
