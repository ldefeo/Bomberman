use crate::generador::Generador;


#[derive(Debug,PartialEq,Clone)]
pub struct Enemigo{
    identificador: String,
    vidas: usize,
}


impl Enemigo{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            Enemigo { identificador: ident, vidas: valor }
        }else{
            Enemigo { identificador: todo!(), vidas: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.vidas
    }

}