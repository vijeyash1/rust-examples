use Banknote::*;
use Country::*;

#[derive(Debug)]
enum Banknote {
    ChocolateBrown(Country),
    Brightyellow(Country),
    Stonegrey(Country),
    Magenta(Country),
}

#[derive(Debug)]
enum Country {
    India,
}
impl Banknote {
    fn get_state_value(&self) -> String {
        match self {
            Banknote::ChocolateBrown(country) => format!(
                "the value of bank note is 10rs and the country is {:?}",
                country
            ),
            Banknote::Brightyellow(country) => format!(
                "the value of bank note is 200rs and the country is {:?}",
                country
            ),
            Banknote::Stonegrey(country) => format!(
                "the value of bank note is 500rs and the country is {:?}",
                country
            ),
            Banknote::Magenta(country) => format!(
                "the value of bank note is 2000rs and the country is {:?}",
                country
            ),
        }
    }
}

fn main() {
    let twohun = Brightyellow(India);
    let ten = ChocolateBrown(India);
    let fivehun = Stonegrey(India);
    let twok = Magenta(India);
    println!("{}", twohun.get_state_value());
    println!("{}", ten.get_state_value());
    println!("{}", fivehun.get_state_value());
    println!("{}", twok.get_state_value());
}
