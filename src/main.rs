use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rcli", author, about, long_about = "")]
struct Opts {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "csv", about = "Show SCV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, StructOpt)]
struct CsvOpts {
    #[structopt(short, long)]
    input: String,

    #[structopt(short, long, default_value = "output.json")]
    output: String,

    #[structopt(short, long, default_value = ",")]
    delimiter: char,

    #[structopt(long)]
    header: bool,
}

fn main() {
    let opts = Opts::from_args();
    print!("{:?}", opts)
}
