#[derive(Clone, Copy)]
pub enum CursorMove {
    Down(usize),
    Home,
    Left(usize),
    Right(usize),
    Up(usize)
}