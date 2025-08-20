use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::typing_session::WordDisplayState;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),     // Typing area
                Constraint::Length(3),  // Statistics
                Constraint::Length(2),  // Instructions
            ])
            .split(area);

        let typing_area = chunks[0];
        let statistic_area = chunks[1];
        let instruction_area = chunks[2];

        // Typing area
        self.render_typing_area(typing_area, buf);

        // Statistics
        self.render_statistics(statistic_area, buf);

        // Instructions
        let instructions = if self.typing_session.is_completed() {
            "Press Enter to start a new test | Press Tab to restart | Press Esc or Ctrl-c to quit"
        } else {
            "Type the text above | Press Tab to restart | Press Esc or Ctrl-c to quit"
        };
        
        let instructions_paragraph = Paragraph::new(instructions)
            .style(Style::default().fg(Color::Gray))
            .centered();
        instructions_paragraph.render(instruction_area, buf);
    }
}

impl App {
    fn render_typing_area(&self, area: Rect, buf: &mut Buffer) {
        let _typing_area = Rect::new(
            area.x + 2,
            area.y + (area.height/2),
            area.width.saturating_sub(4),
            area.height.saturating_sub(4),
        );

        // Create spans for each word with appropriate styling
        let mut spans = Vec::new();

        for (i, word) in self.typing_session.words.iter().enumerate() {
            let word_style = match word.get_display_state() {
                WordDisplayState::Untyped => {
                    if i == self.typing_session.current_word_index {
                        let w = word.text.clone();
                        let mut chars = w.chars();
                        let first_char = chars.next().expect("detected word with zero length");
                        spans.push(Span::styled(String::from(first_char), Style::default().fg(Color::Gray).bg(Color::White)));

                        let rest = chars.as_str();
                        spans.push(Span::styled(String::from(rest), Style::default().fg(Color::Gray)));
                        spans.push(Span::raw(" "));
                        continue;
                    }
                    Style::default().fg(Color::Gray)

                },
                WordDisplayState::Typing => {
                    // Show character-by-character progress for current word
                    if i == self.typing_session.current_word_index {
                        // Split current word into typed and untyped parts
                        let typed_part = &word.typed;

                        let mut remaining_part = "";
                        if word.typed.len() < word.text.len() {
                            remaining_part = &word.text[word.typed.len()..];
                        }
                        
                        // Add typed characters (green if correct, red if wrong)
                        for (j, ch) in typed_part.chars().enumerate() {
                            let expected_char = word.text.chars().nth(j);
                            let char_style = if expected_char == Some(ch) {
                                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                            };
                            spans.push(Span::styled(ch.to_string(), char_style));
                        }
                        
                        // Add cursor
                        if !remaining_part.is_empty() {
                            let cursor_char = remaining_part.chars().next().unwrap_or(' ');
                            spans.push(Span::styled(
                                cursor_char.to_string(),
                                Style::default().bg(Color::White).fg(Color::Black)
                            ));
                            
                            // Add remaining characters
                            if remaining_part.len() > 1 {
                                spans.push(Span::styled(
                                    remaining_part[1..].to_string(),
                                    Style::default().fg(Color::Gray)
                                ));
                            }
                        }
                        
                        spans.push(Span::raw(" "));
                        continue;
                    } else {
                        Style::default().fg(Color::Yellow)
                    }
                },
                WordDisplayState::Correct => Style::default().fg(Color::Green),
                WordDisplayState::Incorrect => Style::default().fg(Color::Red),
            };
            
            // For non-current words, show them as complete units
            //if i != self.typing_session.current_word_index {
            spans.push(Span::styled(word.text.clone(), word_style));

            if word.is_completed && i == self.typing_session.current_word_index {
                spans.push(Span::styled(" ", Style::default().bg(Color::White)));
            } else {
                spans.push(Span::raw(" "));
            }
            //}
        }

        let text = Text::from(Line::from(spans));
        let paragraph = Paragraph::new(text).centered()
            .wrap(Wrap { trim: true });
            //.block(Block::default().borders(Borders::ALL).title("Type this text:"))
        
        paragraph.render(_typing_area, buf);
    }

    fn render_statistics(&self, area: Rect, buf: &mut Buffer) {
        let wpm = self.typing_session.get_wpm();
        let accuracy = self.typing_session.get_accuracy();
        let errors = self.typing_session.errors;
        
        let stats_text = if self.typing_session.start_time.is_some() {
            format!(
                "WPM: {:.1} | Accuracy: {:.1}% | Errors: {}",
                wpm, accuracy, errors
            )
        } else {
            "Start typing to see statistics...".to_string()
        };
        
        let stats_paragraph = Paragraph::new(stats_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Statistics"))
            .centered();
        
        stats_paragraph.render(area, buf);
    }
}
