use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Fruit;

pub fn to_thing<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    let mut result: Thing;
    match fruit {
        Fruit::Apple => result = Thing::new( Sex::Female, false, "pomme", "pommes"),
        Fruit::Orange => result = Thing::new( Sex::Female, true, "orange", "oranges"),
        Fruit::Banana => result = Thing::new( Sex::Female, false, "banane", "bananes"),
        Fruit::Strawberry => result = Thing::new( Sex::Female, false, "fraise", "fraises"),
        Fruit::Pear => result = Thing::new( Sex::Female, false, "poire", "poires" ),
        Fruit::WaterMelon => result = Thing::new( Sex::Female, false, "pastèque", "pastèques" ),
        Fruit::Cherry => result = Thing::new( Sex::Female, false, "cerise", "cerises" ),
        Fruit::Grape => result = Thing::new( Sex::Male, false, "raisin", "raisins" ),
        Fruit::Any => result = Thing::new( Sex::Male, false, "fruit", "fruits")
    }
    result.set_num(num);
    result 
}