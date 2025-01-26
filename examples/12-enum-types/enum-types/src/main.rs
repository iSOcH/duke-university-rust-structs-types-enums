#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Fricktal
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn region_popularity(w: WineRegions) {
    match w {
        WineRegions::Bordeaux => println!("Highly popular!"),
        WineRegions::Fricktal => println!("Almost unknown"),
        _ => println!("Average"),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Summerträumli"),
        region: WineRegions::Fricktal,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
}

/*
What is an enum in Rust, and how is it used as a type? What advantages does it offer over other data types when representing a set of related values?
    Type which unifies multiple "variants". We can than match a value of that enum for the different variants.

In the provided code, what would be the result of uncommenting and executing the lines that print information about `wine1` and `wine2`? How would the program output change?
    obvious
 */