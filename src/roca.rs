
#[derive(Debug,PartialEq,Clone)]
pub struct Roca{
    identificador: String,
}

impl Roca{
    
    pub fn generar(elemento: String) -> Self{
        Roca { identificador:elemento }

    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        0
    }

}