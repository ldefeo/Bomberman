use crate::{generador::Generador, laberinto::Laberinto, objetos::Objeto, vacio::Vacio};

#[derive(Debug, PartialEq, Clone)]
pub struct Enemigo {
    identificador: String,
    vidas: usize,
}

#[derive(Debug)]
pub enum EnemigoError {
    VidasError,
}

impl Enemigo {
    pub fn generar(elemento: String) -> Self {
        let mut valor = 0;
        let resultado = Generador::dividir_string(&elemento);
        if resultado.1 >= 1 && resultado.1 <= 3 {
            valor = resultado.1;
        }
        Enemigo {
            identificador: resultado.0.to_string(),
            vidas: valor,
        }
    }

    pub fn identificador(self) -> String {
        self.identificador
    }

    pub fn vidas(self) -> usize {
        self.vidas
    }

    pub fn manejar(
        &self,
        coord_x: usize,
        coord_y: usize,
        laberinto: &mut Laberinto,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        if !enemigos_impactados.contains(&(coord_y, coord_x)) {
            enemigos_impactados.push((coord_y, coord_x));
            let vidas = self.clone().vidas() - 1;
            laberinto.datos[coord_y][coord_x] = Objeto::Enemigo(Enemigo::generar(format!(
                "{}{}",
                self.clone().identificador(),
                vidas
            )));
            if vidas == 0 {
                laberinto.datos[coord_y][coord_x] = Objeto::Vacio(Vacio::generar("_".to_string()));
            }
        }
    }
}
