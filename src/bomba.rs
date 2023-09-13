use crate::{generador::Generador, laberinto::Laberinto, rafaga::Rafaga, estado::EstadoBomba};


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

    pub fn estado() -> i32{
        let mut estado = EstadoBomba::estado(&EstadoBomba::Normal);
        estado
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto:&mut Laberinto){
        laberinto.detonar(coord_x, coord_y, self.clone().alcance(),BombaNormal::estado());
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

    pub fn estado() -> i32{
        let mut estado = EstadoBomba::estado(&EstadoBomba::Traspaso);
        estado
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto:&mut Laberinto){
        
        laberinto.detonar(coord_x, coord_y, self.clone().alcance(),BombaTraspaso::estado());
    }

}