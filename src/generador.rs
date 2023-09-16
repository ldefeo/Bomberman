use crate::objetos::Objeto;

#[derive(Debug)]
pub struct Generador;

impl Generador {
    pub fn generar_matriz_objetos(valor: &str) -> Vec<Vec<Objeto>> {
        //generador de matrices de objetos
        let mut matriz: Vec<Vec<Objeto>> = Vec::new();
        for f in valor.lines() {
            let mut vector_intermedio: Vec<Objeto> = Vec::new();
            for c in f.split_whitespace() {
                vector_intermedio.push(Objeto::matchear_identificador(c.to_string()));
            }
            matriz.push(vector_intermedio);
        }
        matriz
    }

    pub fn generar_matriz(objetos: &mut Vec<Vec<Objeto>>) -> Vec<Vec<String>> {
        let mut matriz: Vec<Vec<String>> = Vec::new();
        for f in objetos {
            let mut vec_intermedio: Vec<String> = Vec::new();
            for c in f {
                vec_intermedio.push(Objeto::matchear_objeto_laberinto(c));
            }
            matriz.push(vec_intermedio);
        }
        matriz
    }

    pub fn dividir_string(valor: &String) -> (String, usize) {
        let (identificador, numero): (String, String) =
            valor.chars().partition(|&c| c.is_alphabetic());

        let num = match numero.parse::<usize>() {
            Ok(parsed_num) => parsed_num,
            Err(_) => 0,
        };

        (identificador, num)
    }
}
