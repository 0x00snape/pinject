mod modules;

fn main() {
    
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./pinject <PROCESS_NAME>");
        std::process::exit(0);
    }

    // Check process pid and performs process injection
    modules::getPID(args[1].clone());
}
