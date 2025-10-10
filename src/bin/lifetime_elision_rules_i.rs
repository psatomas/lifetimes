fn my_awesome_function(first: &i32, second: String) -> &i32 {
    first
}

fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[0..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = {
        let cities_ref = &cities;
        select_first_two_elements(cities_ref)
    };
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}