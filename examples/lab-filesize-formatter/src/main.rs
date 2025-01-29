use std::env::args;
use std::fmt::{Debug, Display, Formatter};
use std::process::ExitCode;
use crate::FileSize::{Bytes, Gigabytes, Kilobytes, Megabytes};

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl FileSize {
    fn as_raw_bytes(&self) -> u64 {
        match self {
            Kilobytes(n) => (n * 1000f64) as u64,
            Megabytes(n) => (n * 1_000_000f64) as u64,
            Gigabytes(n) => (n * 1_000_000_000f64) as u64,
            Bytes(n) => *n,
        }
    }

    fn as_byte(&self) -> FileSize {
        FileSize::Bytes(self.as_raw_bytes())
    }

    fn as_kilo(&self) -> FileSize {
        let bytes = self.as_raw_bytes();
        Kilobytes(bytes as f64 / 1_000f64)
    }

    fn as_mega(&self) -> FileSize {
        let bytes = self.as_raw_bytes();
        Megabytes(bytes as f64 / 1_000_000f64)
    }

    fn as_giga(&self) -> FileSize {
        let bytes = self.as_raw_bytes();
        Gigabytes(bytes as f64 / 1_000_000_000f64)
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bytes(bytes) => f.write_fmt(format_args!("{} bytes", bytes)),
            Kilobytes(kb) => f.write_fmt(format_args!("{:.2} KB", kb)),
            Megabytes(mb) => f.write_fmt(format_args!("{:.2} MB", mb)),
            Gigabytes(gb) => f.write_fmt(format_args!("{:.2} GB", gb)),
        }
    }
}

#[derive(Debug)]
struct Sizes<'a> {
    bytes: &'a str,
    kilobytes: &'a str,
    megabytes: &'a str,
    gigabytes: &'a str
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => Bytes(size),
        1000..=999_999 => Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => Megabytes(size as f64 / 1_000_000.0),
        _ => Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        Bytes(bytes) => format!("{} bytes", bytes),
        Kilobytes(kb) => format!("{:.2} KB", kb),
        Megabytes(mb) => format!("{:.2} MB", mb),
        Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn parse(input: &str) -> Option<FileSize> {
    let (amount, unit) = input.split_once(' ')?;
    let amount = amount.parse::<f64>().ok()?;

    match unit.to_ascii_lowercase().as_str() {
        "b" => Some(Bytes(amount as u64)),
        "kb" => Some(Kilobytes(amount)),
        "mb" => Some(Megabytes(amount)),
        "gb" => Some(Gigabytes(amount)),
        _ => None
    }
}

fn main() -> ExitCode {
    let cmdline_args: Vec<String> = args().collect();
    if cmdline_args.len() != 2 {
        println!("Usage: {} \"file size\"", cmdline_args[0]);
        return ExitCode::FAILURE;
    }

    let result = parse(&cmdline_args[1]).ok_or(ExitCode::FAILURE);
    match result {
        Ok(size) => {
            println!("The filesize given was {size}");

            let sizes = Sizes {
                bytes: &size.as_byte().to_string(),
                kilobytes: &size.as_kilo().to_string(),
                megabytes: &size.as_mega().to_string(),
                gigabytes: &size.as_giga().to_string(),
            };

            println!("This is equivalent to: {sizes:?}");

            ExitCode::SUCCESS
        },
        Err(exit_code) => {
            println!("Failed to parse argument as Filesize");
            exit_code
        },
    }
}