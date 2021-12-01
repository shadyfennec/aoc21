use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::ui::app::{App, State};

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

    let rows = app.days.iter().flat_map(|d| {
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

        let row = Row::new([number, title, status])
            .bottom_margin(0)
            .style(if d.day.is_some() {
                Style::default()
            } else {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC)
            });

        if app.part_highlight.is_some() {
            if d.number == app.day_highlight.unwrap() + 1 {
                let status =
                    Cell::from(format!("{}", d.status_for_part(1))).style(d.status().style());

                let part_1 = Row::new([
                    Cell::from(String::new()),
                    Cell::from(String::from("Part 1")),
                    status,
                ])
                .bottom_margin(0)
                .style(Style::default().add_modifier(Modifier::ITALIC));

                let status =
                    Cell::from(format!("{}", d.status_for_part(2))).style(d.status().style());

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
    });

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("AoC 2021"))
        .highlight_style(selected_style)
        .highlight_symbol("> ")
        .widths(&[
            Constraint::Min(4),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);

    let mut state = TableState::default();
    if let State::Day = app.state {
        state.select(app.day_highlight);
    } else {
        state.select(
            app.day_highlight
                .map(|i| i + app.part_highlight.unwrap() + 1),
        )
    }
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
        if let Some(part) = app.part_highlight {
            let day = app.days.get(n).unwrap();
            let part = part + 1;

            if day.day.is_some() {
                day.instances
                    .iter()
                    .filter(|i| i.part == part)
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
        }
    } else {
        Vec::new()
    };

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Input files"))
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
        let part = app.part_highlight.unwrap() + 1;
        let path = day
            .instances
            .iter()
            .filter(|i| i.part == part)
            .nth(i)
            .unwrap()
            .input;

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

pub fn draw_outputs<B>(f: &mut Frame<B>, output_rect: Rect, debug_rect: Rect, app: &mut App)
where
    B: Backend,
{
    let (output, debug) = if let Some(i) = app.day_highlight {
        let d = app.days.get(i).unwrap();
        if let Some(part) = app.part_highlight {
            let part = part + 1;
            if let Some(i) = app.input_highlight {
                let i = d
                    .instances
                    .iter()
                    .filter(|i| i.part == part)
                    .nth(i)
                    .unwrap();

                (
                    i.output.split_terminator('\n').map(Spans::from).collect(),
                    i.debug.split_terminator('\n').map(Spans::from).collect(),
                )
            } else {
                (Vec::new(), Vec::new())
            }
        } else {
            (Vec::new(), Vec::new())
        }
    } else {
        (Vec::new(), Vec::new())
    };

    let output = Paragraph::new(output)
        .style(Style::default())
        .block(Block::default().title("Output").borders(Borders::ALL))
        .alignment(Alignment::Left);

    let debug = Paragraph::new(debug)
        .style(Style::default())
        .block(Block::default().title("Debug").borders(Borders::ALL))
        .alignment(Alignment::Left);

    f.render_widget(output, output_rect);
    f.render_widget(debug, debug_rect);
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

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(output_chunk);

    let output_chunk = chunks[0];
    let debug_chunk = chunks[1];

    draw_list(f, days_chunk, app);
    draw_inputs(f, input_chunk, app);
    draw_outputs(f, output_chunk, debug_chunk, app);
    draw_input_preview(f, description_chunk, app)
}
