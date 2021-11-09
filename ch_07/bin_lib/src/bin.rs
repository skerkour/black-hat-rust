use binlib::exploit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: exploit <target>");
        return Ok(());
    }

    exploit(&args[1])?;

    Ok(())
}
