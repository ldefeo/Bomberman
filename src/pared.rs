#[derive(Debug, PartialEq, Clone)]
pub struct Pared {
    identificador: String,
}

impl Pared {
    pub fn generar(elemento: String) -> Self {
        Pared {
            identificador: elemento,
        }
    }

    pub fn identificador(self) -> String {
        self.identificador
    }

    pub fn manejar() -> i32 {
        1
    }
}
