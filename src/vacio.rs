use crate::{laberinto::Laberinto, objetos::Objeto};

#[derive(Debug, PartialEq, Clone)]
pub struct Vacio {
    identificador: String,
}

impl Vacio {
    /// Generador de vacio.
    pub fn generar(elemento: String) -> Self {
        Vacio {
            identificador: elemento,
        }
    }

    pub fn identificador(self) -> String {
        self.identificador
    }

    /// No hace nada con la detonacion, la deja que continue normalmente.
    pub fn manejar(posicion: (usize,usize), laberinto: &mut Laberinto) {
        laberinto.datos[posicion.1][posicion.0] = Objeto::Vacio(Vacio::generar("_".to_string()));
    }
}
