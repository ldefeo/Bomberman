use std::fs;

use Bomberman::laberinto::Laberinto;
fn main(){
    let mut matriz = Laberinto::generador_matriz("_ F3 _ _\nW B4 _ DD\n_ R _ B1\n_ _ R F1");
    let mut laberinto_cambiado = matriz.atravesar_laberinto(1, 1);
    // let contenido = fs::read_to_string("input.txt").expect("Leo el archivo");
    // let mut laberinto = Laberinto::generador_matriz(&contenido);
    // let mut laberinto_cambiado = laberinto.atravesar_laberinto(2, 4);
    // //fs::write("output.txt", laberinto_cambiado).expect("Escribo el archivo");
}