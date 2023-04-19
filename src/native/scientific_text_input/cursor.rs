//! Track the cursor of a text input.
use crate::native::scientific_text_input::value::Value;

/// The cursor of a text input.
#[derive(Debug, Copy, Clone)]
pub struct Cursor {
    state: State,
}

/// The state of a [`Cursor`].
#[derive(Debug, Copy, Clone)]
pub enum State {
    /// Cursor without a selection
    Index(usize),

    /// Cursor selecting a range of text
    Selection {
        /// The start of the selection
        start: usize,
        /// The end of the selection
        end: usize,
    },
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            state: State::Index(0),
        }
    }
}

impl Cursor {
    /// Returns the [`State`] of the [`Cursor`].
    pub fn state(&self, value: &Value) -> State {
        match self.state {
            State::Index(index) => State::Index(index.min(value.len())),
            State::Selection { start, end } => {
                let start = start.min(value.len());
                let end = end.min(value.len());

                if start == end {
                    State::Index(start)
                } else {
                    State::Selection { start, end }
                }
            }
        }
    }

    /// Returns the current selection of the [`Cursor`] for the given [`Value`].
    ///
    /// `start` is guaranteed to be <= than `end`.
    pub fn selection(&self, value: &Value) -> Option<(usize, usize)> {
        match self.state(value) {
            State::Selection { start, end } => Some((start.min(end), start.max(end))),
            _ => None,
        }
    }

    pub(crate) fn select_range(&mut self, start: usize, end: usize) {
        if start == end {
            self.state = State::Index(start);
        } else {
            self.state = State::Selection { start, end };
        }
    }

    pub(crate) fn select_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => {
                // self.select_range(index, index - 1)
                self.state = State::Selection {
                    start: index - 1,
                    end: index,
                };
                self.select_left(value)
            }
            State::Selection { start, end } if end > 0 && start > 0 => {
                if value.graphemes[start.min(end) - 1]
                    .chars()
                    .next()
                    .expect("Grapheme not aqquired")
                    .is_numeric()
                {
                    self.select_range(start - 1, end - 1)
                } else if end > 1 && start > 1 {
                    self.select_range(start - 2, end - 2)
                }
            }
            _ => {}
        }
    }

    pub(crate) fn select_right(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index < value.len() - 1 => {
                self.state = State::Selection {
                    start: index,
                    end: index + 1,
                };
                self.select_right(value)
            }
            State::Selection { start, end } => {
                if end <= value.len() - 1 {
                    if start <= value.len() - 1 {
                        if value.graphemes[start.min(end) + 1]
                            .chars()
                            .next()
                            .expect("Grapheme not aqquired.")
                            .is_numeric()
                        {
                            self.select_range(start + 1, end + 1);
                        } else if end < value.len() - 2 && start < value.len() - 2 {
                            self.select_range(start + 2, end + 2);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
