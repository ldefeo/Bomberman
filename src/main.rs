use std::fs;
use std::{env, num::ParseIntError};

use bomberman::laberinto::Laberinto;

/// me escribe el archivo de salida.
fn escribir_archivo(nombre_archivo: &str, contenido: String) {
    let resultado = fs::write(nombre_archivo, contenido);
    match resultado {
        Ok(resultado) => resultado,
        Err(_) => {
            eprint!("No se creo el archivo");
        }
    }
}

/// me lee la matriz del laberinto linea por linea
fn leer_por_linea(matriz: Vec<Vec<String>>) -> String {
    let mut contenido = String::new();
    for fila in matriz {
        contenido.push_str(&fila.join(" "));
        contenido.push('\n');
    }
    contenido
}

/// del laberinto inicial, pasa al laberinto transformado.
/// Lo lee linea por linea y lo escribe en el archivo de salida.
fn traspasar_laberinto_transformado(
    mut laberinto: Laberinto,
    coord_x: usize,
    coord_y: usize,
    nombre_archivo: &str,
) {
    let resultado = laberinto.atravesar_laberinto(coord_x, coord_y);
    match resultado {
        Ok(_box) => {
            let contenido = leer_por_linea(_box.escribir_laberinto());
            escribir_archivo(nombre_archivo, contenido);
        }
        Err(_) => {
            escribir_archivo(
                nombre_archivo,
                "Error al leer el laberinto modificado".to_string(),
            );
        }
    }
}

/// lee el archivo de entrada y genera el laberinto final con traspasar_laberinto_transformado
fn lectura_archivo(
    nombre_archivo_entrada: &str,
    coord_x: usize,
    coord_y: usize,
    nombre_archivo_salida: &str,
) {
    let matriz_lectura = fs::read_to_string(nombre_archivo_entrada);
    match matriz_lectura {
        Ok(c) => {
            let laberinto = Laberinto::generar_laberinto(&c);
            traspasar_laberinto_transformado(laberinto, coord_x, coord_y, nombre_archivo_salida);
        }
        Err(_) => {
            escribir_archivo(
                nombre_archivo_salida,
                "Error al leer el archivo de entrada".to_string(),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    let output = &args[2];
    let x: Result<usize, ParseIntError> = args[3].parse();
    let y: Result<usize, ParseIntError> = args[4].parse();

    let output_concatenado = format!("{}{}", output, input);

    match (x, y) {
        (Ok(_coord_x), Ok(_coord_y)) => {
            lectura_archivo(input, _coord_x, _coord_y, &output_concatenado);
        }
        (Err(_), Err(_)) => {
            escribir_archivo(
                &output_concatenado,
                "No se pudo parsear correctamente las coordenadas".to_string(),
            );
        }
        (Ok(_), Err(_)) => {
            escribir_archivo(&output_concatenado, "No se puso parsear y".to_string());
        }
        (Err(_), Ok(_)) => {
            escribir_archivo(&output_concatenado, "No se puso parsear x".to_string());
        }
    }
}
