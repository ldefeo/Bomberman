use crate::{laberinto::Laberinto, rafaga::Rafaga, estado::EstadoBomba};


#[derive(Debug,PartialEq)]
pub enum Movimiento{
    Derecha,
    Izquierda,
    Abajo,
    Arriba,
}



impl Movimiento{


    pub fn moverse(&self,coord_x:usize,coord_y:usize,alcance:usize,mut laberinto: &mut Laberinto,estado:i32){

        match &self {
            Movimiento::Derecha => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,0,1,estado);},
            Movimiento::Izquierda => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,0,-1,estado);},
            Movimiento::Abajo => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,1,0,estado);},
            Movimiento::Arriba => {Movimiento::mover(coord_x,coord_y,alcance,&mut laberinto,-1,0,estado);},
            _ => {println!("no es una direccion valida");},
        }
    }

    pub fn desviar(direccion: String,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto: &mut Laberinto,estado:i32){
        match direccion.as_str(){
            "R" => {Movimiento::moverse(&Movimiento::Derecha,coord_x,coord_y,alcance_desviado,laberinto,estado);},
            "L" => {Movimiento::moverse(&Movimiento::Izquierda,coord_x,coord_y,alcance_desviado,laberinto,estado);},
            "D" => {Movimiento::moverse(&Movimiento::Abajo,coord_x,coord_y,alcance_desviado,laberinto,estado);},
            "U" => {Movimiento::moverse(&Movimiento::Arriba,coord_x,coord_y,alcance_desviado,laberinto,estado);},
            _ => {println!("no es una direccion valida");},
        }
    }

       pub fn mover(coord_x: usize,coord_y: usize,alcance: usize,laberinto: &mut Laberinto,dx: isize,dy: isize,estado:i32) {
            let (mut x, mut y) = (coord_x as isize, coord_y as isize);
            let mut alcance_desviado = alcance;
            let mut rafaga = &mut Rafaga::reiniciar_rafaga();
            while alcance_desviado > 0 {
                x += dx;
                y += dy;
    
                if x >= 0 && y >= 0 {
                    if let Some(objeto) = laberinto.clone().get_objeto(x as usize, y as usize) {
                        let valor_objeto = objeto.objeto_encontrado(laberinto, x as usize, y as usize, alcance_desviado,estado);
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

}


