#[cfg(test)]
mod tests {
    use bomberman::laberinto::Laberinto;

    #[test]
    fn test_laberinto_bomba_explota_en_0_0() {
        let mut laberinto = Laberinto::generar_laberinto("B1 _\n_ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _\n_ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_no_encuentra_bomba_en_0_0_tira_error() {
        let mut laberinto = Laberinto::generar_laberinto("F2 _\n_ _");
        assert!(laberinto.atravesar_laberinto(0, 0).is_err());
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_alcance_1_explota_bomba_consecutiva_alcance_1_explota() {
        let mut laberinto = Laberinto::generar_laberinto("B1 B1 _\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_alcance_1_no_explota_bomba_en_2_0() {
        let mut laberinto = Laberinto::generar_laberinto("B1 _ B1\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ B1\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_alcance_2_explota_bomba_en_0_1_alcance_1_explota() {
        let mut laberinto = Laberinto::generar_laberinto("B2 _ _\nB1 _ _\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _\n_ _ _\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_alcance_2_explota_bombas_sucesivas_explotan() {
        let mut laberinto = Laberinto::generar_laberinto("B2 B1 _\nB1 _ _\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _\n_ _ _\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_1_alcance_1_explota_bomba_en_0_0_alcance_2_explota() {
        let mut laberinto = Laberinto::generar_laberinto("B2 _ _\nB1 _ B1\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _\n_ _ B1\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_0_alcance_1_explota_bomba_en_0_0_alcance_2_explota_bomba_en_0_1_alcance_1_explota(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("B2 B1 _\nB1 _ B1\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _\n_ _ B1\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_se_traba_por_roca() {
        let mut laberinto = Laberinto::generar_laberinto("B1 _\nR _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _\nR _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_alcance_2_se_traba_por_roca_no_explota_bomba_en_2_0() {
        let mut laberinto = Laberinto::generar_laberinto("B2 R B1\nB1 _ B1\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ R B1\n_ _ B1\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_0_alcance_4_se_cruza_con_enemigo_1_vida_pasa_a_ser_derrotado_y_explota_todas_las_bombas(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("B4 _ B1\nF1 _ B1\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _\n_ _ _\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_4_golpea_a_enemigo_con_3_vidas_y_se_desvia_y_pega_en_bomba_alcance_1(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("_ F3 _ _\nW B3 _ DD\n_ R _ B1\n_ _ R F1");
        let mut laberinto_transformado =
            Laberinto::generar_laberinto("_ F2 _ _\nW _ _ DD\n_ R _ _\n_ _ R _");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_4_golpea_a_enemigo_con_3_vidas_y_se_desvia_arriba_y_pega_en_enemigo_matandolo(
    ) {
        let mut laberinto =
            Laberinto::generar_laberinto("_ F3 _ F1\nW B4 _ DU\n_ R _ B1\n_ _ R F1");
        let mut laberinto_transformado =
            Laberinto::generar_laberinto("_ F2 _ _\nW _ _ DU\n_ R _ B1\n_ _ R F1");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_2_1_alcance_3_se_desvia_abajo_y_se_desvia_izquierda_y_pega_en_enemigo_matandolo(
    ) {
        let mut laberinto = Laberinto::generar_laberinto(
            "_ _ F3 _ _\n_ DD B3 _ _\nF1 DL _ _ _\n_ _ _ _ _\n_ _ _ _ _",
        );
        let mut laberinto_transformado = Laberinto::generar_laberinto(
            "_ _ F2 _ _\n_ DD _ _ _\n_ DL _ _ _\n_ _ _ _ _\n_ _ _ _ _",
        );
        assert_eq!(
            laberinto.atravesar_laberinto(2, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_4_se_desvia_abajo_y_luego_derecha_y_pega_en_enemigo_matandolo(
    ) {
        let mut laberinto = Laberinto::generar_laberinto(
            "_ _ _ _ _\nW B4 _ DD _\n_ _ _ DR F1\n_ _ _ _ _\n_ _ _ _ _",
        );
        let mut laberinto_transformado =
            Laberinto::generar_laberinto("_ _ _ _ _\nW _ _ DD _\n_ _ _ DR _\n_ _ _ _ _\n_ _ _ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_0_1_alcance_5_explota_desviandose_arriba_y_luego_derecha_quita_vida_enemigo_3_desvia_izquierda_y_no_vuelve_a_descontarle_a_enemigo(
    ) {
        let mut laberinto = Laberinto::generar_laberinto(
            "_ DR F3 DL F1\nB5 DU _ _ _\n_ _ _ _ _\n_ _ _ _ _\n_ _ _ _ _",
        );
        let mut laberinto_transformado = Laberinto::generar_laberinto(
            "_ DR F2 DL F1\n_ DU _ _ _\n_ _ _ _ _\n_ _ _ _ _\n_ _ _ _ _",
        );
        assert_eq!(
            laberinto.atravesar_laberinto(0, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_0_alcance_5_explota_quita_vida_enemigo_luego_se_desvia_en_otra_rafaga_entonces_mata_a_enemigo(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("DD B5 F2\nDR _ DU\n_ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("DD _ _\nDR _ DU\n_ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_5_explota_desviandose_3_veces_explota_bomba_2_0_matando_a_enemigo(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("_ _ B1\nDD B5 F2\nDR _ DU");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ B1\nDD _ _\nDR _ DU");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_8_desviandose_derecha_e_izquierda_pero_no_mata_al_enemigo_por_las_posiciones_de_los_desvios(
    ) {
        let mut laberinto = Laberinto::generar_laberinto(
            "_ _ _ _ _\nDR B8 DL _ F2\n_ _ _ _ _\n_ _ _ _ _\n_ _ _ _ _",
        );
        let mut laberinto_transformado = Laberinto::generar_laberinto(
            "_ _ _ _ _\nDR _ DL _ F2\n_ _ _ _ _\n_ _ _ _ _\n_ _ _ _ _",
        );
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_2_muere_en_el_desvio() {
        let mut laberinto = Laberinto::generar_laberinto("_ F3 _ _\nW B2 _ DD\n_ R _ B1\n_ _ R F1");
        let mut laberinto_transformado =
            Laberinto::generar_laberinto("_ F2 _ _\nW _ _ DD\n_ R _ B1\n_ _ R F1");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_traspaso_explota_en_0_1_alcance_3_explota_bomba_normal_alcance_1_explota(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("B1 R F1\nS2 R B2\n_ _ R");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ R _\n_ R _\n_ _ R");
        assert_eq!(
            laberinto.atravesar_laberinto(0, 1),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_en_2_0_explota_pero_no_mata_a_enemigo_porque_muere_en_los_desvios_cruzados(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("_ _ B7 _ _ _ _\n_ W _ W _ W _\n_ _ DR DL F1 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        let mut laberinto_transformado = Laberinto::generar_laberinto("_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ DR DL F1 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
        assert_eq!(
            laberinto.atravesar_laberinto(2, 0),
            Ok(&mut laberinto_transformado)
        );
    }

    #[test]
    fn test_laberinto_bomba_explota_en_1_1_alcance_8_no_mata_a_enemigo_f1_porque_muere_en_los_desvios_cruzados(
    ) {
        let mut laberinto = Laberinto::generar_laberinto("_ F3 _ _\nW B8 _ DD\n_ R _ DU\n_ _ R F1");
        let mut laberinto_transformado =
            Laberinto::generar_laberinto("_ F2 _ _\nW _ _ DD\n_ R _ DU\n_ _ R F1");
        assert_eq!(
            laberinto.atravesar_laberinto(1, 1),
            Ok(&mut laberinto_transformado)
        );
    }
}
