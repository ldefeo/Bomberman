use crate::{generador::Generador, laberinto::Laberinto, estado_bomba::EstadoBomba};

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

    pub fn identificador(&self) -> &str{
        &self.identificador
    }

    pub fn alcance(&self) -> usize{
        self.alcance
    }

    pub fn estado() -> i32{
        EstadoBomba::estado(&EstadoBomba::Normal)
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,laberinto:&mut Laberinto){
        let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
        laberinto.detonar(coord_x, coord_y, self.clone().alcance(),BombaNormal::estado(),&mut enemigos_impactados);
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

    pub fn identificador(&self) -> &str{
        &self.identificador
    }

    pub fn alcance(&self) -> usize{
        self.alcance
    }

    pub fn estado() -> i32{
        EstadoBomba::estado(&EstadoBomba::Traspaso)
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,laberinto:&mut Laberinto){
        let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
        laberinto.detonar(coord_x, coord_y, self.clone().alcance(),BombaTraspaso::estado(),&mut enemigos_impactados);
    }

}