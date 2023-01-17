#[derive(Clone, Copy)]
pub enum CursorMove {
    Down(usize),
    End,
    Home,
    Left(usize),
    Right(usize),
    Up(usize)
}