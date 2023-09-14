
use crate::objetos::Objeto;


#[derive(Debug)]
pub struct Generador;


impl Generador {
    
    //pub fn generador_matriz(valor: &str) -> Vec<Vec<String>>{  //generador de matrices
    pub fn generador_matriz(valor: &str) -> Vec<Vec<Objeto>>{
        //let mut matriz: Vec<Vec<String>> = Vec::new();
        let mut matriz: Vec<Vec<Objeto>> = Vec::new();
        for f in valor.split("\n"){    // si el valor contiene \n significa fin de fila
            //let mut vector_intermedio: Vec<String> = Vec::new(); 
            let mut vector_intermedio: Vec<Objeto> = Vec::new();
            for c in f.split(" "){  // si el valor contiene espacio entre caracteres significa fin de columna
                vector_intermedio.push(Objeto::matcheo(c.to_string()));
                //vector_intermedio.push(c.to_string());
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