use crate::{laberinto::Laberinto, objetos::Objeto};

#[derive(Debug, PartialEq)]
pub enum Direccion {
    /// enumerable de Direcciones.
    Derecha,
    Izquierda,
    Abajo,
    Arriba,
}

#[derive(Debug, PartialEq)]
pub struct Rafaga{
    pub direccion: Direccion,
    pub enemigos_impactados: Vec<(usize,usize)>,
}

impl Rafaga{

    /// Esta funcion sirve para generar la rafaga en la direccion correspondiente.
    pub fn iniciar(direccion: Direccion, posicion: (usize,usize),alcance: usize, estado: i32,laberinto: &mut Laberinto){
        let mut enemigos_impactados: Vec<(usize,usize)> = Vec::new();
        Direccion::moverse(&direccion,posicion,alcance,estado,&mut enemigos_impactados,laberinto);
    }
}

impl Direccion {

    /// Esta funcion sirve para matchear el Direccion que se debe realizar.
    pub fn moverse(&self, posicion: (usize,usize), alcance: usize, estado: i32,enemigos_impactados: &mut Vec<(usize, usize)>,laberinto: &mut Laberinto){
        match &self {
            Direccion::Derecha => {
                Direccion::mover(
                    posicion,
                    alcance,
                    (1,0),
                    estado,
                    enemigos_impactados,laberinto,
                );
            }
            Direccion::Izquierda => {
                Direccion::mover(
                    posicion,
                    alcance,
                    (-1,
                    0),
                    estado,
                    enemigos_impactados,laberinto,
                );
            }
            Direccion::Abajo => {
                Direccion::mover(
                    posicion,
                    alcance,
                    (0,
                    1),
                    estado,
                    enemigos_impactados,laberinto,
                );
            }
            Direccion::Arriba => {
                Direccion::mover(
                    posicion,
                    alcance,
                    (0,
                    -1),
                    estado,
                    enemigos_impactados,laberinto,
                );
            }
        }
    }

    /// Esta funcion sirve para hacer el desvio segun la direccion recibida.
    pub fn desviarse(
        direccion: String,
        posicion: (usize,usize),
        alcance_desviado: usize,
        laberinto: &mut Laberinto,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>
    ) {
        match direccion.as_str(){
            "R" => {Direccion::moverse(&Direccion::Derecha,posicion,alcance_desviado,estado,enemigos_impactados,laberinto)},
            "D" => {Direccion::moverse(&Direccion::Abajo,posicion,alcance_desviado,estado,enemigos_impactados,laberinto)},
            "L" => {Direccion::moverse(&Direccion::Izquierda,posicion,alcance_desviado,estado,enemigos_impactados,laberinto)},
            "U" => {Direccion::moverse(&Direccion::Arriba,posicion,alcance_desviado,estado,enemigos_impactados,laberinto)},
            _ => {},
        }
    }

    /// Esta funcion se utiliza para realizar el movimiento, es generalizada para cada direccion.
    /// Si el alcance de la bomba se termina, entonces se termina el movimiento.
    /// Segun los valores de dx y dy es para donde se mueve.
    /// En cada posicion va buscando el objeto que ahi se encuentra.
    pub fn mover(
        posicion: (usize,usize),
        alcance: usize,
        direccion: (isize,isize),
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
        laberinto: &mut Laberinto,
    ) {
        let (mut y, mut x) = (posicion.1 as isize, posicion.0 as isize);
        let mut alcance_desviado = alcance;
        while alcance_desviado > 0 {
            y += direccion.1;
            x += direccion.0;
            if y >= 0 && x >= 0 {
                if (y as usize) < laberinto.datos.len() && (x as usize) < laberinto.datos[0].len(){
                    let valor_objeto = Objeto::objeto_encontrado(
                        laberinto,
                        (x as usize, y as usize),
                        alcance_desviado,
                        estado,
                        enemigos_impactados,
                    );
                    if valor_objeto == 1 {
                        break;
                    }
                }
            } else {
                break;
            }
            alcance_desviado -= 1;
        }
    }

}
