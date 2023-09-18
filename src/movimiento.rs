use crate::laberinto::Laberinto;

#[derive(Debug, PartialEq)]
pub enum Movimiento {
    /// enumerable de movimientos.
    Derecha,
    Izquierda,
    Abajo,
    Arriba,
}

impl Movimiento {
    /// Esta funcion sirve para matchear el movimiento que se debe realizar.
    pub fn moverse(
        &self,
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        mut laberinto: &mut Laberinto,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        match &self {
            Movimiento::Derecha => {
                Movimiento::mover(
                    coord_x,
                    coord_y,
                    alcance,
                    &mut laberinto,
                    1,
                    0,
                    estado,
                    enemigos_impactados,
                );
            }
            Movimiento::Izquierda => {
                Movimiento::mover(
                    coord_x,
                    coord_y,
                    alcance,
                    &mut laberinto,
                    -1,
                    0,
                    estado,
                    enemigos_impactados,
                );
            }
            Movimiento::Abajo => {
                Movimiento::mover(
                    coord_x,
                    coord_y,
                    alcance,
                    &mut laberinto,
                    0,
                    1,
                    estado,
                    enemigos_impactados,
                );
            }
            Movimiento::Arriba => {
                Movimiento::mover(
                    coord_x,
                    coord_y,
                    alcance,
                    &mut laberinto,
                    0,
                    -1,
                    estado,
                    enemigos_impactados,
                );
            }
        }
    }

    /// Esta funcion sirve para hacer el desvio segun la direccion recibida.
    pub fn desviar(
        direccion: String,
        coord_x: usize,
        coord_y: usize,
        alcance_desviado: usize,
        laberinto: &mut Laberinto,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        match direccion.as_str() {
            "R" => {
                Movimiento::moverse(
                    &Movimiento::Derecha,
                    coord_x,
                    coord_y,
                    alcance_desviado,
                    laberinto,
                    estado,
                    enemigos_impactados,
                );
            }
            "L" => {
                Movimiento::moverse(
                    &Movimiento::Izquierda,
                    coord_x,
                    coord_y,
                    alcance_desviado,
                    laberinto,
                    estado,
                    enemigos_impactados,
                );
            }
            "D" => {
                Movimiento::moverse(
                    &Movimiento::Abajo,
                    coord_x,
                    coord_y,
                    alcance_desviado,
                    laberinto,
                    estado,
                    enemigos_impactados,
                );
            }
            "U" => {
                Movimiento::moverse(
                    &Movimiento::Arriba,
                    coord_x,
                    coord_y,
                    alcance_desviado,
                    laberinto,
                    estado,
                    enemigos_impactados,
                );
            }
            _ => {}
        }
    }

    /// Esta funcion se utiliza para realizar el movimiento, es generalizada para cada movimiento.
    /// Si el alcance de la bomba se termina, entonces se termina el movimiento.
    /// Segun los valores de dx y dy es para donde se mueve.
    /// En cada posicion va buscando el objeto que ahi se encuentra.
    pub fn mover(
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        laberinto: &mut Laberinto,
        dx: isize,
        dy: isize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        let (mut y, mut x) = (coord_y as isize, coord_x as isize);
        let mut alcance_desviado = alcance;
        while alcance_desviado > 0 {
            y += dy;
            x += dx;
            if y >= 0 && x >= 0 {
                if let Some(objeto) = laberinto.clone().get_objeto(x as usize, y as usize) {
                    let valor_objeto = objeto.objeto_encontrado(
                        laberinto,
                        x as usize,
                        y as usize,
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
