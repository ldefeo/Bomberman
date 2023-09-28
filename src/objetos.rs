use crate::{
    bomba::{BombaNormal, BombaTraspaso},
    desvio::Desvio,
    enemigo::Enemigo,
    laberinto::Laberinto,
    pared::Pared,
    roca::Roca,
    vacio::Vacio,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Objeto {
    /// enumerable de objetos
    BombaNormal(BombaNormal),
    Roca(Roca),
    Pared(Pared),
    Vacio(Vacio),
    Enemigo(Enemigo),
    Desvio(Desvio),
    BombaTraspaso(BombaTraspaso),
}

impl Objeto {
    /// Segun el string que recibe, matchea y genera el Objeto.
    pub fn matchear_identificador(elemento: String) -> Objeto {
        let mut str_elemento = elemento.chars();
        if elemento.len() == 2 {
            match (str_elemento.next(), str_elemento.next()) {
                (Some('B'), _) => Objeto::BombaNormal(BombaNormal::generar(elemento)),
                (Some('F'), _) => Objeto::Enemigo(Enemigo::generar(elemento)),
                (Some('D'), _) => Objeto::Desvio(Desvio::generar(elemento)),
                (Some('S'), _) => Objeto::BombaTraspaso(BombaTraspaso::generar(elemento)),
                _ => Objeto::Vacio(Vacio::generar(elemento)),
            }
        } else {
            match elemento.as_str() {
                "R" => Objeto::Roca(Roca::generar(elemento)),
                "W" => Objeto::Pared(Pared::generar(elemento)),
                _ => Objeto::Vacio(Vacio::generar(elemento)),
            }
        }
    }

    /// Esta funcion se utiliza para encontrar el objeto que hay en determinada posicion.
    /// Cada objeto maneja diferente la detonacion de la bomba: una bomba se detona, una roca frena o no la detonacion (segun la bomba), un desvio
    /// desvia la detonacion, etc.
    pub fn objeto_encontrado(
        laberinto: &mut Laberinto,
        posicion: (usize, usize),
        alcance_desviado: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) -> i32 {
        match &laberinto.datos[posicion.1][posicion.0] {
            Objeto::BombaNormal(_box) => {
                BombaNormal::manejar(_box.clone(), posicion, laberinto);
                0
            }
            Objeto::Roca(_box) => Roca::manejar(estado),
            Objeto::Pared(_box) => Pared::manejar(),
            Objeto::Vacio(_box) => {
                Vacio::manejar(posicion, laberinto);
                0
            }
            Objeto::Enemigo(_box) => {
                Enemigo::manejar(_box.clone(), posicion, laberinto, enemigos_impactados);
                0
            }
            Objeto::Desvio(_box) => {
                Desvio::manejar(
                    _box.clone(),
                    posicion,
                    alcance_desviado,
                    laberinto,
                    estado,
                    enemigos_impactados,
                );
                1
            }
            Objeto::BombaTraspaso(_box) => {
                BombaTraspaso::manejar(_box.clone(), posicion, laberinto);
                0
            }
        }
    }

    /// Esta funcion matchea el objeto del laberinto y formatea su firma (hace el proceso inverso a matchear_identificador()).
    pub fn matchear_objeto_laberinto(&self) -> String {
        match &self {
            Objeto::BombaNormal(_box) => {
                format!("{}{}", _box.clone().identificador(), _box.clone().alcance())
            }
            Objeto::Roca(_box) => _box.clone().identificador().to_string(),
            Objeto::Pared(_box) => _box.clone().identificador().to_string(),
            Objeto::Vacio(_box) => _box.clone().identificador().to_string(),
            Objeto::Enemigo(_box) => {
                format!("{}{}", _box.clone().identificador(), _box.clone().vidas())
            }
            Objeto::Desvio(_box) => {
                format!(
                    "{}{}",
                    _box.clone().identificador(),
                    _box.clone().direccion()
                )
            }
            Objeto::BombaTraspaso(_box) => {
                format!("{}{}", _box.clone().identificador(), _box.clone().alcance())
            }
        }
    }
}
