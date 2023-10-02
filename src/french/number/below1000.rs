mod below100;
use below100::below100;

pub fn below1000(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number < 100 {
        result.push_str(&below100(number));
    } else if number == 100 {
        result.push_str("cent");
    } else if number < 200 {
        result.push_str("cent-");
        diff = number - 100;
        result.push_str(&below100(diff));
    } else if number == 200 {
        result.push_str("deux-cents");
    } else if number < 300 {
        result.push_str("deux-cent-");
        diff = number - 200;
        result.push_str(&below100(diff));
    } else if number == 300 {
        result.push_str("trois-cents");
    } else if number < 400 {
        result.push_str("trois-cent-");
        diff = number - 300;
        result.push_str(&below100(diff));
    } else if number == 400 {
        result.push_str("quatre-cents");
    } else if number < 500 {
        result.push_str("quatre-cent-");
        diff = number - 400;
        result.push_str(&below100(diff));
    } else if number == 500 {
        result.push_str("cinq-cents");
    } else if number < 600 {
        result.push_str("cinq-cent-");
        diff = number - 500;
        result.push_str(&below100(diff));
    } else if number == 600 {
        result.push_str("six-cents");
    } else if number < 700 {
        result.push_str("six-cent-");
        diff = number - 600;
        result.push_str(&below100(diff));
    } else if number == 700 {
        result.push_str("sept-cents");
    } else if number < 800 {
        result.push_str("sept-cent-");
        diff = number - 700;
        result.push_str(&below100(diff));
    } else if number == 800 {
        result.push_str("huit-cents");
    } else if number < 900 {
        result.push_str("huit-cent-");
        diff = number - 800;
        result.push_str(&below100(diff));
    } else if number == 900 {
        result.push_str("neuf-cents");
    } else if number < 1000 {
        result.push_str("neuf-cent-");
        diff = number - 900;
        result.push_str(&below100(diff));
    }
    result
}
