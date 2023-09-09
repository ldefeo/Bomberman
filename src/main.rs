#[derive(Debug,PartialEq)]
pub struct Matriz {
    datos: Vec<Vec<String>>,
}

impl Matriz {

    fn generador_matriz(valor: &str) -> Matriz{  //generador de matrices
        let mut datos: Vec<Vec<String>> = Vec::new();
        for f in valor.split("\n"){    // si el valor contiene \n significa fin de fila
            let mut vector_intermedio: Vec<String> = Vec::new(); 
            for c in f.split(" "){  // si el valor contiene espacio entre caracteres significa fin de columna
                vector_intermedio.push(c.to_string());
            }
            datos.push(vector_intermedio);
        }
        Matriz {datos}
    }

    pub fn atravesar_laberinto(&mut self,coord_x:usize,coord_y:usize) -> &mut Self{
        let mut matriz_vacia = Matriz::generador_matriz(" ");
        if coord_x < self.datos.len() && coord_y < self.datos[coord_x].len() {   //me quedo con el elemento en la coordenada (x,y)
            if self.datos[coord_x][coord_y] != "_".to_string() {   //si no hay un _ entonces hay una bomba
                self.detonar(coord_x,coord_y);
            }
        }else{
            *self = matriz_vacia;
        }
        self
    }

    pub fn detonar(&mut self,coord_x: usize,coord_y: usize) -> &mut Self{
        let resultado = self.dividir_string(&self.datos[coord_x][coord_y]);  //divido el string Bnumero en (B,numero)
        if let Ok((identificador,alcance)) = resultado{
            //derecha
            self.datos[coord_x][coord_y] = "_".to_string();
            for idx in coord_x..=coord_x.min(self.datos.len()-1){  //recorro la fila desde la coordenada x hasta la finalizacion de la fila
                for idy in coord_y..=coord_y.min(self.datos[0].len()-1){ //recorro cada elemento de la fila hasta cumplir el alcance
                    for dx in 1..=alcance {
                        if idx < self.datos.len() && idy + dx < self.datos[0].len() {
                            if self.datos[idx][idy + dx] != "_".to_string() {
                                self.detonar(idx, idy + dx); // Detona la nueva bomba
                            }
                            self.datos[idx][idy + dx] = "_".to_string();
                        }
                    //if self.datos[idx][idy+1] != "_".to_string() {  //si no hay un _, llamo recursivamente a detonar porque encontro otra bomba
                    //    self.detonar(idx, idy);
                    //}
                    //self.datos[idx][idy] = "_".to_string();
                    }
                }
            }
        }
        self
    }
    
    pub fn dividir_string(&self,valor: &String) -> Result<(String, usize), String>{
        let (identificador,numero): (String,String) = valor.chars().partition(|&c| c.is_alphabetic());

        let num:Result<usize, _> = numero.parse();
        match num{
            Ok(num) => {
                Ok((identificador,num))
            },
            Err(_) => {
                Err("No se pudo parsear".to_string())
            },
        }
    }

}


fn main(){

}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_matriz_bomba_explota_en_0_0(){
        let mut matriz = Matriz::generador_matriz("B1 _ \n _ _");
        let mut matriz_transformada = Matriz::generador_matriz("_ _ \n _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_3(){
        let mut matriz = Matriz::generador_matriz("B3 _ _ _ \n _ _ _ _");
        let mut matriz_transformada = Matriz::generador_matriz("_ _ _ _ \n _ _ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_1_explota_bomba_consecutiva_alcance_1(){
        let mut matriz = Matriz::generador_matriz("B1 B1 _ \n _ _ _");
        let mut matriz_transformada = Matriz::generador_matriz("_ _ _ \n _ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_bomba2_explota_en_0_2_alcance_1(){
        let mut matriz = Matriz::generador_matriz("B2 _ _ \n B1 _ _ \n _ _ _");
        let mut matriz_transformada = Matriz::generador_matriz("_ _ _ \n _ _ _ \n _ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }
}


