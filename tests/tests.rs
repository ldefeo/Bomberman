#[cfg(test)]
mod tests{
    use Bomberman::laberinto::Laberinto;
    use super::*;

    #[test]
    fn test_matriz_bomba_explota_en_0_0(){
        let mut matriz = Laberinto::generador_matriz("B1 _\n_ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _\n_ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_1_explota_bomba_consecutiva_alcance_1_explota(){
        let mut matriz = Laberinto::generador_matriz("B1 B1 _\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_1_no_explota_bomba_en_0_2_alcance_1(){
        let mut matriz = Laberinto::generador_matriz("B1 _ B1\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ B1\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_bomba2_explota_en_1_0_alcance_1_explota(){
        let mut matriz = Laberinto::generador_matriz("B2 _ _\nB1 _ _\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\n_ _ _\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_bombas_sucesivas(){
        let mut matriz = Laberinto::generador_matriz("B2 B1 _\nB1 _ _\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\n_ _ _\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_explota_bomba_en_1_0_alcance_1_explota_bomba_en_1_1_alcance_1_explota(){
        let mut matriz = Laberinto::generador_matriz("B2 _ _\nB1 _ B1\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\n_ _ B1\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_1_alcance_2_explota_bomba_en_1_0_alcance_1_explota_bomba_en_1_1_alcance_1_explota(){
        let mut matriz = Laberinto::generador_matriz("B2 B1 _\nB1 _ B1\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\n_ _ B1\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,1),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_se_traba_por_roca(){
        let mut matriz = Laberinto::generador_matriz("B1 _\nR _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _\nR _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_se_traba_por_roca_no_explota_B1_en_0_2(){
        let mut matriz = Laberinto::generador_matriz("B2 R B1\nB1 _ B1\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ R B1\n_ _ B1\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_se_cruza_con_enemigo_1_vida_pasa_a_ser_derrotado_y_explota_todas_las_bombas(){
        let mut matriz = Laberinto::generador_matriz("B2 _ B1\nF1 _ B1\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\n_ _ _\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }

    #[test]
    fn test_matriz_bomba_explota_en_0_0_alcance_2_se_cruza_con_enemigo_3_vidas_pasa_a_tener_2_vidas_y_explota_todas_las_bombas(){
        let mut matriz = Laberinto::generador_matriz("B2 _ B1\nF3 _ B1\n_ _ _");
        let mut matriz_transformada = Laberinto::generador_matriz("_ _ _\nF2 _ _\n_ _ _");
        assert_eq!(matriz.atravesar_laberinto(0,0),&mut matriz_transformada);
    }
}


