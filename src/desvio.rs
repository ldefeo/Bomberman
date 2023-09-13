
#[derive(Debug,PartialEq,Clone)]
pub struct Desvio{
    identificador: String,
    direccion: String,
}

impl Desvio{
    
    pub fn generar(elemento: String) -> Self{
        let mut elemento_modificado= elemento.chars();
        match (elemento_modificado.next(), elemento_modificado.next()) {
            (Some(id),Some(dir)) => Desvio { identificador: id.to_string(), direccion: dir.to_string() },
            _ => Desvio { identificador: "".to_string(), direccion: "".to_string() },
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> String{
        self.direccion
    }

}