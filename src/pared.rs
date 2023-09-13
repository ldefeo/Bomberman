use crate::{objetos::Objeto, laberinto::Laberinto};


#[derive(Debug,PartialEq,Clone)]
pub struct Pared{
    identificador: String,
}

impl Pared{
    
    pub fn generar(elemento: String) -> Self{
        Pared { identificador:elemento }

    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        0
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto:&mut Laberinto){
        laberinto.datos[coord_x][coord_y] = Objeto::Pared(Pared::generar(self.clone().identificador().to_string()));
    }
}