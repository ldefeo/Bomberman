
use crate::objetos::Objeto;


#[derive(Debug)]
pub struct Generador;


impl Generador {
    
    
    pub fn generar_matriz(valor: &str) -> Vec<Vec<Objeto>>{ //generador de matrices de objetos
        let mut matriz: Vec<Vec<Objeto>> = Vec::new();
        for f in valor.split("\n"){ 
            let mut vector_intermedio: Vec<Objeto> = Vec::new();
            for c in f.split(" "){ 
                vector_intermedio.push(Objeto::matchear_identificador(c.to_string()));
            }
            matriz.push(vector_intermedio);
        }
        matriz
    }

    pub fn dividir_string(valor: &String) -> Result<(String, usize), String>{
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