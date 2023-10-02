use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Transport;

pub fn to_thing<'a>(num: u32, transport: Transport) -> Thing<'a> {
    let mut result: Thing;
    match transport {
        Transport::Bus => result = Thing::new(Sex::Male, false, "bus", "bus"),
        Transport::Car => result = Thing::new(Sex::Female, false, "voiture", "voitures"),
        Transport::Any => result = Thing::new(Sex::Male, false, "moyen de transport", "transports")
    }
    result.set_num(num);
    result 
}