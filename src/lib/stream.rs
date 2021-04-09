/// Generic trait that all streams that support peeking and consuming input
/// should "inherit" from.
pub trait PeekableStream<T> {
    /// Are we at the end of the stream?
    fn is_eos(&self) -> bool;

    /// Peek ahead by one character/token and return the character/token
    fn peek(&self) -> T;

    /// Advance the position
    fn advance(&mut self) {
        *self.pos() += 1;
    }

    fn pos(&mut self) -> &mut usize;
}