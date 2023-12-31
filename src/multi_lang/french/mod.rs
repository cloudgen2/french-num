use crate::read_line::read_line;
use crate::entities::Animal;
use crate::entities::Drink;
use crate::entities::Food;
use crate::entities::Fruit;
use crate::entities::Transport;
use crate::french::number::all_num;
use crate::french::animal;
use crate::french::drink;
use crate::french::food;
use crate::french::fruit;
use crate::french::transport;

use crate::french::Thing;
use crate::thisis::ThisIs;
use crate::french::print_what_is_it;
use crate::french::print_correct;
use crate::french::print_correct_ans;
use crate::french::process_ans;
use crate::french::print_level;
use crate::french::print_what_num;
use crate::icons::drink_icon;
use crate::icons::fruit_icon;
use crate::icons::food_icon;
use crate::icons::animal_icon;
use crate::icons::transport_icon;
use std::process;

pub fn print_level_sub(level: u32) {
    print_level(level)
}

fn ask_thing_sub(question: u32, quantity: u32, icon: &String, highlight: &str) -> String {
    print_what_is_it(question, quantity, icon, highlight);
    read_line()
}

pub fn ask_n_check_sub(q: u32, num: u32, icon: &String, check: String) {
    let mut receive_ans = ask_thing_sub(q, num, icon, "");
    if receive_ans.eq(&check) {
        print_correct();
    } else if receive_ans.eq("exit") {
        process::exit(0);
    } else if process_ans(&receive_ans).eq(&process_ans(&check)) { 
        print_correct_ans(&check, &receive_ans);
        print_correct();
    } else {
        print_correct_ans(&check, &receive_ans);
        receive_ans = ask_thing_sub(q, num, &icon, " ** ");
        if receive_ans.eq(&check) {
            print_correct();
        } else if process_ans(&receive_ans).eq(&process_ans(&check)) { 
            print_correct_ans(&check, &receive_ans);
            print_correct();
        } else if receive_ans.eq("exit") {
            process::exit(0);
        } else {
            print_correct_ans(&check, &receive_ans);
        }
    }
}

fn ask_number_sub(question: u32, quantity: u32, highlight: &str) -> String {
    print_what_num(question, quantity, highlight);
    read_line()
}

pub fn ask_num_n_check_sub(q: u32, num: u32, check: String) {
    let mut receive_ans = ask_number_sub(q, num, "");
    if receive_ans.eq(&check) {
        print_correct();
    } else if receive_ans.eq("exit") {
        process::exit(0);
    } else if process_ans(&receive_ans).eq(&process_ans(&check)) {

        print_correct_ans(&check, &receive_ans);
        print_correct();
    } else {
        print_correct_ans(&check, &receive_ans);
        receive_ans = ask_number_sub(q, num, " ** ");
        if receive_ans.eq(&check) {
            print_correct();
        } else if process_ans(&receive_ans).eq(&process_ans(&check)) { 
            print_correct_ans(&check, &receive_ans);
            print_correct();
        } else if receive_ans.eq("exit") {
            process::exit(0);
        } else {
            print_correct_ans(&check, &receive_ans);
        }
    }
}

pub fn ask_n_check_drink_sub(q: u32, num: u32, drink_val:Drink) {
    let drink: Thing = drink(num, drink_val);
    let check=drink.this_is();
    let icon: String = drink_icon(drink_val);
    ask_n_check_sub(q, num, &icon, check);
}

pub fn ask_n_check_fruit_sub(q: u32, num: u32, fruit_val:Fruit) {
    let icon: String = fruit_icon(fruit_val);
    let fruit: Thing = fruit(num, fruit_val);
    let check=fruit.this_is();
    ask_n_check_sub(q, num, &icon, check);
}

pub fn ask_n_check_food_sub(q: u32, num: u32, food_val:Food) {
    let icon: String = food_icon(food_val);
    let food: Thing = food(num, food_val);
    let check = food.this_is();
    ask_n_check_sub(q, num, &icon, check);
}

pub fn ask_n_check_animal_sub(q: u32, num: u32, animal_val:Animal) {
    let icon: String = animal_icon(animal_val);
    let animal: Thing = animal(num, animal_val);
    let check = animal.this_is();
    ask_n_check_sub(q, num, &icon, check);
}

pub fn ask_n_check_transport_sub(q: u32, num: u32, transport_val:Transport) {
    let icon: String = transport_icon(transport_val);
    let transport: Thing = transport(num, transport_val);
    let check = transport.this_is();
    ask_n_check_sub(q, num, &icon, check);
}

pub fn ask_n_check_num_sub(q: u32, num: u32) {
    ask_num_n_check_sub(q, num, all_num(num))
}
