#![windows_subsystem = "windows"]
use another_rust_ui::*;

fn main() {
    let gui = make_gui();
    App::new(gui, 512, 512).start();
}


fn make_gui() -> Gui {
    
    let root = Node::new(
        Pane,
        Style {
            color: Color::WHITE,
            layout: Layout {    
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                direction: Direction::Row,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    let button = Node::from_widget(RadioButton::default());

    let mut gui = Gui::new(root);
    let root_id = gui.root_id();
    let _button_id = gui.insert(root_id, button).unwrap();

    gui
}