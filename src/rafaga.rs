use crate::{laberinto::Laberinto, movimiento::Direccion};

#[derive(Debug, PartialEq)]
pub struct Rafaga {
    pub direccion: Direccion,
    pub enemigos_impactados: Vec<(usize, usize)>,
}

impl Rafaga {
    /// Esta funcion sirve para generar la rafaga en la direccion correspondiente.
    pub fn iniciar(
        direccion: Direccion,
        posicion: (usize, usize),
        alcance: usize,
        estado: i32,
        laberinto: &mut Laberinto,
    ) {
        let mut enemigos_impactados: Vec<(usize, usize)> = Vec::new();
        Direccion::moverse(
            &direccion,
            posicion,
            alcance,
            estado,
            &mut enemigos_impactados,
            laberinto,
        );
    }
}
