use crate::{estado_bomba::EstadoBomba, generador::Generador, laberinto::Laberinto};

#[derive(Debug, PartialEq, Clone)]
pub struct BombaNormal {
    identificador: String,
    alcance: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BombaTraspaso {
    identificador: String,
    alcance: usize,
}

impl BombaNormal {
    /// Generador de una bomba de tipo normal
    pub fn generar(elemento: String) -> Self {
        let resultado = Generador::dividir_string(&elemento);
        BombaNormal {
            identificador: resultado.0.to_string(),
            alcance: resultado.1,
        }
    }

    pub fn identificador(&self) -> &str {
        &self.identificador
    }

    pub fn alcance(&self) -> usize {
        self.alcance
    }

    /// Estado de una bomba de tipo normal
    pub fn estado() -> i32 {
        EstadoBomba::estado(&EstadoBomba::Normal)
    }

    /// Esta funcion genera una lista de enemigos impactados y detona la bomba
    pub fn manejar(self, posicion: (usize,usize), laberinto: &mut Laberinto) {
        laberinto.detonar(
            posicion,
            self.clone().alcance(),
            BombaNormal::estado(),
        );
    }
}

impl BombaTraspaso {
    /// Generador de una bomba de tipo traspaso
    pub fn generar(elemento: String) -> Self {
        let resultado = Generador::dividir_string(&elemento);
        BombaTraspaso {
            identificador: resultado.0.to_string(),
            alcance: resultado.1,
        }
    }

    pub fn identificador(&self) -> &str {
        &self.identificador
    }

    pub fn alcance(&self) -> usize {
        self.alcance
    }

    /// Estado de una bomba de tipo traspaso
    pub fn estado() -> i32 {
        EstadoBomba::estado(&EstadoBomba::Traspaso)
    }

    /// Esta funcion genera una lista de enemigos impactados y detona la bomba
    pub fn manejar(self, posicion: (usize,usize), laberinto: &mut Laberinto) {
        laberinto.detonar(
            posicion,
            self.clone().alcance(),
            BombaTraspaso::estado(),
        );
    }
}
