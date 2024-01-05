fn main() {
    println!("# 1.在函数定义中使用泛型");
    let list = [1, 2, 3, 4, 5, 6];
    let largest = get_largest(&list);
    println!("{list:?} largest is {largest}");
    let largest = get_largest1(&list);
    println!("{list:?} largest is {largest}");

    let list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest1(&list);
    println!("{list:?} largest is {largest}");

    println!("\n# 2.结构体定义中的泛型");
    println!("\n# 4.方法定义中的泛型");
    let point = Point { x: 1, y: 2 };
    println!("point {:?} x:{}", point, point.x);
    println!("point {:?} x:{} y:{}", point, point.x, point.y);

    let point = Point { x: 1.0, y: 2.0 };
    println!(
        "point {:?} distance_from_origin {:?}",
        point,
        point.distance_from_origin()
    );
}

// 结构体中使用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct Point2<T, U> {
    x: T,
    y: U,
}

fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > &largest {
            largest = *item;
        }
    }
    largest
}

// & 为借用
fn get_largest1<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0]; // 借用
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
