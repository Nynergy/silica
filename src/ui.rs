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

use crate::app::*;

type Segment = (String, String, String);

const ASCII_WIDTH: u16 = 6;
const DIGIT_WIDTH: u16 = 4;
const COUNTER_HEIGHT: u16 = 5;
const COUNTER_WIDTH: u16 = ASCII_WIDTH + 1 + (4 * DIGIT_WIDTH) + 1;
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
    let min_width = cmp::max(COUNTER_WIDTH, app.text.len() as u16);
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
    render_counter(f, chunks[3], app);
}

fn render_text<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &mut App
) {
    let text = Paragraph::new(app.text.clone())
        .block(Block::default())
        .style(
            Style::default()
            .fg(Color::White)
        )
        .alignment(Alignment::Center);

    f.render_widget(text, chunk);
}

fn render_counter<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &mut App
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Length(chunk.width / 2 - COUNTER_WIDTH / 2),
            Constraint::Length(ASCII_WIDTH),
            Constraint::Length(1),
            Constraint::Length(DIGIT_WIDTH),
            Constraint::Length(DIGIT_WIDTH),
            Constraint::Length(1),
            Constraint::Length(DIGIT_WIDTH),
            Constraint::Length(DIGIT_WIDTH),
            ]
            .as_ref()
        )
        .split(chunk);

    render_ascii(f, chunks[1]);

    let time = mmss_from_duration(app.time);
    for (i, digit) in time.chars().enumerate() {
        let seg1;
        let seg2;
        let seg3;
        if let Some(digit) = digit.to_digit(10) {
            (seg1, seg2, seg3) = segments_from_digit(digit);
        } else {
            seg1 = String::from(" ");
            seg2 = String::from(":");
            seg3 = String::from(" ");
        }
        let para = raw_para!(seg1, seg2, seg3);

        let color = if app.blink { Color::Red } else { Color::Green };

        let para = Paragraph::new(para)
            .block(Block::default())
            .style(
                Style::default()
                .fg(color)
            );

        // We need to shift the digit chunks down a row
        let c = chunks[i+3];
        let digit_chunk = Rect::new(c.x, c.y+1, c.width, c.height-2);

        f.render_widget(para, digit_chunk);
    }
}

fn render_ascii<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect
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
            .fg(Color::White)
        );

    f.render_widget(ascii, chunk);
}

fn mmss_from_duration(d: Duration) -> String {
    let minutes = d.as_secs() / 60;
    let seconds = d.as_secs() % 60;

    format!("{minutes:02}:{seconds:02}")
}

fn segments_from_digit(digit: u32) -> Segment {
    match digit {
        0 => {
            (
                String::from("┌──┐"),
                String::from("│  │"),
                String::from("└──┘"),
            )
        },
        1 => {
            (
                String::from("  ┐ "),
                String::from("  │ "),
                String::from("  ╵ "),
            )
        },
        2 => {
            (
                String::from("╶──┐"),
                String::from("┌──┘"),
                String::from("└──╴"),
            )
        },
        3 => {
            (
                String::from("╶──┐"),
                String::from(" ──┤"),
                String::from("╶──┘"),
            )
        },
        4 => {
            (
                String::from("╷  ╷"),
                String::from("└──┤"),
                String::from("   ╵"),
            )
        },
        5 => {
            (
                String::from("┌──╴"),
                String::from("└──┐"),
                String::from("╶──┘"),
            )
        },
        6 => {
            (
                String::from("┌──╴"),
                String::from("├──┐"),
                String::from("└──┘"),
            )
        },
        7 => {
            (
                String::from("╶──┐"),
                String::from("   │"),
                String::from("   ╵"),
            )
        },
        8 => {
            (
                String::from("┌──┐"),
                String::from("├──┤"),
                String::from("└──┘"),
            )
        },
        9 => {
            (
                String::from("┌──┐"),
                String::from("└──┤"),
                String::from("╶──┘"),
            )
        },
        _ => panic!("'{digit}' is not a valid digit!")
    }
}
