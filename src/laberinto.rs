
use crate::{generador::Generador, movimiento::Movimiento, objetos::Objeto, vacio::Vacio, bomba::{BombaNormal, BombaTraspaso}};
use std::fmt::Write;

#[derive(Debug,PartialEq,Clone)]
pub struct Laberinto {
    pub datos: Vec<Vec<Objeto>>,
}

#[derive(Debug)]
pub enum LaberintoError {
    BombNotFound,
    ObjectNotFound,
    LaberintoError,
}

impl Laberinto {
    

    pub fn generar_laberinto(matriz: &str) -> Self{  //generador de laberintos
        let datos = Generador::generar_matriz(matriz);
        Laberinto { datos }
    }

    pub fn matriz(self) -> Vec<Vec<Objeto>>{
        self.datos 
    }

    

    pub fn atravesar_laberinto(&mut self,coord_x:usize,coord_y:usize) -> Result<&mut Self,String>{ //arreglar el manejo de errores
        if coord_x < self.datos.len() && coord_y < self.datos[coord_x].len() {   //me quedo con el elemento en la coordenada (x,y)
            match self.clone().get_objeto(coord_x, coord_y){
                Some(objeto) => {let chequeo = self.chequear_bomba(objeto,coord_x,coord_y);if let Ok(_) = chequeo{
                    Ok(self)
                }else{Err(LaberintoError::BombNotFound).expect(&format!("No se encontro una bomba en la posicion ({},{})",coord_x,coord_y))}},
                None => {Err(LaberintoError::ObjectNotFound).expect("Error")},
            }
        }else{
            Err(LaberintoError::LaberintoError).expect("la coordenada no aplica al laberinto")
        }

    }

    pub fn chequear_bomba(&mut self, objeto: &Objeto,coord_x: usize,coord_y: usize) -> Result<&mut Self,LaberintoError>{
        match objeto {
            Objeto::BombaNormal(_box) => {let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
                self.detonar(coord_x,coord_y,_box.clone().alcance(),BombaNormal::estado(),&mut enemigos_impactados); Ok(self)},
            Objeto::BombaTraspaso(_box) => {let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
                self.detonar(coord_x,coord_y,_box.clone().alcance(),BombaTraspaso::estado(),&mut enemigos_impactados); Ok(self)},
            _ => {Err(LaberintoError::BombNotFound)},
        }
    }


    pub fn get_objeto(&mut self, x: usize, y: usize) -> Option<& Objeto> {
        if x < self.datos.len() && y < self.datos[0].len() {
            Some(&self.datos[x][y])
        } else {
            None
        }
    }

    pub fn detonar(&mut self,coord_x: usize,coord_y: usize, alcance:usize,estado: i32,enemigos_impactados:&mut Vec<(usize,usize)>) -> &mut Self{
        self.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));
        self.moverse_izquierda(coord_x,coord_y,alcance,estado,enemigos_impactados);
        self.moverse_abajo(coord_x,coord_y,alcance,estado,enemigos_impactados);
        self.moverse_derecha(coord_x,coord_y,alcance,estado,enemigos_impactados);
        self.moverse_arriba(coord_x,coord_y,alcance,estado,enemigos_impactados);
        self
        
    }

    pub fn moverse_derecha(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32,enemigos_impactados:&mut Vec<(usize,usize)>){
        Movimiento::moverse(&Movimiento::Derecha, coord_x, coord_y, alcance, self,estado,enemigos_impactados);
    }

    pub fn moverse_izquierda(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32,enemigos_impactados:&mut Vec<(usize,usize)>){
        Movimiento::moverse(&Movimiento::Izquierda, coord_x, coord_y, alcance,self,estado,enemigos_impactados);
    }

    pub fn moverse_abajo(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32,enemigos_impactados:&mut Vec<(usize,usize)>){
        Movimiento::moverse(&Movimiento::Abajo, coord_x, coord_y, alcance,self,estado,enemigos_impactados);
    }

    pub fn moverse_arriba(&mut self,coord_x: usize,coord_y: usize,alcance:usize,estado: i32,enemigos_impactados:&mut Vec<(usize,usize)>){
        Movimiento::moverse(&Movimiento::Arriba, coord_x, coord_y, alcance,self,estado,enemigos_impactados);
    }

}





