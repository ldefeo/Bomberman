
use crate::{generador::Generador, movimiento::Movimiento, objetos::Objeto, vacio::Vacio, bomba::{BombaNormal, BombaTraspaso}};

#[derive(Debug,PartialEq,Clone)]
pub struct Laberinto {
    pub datos: Vec<Vec<Objeto>>,
}



impl Laberinto {
    

    pub fn generar_laberinto(matriz: &str) -> Self{  //generador de laberintos
        let datos = Generador::generar_matriz(matriz);
        Laberinto { datos }
    }

    pub fn matriz(self) -> Vec<Vec<Objeto>>{
        self.datos 
    }

    

    pub fn atravesar_laberinto(&mut self,coord_x:usize,coord_y:usize) -> &mut Self{ //agregar manejo de errores
        if coord_x < self.datos.len() && coord_y < self.datos[coord_x].len() {   //me quedo con el elemento en la coordenada (x,y)
            match self.clone().get_objeto(coord_x, coord_y){
                Some(objeto) => {self.chequear_bomba(objeto,coord_x,coord_y);},
                None => {println!("No se hayo objeto en esa posicion");},
            }
        }else{
            *self = Laberinto { datos: Generador::generar_matriz(" ") };
        }
        self
    }

    pub fn chequear_bomba(&mut self, objeto: &Objeto,coord_x: usize,coord_y: usize){
        match objeto {
            Objeto::BombaNormal(_box) => {let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
                self.detonar(coord_x,coord_y,_box.clone().alcance(),BombaNormal::estado(),&mut enemigos_impactados);},
            Objeto::BombaTraspaso(_box) => {let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
                self.detonar(coord_x,coord_y,_box.clone().alcance(),BombaTraspaso::estado(),&mut enemigos_impactados);},
            _ => {println!("No se hayo una bomba en esa posicion");},
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





