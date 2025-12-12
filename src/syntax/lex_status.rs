
#[derive(Debug, Copy, Clone)]
pub enum LexState {
    Code,
    BlockComment,
}