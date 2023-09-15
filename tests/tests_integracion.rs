#[cfg(test)]
mod tests{
    use bomberman::laberinto::Laberinto;

    #[test]
    fn test_ejemplo_laberinto_1(){
        let mut laberinto = Laberinto::generar_laberinto("B2 R R _ F1 _ _\n_ W R W _ W _\nB5 _ _ _ B2 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ R R _ _ _ _\n_ W R W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(laberinto.atravesar_laberinto(0,0),&mut laberinto_transformado);
    }

    #[test]
    fn test_ejemplo_laberinto_2(){
        let mut laberinto = Laberinto::generar_laberinto("_ _ B2 _ B1 _ _\n_ W _ W _ W _\n_ _ B2 R F1 _ _\n_ W _ W R W _\n_ _ B4 _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ R F1 _ _\n_ W _ W R W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1");
        assert_eq!(laberinto.atravesar_laberinto(4,2),&mut laberinto_transformado);
    }

    #[test]
    fn test_ejemplo_laberinto_3(){
        let mut laberinto = Laberinto::generar_laberinto("_ _ _ _ _ _ _\n_ W _ W _ W _\nS4 R R R F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _ _ _ _ _\n_ W _ W _ W _\n_ R R R _ _ _\n_ W _ W _ W _\n_ _ _ _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(laberinto.atravesar_laberinto(4,0),&mut laberinto_transformado);
    }
}