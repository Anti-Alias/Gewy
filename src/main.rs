use another_rust_ui::{App, Gui, Node, Style, Container, Color};

fn main() {
    App::new(make_gui(), 512, 512).start();
}


fn make_gui() -> Gui {
    let root = Node::tagged(
        "root",
        Style {
            color: Color::RED,
            ..Default::default()
        },
        Container
    );
    let (_root_id, gui) = Gui::new(root);
    gui
}