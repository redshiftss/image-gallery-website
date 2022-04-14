use tera::Tera;
use 
// Use globbing
let tera = match Tera::new("templates/**/*.html") {
    Ok(t) => t,
    Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
    }
};

