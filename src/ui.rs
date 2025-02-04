use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Row, Table},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    frame.render_widget(
        std::iter::once(Row::new(std::iter::once("".to_string()).chain(app.task_count.keys().map(|k| k.to_string()).collect::<Vec<_>>()))).chain(
        app.lang_count.iter()
            .map(|(lang, lcount)| -> Row {
                std::iter::once(lang.to_string()).chain(
                    app.task_count.iter()
                    .map(|(task, _)| { 
                        if let Some(c) = app.status.get(&(lang.to_string(),task.to_string())) {
                            format!("{}/{}",c,app.runs)
                        } else {
                            "/".to_string()
                        }
                    })
                )
                .chain(std::iter::once(lcount.to_string()))
                .collect()
            })
            .chain(std::iter::once(
                Row::new(std::iter::once("".to_string()).chain(app.task_count.values().map(|k| k.to_string()).collect::<Vec<_>>()))
            )))
            .collect::<Table>()
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        frame.area(),
    )
}
