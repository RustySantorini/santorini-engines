fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Input the name of the engine you want to deploy! Usage: engine-deploy name");
        return;
    }

    let _engine_name = args.swap_remove(1);

    
}
