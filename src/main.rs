use clap::Parser;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'A', long = "show-all", help = "equivalent to -vET")]
    show_all: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool,

    #[arg(short = 'e', help = "equivalent to -vE")]
    v_e: bool,

    #[arg(short = 'E', long = "show-ends", help = "display $ at end of each line")]
    show_ends: bool,

    #[arg(short = 'n', long = "number", help = "number all output lines")]
    number: bool,

    #[arg(short = 's', long = "squeeze-blank", help = "suppress repeated empty output lines")]
    squeeze_blank: bool,

    #[arg(short = 't', help = "equivalent to -vT")]
    v_t: bool,

    #[arg(short = 'T', long = "show_tabs", help = "display TAB characters as ^I")]
    show_tabs: bool,

    #[arg(short = 'u', help = "(ignored)")]
    ignore: bool,

    #[arg(short = 'v', long = "show-nonprinting", help = "use ^ and M- notation, except for LFD and TAB")]
    show_nonprinting: bool,

    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let program: String = std::env::args()
        .next()
        .as_ref()
        .unwrap_or(&"rat".to_string())
        .to_owned();

    for path_str in args.paths {
        if path_str == "-"
        {
            let mut buff = String::new();
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buff).unwrap();
            continue;
        };

        let path = std::path::Path::new(&path_str);
        let meta = match std::fs::metadata(&path) {
            Ok(meta) => meta,
            Err(err) => {
                println!("{program}: {}: {}", path.display(), err);
                return;
            }
        };

        if !meta.is_file() {
            if meta.is_dir() {
                println!("{program}: {}: Is a directory", path.display());
                return;
            };
            if meta.is_symlink() {
                println!("{program}: {}: Is a symlink", path.display());
                return;
            };
        };

        let mut file = match std::fs::read_to_string(path) {
            Ok(file) => file,
            Err(err) => {
                println!("{program}: {}: {}", path.display(), err);
                return;
            }
        };

        if args.show_tabs {
            file = file.replace("\t", "^I");
        };

        if args.number_nonblank || args.number {
            file = file
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    if args.number_nonblank {
                        if line.is_empty() {
                            return "\n".to_string();
                        };
                    };
                    let ln = (i+1).to_string();

                    let suffix = if ln.len() > 6 {
                        ln
                    } else {
                        " ".repeat(6 - ln.len()).as_str().to_owned() + &ln
                    };

                    format!("{}\t{}\n", suffix, line)
                })
                .collect::<String>();
        };

        if args.show_ends {
            file = file
                .lines()
                .map(|line| line.to_owned() + "$\n")
                .collect::<String>();
        };

        print!("{}", file);
    }
}
