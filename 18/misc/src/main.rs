extern crate serde;

fn main() {
    {
        use std::io::Write;

        let mut buffer: Vec<u8> = Vec::new();

        writeln!(buffer, "format! {}", "implementation").unwrap();

        let string = String::from_utf8(buffer).unwrap();

        assert_eq!(string, "format! implementation\n");
    }
    {
        use std::io::BufRead;
        use std::io::Cursor;
        use std::io::Result;

        let input = "3.1415926535\n2.7182818285\n".to_string();
        let cursor = Cursor::new(&input);

        let lines: Result<Vec<String>> = cursor.lines().collect();

        match lines {
            Ok(lines) => for line in lines {
                println!("{}", line)  
            },
            Err(err) => println!("Error: {}", err),
        }
    }
    {
        use std::collections::HashMap;
        use std::io;
        use serde::Serialize;
        use serde_json::Serializer;

        let mut map = HashMap::new();
        map.insert("Cobble Crawl".to_string(),
            vec![('W', "Debris Room".to_string())]);

        map.insert("Debris Room".to_string(),
            vec![('E', "Cobble Crawl".to_string()),
            ('W', "Sloping Canyon".to_string())]);

        let mut serializer = Serializer::new(io::stdout());
        map.serialize(&mut serializer).unwrap();
        println!();
    }
    {
        use serde::Serialize;

        #[derive(Serialize)]
        struct Player {
            location: String,
            items: Vec<String>,
            health: u32
        }

        let players = vec![
            Player { location: "West".to_string(), items: vec!["foo".to_string()], health: 20 },
            Player { location: "North".to_string(), items: vec!["bar".to_string(), "baz".to_string()], health: 43 },
        ];

        let serialized = serde_json::to_string(&players).unwrap();

        println!("{}", serialized);
    }
    {
        use std::path::Path;
        use std::ffi::OsStr;

        assert_eq!(Path::new("/home/fwolfe/program.txt").parent(), Some(Path::new("/home/fwolfe")));
        assert_eq!(Path::new("/home/fwolfe/program.txt").file_name(), Some(OsStr::new("program.txt")));

        let path1 = Path::new("/usr/share/dict");
        assert_eq!(path1.join("words"), Path::new("/usr/share/dict/words"));
    }
    {
        use std::path::Path;
        let path = Path::new(r"C:\");
        for entry_result in path.read_dir().unwrap() {
            let entry = entry_result.unwrap();
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
}
