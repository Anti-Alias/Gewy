use another_rust_ui::*;

fn main() {
    let gui = make_gui();
    App::new(gui, 512, 512).start();
}


fn make_gui() -> Gui {
    let root = Node::from_widget(Root);
    let gui = Gui::new(root).unwrap();
    gui
}

pub struct Root;
impl Widget for Root {

    fn event(&mut self, _style: &mut Style, _subtree: Subtree, _ctl: &mut EventControl) -> Result<()> {
        println!("Root clicked");
        Ok(())
    }

    fn style(&self, s: &mut Style) {
        s.color = Color::RED;
        s.corners = Corners::all(Val::Px(10.0));
        s.layout.justify_content = JustifyContent::SpaceEvenly;
        s.layout.align_items = AlignItems::Center;
        s.layout.direction = Direction::Row;
    }

    fn descendants(&self, mut subtree: Subtree) -> Result<()> {
        // Blue child
        subtree.insert(subtree.widget_id(), Node::new(
            Pane,
            Style {
                color: Color::BLUE,
                corners: Corners::all(Val::Px(10.0)),
                width: Val::Px(64.0),   
                height: Val::Px(64.0),
                ..Default::default()
            }
        ))?;
        // Green child
        subtree.insert(subtree.widget_id(), Node::new(
            Pane,
            Style {
                color: Color::GREEN,
                corners: Corners::all(Val::Px(10.0)),
                width: Val::Px(128.0),
                height: Val::Px(128.0),
                ..Default::default()
            }
        ))?;
        Ok(())
    }

    fn paint(&self, style: &Style, painter: &mut Painter, canvas: Canvas) {
        util::paint_pane(style, painter, canvas);
    }
}