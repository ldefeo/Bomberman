use crate::generador::Generador;


#[derive(Debug,PartialEq,Clone)]
pub struct BombaNormal{
    identificador: String,
    alcance: usize,
}

#[derive(Debug,PartialEq,Clone)]
pub struct BombaTraspaso{
    pub(crate) identificador: String,
    pub(crate) alcance: usize,
}

impl BombaNormal{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            BombaNormal
     { identificador: ident, alcance: valor }
        }else{
            BombaNormal
     { identificador: todo!(), alcance: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.alcance
    }

}

impl BombaTraspaso{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            BombaTraspaso
     { identificador: ident, alcance: valor }
        }else{
            BombaTraspaso
     { identificador: todo!(), alcance: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.alcance
    }

}