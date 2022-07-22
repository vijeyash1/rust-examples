#[derive(Debug)]
enum Banknote {
    ChocolateBrown,
    Brightyellow,
    Stonegrey,
    Magenta,
}

impl Banknote {
    fn get_value(&self) -> &str {
        match self {
            Banknote::ChocolateBrown => "10rs",
            Banknote::Brightyellow => "200rs",
            Banknote::Stonegrey => "500rs",
            Banknote::Magenta => "2000rs",
        }
    }
}

fn main() {
    let twohun = Banknote::Brightyellow;
    let ten = Banknote::ChocolateBrown;
    let fivehun = Banknote::Stonegrey;
    let twok = Banknote::Magenta;
    println!(
        "Bank note value is : {} and the color is {:?}",
        twohun.get_value(),
        twohun
    );
    println!(
        "Bank note value is : {} and the color is {:?}",
        ten.get_value(),
        ten
    );
    println!(
        "Bank note value is : {} and the color is {:?}",
        fivehun.get_value(),
        fivehun
    );
    println!(
        "Bank note value is : {} and the color is {:?}",
        twok.get_value(),
        twok
    );
}
