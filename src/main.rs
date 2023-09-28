use std::fs;
use std::path::{PathBuf, Path};
use std::{env, num::ParseIntError};

use bomberman::laberinto::Laberinto;

/// me escribe el archivo de salida.
fn escribir_archivo(nombre_archivo: &Path, contenido: String) {
    let resultado = fs::write(nombre_archivo, contenido);
    match resultado {
        Ok(resultado) => resultado,
        Err(_) => {
            eprint!("ERROR: no se creo el archivo");
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
    nombre_archivo: &Path,
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
                "ERROR: no se pudo leer bien el laberinto modificado".to_string(),
            );
        }
    }
}

/// lee el archivo de entrada y genera el laberinto final con traspasar_laberinto_transformado
fn lectura_archivo(
    nombre_archivo_entrada: &Path,
    coord_x: usize,
    coord_y: usize,
    nombre_archivo_salida: &Path,
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
                "ERROR: no se pudo leer bien el archivo".to_string(),
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);
    let x: Result<usize, ParseIntError> = args[3].parse();
    let y: Result<usize, ParseIntError> = args[4].parse();

    let mut output_formalizado: PathBuf = PathBuf::new();
    if output.is_dir(){
        if let Some(nombre_archivo) = input.file_name(){
            output_formalizado = output.join(nombre_archivo);
        }else{
            eprint!("ERROR: error de concatenacion");
        }
    }

    match (x, y) {
        (Ok(_coord_x), Ok(_coord_y)) => {
            lectura_archivo(input, _coord_x, _coord_y, &output_formalizado);
        }
        (Err(_), Err(_)) => {
            escribir_archivo(
                &output_formalizado,
                "ERROR: no se pudo parsear correctamente las coordenadas".to_string(),
            );
        }
        (Ok(_), Err(_)) => {
            escribir_archivo(&output_formalizado, "ERROR: no se puso parsear y".to_string());
        }
        (Err(_), Ok(_)) => {
            escribir_archivo(&output_formalizado, "ERROR: no se puso parsear x".to_string());
        }
    }
}
