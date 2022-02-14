use crate::structures::Entity;
use rand::prelude::*;

pub fn generate_case<'a >(case: &Vec<Entity>) -> String {
    let mut buff = String::new();

    let mut rng = rand::thread_rng();

    for i in case {
        // TODO: Making string from string slice and then pushing it is stupid
        buff.push_str(&match i {
            Entity::Variable(var) => rng.gen_range(var.lower..var.upper).to_string(),
            Entity::Text(text) => text.to_string(),
        });
    }

    buff
}
