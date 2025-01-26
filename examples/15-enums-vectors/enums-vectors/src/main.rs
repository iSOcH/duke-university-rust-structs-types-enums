#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64),
}

// <'a> denotes a lifetime. it allows us to construct a ShapeWithArea with only a borrowed Shape
// instead of an owned one (which would require either moving or cloning), but ShapeWithArea cannot
// live longer than the Shape it borrows
#[derive(Debug)]
struct ShapeWithArea<'a> {
    shape: &'a Shape,
    area: f64,
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 13.0, 15.0)];

    let total_area: f64 = shapes
        .iter()
        .map(shape_area)
        .sum();

    println!("Total area: {} sq. units", total_area);

    let max_area = largest_shape_by_area(&shapes);
    println!("Max area: {max_area:?}");
}

fn erase<T>(_: T) {}

fn shape_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(a, b, c) => {
            // https://en.wikipedia.org/wiki/Heron%27s_formula
            let semi_perimeter = (a + b + c) / 2.0;
            (semi_perimeter * (semi_perimeter - a) * (semi_perimeter - b) * (semi_perimeter - c)).sqrt()
        }
    }
}

fn largest_shape_by_area(shapes: &[Shape]) -> Option<ShapeWithArea> {
    shapes.iter().map(|s| ShapeWithArea { shape: s, area: shape_area(s) }).max_by(|s1, s2| s1.area.total_cmp(&s2.area))
}

/*
How does the `Shape` enum help in organizing and representing different shapes? What advantages does it provide over using separate variables or constants for each shape type?
    The shapes can contain different data, for example if we also added a Rectangle variant we could store both lengths

In the provided code, what would be the result of running the program with different shapes and their respective dimensions? How would the program output change?
    shown in video, again
 */