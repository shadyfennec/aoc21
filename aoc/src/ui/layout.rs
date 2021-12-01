use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::ui::app::App;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};
use tui::Frame;

pub fn draw_list<B>(f: &mut Frame<B>, rect: Rect, app: &mut App)
where
    B: Backend,
{
    let selected_style = match app.day_highlight {
        Some(n) => {
            if app.days.get(n).unwrap().day.is_some() {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            }
        }
        None => Style::default(),
    };

    let header_cells = ["Day", "Title", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.days.iter().map(|d| {
        let number = Cell::from(format!("{}", d.number));
        let title = Cell::from(
            d.day
                .as_ref()
                .map(|d| d.title())
                .unwrap_or_else(String::new),
        );

        let status = if d.day.is_some() {
            Cell::from(format!("{}", d.status())).style(d.status().style())
        } else {
            Cell::from(String::new())
        };

        Row::new([number, title, status])
            .bottom_margin(0)
            .style(if d.day.is_some() {
                Style::default()
            } else {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC)
            })
    });

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("AoC 2021"))
        .highlight_style(selected_style)
        .highlight_symbol("> ")
        .widths(&[
            Constraint::Min(4),
            Constraint::Percentage(50),
            Constraint::Min(14),
        ]);

    let mut state = TableState::default();
    state.select(app.day_highlight);

    f.render_stateful_widget(table, rect, &mut state);
}

pub fn draw_inputs<B>(f: &mut Frame<B>, rect: Rect, app: &mut App)
where
    B: Backend,
{
    let header_cells = ["Input file", "Time", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = if let Some(n) = app.day_highlight {
        let day = app.days.get(n).unwrap();

        if day.day.is_some() {
            day.instances
                .iter()
                .map(|r| {
                    let i = Path::new(r.input)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    let t = r.duration().unwrap_or_else(String::new);

                    let s = Cell::from(format!("{}", r.status)).style(r.status.style());

                    Row::new([Cell::from(i), Cell::from(t), s]).bottom_margin(0)
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("AoC 2021"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("> ")
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ]);

    let mut state = TableState::default();
    state.select(app.input_highlight);

    f.render_stateful_widget(table, rect, &mut state);
}

pub fn draw_input_preview<B>(f: &mut Frame<B>, rect: Rect, app: &mut App)
where
    B: Backend,
{
    let text = if let Some(i) = app.input_highlight {
        let day = app.days.get(app.day_highlight.unwrap()).unwrap();
        let path = day.instances.get(i).unwrap().input;

        OpenOptions::new()
            .read(true)
            .write(false)
            .open(path)
            .map(|f| {
                BufReader::new(f)
                    .lines()
                    .filter_map(|l| l.ok().map(Spans::from))
                    .take(rect.height as usize)
                    .collect::<Vec<_>>()
            })
            .ok()
            .unwrap_or_else(Vec::new)
    } else {
        Vec::new()
    };

    let paragraph = Paragraph::new(text)
        .style(Style::default())
        .block(
            Block::default()
                .title("Input preview")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Left);
    // .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect)
}

pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(size);

    let main_chunk = chunks[0];
    let output_chunk = chunks[1];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_chunk);

    let days_chunk = chunks[0];
    let details_chunk = chunks[1];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(details_chunk);

    let input_chunk = chunks[0];
    let description_chunk = chunks[1];

    let output = Block::default().borders(Borders::all()).title("Output");

    draw_list(f, days_chunk, app);
    draw_inputs(f, input_chunk, app);
    f.render_widget(output, output_chunk);
    draw_input_preview(f, description_chunk, app)
}
