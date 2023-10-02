mod below1000;
use below1000::below1000;

pub fn all_num(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number<1000 {
        result.push_str(&below1000( number));
    } else if number == 1000 {
        result.push_str("mille");
    } else if number < 2000 {
        result.push_str("mille-");
        diff = number - 1000;
        result.push_str(&below1000( diff));
    } else if number == 2000 {
        result.push_str("deux-mille");
    } else if number < 3000 {
        result.push_str("deux-mille-");
        diff = number - 2000;
        result.push_str(&below1000( diff));
    } else if number == 3000 {  
        result.push_str("trois-mille");
    } else if number < 4000 {
        result.push_str("trois-mille-");
        diff = number - 3000;
        result.push_str(&below1000( diff));
    } else if number == 4000 {
        result.push_str("quatre-mille");
    } else if number < 5000 {
        result.push_str("quatre-mille-");
        diff = number - 4000;
        result.push_str(&below1000( diff));
    } else if number == 5000 {
        result.push_str("cinq-mille");
    } else if number < 6000 {
        result.push_str("cinq-mille-");
        diff = number - 5000; 
        result.push_str(&below1000( diff));
    } else if number == 6000 {
        result.push_str("six-mille");
    } else if number < 7000 {
        result.push_str("six-mille-");
        diff = number - 6000;
        result.push_str(&below1000( diff));
    } else if number == 7000 {
        result.push_str("sept-mille");
    } else if number < 8000 {
        result.push_str("sept-mille-");
        diff = number - 7000;
        result.push_str(&below1000( diff));
    } else if number == 8000 {
        result.push_str("huit-mille");
    } else if number < 9000 {
        result.push_str("huit-mille-");
        diff = number - 8000;
        result.push_str(&below1000(diff));
    } else if number == 9000 {
        result.push_str("neuf-mille");
    } else if number < 10000 {
        result.push_str("neuf-mille-");
        diff = number - 9000;
        result.push_str(&below1000( diff ));
    }
    result
}