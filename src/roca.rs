

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

    pub fn manejar(estado:i32) -> i32{
        if estado == 0{
            0
        }else{
            1
        }
    }

}