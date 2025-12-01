use std::iter::Peekable;

pub struct ArgCursor<I: Iterator<Item = String>> {
    iter: Peekable<I>, 
}

impl<I: Iterator<Item = String>> ArgCursor<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }

    pub fn next(&mut self) -> Option<String> {
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<&String> {
        self.iter.peek()
    }

    pub fn is_next_flag(&mut self) -> bool {
        self.peek()
            .map(|s| s.starts_with("-"))
            .unwrap_or(false)
    }

    pub fn next_if_value(&mut self) -> Option<String> {
        if self.is_next_flag() {
            None
        } else {
            self.next()
        }
    }
}