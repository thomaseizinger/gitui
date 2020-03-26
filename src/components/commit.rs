use super::{CommandInfo, Component};
use crate::{strings, ui};
use asyncgit::sync;
use crossterm::event::{Event, KeyCode};
use std::borrow::Cow;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Frame,
};

#[derive(Default)]
pub struct CommitComponent {
    msg: String,
    // focused: bool,
    visible: bool,
}

impl Component for CommitComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, _rect: Rect) {
        if self.visible {
            let txt = if !self.msg.is_empty() {
                [Text::Raw(Cow::from(self.msg.clone()))]
            } else {
                [Text::Styled(
                    Cow::from(strings::COMMIT_MSG),
                    Style::default().fg(Color::DarkGray),
                )]
            };

            ui::Clear::new(
                Paragraph::new(txt.iter())
                    .block(
                        Block::default()
                            .title(strings::COMMIT_TITLE)
                            .borders(Borders::ALL),
                    )
                    .alignment(Alignment::Left),
            )
            .render(f, ui::centered_rect(60, 20, f.size()));
        }
    }

    fn commands(&self) -> Vec<CommandInfo> {
        vec![
            CommandInfo::new(
                strings::COMMIT_CMD_ENTER,
                self.can_commit(),
                self.visible,
            ),
            CommandInfo::new(
                strings::COMMIT_CMD_CLOSE,
                true,
                self.visible,
            ),
        ]
    }

    fn event(&mut self, ev: Event) -> bool {
        if let Event::Key(e) = ev {
            return match e.code {
                KeyCode::Esc => {
                    self.hide();
                    true
                }
                KeyCode::Char(c) => {
                    self.msg.push(c);
                    true
                }
                KeyCode::Enter if self.can_commit() => {
                    self.commit();
                    true
                }
                KeyCode::Backspace if !self.msg.is_empty() => {
                    self.msg.pop().unwrap();
                    true
                }
                _ => false,
            };
        }

        false
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false
    }

    fn show(&mut self) {
        self.visible = true
    }
}

impl CommitComponent {
    fn commit(&mut self) {
        sync::commit(&self.msg);
        self.msg.clear();

        self.hide();
    }

    fn can_commit(&self) -> bool {
        !self.msg.is_empty()
    }
}
