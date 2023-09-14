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

    pub fn vidas(self) -> usize{
        self.vidas
    }

    pub fn manejar(&self,coord_x:usize,coord_y:usize,laberinto:&mut Laberinto,enemigos_impactados: &mut Vec<(usize,usize)>){
        
        if !enemigos_impactados.contains(&(coord_x,coord_y)){
            enemigos_impactados.push((coord_x,coord_y));
            let vidas = self.clone().vidas() - 1;
            laberinto.datos[coord_x][coord_y] = Objeto::Enemigo(Enemigo::generar(format!("{}{}",self.clone().identificador(),vidas)));
            if vidas == 0{
                laberinto.datos[coord_x][coord_y] = Objeto::Vacio(Vacio::generar("_".to_string()));}
        }
            
        
    }
}