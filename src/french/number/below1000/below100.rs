pub fn below100(number: u32) -> String {
    let numbers = vec!["zéro","un","deux","trois","quatre","cinq","six","sept","huit","neuf","dix","onze","douze","treize","quatorze","quinze","seize","dix-sept","dix-huit","dix-neuf","vingt"];
    let mut result: String;
    let diff: u32;
    if number < 21 {
        result=String::from(numbers[number as usize]);
    } else if number < 30 {
        result=String::from(numbers[20 as usize]);
        diff = number - 20;
        if diff == 1 {
            result.push_str("-et-");
        } else {
            result.push_str("-");
        }
        result.push_str(numbers[diff as usize]);
    } else if number < 40 {
        result=String::from("trente");
        diff = number - 30;
        if diff > 0 {
            if diff == 1 {
                result.push_str("-et-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 50 {
        result=String::from("quarante");
        diff = number - 40;
        if diff > 0 {
            if diff == 1 {
                result.push_str("-et-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 60 {
        result=String::from("cinquante");
        diff = number - 50;
        if diff > 0 {
            if diff == 1 {
                result.push_str("-et-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 80 {
        result=String::from("soixante");
        diff = number - 60;
        if diff > 0 {
            if diff == 1 || diff == 11 {
                result.push_str("-et-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number == 80 {
        result=String::from("quatre-vingts");
    } else if number < 100 {
        result=String::from("quatre-vingt-");
        diff = number - 80;
        result.push_str(numbers[diff as usize]);
    } else {
        result = String::new();
    }
    result
}