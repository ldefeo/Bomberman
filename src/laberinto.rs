
use crate::{generador::Generador, movimiento::Movimiento, objetos::Objeto, vacio::Vacio};
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
                self.detonar(coord_x,coord_y,bomba_normal.clone().alcance());
            }
            if let Objeto::BombaTraspaso(bomba_traspaso) = &self.datos[coord_x][coord_y]{
                self.detonar(coord_x,coord_y,bomba_traspaso.clone().alcance());
            }
        }else{
            *self = matriz_vacia;
        }
        self
    }

    pub fn detonar(&mut self,coord_x: usize,coord_y: usize, alcance:usize) -> &mut Self{
        self.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));
        self.moverse_izquierda(coord_x,coord_y,alcance);
        self.moverse_abajo(coord_x,coord_y,alcance);
        self.moverse_derecha(coord_x,coord_y,alcance);
        self.moverse_arriba(coord_x,coord_y,alcance);
        //         // Movimiento::moverse(&Movimiento::Abajo,coord_x,coord_y,alcance,self).to_vec();
        //         // Movimiento::moverse(&Movimiento::Arriba,coord_x,coord_y,alcance,self).to_vec();
        //         // Movimiento::moverse(&Movimiento::Izquierda,coord_x,coord_y,alcance,self).to_vec();
        //         // Movimiento::moverse(&Movimiento::Derecha,coord_x,coord_y,alcance,self).to_vec();
        //      }
        self
        
    }

    pub fn moverse_derecha(&mut self,coord_x: usize,coord_y: usize,alcance:usize){
        Movimiento::moverse(&Movimiento::Derecha, coord_x, coord_y, alcance, self);
    }

    pub fn moverse_izquierda(&mut self,coord_x: usize,coord_y: usize,alcance:usize){
        Movimiento::moverse(&Movimiento::Izquierda, coord_x, coord_y, alcance, self);
    }

    pub fn moverse_abajo(&mut self,coord_x: usize,coord_y: usize,alcance:usize){
        Movimiento::moverse(&Movimiento::Abajo, coord_x, coord_y, alcance, self);
    }

    pub fn moverse_arriba(&mut self,coord_x: usize,coord_y: usize,alcance:usize){
        Movimiento::moverse(&Movimiento::Arriba, coord_x, coord_y, alcance, self);
    }

//     pub fn moverse_derecha(&mut self,coord_x: usize,coord_y:usize,alcance:usize){
//         let finish = self.chequeo_mayor(coord_y,alcance);
//         for idy in coord_y..=finish{
//             if (coord_x >= 0 && coord_x < self.datos.len()) &&  (idy >= 0 && idy < self.datos[0].len()){
//                 match &self.datos[coord_x][idy]{
//                     Objeto::BombaNormal(_box) => {self.detonar(coord_x, idy, _box.clone().alcance());},
//                     Objeto::BombaTraspaso(_box) => {
//                         self.detonar(coord_x, idy, _box.clone().alcance());},
//                     Objeto::Pared(_box) => {break;},
//                     Objeto::Roca(_box) => {break;},
//                     Objeto::Enemigo(_box) => {let vidas = _box.clone().alcance()-1;
//                         self.datos[coord_x][idy] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",_box.clone().identificador(),vidas)));
//                     if vidas == 0{
//                         self.datos[coord_x][idy]= Objeto::Vacio(Vacio::generar("_".to_string()));
//                     } }
//                     Objeto::Desvio(_box) => {
//                     },
//                     _ => {continue;}
//                  }
//             }

//         }
//     }

//     pub fn moverse_abajo(&mut self,coord_x: usize,coord_y:usize,alcance:usize){
//         let finish = self.chequeo_mayor(coord_x,alcance);
//         for idx in coord_x..=finish{
//             if (idx >= 0 && idx < self.datos.len()) &&  (coord_y >= 0 && coord_y < self.datos[0].len()){
//                 match & self.datos[idx][coord_y]{
//                      Objeto::BombaNormal(_box) => {self.detonar(idx, coord_y, _box.clone().alcance());},
//                      Objeto::BombaTraspaso(_box) => {self.detonar(idx, coord_y, _box.clone().alcance());},
//                      Objeto::Pared(_box) => {break;},
//                      Objeto::Roca(_box) => {break;},
//                      Objeto::Enemigo(_box) => {let vidas = _box.clone().alcance()-1;
//                         self.datos[idx][coord_y] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",_box.clone().identificador(),vidas)));
//                     if vidas == 0{
//                         self.datos[idx][coord_y]= Objeto::Vacio(Vacio::generar("_".to_string()));
//                     } },
//                      _ => {continue;}
//                  }
//             }
//         }

//     }

//     pub fn moverse_izquierda(&mut self,coord_x: usize,coord_y:usize,alcance:usize){
//         let start: usize = self.chequeo(coord_x, alcance);
//         for idx in (start..=coord_x).rev(){
//             if (idx >= 0 && idx < self.datos.len()) &&  (coord_y >= 0 && coord_y < self.datos[0].len()){
//                 match & self.datos[idx][coord_y]{
//                      Objeto::BombaNormal(_box) => {self.detonar(idx, coord_y, _box.clone().alcance());},
//                      Objeto::BombaTraspaso(_box) => {self.detonar(idx, coord_y, _box.clone().alcance());},
//                      Objeto::Pared(_box) => {break;},
//                      Objeto::Roca(_box) => {break;},
//                      Objeto::Enemigo(_box) => {let vidas = _box.clone().alcance()-1;
//                         self.datos[idx][coord_y] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",_box.clone().identificador(),vidas)));
//                     if vidas == 0{
//                         self.datos[idx][coord_y]= Objeto::Vacio(Vacio::generar("_".to_string()));
//                     } },
//                      _ => {continue;}
//                  }
//             }
//         }

//     }

//     pub fn moverse_arriba(&mut self,coord_x: usize,coord_y:usize,alcance:usize){
//         let start: usize = self.chequeo(coord_y,alcance);
//         for idy in (start..=coord_y).rev(){
//             if (coord_x >= 0 && coord_x < self.datos.len()) &&  (idy >= 0 && idy < self.datos[0].len()){
//                 match & self.datos[coord_x][idy]{
//                     Objeto::BombaNormal(_box) => {self.detonar(coord_x, idy, _box.clone().alcance());},
//                     Objeto::BombaTraspaso(_box) => {self.detonar(coord_x, idy, _box.clone().alcance());},
//                     Objeto::Pared(_box) => {break;},
//                     Objeto::Roca(_box) => {break;},
//                     Objeto::Enemigo(_box) => {let vidas = _box.clone().alcance()-1;
//                         self.datos[coord_x][coord_y] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",_box.clone().identificador(),vidas)));
//                     if vidas == 0{
//                         self.datos[coord_x][idy]= Objeto::Vacio(Vacio::generar("_".to_string()));
//                     } },
//                     _ => {continue;}
//                     }
//     }
//     }
//  }

//     pub fn chequeo(&mut self,coordenada:usize,alcance:usize) -> usize{
//         let mut start = 0;
//         if(coordenada < alcance){
//             start = 0;
//         }
//         else{
//             start = coordenada-alcance;
//         }
//         start
//     }

//     pub fn chequeo_mayor(&mut self,coordenada:usize,alcance:usize) -> usize{
//         let mut finish = 0;
//         if(coordenada+alcance > self.datos.len()){
//             finish = self.datos.len()-1;
//         }
//         else{
//             finish = coordenada+alcance;
//         }
//         finish
//     }

}





