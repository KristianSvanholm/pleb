
use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Row, Table, Cell},
    Frame,
};

use crate::app::{Display, App};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let title_style = Style::new().blue().bold();
    let missing_style = Style::new().red();
    let total_style = Style::new().gray();

    frame.render_widget(
        std::iter::once(
            Row::new(std::iter::once("".to_string())
                .chain(
                    app.task_count.keys()
                        .map(|k| k.to_string()).collect::<Vec<_>>())
            ).style(title_style)).chain(
        app.lang_count.iter()
            .map(|(lang, lcount)| -> Row {
                std::iter::once(Cell::from(lang.to_string()).style(title_style)).chain(
                    app.task_count.iter()
                    .map(|(task, _)| { 
                        if let Some(c) = app.status.get(&(lang.to_string(),task.to_string())) {
                            task_style(&app.display_mode, *c, app.runs)
                        } else {
                            Cell::from("-".to_string()).style(missing_style)
                        }
                    })
                )
                .chain(std::iter::once(Cell::from(lcount.to_string()).style(total_style)))
                .collect()
            })
            .chain(std::iter::once(
                Row::new(std::iter::once(Cell::from("".to_string())).chain(app.task_count.values().map(|t| Cell::from(t.to_string()).style(total_style)).collect::<Vec<_>>()))
            )))
            .collect::<Table>()
        .block(
            Block::bordered()
                .title("Benchmarks")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black)),
        frame.area(),
    )
}

fn task_style(display_mode: &Display, count: u64, runs: u64) -> Cell {
    let count = count as f64;
    let runs = runs as f64;
    let c = Cell::from(match display_mode {
        Display::Percent => format!("{:.0}%",count/runs*100.0),
        Display::Fraction => format!("{}/{}", count, runs),
    }).style(dynamic_color(count, runs));
    c
}


fn dynamic_color(count: f64, runs: f64) -> Style {
    let percent = count / runs;

    // Approx. equal thirds
    let color = match percent {
        0.0..0.333 => Color::Red,
        0.333..0.666 => Color::Yellow,
        _ => Color::Green,
    };

    Style::from(color)
}

// Potentially cleaner solution?
#[allow(dead_code)]
fn table(app: &App) -> Table {
    let mut rows = vec![Row::new(std::iter::once("".to_string()).chain(app.task_count.keys().map(|k| k.to_string()).collect::<Vec<_>>()))];
    for (lang, lcount) in &app.lang_count {
        let mut x: Vec<String> = app.task_count.iter().map(|(task,_)| {
            if let Some(c) = app.status.get(&(lang.to_string(),task.to_string())) {
                format!("{}/{}", c, app.runs)
            } else {
                "/".to_string()
            }
        }).collect();
        x.push(lcount.to_string());
        rows.push(Row::new(x));
    }

    rows.iter().cloned().collect::<Table>()
}
