use crate::objetos::Objeto;

#[derive(Debug)]
pub struct Generador;

impl Generador {
    /// Esta funcion recibe la matriz de tipo string y separa las filas por linea y las columnas por espacio.
    /// Identifica cada elemento de la matriz en un objeto y lo genera, luego los agrega a la matriz.
    /// Genera una matriz de objetos.
    pub fn generar_matriz_objetos(valor: &str) -> Vec<Vec<Objeto>> {
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

    /// Esta funcion recibe la matriz de objetos y genera una matriz de strings.
    /// Formatea cada objeto de la matriz de objetos en un string y los agrega a la matriz.
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

    /// Esta funcion divide el string que recibe en parte string parte usize
    pub fn dividir_string(valor: &str) -> (String, usize) {
        let (identificador, numero): (String, String) =
            valor.chars().partition(|&c| c.is_alphabetic());

        let num = match numero.parse::<usize>() {
            Ok(parsed_num) => parsed_num,
            Err(_) => {
                eprint!("ERROR: mal parseo del numero");
                0
            }
        };

        (identificador, num)
    }
}
