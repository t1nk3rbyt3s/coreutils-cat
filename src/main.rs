use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        let mut buf = String::new();

        loop {
            let _ = io::stdin().read_line(&mut buf);

            println!("{buf}");
        }
    }

    let buf = &args[1];

    let file: File = File::open(buf).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
