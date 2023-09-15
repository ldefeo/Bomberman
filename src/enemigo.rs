use crate::{generador::Generador, laberinto::Laberinto, vacio::Vacio, objetos::Objeto};


#[derive(Debug,PartialEq,Clone)]
pub struct Enemigo{
    identificador: String,
    vidas: usize,
}

#[derive(Debug)]
pub enum EnemigoError {
    VidasError,
}


impl Enemigo{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            if valor >= 1 || valor <= 3{ //agregar manejo de errores
                Enemigo { identificador: ident, vidas: valor }
            }else{
                Enemigo { identificador: "_".to_string(), vidas: 0 }
            }
            
        }else{
            Enemigo { identificador: "_".to_string(), vidas: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn vidas(self) -> usize{
        self.vidas
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,laberinto:&mut Laberinto,enemigos_impactados: &mut Vec<(usize,usize)>){
        
        if !enemigos_impactados.contains(&(coord_y,coord_x)){
            enemigos_impactados.push((coord_y,coord_x));
            let vidas = self.clone().vidas() - 1;
            laberinto.datos[coord_y][coord_x] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",self.clone().identificador(),vidas)));
            if vidas == 0{
                laberinto.datos[coord_y][coord_x] = Objeto::Vacio(Vacio::generar("_".to_string()));}
        }
            
        
    }
}