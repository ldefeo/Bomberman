use crate::{
    bomba::{BombaNormal, BombaTraspaso},
    generador::Generador,
    movimiento::Movimiento,
    objetos::Objeto,
    vacio::Vacio,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Laberinto {
    pub datos: Vec<Vec<Objeto>>,
}

impl Laberinto {
    pub fn generar_laberinto(matriz: &str) -> Self {
        //generador de laberintos
        let datos = Generador::generar_matriz_objetos(matriz);
        Laberinto { datos }
    }

    pub fn matriz(self) -> Vec<Vec<Objeto>> {
        self.datos
    }

    pub fn atravesar_laberinto(
        &mut self,
        coord_x: usize,
        coord_y: usize,
    ) -> Result<&mut Self, String> {
        if coord_y < self.datos.len() && coord_x < self.datos[coord_y].len() {
            match self.clone().get_objeto(coord_x, coord_y) {
                Some(objeto) => {
                    let resultado = self.chequear_bomba(objeto, coord_x, coord_y);
                    if resultado == 0 {
                        return Ok(self);
                    } else {
                        return Err("No hay bomba en esa posicion".to_string());
                    }
                }
                None => {
                    return Err("No se hayo objeto en esa posicion".to_string());
                }
            }
        } else {
            return Err("no son coordenadas validas".to_string());
        }
    }

    pub fn escribir_laberinto(&mut self) -> Vec<Vec<String>> {
        let matriz = Generador::generar_matriz(&mut self.datos);
        matriz
    }

    pub fn chequear_bomba(&mut self, objeto: &Objeto, coord_x: usize, coord_y: usize) -> usize {
        match objeto {
            Objeto::BombaNormal(_box) => {
                let mut enemigos_impactados: Vec<(usize, usize)> = Vec::new();
                self.detonar(
                    coord_x,
                    coord_y,
                    _box.clone().alcance(),
                    BombaNormal::estado(),
                    &mut enemigos_impactados,
                );
                0
            }
            Objeto::BombaTraspaso(_box) => {
                let mut enemigos_impactados: Vec<(usize, usize)> = Vec::new();
                self.detonar(
                    coord_x,
                    coord_y,
                    _box.clone().alcance(),
                    BombaTraspaso::estado(),
                    &mut enemigos_impactados,
                );
                0
            }
            _ => 1,
        }
    }

    pub fn get_objeto(&mut self, x: usize, y: usize) -> Option<&Objeto> {
        if y < self.datos.len() && x < self.datos[0].len() {
            Some(&self.datos[y][x])
        } else {
            None
        }
    }

    pub fn detonar(
        &mut self,
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) -> &mut Self {
        self.datos[coord_y][coord_x] = Objeto::Vacio(Vacio::generar("_".to_string()));
        self.moverse_izquierda(coord_x, coord_y, alcance, estado, enemigos_impactados);
        self.moverse_abajo(coord_x, coord_y, alcance, estado, enemigos_impactados);
        self.moverse_derecha(coord_x, coord_y, alcance, estado, enemigos_impactados);
        self.moverse_arriba(coord_x, coord_y, alcance, estado, enemigos_impactados);
        self
    }

    pub fn moverse_derecha(
        &mut self,
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        Movimiento::moverse(
            &Movimiento::Derecha,
            coord_x,
            coord_y,
            alcance,
            self,
            estado,
            enemigos_impactados,
        );
    }

    pub fn moverse_izquierda(
        &mut self,
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        Movimiento::moverse(
            &Movimiento::Izquierda,
            coord_x,
            coord_y,
            alcance,
            self,
            estado,
            enemigos_impactados,
        );
    }

    pub fn moverse_abajo(
        &mut self,
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        Movimiento::moverse(
            &Movimiento::Abajo,
            coord_x,
            coord_y,
            alcance,
            self,
            estado,
            enemigos_impactados,
        );
    }

    pub fn moverse_arriba(
        &mut self,
        coord_x: usize,
        coord_y: usize,
        alcance: usize,
        estado: i32,
        enemigos_impactados: &mut Vec<(usize, usize)>,
    ) {
        Movimiento::moverse(
            &Movimiento::Arriba,
            coord_x,
            coord_y,
            alcance,
            self,
            estado,
            enemigos_impactados,
        );
    }
}
