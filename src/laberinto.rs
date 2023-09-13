
use std::f32::consts::E;

use crate::{generador::Generador, movimiento::Movimiento, objetos::Objeto, vacio::Vacio, rafaga::Rafaga, estado::EstadoBomba, bomba::{BombaNormal, BombaTraspaso}};
#[derive(Debug,PartialEq,Clone)]
pub struct Laberinto {
    pub datos: Vec<Vec<Objeto>>,
}

impl Laberinto {
    

    pub fn generador_matriz(valor: &str) -> Self{  //generador de matrices
        let datos = Generador::generador_matriz(valor);
        Laberinto { datos }
    }

    pub fn matriz(self) -> Vec<Vec<Objeto>>{
        self.datos 
    }

    pub fn atravesar_laberinto(&mut self,coord_x:usize,coord_y:usize) -> &mut Self{
        let matriz_vacia = Laberinto::generador_matriz(" ");
        if coord_x < self.datos.len() && coord_y < self.datos[coord_x].len() {   //me quedo con el elemento en la coordenada (x,y)
            if let Objeto::BombaNormal(bomba_normal) = &self.datos[coord_x][coord_y]{
                self.detonar(coord_x,coord_y,bomba_normal.clone().alcance(),BombaNormal::estado());
            }
            if let Objeto::BombaTraspaso(bomba_traspaso) = &self.datos[coord_x][coord_y]{
                self.detonar(coord_x,coord_y,bomba_traspaso.clone().alcance(),BombaTraspaso::estado());
            }
        }else{
            *self = matriz_vacia;
        }
        self
    }

    pub fn get_objeto(&mut self, x: usize, y: usize) -> Option<& Objeto> {
        if x < self.datos.len() && y < self.datos[0].len() {
            Some(&self.datos[x][y])
        } else {
            None
        }
    }

    pub fn detonar(&mut self,coord_x: usize,coord_y: usize, alcance:usize,estado: i32) -> &mut Self{
        self.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));
        self.moverse_izquierda(coord_x,coord_y,alcance,estado);
        self.moverse_abajo(coord_x,coord_y,alcance,estado);
        self.moverse_derecha(coord_x,coord_y,alcance,estado);
        self.moverse_arriba(coord_x,coord_y,alcance,estado);
        self
        
    }

    pub fn moverse_derecha(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32){
        Movimiento::moverse(&Movimiento::Derecha, coord_x, coord_y, alcance, self,estado);
    }

    pub fn moverse_izquierda(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32){
        Movimiento::moverse(&Movimiento::Izquierda, coord_x, coord_y, alcance,self,estado);
    }

    pub fn moverse_abajo(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32){
        Movimiento::moverse(&Movimiento::Abajo, coord_x, coord_y, alcance,self,estado);
    }

    pub fn moverse_arriba(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32){
        Movimiento::moverse(&Movimiento::Arriba, coord_x, coord_y, alcance,self,estado);
    }

}





