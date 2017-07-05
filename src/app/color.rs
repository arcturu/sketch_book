#[derive(Clone)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Color<T> {
        Color {r: r, g: g, b: b, a: a}
    }
}

pub struct ColorRefMut<'a, T: 'static> {
    pub r: &'a mut T,
    pub g: &'a mut T,
    pub b: &'a mut T,
    pub a: &'a mut T,
}
