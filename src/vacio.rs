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

    pub fn alcance(self) -> usize{
        0
    }

    pub fn manejar(&self,coord_x:usize, coord_y:usize,alcance_desviado:usize,laberinto: &mut Laberinto){
        laberinto.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));
    }
}