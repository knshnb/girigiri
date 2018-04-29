pub enum Color {
    Black,
    White,
    Null,
}

impl PartialEq for Color {
    fn eq(&self, other:&Color) -> bool {
        match (self, other) {
            (&Color::Black, &Color::Black) | (&Color::White, &Color::White) | (&Color::Null, &Color::Null) => true,
            _ => false
        }
    }
}
