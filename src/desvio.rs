use crate::{laberinto::Laberinto, movimiento::Movimiento};

#[derive(Debug, PartialEq, Clone)]
pub struct Desvio {
    identificador: String,
    direccion: String,
}

impl Desvio {
    pub fn generar(elemento: String) -> Self {
        let mut elemento_modificado = elemento.chars();
        match (elemento_modificado.next(), elemento_modificado.next()) {
            (Some(id), Some(dir)) => Desvio {
                identificador: id.to_string(),
                direccion: dir.to_string(),
            },
            _ => Desvio {
                identificador: "".to_string(),
                direccion: "".to_string(),
            },
        }
    }

    pub fn identificador(self) -> String {
        self.identificador
    }

    pub fn direccion(self) -> String {
        self.direccion
    }

    pub fn manejar(
        &self,
        coord_x: usize,
        coord_y: usize,
        alcance_desviado: usize,
        laberinto: &mut Laberinto,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        if alcance_desviado > 1 {
            Movimiento::desviar(
                self.clone().direccion(),
                coord_x,
                coord_y,
                alcance_desviado,
                laberinto,
                estado,
                enemigos_impactados,
            );
        }
    }
}
