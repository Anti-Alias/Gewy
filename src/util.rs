use crate::{Style, Painter, Canvas};

/// Utility function for painting pane-like widgets.
pub fn paint_pane(style: &Style, painter: &mut Painter, canvas: Canvas) {
    let Canvas { size, corners } = canvas;
    painter.set_color(style.color);
    painter.paint_rounded_rect(size, corners.top_left, corners.top_right, corners.bottom_right, corners.bottom_left);
}

// Iterator over a slice that either travels forwards or backwards depending on a flag.
pub(crate) struct SliceIter<'s, T> {
    index: isize,
    direction: isize,
    slice: &'s [T]
}

impl<'s, T: Copy> Iterator for SliceIter<'s, T> {

    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let uindex = self.index as usize;
        if index < 0 || uindex == self.slice.len() {
            None
        }
        else {
            let result = unsafe { self.slice.get_unchecked(uindex)};
            self.index += self.direction;
            Some(result)
        }
    }
}

impl <'s, T> SliceIter<'s, T> {
    pub fn new(slice: &'s [T], is_reversed: bool) -> Self {
        if is_reversed {
            Self {
                index: slice.len() as isize - 1,
                direction: -1,
                slice
            }
        }
        else {
            Self {
                index: 0,
                direction: 1,
                slice
            }
        }
    }
}