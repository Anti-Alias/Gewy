use another_rust_ui::{App, Gui, Node, Style, Pane, Color, Val, Layout, Corners};
use glam::Vec2;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

fn main() {
    let mut app = App::new(make_gui(), WINDOW_WIDTH, WINDOW_HEIGHT);
    // app.debug = true;
    app.start();
}


fn make_gui() -> Gui {
    let root = Node::tagged(
        "root",
        Style {
            color: Color::RED,
            layout: Layout {
                ..Default::default()
            },
            ..Default::default()
        },
        Pane
    );

    let blue = Node::tagged(
        "blue",
        Style {
            color: Color::BLUE,
            corners: Corners::all(10.0),
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            ..Default::default()
        },
        Pane
    );

    let green = Node::tagged(
        "green",
        Style {
            color: Color::GREEN,
            corners: Corners::all(1000.0),
            width: Val::Px(128.0),
            height: Val::Px(64.0),
            ..Default::default()
        },
        Pane
    );

    let gui_size = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let (root_id, mut gui) = Gui::new(root, gui_size);
    let _blue_id = gui.insert(blue, root_id).unwrap();
    let _green_id = gui.insert(green, root_id).unwrap();

    gui
}