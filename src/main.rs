use another_rust_ui::{App, Gui, Node, Style, Container, Color};

fn main() {
    let root = Node::tagged(
        "root",
        Style {
            color: Color::RED,
            ..Default::default()
        },
        Container
    );
    
    let (root_id, gui) = Gui::new(root);
    
    App::new(gui, 512, 512).start();
}
