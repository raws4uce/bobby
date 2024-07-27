use std::io;    
use std::collections::HashMap;

struct ShaolinCapital {
    clients: HashMap<u32, (String, f64)>, 
    next_id: u32,
}
impl ShaolinCapital {
    fn new() -> Self {
        ShaolinCapital {
            clients: HashMap::new(),
            next_id: 0,
        }
    }
}

fn main() {
    let mut Shaolin = ShaolinCapital::new();
    println!("WELCOME TO THE RUGGED SLUMS OF SHAOLIN, WHERE WU TANG CLAN STRIKES AGAIN,");
    println!("DO YOU KNOW YOUR AT RISK OF GOING BUST, THATS WHY YOU GOTTA UNIFY YOUR BONDS");
    println!("          ShaolinCapital : ft. RZA, Ghostface Killah ");
    loop{
        for (k, v) in Shaolin.clients.clone().into_iter() {
            println!("{:#?} , {:#?}", k, v);
        }
        println!();
        println!("AY LISTEN UP, if you ever need to exit, type in 'exit', if thats yo name, bad luck");
        println!("1, add client");
        println!("2, update balance");
        println!("3, search ID");
        println!("4, list em all");
        println!("pick a numba $Wuuu ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error wif somtheing");
        let input = input.trim();
        match input{
            "1" => add_client(&mut Shaolin),
            "2" => update_balance(&mut Shaolin),
            "3" => search_ID(&mut Shaolin),
            // "4" => list_all(&mut Shaolin),
            "exit" => break,
            _ => {
                println!("AYO, THE WU IS BACK");
                continue;
            }
        }
    }   
}

fn search_ID(Shaolin : &mut ShaolinCapital) {
    let mut id = String::new();


    println("{id}, {Shaolin.clients.get(&id).cloned()}");
    
}

fn update_balance(Shaolin : &mut ShaolinCapital){
    let mut id_input = String::new();
    println!("Enter client ID to update balance: ");
    io::stdin().read_line(&mut id_input).expect("Failed to read input");
    let id: u32 = match id_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error in parsing ID. Use the wisdom from the clan to enter a number");
            return;
        }
    };


    if let Some((name, balance)) = Shaolin.clients.get_mut(&id) {
        let mut action_input = String::new();
        println!("Enter '1' to withdraw or '2' to deposit: ");
        io::stdin().read_line(&mut action_input).expect("Failed to read input");

        let action = match action_input.trim() {
            "1" => -1.0, // Withdraw
            "2" => 1.0,  // Deposit
            _ => {
                println!("Invalid choice. Enter '1' for withdrawal or '2' for deposit.");
                return;
            }
        };

        let mut amount_input = String::new();
        println!("Enter amount: ");
        io::stdin().read_line(&mut amount_input).expect("Failed to read input");
        let amount: f64 = match amount_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error in parsing amount. Please enter a valid number.");
                return;
            }
        };
        if action == -1.0 && *balance < amount {
            println!("Not enough funds for withdrawal.");
        } else {
            *balance += action * amount;
            println!("Updated balance for {}: {:.2}", name, balance);
        }
    } else {
        println!("Client ID not found.");
    }
}


fn add_client(Shaolin: &mut ShaolinCapital) {
    println!("so you want someone to join the Shaolin Temple...");
    println!("wise decision....");
    let mut name = String::new();
    println!("who is this chosen one : ");
    io::stdin()
        .read_line(&mut name)
        .expect("this is badddd");
    let name = name.trim();

    let mut balance = String::new();
    println!("Shaolin needs their Captial : ");
    io::stdin()
        .read_line(&mut balance)
        .expect("this is badddd");
    let balance_int: f64 = match balance.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for balance. Please enter a valid number.");
            return; // Exit the function or handle as needed
        }
    };
    Shaolin.clients.entry(Shaolin.next_id +1).or_insert((name.to_owned(), balance_int));
    println!("'{name}' has been added successfully, with the balance of {}", balance_int);
    Shaolin.next_id += 1;
}
