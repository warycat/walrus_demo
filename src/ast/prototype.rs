use super::*;

#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub params: Vec<String>,
}

impl Prototype {
    pub fn new(name: String, params: Vec<String>) -> Self {
        Prototype { name, params }
    }
}

impl Compile for Prototype {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        for param in &self.params {
            let id = local_ids[param];
            // builder.local_set(id);
        }
    }
}
