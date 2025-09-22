use crate::Pessoa;

pub struct Funcionario{
    cargo: Cargo,
    nome: String,
    sal: f64,
    data_admissao: String,
}

impl Funcionario{
    fn build(cargo: Cargo, nome: String, sal: f64, data_admissao: String) -> Result<Self, &'static str>{
        if sal < 0.0 {
            Err("Salário inválido")
        } else{
            Ok(Self { cargo, nome, sal, data_admissao })
        }       
    }
}

impl Pessoa for Funcionario{
    fn cadastrar(&self) {
        unimplemented!()
    }
    fn excluir(&self) -> bool {
        unimplemented!()
    }
}

pub enum Cargo{
    Gerente(f64),
    Estagio(f64),
    Testes(f64),
    Desenvolvimento(f64),
    Suporte(f64),
}