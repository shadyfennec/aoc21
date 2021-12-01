use crate::ui::Day;
use crate::ui::{cycle_down, cycle_up, App, NavigableWidget, Widget};

use tui::layout::Constraint;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};

pub struct DayList {
    day_state: TableState,
    part_state: TableState,
}

impl DayList {
    pub fn new() -> Self {
        Self {
            day_state: TableState::default(),
            part_state: TableState::default(),
        }
    }

    pub fn selected_style(&self, app: &App) -> Style {
        if let Some(idx) = self.day_state.selected() {
            if app.is_day_present(idx) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            }
        } else {
            Style::default()
        }
    }

    pub fn row_style(&self, day: &Day) -> Style {
        if day.is_present() {
            Style::default()
        } else {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC)
        }
    }

    pub fn rows(&self, day: &Day) -> Vec<Row> {
        let number = Cell::from(format!("{}", day.number));
        let title = Cell::from(
            day.day
                .as_ref()
                .map(|d| d.title())
                .unwrap_or_else(String::new),
        );

        let status = if day.is_present() {
            Cell::from(format!("{}", day.status())).style(day.status().style())
        } else {
            Cell::from(String::new())
        };

        let row = Row::new([number, title, status])
            .bottom_margin(0)
            .style(self.row_style(day));

        if let Some(idx) = self.part_state.selected() {
            if day.number == idx + 1 {
                let status =
                    Cell::from(format!("{}", day.status_for_part(1))).style(day.status().style());

                let part_1 = Row::new([
                    Cell::from(String::new()),
                    Cell::from(String::from("Part 1")),
                    status,
                ])
                .bottom_margin(0)
                .style(Style::default().add_modifier(Modifier::ITALIC));

                let status =
                    Cell::from(format!("{}", day.status_for_part(2))).style(day.status().style());

                let part_2 = Row::new([
                    Cell::from(String::new()),
                    Cell::from(String::from("Part 2")),
                    status,
                ])
                .bottom_margin(0)
                .style(Style::default().add_modifier(Modifier::ITALIC));
                vec![row, part_1, part_2]
            } else {
                vec![row]
            }
        } else {
            vec![row]
        }
    }
}

impl Widget for DayList {
    fn draw<B>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect, app: &mut App)
    where
        B: tui::backend::Backend,
    {
        let header_cells = ["Day", "Title", "Status"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));

        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = app.days.iter().flat_map(|d| self.rows(d));

        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("AoC 2021"))
            .highlight_style(self.selected_style(app))
            .highlight_symbol("> ")
            .widths(&[
                Constraint::Min(4),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]);

        let mut state = if app.selecting_day() {
            self.day_state.clone()
        } else {
            self.part_state.clone()
        };

        f.render_stateful_widget(table, rect, &mut state);
    }
}

impl NavigableWidget for DayList {
    fn up(&mut self, app: &App) {
        let state = if app.selecting_day() {
            &mut self.day_state
        } else {
            &mut self.part_state
        };

        let len = app.days.len();

        let mut s = state.selected();
        cycle_up(&mut s, len - 1);

        state.select(s);
    }

    fn down(&mut self, app: &App) {
        let state = if app.selecting_day() {
            &mut self.day_state
        } else {
            &mut self.part_state
        };

        let len = app.days.len();

        let mut s = state.selected();
        cycle_down(&mut s, len - 1);

        state.select(s);
    }
}
