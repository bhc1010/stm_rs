#[derive(Debug, Clone)]
pub struct Vector2<T>
where
    T: Default + Clone + Copy,
{
    x: T,
    y: T,
}
