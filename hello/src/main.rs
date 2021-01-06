use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yaml::Error> {
    let mut points = Vec::new();

    let point = Point { x: 1.0, y: 2.0 };
    points.push(point);

    let point = Point { x: 3.0, y: 4.0 };
    points.push(point);

    let s = serde_yaml::to_string(&points)?;
    // assert_eq!(s, "---\nx: 1.0\ny: 2.0");

    println!("yaml: ------\n{}", s);

    let deserialized_points: Vec<Point> = serde_yaml::from_str(&s)?;
    // assert_eq!(point, deserialized_point);

    println!("deserialized: {}", deserialized_points.len());

    Ok(())
}
