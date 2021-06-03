use structopt::StructOpt;
use std::path::PathBuf;
use say::Schema;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    #[structopt(short = "s", long = "schema")]
    schema: Option<String>,
}

fn main() {
    let args: Args = Args::from_args();

    if let Some(schema) = args.schema {
        let content = std::fs::read_to_string(schema).unwrap();
        let x: Schema = serde_json::from_str(&content).unwrap();
        dbg!(x);
    }
}