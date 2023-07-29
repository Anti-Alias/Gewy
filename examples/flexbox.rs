use gewy::*;
use gewy::dsl::*;
use gewy::winit::*;

fn main() {
    let root = Node::from_widget(Root);
    let gewy = Gewy::new(root);
    WinitApp::new(gewy, 512, 512).start();
}

pub struct Root;
impl Widget for Root {

    fn descendants(&self, d: &mut Descendants) {
        rect((c_red, c_round), d);
        pane((c_green, c_round), d, |d| {
            radio_button(c_button, d);
            radio_button(c_button, d);
            radio_button(c_button, d);
        });
        rect((c_blue, c_round), d);
    }

    fn style(&self, s: &mut Style) {
        s.color = Color::GRAY;
        s.layout.direction = Direction::Row;
        s.layout.justify = Justify::Center;
        s.layout.align = Align::Center;
    }

    fn paint(&self, style: &Style, painter: &mut Painter, canvas: Canvas) {
        util::paint_pane(style, painter, canvas);
    }
}

// -------- Classes --------
fn c_round(s: &mut Style) {
    s.corners = Corners::all(Val::Px(10.0));
}

fn c_red(s: &mut Style) {
    s.color = Color::RED;
    s.size.width = Val::Px(128.0);  
    s.size.height = Val::Px(128.0);
    s.max_size.width = Val::Px(256.0);
    s.padding.left = Val::Px(32.0);
    s.padding.right = Val::Px(32.0);
    s.layout.direction = Direction::Column;
    s.config.grow = 1.0;
}

fn c_green(s: &mut Style) {
    s.color = Color::GREEN;
    s.size.width = Val::Px(128.0);
    s.size.height = Val::Px(128.0);
    s.layout.justify = Justify::Center;
    s.layout.align = Align::Center;
    s.config.grow = 1.0;
}

fn c_blue(s: &mut Style) {
    s.color = Color::BLUE;
    s.size.width = Val::Px(128.0);  
    s.size.height = Val::Px(128.0);
    s.padding.left = Val::Px(32.0);
    s.padding.right = Val::Px(32.0);
    s.config.grow = 1.0;
}

fn c_button(s: &mut Style) {
    //s.margin = Margin::all(Val::Px(5.0));
    s.size = Size::all(Val::Px(17.0));
    s.min_size = Size::all(Val::Px(17.0));
    s.max_size = Size::all(Val::Px(17.0));
    s.margin = Margin::all(Val::Px(2.0));
}