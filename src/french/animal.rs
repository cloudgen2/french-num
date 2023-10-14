use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Animal;

pub fn to_thing<'a>(num: u32, animal: Animal) -> Thing<'a> {
    let mut result: Thing;
    match animal {
        Animal::Bird => result = Thing::new( Sex::Male, true, "oiseau", "oiseaux"),
        Animal::Cat => result = Thing::new( Sex::Male, false, "chat", "chats"),
        Animal::Dog => result = Thing::new( Sex::Male, false, "chien", "chiens"),
        Animal::Fish => result = Thing::new( Sex::Male, false, "poisson", "poissons" ),
        Animal::Horse => result = Thing::new( Sex::Male, false, "cheval", "chevaux"),
        Animal::Rabbit => result = Thing::new( Sex::Male, false, "lapin", "lapins"),
        Animal::Pig => result = Thing::new( Sex::Male, false, "cochon", "cochons"),
        Animal::Any => result = Thing::new( Sex::Male, true, "animal", "animaux")
    }
    result.set_num(num);
    result 
}
