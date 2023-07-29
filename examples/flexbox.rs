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

    // Default style for widget
    fn style(&self, s: &mut Style) {
        s.color = Color::GRAY;
        s.direction = Direction::Row;
        s.justify = Justify::Start;
        s.align = Align::Center;
    }

    // Nodes that are implicitly inserted under Root.
    fn descendants(&self, d: &mut Descendants) {
        rect((c_red, c_round), d);
        pane((c_green, c_round), d, |d| {
            radio_button(c_button, d);
            radio_button(c_button, d);
            radio_button(c_button, d);
        });
        rect((c_blue, c_round), d);
    }

    // Paints primitive shapes of Root (not including children).
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
    s.width = Val::Px(128.0);  
    s.align_self = AlignSelf::Stretch;
    s.max_width = Val::Px(256.0);
    s.max_height = Val::Px(256.0);
    s.padding.left = Val::Px(64.0);
    s.padding.right = Val::Px(64.0);
    s.direction = Direction::Column;
    s.grow = 1.0;
}

fn c_green(s: &mut Style) {
    s.color = Color::GREEN;
    s.width = Val::Px(128.0);
    s.height = Val::Px(128.0);
    s.justify = Justify::Center;
    s.align = Align::Center;
    s.grow = 1.0;
}

fn c_blue(s: &mut Style) {
    s.color = Color::BLUE;
    s.width = Val::Px(128.0);  
    s.height = Val::Px(128.0);
    s.padding.left = Val::Px(32.0);
    s.padding.right = Val::Px(32.0);
    s.grow = 1.0;
}

fn c_button(s: &mut Style) {
    s.margin = Margin::all(Val::Px(2.0));
}