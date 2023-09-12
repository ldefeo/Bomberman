
#[derive(Debug)]
pub struct Generador{
    matriz: Vec<Vec<String>>,
}

#[derive(Debug,PartialEq)]
pub enum Objeto{
    bomba(Bomba),
    roca(Roca),
    pared(Pared),
    vacio(Vacio),
    enemigo(Enemigo),
}

#[derive(Debug,PartialEq,Clone)]
pub struct Bomba{
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

impl Objeto{

    pub fn matcheo(elemento: String) -> Objeto{
        let mut str_elemento = elemento.chars();
        if elemento.len() == 2{
            match (str_elemento.next(),str_elemento.next()){
                (Some('B'),_) => {Objeto::bomba(Bomba::generar(elemento))},
                (Some('F'),_) => {Objeto::enemigo(Enemigo::generar(elemento))},
                _ => {Objeto::vacio(Vacio::generar(elemento))},
            }
        }else{
            match elemento.as_str() {
                "R" => {Objeto::roca(Roca::generar(elemento))},
                "W" => {Objeto::pared(Pared::generar(elemento))},
                _ => {Objeto::vacio(Vacio::generar(elemento))},
            }
        }
    }

}

impl Bomba{
    
    pub fn generar(elemento: String) -> Self{
        let resultado = Generador::dividir_string(&elemento);
        if let Ok((ident,valor)) = resultado {
            Bomba { identificador: ident, alcance: valor }
        }else{
            Bomba { identificador: todo!(), alcance: 0 }
        }
    }

    pub fn identificador(self) -> String{
        self.identificador
    }

    pub fn alcance(self) -> usize{
        self.alcance
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