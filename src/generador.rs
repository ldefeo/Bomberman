use std::str::Chars;


#[derive(Debug)]
pub struct Generador{
    matriz: Vec<Vec<String>>,
}

#[derive(Debug,PartialEq)]
pub enum Objeto{
    BombaNormal(BombaNormal),
    Roca(Roca),
    Pared(Pared),
    Vacio(Vacio),
    Enemigo(Enemigo),
    Desvio(Desvio),
    BombaTraspaso(BombaTraspaso),
}

#[derive(Debug,PartialEq,Clone)]
pub struct BombaNormal{
    identificador: String,
    alcance: usize,
}

#[derive(Debug,PartialEq)]
pub struct Vacio{
    identificador: String,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Enemigo{
    identificador: String,
    vidas: usize,
}

#[derive(Debug,PartialEq)]
pub struct Roca{
    identificador: String,
}

#[derive(Debug,PartialEq)]
pub struct Pared{
    identificador: String,
}

#[derive(Debug,PartialEq)]
pub struct Desvio{
    identificador: String,
    direccion: String,
}

#[derive(Debug,PartialEq,Clone)]
pub struct BombaTraspaso{
    identificador: String,
    alcance: usize,
}

impl Objeto{

    pub fn matcheo(elemento: String) -> Objeto{
        let mut str_elemento = elemento.chars();
        if elemento.len() == 2{
            match (str_elemento.next(),str_elemento.next()){
                (Some('B'),_) => {Objeto::BombaNormal(BombaNormal::generar(elemento))},
                (Some('F'),_) => {Objeto::Enemigo(Enemigo::generar(elemento))},
                (Some('D'),_) => {Objeto::Desvio(Desvio::generar(str_elemento))},
                (Some('S'),_) => {Objeto::BombaTraspaso(BombaTraspaso::generar(elemento))},
                _ => {Objeto::Vacio(Vacio::generar(elemento))},
            }
        }else{
            match elemento.as_str() {
                "R" => {Objeto::Roca(Roca::generar(elemento))},
                "W" => {Objeto::Pared(Pared::generar(elemento))},
                _ => {Objeto::Vacio(Vacio::generar(elemento))},
            }
        }
    }

}

impl BombaNormal{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            BombaNormal
     { identificador: ident, alcance: valor }
        }else{
            BombaNormal
     { identificador: todo!(), alcance: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.alcance
    }

}

impl BombaTraspaso{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            BombaTraspaso
     { identificador: ident, alcance: valor }
        }else{
            BombaTraspaso
     { identificador: todo!(), alcance: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.alcance
    }

}

impl Desvio{
    
    pub fn generar(mut elemento: Chars) -> Self{
        let primer_caracter = elemento.next();
        let segundo_caracter = elemento.next();
        match (primer_caracter, segundo_caracter) {
            (Some(id), Some(dir)) => Desvio { identificador: id.to_string(), direccion: dir.to_string() },
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

impl Vacio{
    
    pub fn generar(elemento: String) -> Self{
        Vacio { identificador:elemento }

    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        0
    }
}

impl Enemigo{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            Enemigo { identificador: ident, vidas: valor }
        }else{
            Enemigo { identificador: todo!(), vidas: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.vidas
    }

}

impl Roca{
    
    pub fn generar(elemento: String) -> Self{
        Roca { identificador:elemento }

    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        0
    }

}

impl Pared{
    
    pub fn generar(elemento: String) -> Self{
        Pared { identificador:elemento }

    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        0
    }
}


impl Generador {
    
    //pub fn generador_matriz(valor: &str) -> Vec<Vec<String>>{  //generador de matrices
    pub fn generador_matriz(valor: &str) -> Vec<Vec<Objeto>>{
        //let mut matriz: Vec<Vec<String>> = Vec::new();
        let mut matriz: Vec<Vec<Objeto>> = Vec::new();
        for f in valor.split("\n"){    // si el valor contiene \n significa fin de fila
            //let mut vector_intermedio: Vec<String> = Vec::new(); 
            let mut vector_intermedio: Vec<Objeto> = Vec::new();
            for c in f.split(" "){  // si el valor contiene espacio entre caracteres significa fin de columna
                vector_intermedio.push(Objeto::matcheo(c.to_string()));
                //vector_intermedio.push(c.to_string());
            }
            matriz.push(vector_intermedio);
        }
        matriz
    }

    pub fn dividir_string(valor: &String) -> Result<(String, usize), String>{
        let (identificador,numero): (String,String) = valor.chars().partition(|&c| c.is_alphabetic());
    
        let num:Result<usize, _> = numero.parse();
        match num{
            Ok(num) => {
                Ok((identificador,num))
            },
            Err(_) => {
                Err("No se pudo parsear".to_string())
            },
        }
    }
}