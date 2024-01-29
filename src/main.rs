use std::process::ExitCode;

use mindus::{
    data::{DataRead, DataWrite},
    Schematic, Serializable as _,
};

fn main() -> ExitCode {
    let mut a = std::env::args().skip(1);
    let tags: Vec<String> = a.by_ref().take_while(|x| x != "--").collect();
    for arg in a {
        let Ok(x) = std::fs::read(&arg) else {
            comat::cprintln!("{arg:red} is not readable");
            return ExitCode::FAILURE;
        };
        let Ok(mut x) = Schematic::deserialize(&mut DataRead::new(&x)) else {
            comat::cprintln!("{arg:red} is not a schematic");
            return ExitCode::FAILURE;
        };
        x.tags.insert("labels".to_string(), format!("{tags:?}"));
        let mut out = DataWrite::default();
        x.serialize(&mut out).unwrap();
        std::fs::write(arg, out.consume()).unwrap();
        println!("modified {}", x.tags["name"]);
    }
    ExitCode::SUCCESS
}
