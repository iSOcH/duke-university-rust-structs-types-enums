enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1_000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        _ => FileSize::Terabytes(size as f64 / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
    }
}


fn main() {
    for i in 0..16 {
        let size: u64 = 10u64.pow(i);
        println!("{}", format_size(size));
    }

    // second challenge question seems unclear. isn't format_size already doing what is asked here?
}

/*
How does the `FileSize` enum help in representing different file sizes? What advantages does it provide over using separate variables or constants for each file size type?
    The solution using enums allows treating all FileSize variants uniformly

In the provided code, what would be the result of running the program with different size values? How would the program output change?
    this was shown in the video
 */