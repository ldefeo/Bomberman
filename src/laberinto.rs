use crate::{
    bomba::{BombaNormal, BombaTraspaso},
    generador::Generador,
    movimiento::Direccion,
    objetos::Objeto,
    rafaga::Rafaga,
    vacio::Vacio,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Laberinto {
    /// Estructura laberinto con una matriz de objetos como atributo
    pub datos: Vec<Vec<Objeto>>,
}

impl Laberinto {
    /// Genera el laberinto utilizando al Generador para obtener la matriz de objetos.
    pub fn generar_laberinto(matriz: &str) -> Self {
        let datos = Generador::generar_matriz_objetos(matriz);
        Laberinto { datos }
    }

    pub fn matriz(self) -> Vec<Vec<Objeto>> {
        self.datos
    }

    /// Esta funcion se fija si la coordenada recibida esta entre los parametros del laberinto.
    /// Luego chequea si hay una bomba en esa coordenada y sino devuelve un error.
    pub fn atravesar_laberinto(
        &mut self,
        coord_x: usize,
        coord_y: usize,
    ) -> Result<&mut Self, String> {
        if coord_y < self.datos.len() && coord_x < self.datos[coord_y].len() {
            let resultado = self.chequear_bomba((coord_x, coord_y));
            match resultado {
                0 => Ok(self),
                _ => Err("ERROR: no hay bomba en esa posicion".to_string()),
            }
        } else {
            Err("ERROR: no son coordenadas validas".to_string())
        }
    }

    /// Me devuelve la matriz de strings generada por el Generador (seria el laberinto finalizado)
    pub fn escribir_laberinto(&mut self) -> Vec<Vec<String>> {
        Generador::generar_matriz(&mut self.datos)
    }

    /// Esta funcion matchea el objeto que hay en la coordenada provista, si hay bomba entonces la detona devolviendo true, sino devuelve false.
    pub fn chequear_bomba(&mut self, posicion: (usize, usize)) -> usize {
        match &self.datos[posicion.1][posicion.0] {
            Objeto::BombaNormal(_box) => {
                self.detonar(posicion, _box.clone().alcance(), BombaNormal::estado());
                0
            }
            Objeto::BombaTraspaso(_box) => {
                self.detonar(posicion, _box.clone().alcance(), BombaTraspaso::estado());
                0
            }
            _ => 1,
        }
    }

    /// Funcion para detonar la bomba moviendose en todas las direcciones.
    pub fn detonar(&mut self, posicion: (usize, usize), alcance: usize, estado: i32) -> &mut Self {
        self.datos[posicion.1][posicion.0] = Objeto::Vacio(Vacio::generar("_".to_string()));
        Rafaga::iniciar(Direccion::Izquierda, posicion, alcance, estado, self);
        Rafaga::iniciar(Direccion::Abajo, posicion, alcance, estado, self);
        Rafaga::iniciar(Direccion::Derecha, posicion, alcance, estado, self);
        Rafaga::iniciar(Direccion::Arriba, posicion, alcance, estado, self);
        self
    }
}
