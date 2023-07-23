use crate::*;

const BACKGROUND: Color = Color::LIGHT_GRAY;
const UNSELECTED: Color = Color::DARK_GRAY;
const SELECTED: Color = Color::BLACK;

const STEVE: Name = 0;

#[derive(Clone, Default, Debug)]
pub struct RadioButton {
    pub selected: bool,
    pub entered: bool
}
impl Widget for RadioButton {

    fn style<'n>(&self, style: &mut Style) {
        style.width = Val::Px(17.0);
        style.height = Val::Px(17.0);
    }

    fn event(&mut self, _style: &mut Style, mut _subtree: Subtree, ctl: &mut EventControl) -> Result<()> {
        if let Some((event, origin)) = ctl.event::<ReleaseEvent>(STEVE) {
            self.selected = true;
            ctl.repaint = true;
        }
        else if let Some((event, origin)) = ctl.event::<EnterEvent>(STEVE) {
            self.entered = true;
            ctl.repaint = true;
        }
        else if let Some((event, origin)) = ctl.event::<ReleaseEvent>(STEVE) {
            self.entered = false;
            ctl.repaint = true;
        }
        Ok(())
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