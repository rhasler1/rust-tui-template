use ratatui::prelude::{Layout,Direction,Constraint,Alignment,Style,Span,Color};
use ratatui::widgets::Paragraph;

pub struct App {
    message: String
}

impl App {
    pub fn default() -> Self {
        let message: String = String::from("Hello, press 'q' or left click to exit.");
        Self {
            message
        }
    }

    pub fn draw(&mut self, frame: &mut ratatui::Frame) -> anyhow::Result<()> {
        let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1)
            ])
            .split(frame.size());
        
        let span = Span::styled(self.message.clone(), Style::new().fg(Color::Green));
        let paragraph = Paragraph::new(span).alignment(Alignment::Center);
        frame.render_widget(paragraph, chunks[0]);

        Ok(())
    }
}
