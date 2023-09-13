use crate::{laberinto::Laberinto, objetos::Objeto};


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

    pub fn manejar(&self,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto:&mut Laberinto){
        laberinto.datos[coord_x][coord_y] = Objeto::Roca(Roca::generar(self.clone().identificador().to_string()));
    }

}