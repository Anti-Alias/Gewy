use crate::*;

const BACKGROUND: Color = Color::LIGHT_GRAY;
const UNSELECTED: Color = Color::DARK_GRAY;
const SELECTED: Color = Color::BLACK;

#[derive(Clone, Default, Debug)]
pub struct RadioButton {
    pub selected: bool,
    pub entered: bool
}
impl Widget for RadioButton {

    fn event(&mut self, _style: &mut Style, _children: Children, ctl: &mut EventControl) -> Result<()> {
        if ctl.is_event::<EnterEvent>() {
            ctl.set_cursor_icon(CursorIcon::Hand);
        }
        else if ctl.is_event::<ExitEvent>() {
            ctl.set_cursor_icon(CursorIcon::Default);
        }
        else if ctl.is_event::<PressEvent>() {
            ctl.press();
        }
        else if ctl.is_event::<ReleaseEvent>() {
            self.selected = !self.selected;
            ctl.stop();
            ctl.repaint();
        }
        Ok(())
    }

    fn style<'n>(&self, style: &mut Style) {
        style.size.width = Val::Px(17.0);
        style.size.height = Val::Px(17.0);
    }

    fn paint(&self, _style: &Style, painter: &mut Painter, canvas: Canvas) {
        let old_color = painter.color;

        let center = canvas.size * 0.5;

        let outer_radius = center.min_element();
        let inner_radius = outer_radius * 0.75;
        let dot_radius = inner_radius * 0.75;

        let color = if self.selected { SELECTED } else { UNSELECTED };

        painter.color = color;
        painter.circle(center, outer_radius);
        painter.color = BACKGROUND;
        painter.circle(center, inner_radius);
        painter.color = color;
        painter.circle(center, dot_radius);

        painter.color = old_color;
    }
}