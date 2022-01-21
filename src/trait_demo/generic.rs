struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct SinglePoint<T> {
    x: T,
    y: T,
}

impl<T> SinglePoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl SinglePoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let sp = SinglePoint { x: 5, y: 10 };

    println!("sp.x = {}", sp.x());

    let fp = SinglePoint { x: 5.0, y: 7.5 };
    println!(
        "The sp distance from origin = {}.",
        fp.distance_from_origin()
    );
}
