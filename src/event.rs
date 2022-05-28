use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppEvent {
  Quit,
  ForceQuit,

  ToggleScope,

  NextProc,
  PrevProc,
  StartProc,
  TermProc,
  KillProc,
  RestartProc,
  ForceRestartProc,
  ShowAddProc,
  AddProc(String),

  ScrollDown,
  ScrollUp,

  #[serde(skip)]
  SendKey(KeyEvent),
}
