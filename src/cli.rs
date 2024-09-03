use std::{collections::HashSet, path::PathBuf};

use crate::{crypt::{Crypt, CryptType}, game::Game, Error};

#[derive(Default)]
struct Args {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    encrypt: bool,
    game: Game,
    key: Option<PathBuf>,
    crypt: CryptType,
    file_list: Option<PathBuf>,
    help: bool,
}

#[inline]
fn require_next_arg(args: &mut std::env::Args, switch_name: &str) -> String {
    args.next().unwrap_or_else(|| {
        println!("+ Missing parameter for {}", switch_name);
        std::process::exit(128);
    })
}

impl Args {
    fn parse() -> Option<Args> {
        let mut args = Args::default();

        let mut iter = std::env::args();
        iter.next();

        let mut has_args = false;
        loop {
            let Some(arg) = iter.next() else {
                break;
            };

            has_args = true;

            match arg.as_str() {
                "-e" | "--encrypt" => args.encrypt = true,
                "-d" | "--decrypt" => args.encrypt = false,
                s @ ("-g" | "--game") => args.game = match require_next_arg(&mut iter, s).as_str() {
                    "none" => Game::None,
                    "umapd" => Game::UmaPD,
                    s @ _ => {
                        println!("Invalid game: {}", s);
                        std::process::exit(128);
                    }
                },
                s @ ("-k" | "--key") => args.key = Some(require_next_arg(&mut iter, s).into()),
                s @ ("-c" | "--crypt") => args.crypt = match require_next_arg(&mut iter, s).as_str() {
                    "none" => CryptType::None,
                    "md5" => CryptType::MD5,
                    s @ _ => {
                        println!("Invalid crypt type: {}", s);
                        std::process::exit(128);
                    }
                },
                s @ ("-l" | "--file_list") => args.file_list = Some(require_next_arg(&mut iter, s).into()),
                "--help" => args.help = true,

                s @ _ => {
                    if args.input.is_none() {
                        args.input = Some(s.into())
                    }
                    else if args.output.is_none() {
                        args.output = Some(s.into())
                    }
                    else {
                        println!("Invalid argument: {}", s);
                        std::process::exit(128);
                    }
                }
            }
        }

        if has_args { Some(args) } else { None }
    }
}

fn print_help() {
    println!("\
Arxide (v{}) - ArcSys asset encrypt/decryption tool
Usage: arxide [OPTIONS] INPUT OUTPUT

INPUT and OUTPUT can be files or directories

Options:
  -d, --decrypt: enable decryption mode (default)
  -e, --encrypt: enable encryption mode
  -g, --game: specify game
    If no other options are specified, game will be set
    to a default value.
  -k, --key: specify decryption key file
  -c, --crypt: specify crypt type,
  -l, --file_list: specify file list (text file)
  --help: show this help message
\
    ", env!("CARGO_PKG_VERSION"));
}

pub fn run() -> Result<bool, Error> {
    let Some(mut args) = Args::parse() else {
        print_help();
        return Ok(false);
    };

    if args.help {
        print_help();
        return Ok(true);
    }

    let Some(input_path) = args.input else {
        println!("+ No input path specified");
        return Ok(true);
    };

    if input_path.as_os_str().is_empty() {
        println!("+ Input path cannot be empty");
        return Ok(true);
    }

    let Some(output_path) = args.output else {
        println!("+ No output path specified");
        return Ok(true);
    };

    if output_path.as_os_str().is_empty() {
        println!("+ Output path cannot be empty");
        return Ok(true);
    }

    if args.game == Game::None && args.crypt == CryptType::None && args.key.is_none() {
        println!("+ No game or crypt options specified, setting game to UmaPD");
        args.game = Game::UmaPD;
    }

    if args.crypt == CryptType::None {
        args.crypt = match args.game {
            Game::None => {
                println!("+ No crypt type or game specified");
                std::process::exit(128);
            },
            Game::UmaPD => CryptType::MD5
        };
        println!("-- Game crypt type: {:?}", args.crypt);
    }

    let file_list = if let Some(path) = args.file_list {
        Crypt::load_file_list(&path)?
    }
    else {
        HashSet::new()
    };

    let crypt = Crypt {
        type_: args.crypt,
        key_path: args.key,
        game: args.game,
        file_list
    };
    crypt.process(&input_path, &output_path, args.encrypt)?;

    Ok(true)
}