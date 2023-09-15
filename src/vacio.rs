use crate::{laberinto::Laberinto, objetos::Objeto};

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

    pub fn manejar(&self,coord_x:usize, coord_y:usize,laberinto: &mut Laberinto){
        laberinto.datos[coord_y][coord_x] = Objeto::Vacio(Vacio::generar("_".to_string()));
    }
}