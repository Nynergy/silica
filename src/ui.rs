use std::{
    cmp,
    time::Duration,
};
use tui::{
    backend::Backend,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block,
        Clear,
        Paragraph,
    },
    Frame,
};

use crate::{
    app::{
        App,
        AppState,
    },
    args::DigitSize,
};

type Segments = Vec<String>;

const ASCII_WIDTH: u16 = 6;
const COUNTER_HEIGHT: u16 = 5;
//const COUNTER_WIDTH: u16 = ASCII_WIDTH + 1 + (4 * DIGIT_WIDTH) + 1;
const TOTAL_HEIGHT: u16 = COUNTER_HEIGHT + 2;

macro_rules! raw_para {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_para = Vec::new();
            $(
                temp_para.push(
                    Spans::from(
                        Span::raw($x)
                    )
                );
            )*
            temp_para
        }
    };
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let digit_width = match app.digit_size {
        DigitSize::Small => 1,
        DigitSize::Medium => 4,
        DigitSize::Large => 6,
    };
    let digit_height = match app.digit_size {
        DigitSize::Small => 1,
        DigitSize::Medium => 3,
        DigitSize::Large => 5,
    };
    let counter_width = (if app.noascii { 0 } else { ASCII_WIDTH + 1 }) + (4 * digit_width) + 1;
    let min_width = cmp::max(counter_width, app.text.len() as u16);
    if (f.size().height < TOTAL_HEIGHT + 1) || (f.size().width < min_width) {
        f.render_widget(Clear, f.size());
        return;
    }

    let empty_height = f.size().height / 2 - TOTAL_HEIGHT / 2;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(empty_height), // Vertical centering
            Constraint::Length(1), // Text
            Constraint::Length(1), // Empty space
            Constraint::Length(COUNTER_HEIGHT), // Counter
            ]
            .as_ref()
        )
        .split(f.size());

    render_text(f, chunks[1], app);
    render_counter(f, chunks[3], app, counter_width, digit_width, digit_height);
}

fn render_text<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &mut App
) {
    let text = match app.state {
        AppState::Counting => app.text.clone(),
        AppState::Elapsed => app.post_text.clone().unwrap_or(app.text.clone()),
    };

    let text = Paragraph::new(text)
        .block(Block::default())
        .style(
            Style::default()
            .fg(Color::Indexed(app.text_color))
        )
        .alignment(Alignment::Center);

    f.render_widget(text, chunk);
}

fn render_counter<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &mut App,
    counter_width: u16,
    digit_width: u16,
    digit_height: u16,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Length(chunk.width / 2 - counter_width / 2),
            Constraint::Length(if app.noascii { 0 } else { ASCII_WIDTH }),
            Constraint::Length(if app.noascii { 0 } else { 1 }),
            Constraint::Length(digit_width),
            Constraint::Length(digit_width),
            Constraint::Length(1),
            Constraint::Length(digit_width),
            Constraint::Length(digit_width),
            ]
            .as_ref()
        )
        .split(chunk);

    if !app.noascii {
        render_ascii(f, chunks[1], app);
    }

    let time = mmss_from_duration(app.time);
    for (i, digit) in time.chars().enumerate() {
        let segments;
        if let Some(digit) = digit.to_digit(10) {
            segments = segments_from_digit(digit, app);
        } else {
            segments = separator_segments(app);
        }
        let para = segments.iter()
            .map(|s| Spans::from(Span::raw(s)))
            .collect::<Vec<_>>();

        let color = if app.blink {
            Color::Indexed(app.blink_color)
        } else {
            Color::Indexed(app.digit_color)
        };

        let para = Paragraph::new(para)
            .block(Block::default())
            .style(
                Style::default()
                .fg(color)
            );

        // We need to shift the digit chunks down a row
        let c = chunks[i+3];
        let digit_chunk = Rect::new(c.x, c.y + COUNTER_HEIGHT / 2 - digit_height / 2, c.width, digit_height);

        f.render_widget(para, digit_chunk);
    }
}

fn render_ascii<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &App,
) {
    let ascii = raw_para!(
        "+====+",
        "|(::)|",
        "| )( |",
        "|(..)|",
        "+====+"
    );

    let ascii = Paragraph::new(ascii)
        .block(Block::default())
        .style(
            Style::default()
            .fg(Color::Indexed(app.ascii_color))
        );

    f.render_widget(ascii, chunk);
}

fn mmss_from_duration(d: Duration) -> String {
    let minutes = d.as_secs() / 60;
    let seconds = d.as_secs() % 60;

    format!("{minutes:02}:{seconds:02}")
}

fn segments_from_digit(digit: u32, app: &App) -> Segments {
    match app.digit_size {
        DigitSize::Small => small_segments_from_digit(digit),
        DigitSize::Medium => medium_segments_from_digit(digit),
        DigitSize::Large => large_segments_from_digit(digit),
    }
}

fn small_segments_from_digit(digit: u32) -> Segments {
    vec![format!("{digit}")]
}

fn medium_segments_from_digit(digit: u32) -> Segments {
    match digit {
        0 => {
            vec![
                String::from("┌──┐"),
                String::from("│  │"),
                String::from("└──┘"),
            ]
        },
        1 => {
            vec![
                String::from("  ┐ "),
                String::from("  │ "),
                String::from("  ╵ "),
            ]
        },
        2 => {
            vec![
                String::from("╶──┐"),
                String::from("┌──┘"),
                String::from("└──╴"),
            ]
        },
        3 => {
            vec![
                String::from("╶──┐"),
                String::from(" ──┤"),
                String::from("╶──┘"),
            ]
        },
        4 => {
            vec![
                String::from("╷  ╷"),
                String::from("└──┤"),
                String::from("   ╵"),
            ]
        },
        5 => {
            vec![
                String::from("┌──╴"),
                String::from("└──┐"),
                String::from("╶──┘"),
            ]
        },
        6 => {
            vec![
                String::from("┌──╴"),
                String::from("├──┐"),
                String::from("└──┘"),
            ]
        },
        7 => {
            vec![
                String::from("╶──┐"),
                String::from("   │"),
                String::from("   ╵"),
            ]
        },
        8 => {
            vec![
                String::from("┌──┐"),
                String::from("├──┤"),
                String::from("└──┘"),
            ]
        },
        9 => {
            vec![
                String::from("┌──┐"),
                String::from("└──┤"),
                String::from("╶──┘"),
            ]
        },
        _ => panic!("'{digit}' is not a valid digit!")
    }
}

fn large_segments_from_digit(digit: u32) -> Segments {
    match digit {
        0 => {
            vec![
                String::from("┌────┐"),
                String::from("│    │"),
                String::from("│    │"),
                String::from("│    │"),
                String::from("└────┘"),
            ]
        },
        1 => {
            vec![
                String::from("   ╶┐ "),
                String::from("    │ "),
                String::from("    │ "),
                String::from("    │ "),
                String::from("    ╵ "),
            ]
        },
        2 => {
            vec![
                String::from("╶────┐"),
                String::from("     │"),
                String::from("┌────┘"),
                String::from("│     "),
                String::from("└────╴"),
            ]
        },
        3 => {
            vec![
                String::from("╶────┐"),
                String::from("     │"),
                String::from(" ────┤"),
                String::from("     │"),
                String::from("╶────┘"),
            ]
        },
        4 => {
            vec![
                String::from("╷    ╷"),
                String::from("│    │"),
                String::from("└────┤"),
                String::from("     │"),
                String::from("     ╵"),
            ]
        },
        5 => {
            vec![
                String::from("┌────╴"),
                String::from("│     "),
                String::from("└────┐"),
                String::from("     │"),
                String::from("╶────┘"),
            ]
        },
        6 => {
            vec![
                String::from("┌────╴"),
                String::from("│     "),
                String::from("├────┐"),
                String::from("│    │"),
                String::from("└────┘"),
            ]
        },
        7 => {
            vec![
                String::from("╶────┐"),
                String::from("     │"),
                String::from("     │"),
                String::from("     │"),
                String::from("     ╵"),
            ]
        },
        8 => {
            vec![
                String::from("┌────┐"),
                String::from("│    │"),
                String::from("├────┤"),
                String::from("│    │"),
                String::from("└────┘"),
            ]
        },
        9 => {
            vec![
                String::from("┌────┐"),
                String::from("│    │"),
                String::from("└────┤"),
                String::from("     │"),
                String::from("╶────┘"),
            ]
        },
        _ => panic!("'{digit}' is not a valid digit!")
    }
}

fn separator_segments(app: &App) -> Segments {
    match app.digit_size {
        DigitSize::Small => {
            vec![String::from(":")]
        },
        DigitSize::Medium => {
            vec![
                String::from(" "),
                String::from(":"),
                String::from(" "),
            ]
        }
        DigitSize::Large => {
            vec![
                String::from(" "),
                String::from("◦"),
                String::from(" "),
                String::from("◦"),
                String::from(" "),
            ]
        }
    }
}
