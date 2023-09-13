#[derive(Debug,PartialEq)]
pub enum EstadoBomba{
    Normal,
    Traspaso,
}

impl EstadoBomba{

    pub fn estado(&self) -> i32{
        match &self{
            EstadoBomba::Normal => {1}
            EstadoBomba::Traspaso => {0}
        }
    }

}