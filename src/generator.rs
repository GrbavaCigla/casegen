use crate::structures::Entity;

fn generate_case(case: &Vec<Entity>) -> String {
    let mut buff = String::new();

    for i in case {
        buff.push_str(match i {
            Entity::Variable(var) => unimplemented!(),
            Entity::Text(text) => text
        }); 
    }

    buff
}