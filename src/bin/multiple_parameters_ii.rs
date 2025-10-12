fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("The second is {second}");
    first
}

fn main() {
    let orlando = String::from("Orlando");
    let result = {
        let san_francisco = String::from("San Francisco");
        longest(&orlando, &san_francisco)
    };
    println!("{result}");
}