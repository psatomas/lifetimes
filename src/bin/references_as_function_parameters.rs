fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}

fn main() {
    let cities =vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = select_first_two_elements(&cities);
    println!("{two_cities:?}");

    {
        let coffees = [String::from("latte"),String::from("mocha"),];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}