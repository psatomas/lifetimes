#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'b str,
    to: &'a str,
}

fn main() {
    let from = String::from("Portland");    
    let plan = figure_out_ending_pint(&from);

    println!("{plan}")
}

fn figure_out_ending_pint(from: &str) -> &str {
    let to = String::from("Bangor");

    let travel_plan = TravelPlan {
        from: &from,
        to: &to,
    };

    travel_plan.from
}