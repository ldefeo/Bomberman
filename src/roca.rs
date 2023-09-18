#[derive(Debug, PartialEq, Clone)]
pub struct Roca {
    identificador: String,
}

impl Roca {
    /// Generador de roca
    pub fn generar(elemento: String) -> Self {
        Roca {
            identificador: elemento,
        }
    }

    pub fn identificador(self) -> String {
        self.identificador
    }

    pub fn alcance(self) -> usize {
        0
    }

    /// Si el estado es de traspaso, entonces no frena la detonacion
    pub fn manejar(estado: i32) -> i32 {
        if estado == 0 {
            0
        } else {
            1
        }
    }
}
