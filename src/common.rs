pub struct ImageParameters{
    pub width : usize,
    pub height : usize,
}

impl ImageParameters{
    pub fn len(&self) -> usize {
        self.width*self.height
    }
}
