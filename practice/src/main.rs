fn main() {
    //let age = 17;

   // match age{
       // 18 | 19 => println!("You're an adult"),
       // 10..=15 => println!("You are 10!"),
       // _ => println!("Invalid option")
    //}

    let account_balance: Option<i32> = None;

    match account_balance {
        Some(value) => println!("Value was retrieved!: {}", value),
        None => println!("Nothing was retrieved!")
    }


}
