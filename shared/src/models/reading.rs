pub struct Reading {
    pub id: usize,
    pub value: f64,
}

impl Reading {
    pub fn new(id: usize, value: f64) -> Self {
        Reading { id, value }
    }
}
