use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("Cameron".to_string(),
            vec!["Terminator".to_string(),
                 "Aliens".to_string()]);
    table.insert("Stone".to_string(),
            vec!["Doors".to_string(),
                 "JFK".to_string(),
                 "Platoon".to_string()]);
    table.insert("Scorsese".to_string(),
            vec!["Casino".to_string(),
                 "The Departed".to_string(),
                 "The Walf of Wall Street".to_string()]);

    show(&table);
    sort_works(&mut table);

    assert_eq!("Aliens", table["Cameron"][0]);
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works of {}", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for(_, works) in table {
        works.sort();
    }
}