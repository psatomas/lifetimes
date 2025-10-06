fn main() {
    let a = 1;

    {
        let b = 2;
    }

    let c = String::from("Winter");

    drop(c);
    
}