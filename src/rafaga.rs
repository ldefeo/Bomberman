use crate::objetos::Objeto;

#[derive(Debug,PartialEq,Clone)]
pub struct Rafaga{
    pub objetos: Vec<(Objeto,usize,usize)>,
}

impl Rafaga{

    pub fn reiniciar_rafaga() -> Self{
        let objetos: Vec<(Objeto,usize,usize)> = Vec::new();
        Rafaga {objetos}
    }

    pub fn agregar(&mut self,objetos: Vec<(Objeto, usize, usize)>,objeto: &Objeto,coord_x:usize,coord_y:usize) -> Self{
        self.objetos.push((objeto.clone(),coord_x,coord_y));
        Rafaga {objetos}
    }

    pub fn rafaga(&self) -> Vec<(Objeto, usize, usize)>{
       self.objetos.clone()
    }

}