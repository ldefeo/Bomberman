use std::fs;

use Bomberman::laberinto::Laberinto;
fn main(){
    let mut matriz = Laberinto::generador_matriz("_ _ _ _ _\nDR B8 DL _ F2\n_ _ _ _ _\n_ _ _ _ _\n_ _ _ _ _");
    matriz.atravesar_laberinto(1, 1);
    //println!("{:?}",matriz_cambiada);
    // let contenido = fs::read_to_string("input.txt").expect("Leo el archivo");
    // let mut laberinto = Laberinto::generador_matriz(&contenido);
    // let mut laberinto_cambiado = laberinto.atravesar_laberinto(2, 4);
    // //fs::write("output.txt", laberinto_cambiado).expect("Escribo el archivo");
}