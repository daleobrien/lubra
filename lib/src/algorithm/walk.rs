// 0.6.5

use std::cmp::{max, min};

use rand::Rng;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPoint {
    x: i32,
    y: i32,
}

fn distance(a: IPoint, b: IPoint) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn random_point_between(a: IPoint, b: IPoint, max_point: IPoint, exceed: i32) -> IPoint {
    let mut r = rand::thread_rng();

    let x_min = min(a.x, b.x) - exceed;
    let x_max = max(a.x, b.x) + exceed;
    let y_min = min(a.y, b.y) - exceed;
    let y_max = max(a.y, b.y) + exceed;

    let x = if x_min != x_max {
        r.gen_range(x_min..x_max)
    } else {
        x_min
    };
    let y = if y_min != y_max {
        r.gen_range(y_min..y_max)
    } else {
        y_min
    };

    // Bound point to limits, can't exceed max_point
    return IPoint {
        x: min(max(x, 0), max_point.x),
        y: min(max(y, 0), max_point.y),
    };
}

// Split a segment into two segments
pub fn bisect_segment(segment: &mut Vec<IPoint>, index: usize, max_point: IPoint) {
    if index + 1 >= segment.len() {
        return;
    }
    let p0 = segment[index];
    let p1 = segment[index + 1];

    let d = distance(p0, p1);

    if d < 4 {
        return;
    }

    if d > 25 {
        segment.insert(index + 1, random_point_between(p0, p1, max_point, 3));
        segment.insert(index + 1, random_point_between(p0, p1, max_point, 3));
        segment.insert(index + 1, random_point_between(p0, p1, max_point, 3));
    } else if d > 10 {
        segment.insert(index + 1, random_point_between(p0, p1, max_point, 2));
    } else {
        segment.insert(index + 1, random_point_between(p0, p1, max_point, 1));
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::{max, min};
    use std::{assert_eq, vec};

    use crate::algorithm::walk::{bisect_segment, IPoint};

    pub fn assert_is_between_points(a: IPoint, mid_point: IPoint, c: IPoint) {
        assert!(max(a.x, c.x) >= mid_point.x);
        assert!(min(a.x, c.x) <= mid_point.x);
        assert!(max(a.y, c.y) >= mid_point.y);
        assert!(min(a.y, c.y) <= mid_point.y);
    }

    #[test]
    fn split_long_line() {
        let mut segment: Vec<IPoint> = Vec::new();
        segment.push(IPoint { x: 0, y: 0 });
        segment.push(IPoint { x: 100, y: 100 });

        bisect_segment(&mut segment, 0, IPoint { x: 100, y: 100 });

        let first_point = segment[0];
        let last_point = segment[segment.len() - 1];
        assert_eq!(first_point, IPoint { x: 0, y: 0 });
        assert_is_between_points(IPoint { x: 0, y: 0 }, segment[1], IPoint { x: 100, y: 100 });
        assert_eq!(last_point, IPoint { x: 100, y: 100 });
    }

    #[test]
    fn reverse_segment() {
        let mut segment: Vec<IPoint> = Vec::new();
        segment.push(IPoint { x: 74, y: 56 });
        segment.push(IPoint { x: 41, y: 39 });

        bisect_segment(&mut segment, 0, IPoint { x: 74, y: 56 });

        let first_point = segment[0];
        let last_point = segment[segment.len() - 1];
        assert_eq!(first_point, IPoint { x: 74, y: 56 });
        assert_is_between_points(IPoint { x: 41, y: 39 }, segment[1], IPoint { x: 74, y: 56 });
        assert_eq!(last_point, IPoint { x: 41, y: 39 });
    }

    #[test]
    fn too_short() {
        let mut segment: Vec<IPoint> = Vec::new();
        segment.push(IPoint { x: 0, y: 0 });
        segment.push(IPoint { x: 1, y: 1 });

        bisect_segment(&mut segment, 0, IPoint { x: 1, y: 1 });

        assert_eq!(segment, vec![IPoint { x: 0, y: 0 }, IPoint { x: 1, y: 1 }]);
    }
}

pub fn walk() {
    println!("Walk called");

    let mut segment: Vec<IPoint> = Vec::new();
    let max_point = IPoint { x: 20, y: 20 };
    segment.push(IPoint { x: 0, y: 0 });
    segment.push(IPoint { x: 20, y: 20 });

    let mut last_len = segment.len();
    for _j in 0..100 {
        let l = segment.len();
        // The length will change and get bigger as we loop, so iterator in reverse.
        for i in (0..l - 1).rev() {
            bisect_segment(&mut segment, i, max_point);
            segment.dedup();
        }
        if last_len == segment.len() {
            // exit if nothing changed
            break;
        }
        last_len = segment.len();
        // if last_len > 40 {
        //    break;
        // }
    }

    // print out all the segments
    print!("pts = {{");
    for i in 0..segment.len() {
        let v = segment[i];
        if i + 1 < segment.len() {
            print!("{{{}, {}}},", v.x, v.y);
        } else {
            print!("{{{}, {}}}", v.x, v.y);
        }
    }
    println!("}};");

    //println!("{:?}", segment);
}
