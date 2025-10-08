fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let favorite_cities = &cities[0..2];
    println!("{favorite_cities:?}");
    let places = cities;
}