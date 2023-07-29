use crate::*;

const LIGHT: Color = Color::LIGHT_GRAY;
const DARK: Color = Color::DARK_GRAY;
const SELECTED: Color = Color::BLACK;

#[derive(Clone, Default, Debug)]
pub struct RadioButton {
    pub selected: bool,
    pub entered: bool
}
impl Widget for RadioButton {

    fn event(&mut self, _style: &mut Style, _descendants: &mut Descendants, ctl: &mut EventControl) -> Result<()> {
        if ctl.is_event::<EnterEvent>() {
            ctl.set_cursor_icon(CursorIcon::Hand);
            ctl.stop();
        }
        else if ctl.is_event::<ExitEvent>() {
            ctl.set_cursor_icon(CursorIcon::Default);
            ctl.stop();
        }
        else if ctl.is_event::<PressEvent>() {
            ctl.press();
            ctl.stop();
        }
        else if ctl.is_event::<ReleaseEvent>() {
            self.selected = !self.selected;
            ctl.stop();
            ctl.repaint();
            ctl.stop();
        }
        Ok(())
    }

    fn style<'n>(&self, style: &mut Style) {
        const SIZE: Val = Val::Px(17.0);
        style.width = SIZE;
        style.height = SIZE;
        style.min_width = SIZE;
        style.min_height = SIZE;
        style.max_width = SIZE;
        style.max_height = SIZE;
    }

    fn paint(&self, style: &Style, painter: &mut Painter, canvas: Canvas) {
        let center = canvas.size * 0.5;
        let outer_radius = center.min_element();
        let inner_radius = outer_radius * 0.75;
        let dot_radius = inner_radius * 0.75;

        let light_color = style.color * LIGHT;
        let selected_color = style.color * if self.selected { SELECTED } else { DARK };

        painter.color = selected_color;
        painter.circle(center, outer_radius);
        painter.color = light_color;
        painter.circle(center, inner_radius);
        painter.color = selected_color;
        painter.circle(center, dot_radius);
    }
}