#[derive(Debug, PartialEq)]
pub enum EstadoBomba {
    /// Un enumerable de estados de bomba
    Normal,
    Traspaso,
}

impl EstadoBomba {
    /// Si el estado de la bomba es de traspaso, devuelve un true para que pueda traspasar rocas
    pub fn estado(&self) -> i32 {
        match &self {
            EstadoBomba::Normal => 1,
            EstadoBomba::Traspaso => 0,
        }
    }
}
