use std::fs;

use Bomberman::laberinto::Laberinto;
fn main(){
    let mut matriz = Laberinto::generador_matriz("B2 B1 _\nB1 _ B1\n_ _ _");
    let mut laberinto_cambiado = matriz.atravesar_laberinto(0, 1);
    // let contenido = fs::read_to_string("input.txt").expect("Leo el archivo");
    // let mut laberinto = Laberinto::generador_matriz(&contenido);
    // let mut laberinto_cambiado = laberinto.atravesar_laberinto(2, 4);
    // //fs::write("output.txt", laberinto_cambiado).expect("Escribo el archivo");
}