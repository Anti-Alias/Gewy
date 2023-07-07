use another_rust_ui::{App, Gui, Node, Style, Container, Color};
use glam::Vec2;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

fn main() {
    App::new(make_gui(), WINDOW_WIDTH, WINDOW_HEIGHT).start();
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

    let blue = Node::tagged(
        "blue",
        Style {
            color: Color::BLUE,
            ..Default::default()
        },
        Container
    );

    let gui_size = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let (root_id, mut gui) = Gui::new(root, gui_size);
    let _blue_id = gui.insert(blue, root_id).unwrap();

    gui
}