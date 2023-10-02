use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Drink;

pub fn to_thing<'a>(num: u32, drink: Drink) -> Thing<'a> {
    let mut result: Thing;
    match drink {
        Drink::Beer => result = Thing::new( Sex::Male, false, "verre de bière", "verres de bière"),
        Drink::Coffee => result = Thing::new( Sex::Female, false, "tasse de café", "tasses de café"),
        Drink::Milk => result = Thing::new( Sex::Male, false, "verre de lait", "verres de lait"),
        Drink::Tea => result = Thing::new( Sex::Female, false, "tasse de thé", "tasses de thé"),
        Drink::Water => result = Thing::new( Sex::Male, false, "verre d'eau", "verres d'eau"),
        Drink::Wine => result = Thing::new( Sex::Male, false, "verre de vin", "verres de vin"),
        Drink::Any => result = Thing::new( Sex::Male, false, "verre de boisson", "verres de boisson")
    }
    result.set_num(num);
    result 
}