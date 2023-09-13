use crate::{objetos::Objeto, laberinto::Laberinto};


#[derive(Debug,PartialEq)]
pub enum Movimiento{
    Derecha,
    Izquierda,
    Abajo,
    Arriba,
}



impl Movimiento{


    pub fn moverse(&self,coord_x:usize,coord_y:usize,alcance:usize,mut laberinto: &mut Laberinto){

        match &self {
            Movimiento::Derecha => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,0,1);},
            Movimiento::Izquierda => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,0,-1);},
            Movimiento::Abajo => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,1,0);},
            Movimiento::Arriba => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,-1,0);},
            _ => {println!("no es una direccion valida");},
        }
    }

    pub fn desviar(direccion: String,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto: &mut Laberinto){
        match direccion.as_str(){
            "R" => {Movimiento::moverse(&Movimiento::Derecha,coord_x,coord_y,alcance_desviado,laberinto);},
            "L" => {Movimiento::moverse(&Movimiento::Izquierda,coord_x,coord_y,alcance_desviado,laberinto);},
            "D" => {Movimiento::moverse(&Movimiento::Abajo,coord_x,coord_y,alcance_desviado,laberinto);},
            "U" => {Movimiento::moverse(&Movimiento::Arriba,coord_x,coord_y,alcance_desviado,laberinto);},
            _ => {println!("no es una direccion valida");},
        }
    }

       pub fn mover(coord_x: usize,coord_y: usize,alcance: usize,laberinto: &mut Laberinto,dx: isize,dy: isize) {
            let (mut x, mut y) = (coord_x as isize, coord_y as isize);
            let mut alcance_desviado = alcance;
    
            while alcance_desviado > 0 {
                x += dx;
                y += dy;
    
                if x >= 0 && y >= 0 {
                    if let Some(objeto) = laberinto.clone().get_objeto(x as usize, y as usize) {
                        let valor_objeto = objeto.objeto_encontrado(laberinto, x as usize, y as usize, alcance_desviado);
                        if valor_objeto == 1 {
                            break;
                        }
                    }
                } else {
                    break; // Salir si se sale del laberinto
                }
    
                alcance_desviado -= 1;
            }
        }
    

    
    // pub fn moverse_derecha(coord_x:usize,coord_y:usize,alcance:usize,laberinto: &mut Laberinto){
    //     let finish = Movimiento::chequear_mayor(&mut laberinto.clone().matriz(),coord_y,alcance);
    //     let mut alcance_desviado = alcance;
    //     let mut valor_objeto = 0;
    //     for idy in coord_y..=finish{
    //         if (coord_x >= 0 && coord_x < laberinto.clone().matriz().len()) &&  (idy >= 0 && idy < laberinto.clone().matriz()[0].len()){
    //             alcance_desviado = Movimiento::chequear_menor(alcance_desviado, 1);
    //             valor_objeto = Objeto::objeto_encontrado(&laberinto.clone().matriz()[coord_x][idy],laberinto,coord_x,idy,alcance_desviado);
    //         }
    //         if valor_objeto == 1 {
    //             break;
    //         }if valor_objeto == 0{
    //             continue;
    //         }
    //     }
    // }

    // pub fn moverse_arriba(coord_x:usize,coord_y:usize,alcance:usize,laberinto: &mut Laberinto){
    //     let start = Movimiento::chequear_menor(coord_y,alcance);
    //     let mut valor_objeto = 0;
    //     let mut alcance_desviado = alcance+1;
    //     for idx in (start..=coord_x).rev(){
    //         if (idx >= 0 &&idx < laberinto.clone().matriz().len()) &&  (coord_y >= 0 && coord_y < laberinto.clone().matriz()[0].len()){
    //             alcance_desviado = Movimiento::chequear_menor(alcance_desviado, 1);
    //             valor_objeto =Objeto::objeto_encontrado(&laberinto.clone().matriz()[idx][coord_y],laberinto,idx,coord_y,alcance_desviado);
    //         }
    //         if valor_objeto == 1 {
    //             break;
    //         }if valor_objeto == 0{
    //             continue;
    //         }
    //     }
    // }

    // pub fn moverse_abajo(coord_x:usize,coord_y:usize,alcance:usize,laberinto: &mut Laberinto){
    //     let finish = Movimiento::chequear_mayor(&mut laberinto.clone().matriz(),coord_y,alcance);
    //     let mut valor_objeto = 0;
    //     let mut alcance_desviado = alcance+1;
    //     for idx in coord_x..=finish{
    //         if (idx >= 0 &&idx < laberinto.clone().matriz().len()) &&  (coord_y >= 0 && coord_y < laberinto.clone().matriz()[0].len()){
    //             alcance_desviado = Movimiento::chequear_menor(alcance_desviado, 1);
    //             valor_objeto =Objeto::objeto_encontrado(&laberinto.clone().matriz()[idx][coord_y],laberinto,idx,coord_y,alcance_desviado);
    //         }
    //         if valor_objeto == 1 {
    //             break;
    //         }if valor_objeto == 0{
    //             continue;
    //         }
    //     }
    // }

    // pub fn moverse_izquierda(coord_x:usize,coord_y:usize,alcance:usize,laberinto: &mut Laberinto){
    //     let start = Movimiento::chequear_menor(coord_y,alcance);
    //     let mut alcance_desviado = alcance+1;
    //     let mut valor_objeto = 0;
    //     for idy in (start..=coord_y).rev(){
    //         if (coord_x >= 0 && coord_x < laberinto.clone().matriz().len()) &&  (idy >= 0 && idy < laberinto.clone().matriz()[0].len()){
    //             alcance_desviado = Movimiento::chequear_menor(alcance_desviado, 1);
    //             valor_objeto =Objeto::objeto_encontrado(&laberinto.clone().matriz()[coord_x][idy],laberinto,coord_x,idy,alcance_desviado);

    //         }
    //         if valor_objeto == 1 {
    //             break;
    //         }if valor_objeto == 0{
    //             continue;
    //         }
    //     }
    // }

    // pub fn chequear_menor(coordenada:usize,alcance:usize) -> usize{
    //     let mut start = 0;
    //     if coordenada <= alcance {
    //         start = 0;
    //     }
    //     else{
    //         start = coordenada-alcance;
    //     }
    //     start
    // }

    
    // pub fn chequear_mayor(matriz: &mut Vec<Vec<Objeto>>,coordenada:usize,alcance:usize) -> usize{
    //     let mut finish = 0;
    //     if coordenada+alcance >= matriz.len() {
    //         finish = matriz.len()-1;
    //     }
    //     else{
    //         finish = coordenada+alcance;
    //     }
    //     finish
    // }

}


