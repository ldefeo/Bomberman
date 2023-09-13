use crate::{bomba::{BombaNormal, BombaTraspaso}, roca::Roca, pared::Pared, vacio::Vacio, enemigo::Enemigo, desvio::Desvio, laberinto::Laberinto, movimiento::Movimiento};


#[derive(Debug,PartialEq,Clone)]
pub enum Objeto{
    BombaNormal(BombaNormal),
    Roca(Roca),
    Pared(Pared),
    Vacio(Vacio),
    Enemigo(Enemigo),
    Desvio(Desvio),
    BombaTraspaso(BombaTraspaso),
}


impl Objeto{

    pub fn matcheo(elemento: String) -> Objeto{
        let mut str_elemento = elemento.chars();
        if elemento.len() == 2{
            match (str_elemento.next(),str_elemento.next()){
                (Some('B'),_) => {Objeto::BombaNormal(BombaNormal::generar(elemento))},
                (Some('F'),_) => {Objeto::Enemigo(Enemigo::generar(elemento))},
                (Some('D'),_) => {Objeto::Desvio(Desvio::generar(elemento))},
                (Some('S'),_) => {Objeto::BombaTraspaso(BombaTraspaso::generar(elemento))},
                _ => {Objeto::Vacio(Vacio::generar(elemento))},
            }
        }else{
            match elemento.as_str() {
                "R" => {Objeto::Roca(Roca::generar(elemento))},
                "W" => {Objeto::Pared(Pared::generar(elemento))},
                _ => {Objeto::Vacio(Vacio::generar(elemento))},
            }
        }
    }

    pub fn objeto_encontrado(&self,laberinto: &mut Laberinto,coord_x:usize,coord_y:usize,alcance_desviado:usize) -> i32{
        let mut estado_inicial = Objeto::Vacio(Vacio::generar("_".to_string()));
        match self{
            Objeto::BombaNormal(_box) => {laberinto.detonar(coord_x, coord_y, _box.clone().alcance());2},
            Objeto::Roca(_box) => {1},
            Objeto::Pared(_box) => {1},
            Objeto::Vacio(_box) => {laberinto.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));;0},
            Objeto::Enemigo(_box) => {let vidas = _box.clone().alcance() - 1;
                                                laberinto.datos[coord_x][coord_y] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",_box.clone().identificador(),vidas)));
                                                if vidas == 0{
                                                    laberinto.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));;} 2},
            Objeto::Desvio(_box) => {if alcance_desviado > 0{Movimiento::desviar(_box.clone().alcance(),coord_x,coord_y,alcance_desviado,laberinto);}2},
            Objeto::BombaTraspaso(_box) => {estado_inicial = Objeto::BombaTraspaso(_box.clone());
                                                            laberinto.detonar(coord_x, coord_y, _box.clone().alcance()); 2}, //me falta la parte de seguir cuando se cruze con roca
            _ => {-1}                        
        }

    }

}