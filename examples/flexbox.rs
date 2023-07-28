use gewy::*;
use gewy::dsl::*;
use gewy::winit::*;

fn main() {
    let root = Node::from_widget(Root);
    let gui = Gui::new(root);
    WinitApp::new(gui, 512, 512).start();
}

pub struct Root;
impl Widget for Root {

    fn style(&self, s: &mut Style) {
        s.color = Color::GRAY;
        s.layout.direction = Direction::Row;
        s.layout.justify_content = JustifyContent::SpaceBetween;
        s.layout.align_items = AlignItems::Center;
        s.margin = Margin::all(Val::Px(40.0));
    }

    fn children(&self, mut children: Children) {
        let c = &mut children;
        rect((c_red, c_round), c);
        pane((c_green, c_round), c, |c| {
            radio_button(c_button, c);
            radio_button(c_button, c);
            radio_button(c_button, c);
        });
        rect((c_blue, c_round), c);
    }

    fn paint(&self, style: &Style, painter: &mut Painter, canvas: Canvas) {
        util::paint_pane(style, painter, canvas);
    }
}

// -------- Classes --------
fn c_round(s: &mut Style) {
    s.corners = Corners::all(Val::Px(5.0));
}

fn c_red(s: &mut Style) {
    s.color = Color::RED;
    s.size.width = Val::Px(128.0);  
    s.size.height = Val::Px(128.0);
    s.padding.left = Val::Px(32.0);
    s.padding.right = Val::Px(32.0);
    s.layout.direction = Direction::Column;
    s.config.shrink = 1.0;
}

fn c_green(s: &mut Style) {
    s.color = Color::GREEN;
    s.size.width = Val::Px(128.0);  
    s.size.height = Val::Px(128.0);
    s.padding.left = Val::Px(32.0);
    s.padding.right = Val::Px(32.0);
    s.config.shrink = 2.0;
    s.layout.justify_content = JustifyContent::Center;
    s.layout.align_items = AlignItems::Center;
}

fn c_blue(s: &mut Style) {
    s.color = Color::BLUE;
    s.size.width = Val::Px(128.0);  
    s.size.height = Val::Px(128.0);
    s.padding.left = Val::Px(32.0);
    s.padding.right = Val::Px(32.0);
    s.config.shrink = 3.0;
}

fn c_button(s: &mut Style) {
    s.margin = Margin::all(Val::Px(5.0));
}