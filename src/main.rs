use another_rust_ui::*;

fn main() {
    let gui = make_gui();
    App::new(gui, 512, 512).start();
}


fn make_gui() -> Gui {
    
    let root = Node::new(
        Pane,
        Tag::None,
        Style {
            color: Color::RED,
            corners: Corners::all(Val::Px(10.0)),
            layout: Layout {    
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                direction: Direction::Row,
                ..Default::default()
            },
            ..Default::default()
        }
    );

    let blue = Node::new(
        Pane,
        Tag::None,
        Style {
            color: Color::BLUE,
            corners: Corners::all(Val::Px(10.0)),
            width: Val::Px(64.0),
            height: Val::Px(64.0),
            ..Default::default()
        }
    );

    let green = Node::new(
        Pane,
        Tag::None,
        Style {
            color: Color::GREEN,
            corners: Corners::all(Val::Px(10.0)),
            width: Val::Px(128.0),
            height: Val::Px(128.0),
            ..Default::default()
        }
    );

    let (root_id, mut gui) = Gui::new(root);
    let _blue_id = gui.insert(blue, root_id).unwrap();
    let _green_id = gui.insert(green, root_id).unwrap();

    gui
}