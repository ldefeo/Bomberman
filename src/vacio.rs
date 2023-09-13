#[derive(Debug,PartialEq,Clone)]
pub struct Vacio{
    identificador: String,
}

impl Vacio{
    
    pub fn generar(elemento: String) -> Self{
        Vacio { identificador:elemento }

    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        0
    }
}