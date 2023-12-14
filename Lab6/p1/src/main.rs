use std::fs;

use rusqlite::Connection;

use anyhow::anyhow;
use anyhow::Result;
trait CommandTrait {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) -> Result<()>;
}

struct PingCommand {}
struct CountCommand {}
struct TimesCommand {
    count: i32,
}

struct IncCommand {}

struct BmCommand {}

struct Bm {
    name: String,
    url: String,
}

impl CommandTrait for BmCommand {
    fn get_name(&self) -> &str {
        "bm"
    }
    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.len() < 2 || args.len() > 3 {
            return Err(anyhow!("Wrong number of arguments!"));
        }
        let conn = Connection::open("BMDataBase.db")?;
        let create = r"
    create table if not exists BMDataBase (
        name TEXT    not null,
        url  TEXT    not null
    );
    ";
        conn.execute(create, ())?;

        if args[0].to_string().to_lowercase() == "add" {
            match conn.execute(
                "insert into BMDataBase (name, url) values (?1, ?2);",
                (args[1], args[2]),
            ) {
                Ok(_) => println!("Entry added succsesfully!"),
                Err(e) => println!("{:?}", e),
            }
        } else if args[0].to_string().to_lowercase() == "search" {
            let mut stmt = conn.prepare("select * from BMDataBase")?;
            let bm_iter = stmt.query_map([], |row| {
                Ok(Bm {
                    name: row.get("name")?,
                    url: row.get("url")?,
                })
            })?;
            let mut found = false;
            for i in bm_iter {
                let i = i?;
                if i.name.contains(args[1]) {
                    found = true;
                    println!("{:?} {:?}", i.name, i.url);
                }
            }
            if found == false {
                println!("No entries contain the string {}", args[1]);
            }
        } else {
            return Err(anyhow!("Wrong BM command!"));
        }

        Ok(())
    }
}

impl CommandTrait for IncCommand {
    fn get_name(&self) -> &str {
        "increment"
    }
    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.len() != 1 {
            return Err(anyhow!("Expected exactly 1 argument!"));
        }
        if args[0].len() > 9 {
            return Err(anyhow!("Overflow!"));
        } else {
            let mut numeric = true;
            let mut val: i32 = 0;
            for i in args[0].chars() {
                if !i.is_numeric() {
                    numeric = false;
                    break;
                }
                val = val * 10 + (i as u8 - '0' as u8) as i32;
            }
            if numeric == false {
                return Err(anyhow!("The input is not a positive integer!"));
            } else {
                val += 1;
                println!("The incremented value is {}", val);
            }
        }
        Ok(())
    }
}

impl CommandTrait for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        println!("Pong!");
        Ok(())
    }
}

impl CommandTrait for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        println!("Counted {} args", args.len());
        Ok(())
    }
}

impl CommandTrait for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        self.count += 1;
        println!("The command has been called {} times", self.count);
        Ok(())
    }
}

struct Terminal {
    commands: Vec<Box<dyn CommandTrait>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn CommandTrait>) {
        self.commands.push(command);
    }

    fn run(&mut self) -> Result<()> {
        if let Ok(s) = fs::read_to_string("src/input.txt") {
            for com_line in s.lines() {
                let com_line = com_line.trim();
                if com_line.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = com_line.split_whitespace().collect();
                let com_name = parts[0];
                if com_name == "stop" {
                    let conn = Connection::open("bm.db")?;
                    conn.execute("drop table BMDataBase;", ())?;
                    break;
                }
                let mut found: bool = false;
                'cmd_for: for cmd in &mut self.commands {
                    if cmd.get_name() == com_name {
                        found = true;
                        match cmd.exec(&parts[1..]) {
                            Ok(_) => {
                                break 'cmd_for;
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                break 'cmd_for;
                            }
                        }
                    }
                    if com_name.to_lowercase() == cmd.get_name() {
                        found = true;
                        println!("Did you mean '{}'?", cmd.get_name());
                        break;
                    }
                }
                if found == false {
                    println!("Unknown Command!");
                }
            }
        } else {
            return Err(anyhow!("Error reading from the file!"));
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(IncCommand {}));
    terminal.register(Box::new(BmCommand {}));
    terminal.run()?;
    Ok(())
}
