use crate::{generador::Generador, laberinto::Laberinto, vacio::Vacio, objetos::Objeto};


#[derive(Debug,PartialEq,Clone)]
pub struct Enemigo{
    identificador: String,
    vidas: usize,
}


impl Enemigo{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            Enemigo { identificador: ident, vidas: valor }
        }else{
            Enemigo { identificador: "_".to_string(), vidas: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.vidas
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,alcance_desviado:usize,laberinto:&mut Laberinto){
        let vidas = self.clone().alcance() - 1;
        laberinto.datos[coord_x][coord_y] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",self.clone().identificador(),vidas)));
        if vidas == 0{
            laberinto.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));}
    }
}