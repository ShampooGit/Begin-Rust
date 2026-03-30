// --- THE TOOLBOX ---
// 1. serde & serde_json -> Translates JSON files into Rust Structs.
// 2. clap              -> Makes it easy to build a CLI (Command Line Interface).
// 3. pdf-extract       -> Pulls metadata (Title, Author) from PDF files.
// 4. dirs              -> Finds standard folders (Downloads, AppData) on any OS.


use rusqlite::{Connection, Result};

struct MC {
    name: String,
    age: u32,
    main_colors: [&'static str; 3],
    alive: bool,

}

pub trait MCdb {
    fn save_to_db(&self, connection: &Connection) -> Result<()>;
}

impl MCdb for MC {
    fn save_to_db(&self, connection: &Connection) -> Result<()> {
        connection.execute(
            "INSERT INTO characters (name, age, alive, main_colors) VALUES (?1, ?2, ?3, ?4)",
            (&self.name, &self.age, &self.alive, &self.main_colors.join(", "))
        )?;
        Ok(())
    }
}

trait MCinfo {
    fn print_info(&self);
}


impl MCinfo for MC {
    fn print_info(&self) {
        println!("name = {}", self.name);
        println!("age = {}", self.age);
        println!("alive? = {}", self.alive);
        println!("main colors =");
        for color in self.main_colors {
            println!("  + {}", color)
        };
    }
}

fn main() {
    let connection = connect_db() .expect("No connection");

    let mcchacters = get_data_db(&connection) .expect("Can not read data");

    println!("Data form db");
    for row in mcchacters{
        row.print_info();
        println!("==================");
    }
}

fn connect_db() -> Result<Connection> {
    let connection: Connection = Connection::open("MC_chacters.db")?;
    
    connection.execute(
        "CREATE TABLE IF NOT EXISTS characters (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            age         INTEGER,
            alive       BOOLEAN,
            main_colors TEXT
        )",
        (),
    )?;

    Ok(connection)
}

fn get_data_db(connection: &Connection) -> Result<Vec<MC>> {
    let mut stmt = connection.prepare("SELECT name, age, alive, main_colors FROM characters")?;

    let mc_iter = stmt.query_map([], |row| {
        Ok(MC {
            name: row.get(0)?,
            age: row.get(1)?,
            alive: row.get(2)?,
            main_colors: ["White", "Gold", "Blue gray"],
        })
    })?;
    let mut results = Vec::new();
    for mc in mc_iter {
        results.push(mc?);
    }
    Ok(results)
}


// mocking data
/* 
fn sim_metadata() -> String {
    r#"
        "name": "Frieren",
        "age": "1000",
        "alive": "true",
        "colors": "White, Gold, Blue gray"
    "# .to_string()

}
*/
/*
let frieren: MC = MC { 
        name: String::from("Frieren"),
        age: 1000,
        main_colors: [ "White", "Blue gray", "Gold", ],
        alive: true,
    };
    let harley_quinn: MC = MC {
        name: String::from("Harley Quinn"),
        age: 37,
        alive: true,
        main_colors: ["white", "black", "red"],
    };
    let hatsune_miku: MC = MC {
        name: String::from("Hatsune Miku"),
        age: 18,
        alive: true,
        main_colors: ["Terqouse", "Black", "Red"],
    };
*/
