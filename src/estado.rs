#[derive(Debug,PartialEq,Clone)]

pub enum EstadoBomba{
    Normal,
    Traspaso,
}

impl EstadoBomba{

    pub fn _es_bomba(es_bomba: String) -> i32{
        match es_bomba{
            "normal" => {2},
            "traspaso" => {0},
        }
    }

}