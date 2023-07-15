use another_rust_ui::{App, Gui, Node, Style, Pane, Color, Val, Layout, Corners, Direction, JustifyContent, Config, AlignSelf, AlignItems};

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

fn main() {
    let gui = make_gui();
    let app = App::new(gui, WINDOW_WIDTH, WINDOW_HEIGHT);
    app.start();
}


fn make_gui() -> Gui {
    
    let root = Node::tagged(
        "root",
        Style {
            color: Color::RED,
            corners: Corners::all(10.0),
            layout: Layout {    
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                direction: Direction::ColumnReverse,
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
            config: Config {
                grow: 0.0,
                ..Default::default()
            },
            ..Default::default()
        },
        Pane
    );

    let green = Node::tagged(
        "green",
        Style {
            color: Color::GREEN,
            corners: Corners::all(10.0),
            width: Val::Px(128.0),
            height: Val::Px(128.0),
            config: Config {
                grow: 0.0,
                align_self: AlignSelf::Stretch,
                ..Default::default()
            },
            ..Default::default()
        },
        Pane
    );

    let (root_id, mut gui) = Gui::new(root);
    let _blue_id = gui.insert(blue, root_id).unwrap();
    let _green_id = gui.insert(green, root_id).unwrap();

    gui
}