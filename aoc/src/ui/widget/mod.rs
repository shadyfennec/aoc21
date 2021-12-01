use crate::ui::App;
use tui::layout::{Constraint, Direction, Layout as TuiLayout, Rect};
use tui::{backend::Backend, Frame};

mod daylist;
pub use daylist::*;

pub(crate) fn cycle_up(value: &mut Option<usize>, max: usize) {
    if let Some(v) = value {
        if *v == max {
            *v = 0;
        } else {
            *v += 1;
        }
    } else {
        *value = Some(0);
    }
}

pub(crate) fn cycle_down(value: &mut Option<usize>, max: usize) {
    if let Some(v) = value {
        if *v == 0 {
            *v = max;
        } else {
            *v -= 1;
        }
    } else {
        *value = Some(0);
    }
}

pub trait Widget {
    fn draw<B>(&mut self, f: &mut Frame<B>, rect: Rect, app: &mut App)
    where
        B: Backend;
}

pub trait NavigableWidget {
    fn up(&mut self, app: &App);
    fn down(&mut self, app: &App);
}

pub struct WidgetList {
    day_list: DayList,
}

struct Layout {
    pub(crate) days: Rect,
    pub(crate) inputs: Rect,
    pub(crate) description: Rect,
    pub(crate) output: Rect,
    pub(crate) debug: Rect,
}

impl WidgetList {
    pub fn new() -> Self {
        Self {
            day_list: DayList::new(),
        }
    }

    fn layout<B>(&self, f: &Frame<B>) -> Layout
    where
        B: Backend,
    {
        let size = f.size();

        let chunks = TuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(size);

        let main_chunk = chunks[0];
        let output_chunk = chunks[1];

        let chunks = TuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(main_chunk);

        let days_chunk = chunks[0];
        let details_chunk = chunks[1];

        let chunks = TuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(details_chunk);

        let input_chunk = chunks[0];
        let description_chunk = chunks[1];

        let chunks = TuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(output_chunk);

        let output_chunk = chunks[0];
        let debug_chunk = chunks[1];

        Layout {
            days: days_chunk,
            inputs: input_chunk,
            description: description_chunk,
            output: output_chunk,
            debug: debug_chunk,
        }
    }

    pub fn draw<B>(&mut self, f: &mut Frame<B>, app: &mut App)
    where
        B: Backend,
    {
        let layout = self.layout(f);

        self.day_list.draw(f, layout.days, app);
    }
}
