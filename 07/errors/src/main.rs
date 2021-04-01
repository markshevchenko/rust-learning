// use std::error::Error;
// use std::io::{Write, stderr};

fn main() {
    {
        fn pirate_share(total: u64, crew_size: usize) -> u64 {
            let half = total / 2;
            half / crew_size as u64
        }

        println!("{}", pirate_share(1000, 3));
        // println!("{}", pirate_share(1000, 0));
    }

    {
        fn get_wether(city: &str) -> Result<i32, String> {
            if city == "Moscow" {
                Ok(23)
            } else {
                Err("Unknown city".to_string())
            }
        }

        println!("Moscow");
        match get_wether("Moscow") {
            Ok(temperature) => println!("Temperature: {}", temperature),
            Err(message) => println!("Error: {}", message),
        }

        println!("Kazan");
        match get_wether("Kazan") {
            Ok(temperature) => println!("Temperature: {}", temperature),
            Err(message) => println!("Error: {}", message),
        }
    }

    {
        // fn print_error(mut err: &Error) {
        //     let _ = writeln!(stderr(), "error: {}", err);
        //     while let Some(source) = err.source() {
        //         let _ = writeln!(stderr(), "  caused by: {}", source);
        //     }
        // }
    }
}

