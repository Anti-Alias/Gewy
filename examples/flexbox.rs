#![windows_subsystem = "windows"]

use another_rust_ui::*;
use another_rust_ui::dsl::*;

fn main() {
    let root = Node::from_widget(Root);
    let gui = Gui::new(root);
    App::new(gui, 512, 512).start();
}

pub struct Root;
impl Widget for Root {

    fn style(&self, s: &mut Style) {
        s.color = Color::RED;
        s.corners = Corners::all(Val::Px(10.0));
        s.layout.justify_content = JustifyContent::SpaceAround;
        s.layout.align_items = AlignItems::Center;
        s.layout.direction = Direction::Row;
    }

    fn children(&self, mut children: Children) {
        let c = &mut children;
        rect((c_round, c_blue), c);
        pane((c_round, c_green, c_centered), c, |c| {
            radio_button((c_round, c_button), c);
            radio_button((c_round, c_button), c);
        });
    }

    fn paint(&self, style: &Style, painter: &mut Painter, canvas: Canvas) {
        util::paint_pane(style, painter, canvas);
    }
}

// -------- Classes --------
fn c_round(s: &mut Style) {
    s.corners = Corners::all(Val::Px(5.0));
}

fn c_centered(s: &mut Style) {
    s.layout.justify_content = JustifyContent::Center;
    s.layout.align_items = AlignItems::Center;
}

fn c_blue(s: &mut Style) {
    s.color = Color::BLUE;
    s.width = Val::Px(64.0);  
    s.height = Val::Px(64.0);
}

fn c_green(s: &mut Style) {
    s.color = Color::GREEN;
    s.width = Val::Px(128.0);  
    s.height = Val::Px(128.0);
    s.layout.direction = Direction::Column;
}

fn c_button(s: &mut Style) {
    s.margin = Margin::all(Val::Px(5.0));
}