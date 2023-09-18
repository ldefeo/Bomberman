#[derive(Debug, PartialEq, Clone)]
pub struct Pared {
    identificador: String,
}

impl Pared {
    /// Generador de pared
    pub fn generar(elemento: String) -> Self {
        Pared {
            identificador: elemento,
        }
    }

    pub fn identificador(self) -> String {
        self.identificador
    }

    /// Frena la detonacion
    pub fn manejar() -> i32 {
        1
    }
}
