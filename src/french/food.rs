use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Food;

pub fn to_thing<'a>(num: u32, food: Food) -> Thing<'a> {
    let mut result: Thing;
    match food {
        Food::Bread => result = Thing::new( Sex::Male, false, "morceau de pain", "morceaux de pain"),
        Food::Croissant => result = Thing::new( Sex::Male, false, "croissant", "croissants"),
        Food::Cake => result = Thing::new( Sex::Male, false, "morceau de gâteau", "morceaux de gâteau"),
        Food::Pizza => result = Thing::new( Sex::Male, false, "morceau de pizza", "morceaux de pizza"),
        Food::Rice => result = Thing::new( Sex::Male, false, "bol de riz", "bols de riz"),
        Food::Soup => result = Thing::new( Sex::Male, false, "plat de soupe", "plats de soupe"),
        Food::Any => result = Thing::new( Sex::Male,false, "aliment","aliments")
    }
    result.set_num(num);
    result 
}