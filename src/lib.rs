pub mod funcionario;
use funcionario::Funcionario;

trait Pessoa{
    fn cadastrar(&self);
    fn excluir(&self) -> bool;
}

enum Estado{
    Todo,
    EmProgresso,
    Concluida,
}

struct Tarefa<'a>{
    estado: Estado,
    funcs: Vec<&'a Funcionario>,
    data_inic: String,
    data_fim: Option<String>
}

impl<'a> Tarefa<'a>{
    fn create()->Self{
        Self { estado: Estado::Todo, funcs: Vec::new(), data_inic: String::new(), data_fim: None }
    }

    fn atribuir_func(&mut self, funcionario: &'a Funcionario){
       self.funcs.push(funcionario);
    }
}
