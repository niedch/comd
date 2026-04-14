use crossterm::event::{EventStream, KeyCode, KeyEventKind, KeyModifiers};
use futures::stream::StreamExt;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::actions::{StartRequest, StreamResult, StreamType};

enum InputMode {
    Editing,
    Processing,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    _result: String,

    start_tx: UnboundedSender<StartRequest>,
    result_rx: UnboundedReceiver<StreamResult>,
}

impl App {
    pub const fn new(
        start_tx: UnboundedSender<StartRequest>,
        result_rx: UnboundedReceiver<StreamResult>,
    ) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            _result: String::new(),
            character_index: 0,
            start_tx,
            result_rx,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if !is_not_cursor_leftmost {
            return;
        }

        // Method "remove" is not used on the saved text for deleting the selected char.
        // Reason: Using remove on String works on bytes instead of the chars.
        // Using remove would require special care because of char boundaries.
        let current_index = self.character_index;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = self.input.chars().skip(current_index);

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        let start_req = StartRequest {
            prompt: self.input.clone(),
        };
        match self.start_tx.send(start_req) {
            Ok(_) => {}
            Err(e) => println!("Println {:?}", e),
        }

        self.input.clear();
        self.reset_cursor();
        self.input_mode = InputMode::Processing
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn enter_string(&mut self, new_string: String) {
        let index = self.byte_index();
        self.input.insert_str(index, &new_string);
        self.character_index += new_string.chars().count();
    }

    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Option<String> {
        loop {
            terminal.draw(|f| self.render(f)).ok();
            let mut event_stream = EventStream::new();

            tokio::select! {
                    event = event_stream.next() => {
                    if let Some(Ok(crossterm::event::Event::Key(key))) = event {
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                            return None;
                        }

                        if key.kind == KeyEventKind::Press {
                            match self.input_mode {
                                InputMode::Editing => match key.code {
                                    KeyCode::Enter => self.submit_message(),
                                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                                    KeyCode::Backspace => self.delete_char(),
                                    KeyCode::Left => self.move_cursor_left(),
                                    KeyCode::Right => self.move_cursor_right(),
                                    KeyCode::Esc => return None,

                                    _ => {}
                                },
                                InputMode::Processing => {}
                            }
                        }
                    }
                }

                Some(stream) = self.result_rx.recv() => {
                    match stream.action_type {
                        StreamType::StreamResult => {
                            self.input_mode = InputMode::Processing;
                            println!("Received {0}", stream.result);
                            self.enter_string(stream.result);
                        }
                        StreamType::StreamEnd => return Some(self.input),
                    }
                    terminal.draw(|f| self.render(f)).ok();
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Length(3)]);
        let [input_area] = frame.area().layout(&layout);

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(Line::from(vec![
                Span::styled(" ⌘OK", Style::default().fg(Color::DarkGray).bold()),
                Span::styled(" · ", Style::default().fg(Color::DarkGray)),
                Span::styled("Gemini", Style::default().fg(Color::LightCyan)),
            ]));

        match self.input_mode {
            InputMode::Editing => {
                frame.set_cursor_position(Position::new(
                    input_area.x + self.character_index as u16 + 1,
                    input_area.y + 1,
                ));
            }
            InputMode::Processing => {
                frame.set_cursor_position(Position::new(
                    input_area.x + self.character_index as u16 + 1,
                    input_area.y + 1,
                ));

                block = block.clone().title_bottom(Line::from(Span::styled(
                    "Processing...",
                    Style::default().fg(Color::Yellow),
                )));
            }
        }

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Editing => Style::default().fg(Color::Yellow),
                InputMode::Processing => Style::default().fg(Color::Green),
            })
            .block(block);
        frame.render_widget(input, input_area);
    }
}
