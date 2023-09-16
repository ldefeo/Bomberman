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
    BombaNormal(BombaNormal),
    Roca(Roca),
    Pared(Pared),
    Vacio(Vacio),
    Enemigo(Enemigo),
    Desvio(Desvio),
    BombaTraspaso(BombaTraspaso),
}

impl Objeto {
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

    pub fn objeto_encontrado(
        &self,
        laberinto: &mut Laberinto,
        coord_x: usize,
        coord_y: usize,
        alcance_desviado: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) -> i32 {
        match self {
            Objeto::BombaNormal(_box) => {
                BombaNormal::manejar(_box, coord_x, coord_y, laberinto);
                0
            }
            Objeto::Roca(_box) => Roca::manejar(estado),
            Objeto::Pared(_box) => Pared::manejar(),
            Objeto::Vacio(_box) => {
                Vacio::manejar(_box, coord_x, coord_y, laberinto);
                0
            }
            Objeto::Enemigo(_box) => {
                Enemigo::manejar(_box, coord_x, coord_y, laberinto, enemigos_impactados);
                0
            }
            Objeto::Desvio(_box) => {
                Desvio::manejar(
                    _box,
                    coord_x,
                    coord_y,
                    alcance_desviado,
                    laberinto,
                    estado,
                    enemigos_impactados,
                );
                1
            }
            Objeto::BombaTraspaso(_box) => {
                BombaTraspaso::manejar(_box, coord_x, coord_y, laberinto);
                0
            } //me falta la parte de seguir cuando se cruze con roca
        }
    }

    pub fn matchear_objeto_laberinto(&self) -> String{
        match &self {
            Objeto::BombaNormal(_box) => {
                format!(
                    "{}{}",
                    _box.clone().identificador(),
                    _box.clone().alcance()
                )
            }
            Objeto::Roca(_box) => {
                format!("{}", _box.clone().identificador())
            }
            Objeto::Pared(_box) => {
                format!("{}", _box.clone().identificador())
            }
            Objeto::Vacio(_box) => {
                format!("{}", _box.clone().identificador())
            }
            Objeto::Enemigo(_box) => {
                format!(
                    "{}{}",
                    _box.clone().identificador(),
                    _box.clone().vidas()
                )
            }
            Objeto::Desvio(_box) => {
                format!(
                    "{}{}",
                    _box.clone().identificador(),
                    _box.clone().direccion()
                )
            }
            Objeto::BombaTraspaso(_box) => {
                format!(
                    "{}{}",
                    _box.clone().identificador(),
                    _box.clone().alcance()
                )
            }
        }
    }
}
