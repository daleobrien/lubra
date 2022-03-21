use array_tool::vec::Intersect;
use itertools::Itertools;

// 0.8.2

fn main() {
    let points: Vec<Vec<u8>> = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]];
    let lines: Vec<Vec<u8>> = vec![vec![1, 6], vec![2, 5], vec![3, 4]];
    let curves: Vec<Vec<u8>> = vec![vec![4, 5], vec![5, 6], vec![6, 4]];

    let mut count = 0;

    for points_k in 0..=points.len() {
        for perm_points in points.iter().combinations(points_k).unique() {
            let flat_points = perm_points
                .into_iter()
                .flat_map(|s| s.clone())
                .collect::<Vec<u8>>();

            for points_l in 0..=lines.len() {
                for perm_lines in lines.iter().combinations(points_l).unique() {
                    // Check they don't intersect
                    let flat_lines = perm_lines
                        .into_iter()
                        .flat_map(|s| s.clone())
                        .collect::<Vec<u8>>();

                    if flat_points.intersect(flat_lines.clone()).len() > 0 {
                        continue;
                    }

                    let mut last_perm_curves_len = 0;
                    for curves_l in 0..curves.len() {
                        for perm_curves in curves.iter().combinations(curves_l).unique() {
                            // Check they don't intersect
                            let flat_curves = perm_curves
                                .into_iter()
                                .flat_map(|s| s.clone())
                                .collect::<Vec<u8>>();

                            if flat_points.intersect(flat_curves.clone()).len() > 0 {
                                continue;
                            }

                            if flat_lines.intersect(flat_curves.clone()).len() > 0 {
                                continue;
                            }
                            // Skip if last collection of curves had 2 or more curves, since it's a
                            // fully connected, in really the same way.
                            let perm_curves_len = flat_curves.len();
                            if perm_curves_len >= 4 && last_perm_curves_len >= 4 {
                                continue;
                            }
                            count += 1;
                            print!("{count} ");
                            if flat_points.len() > 0 {
                                for x in flat_points.clone() {
                                    if [1, 2, 3].contains(&x) {
                                        print!("N{x},");
                                    } else {
                                        print!("T{x},");
                                    }
                                }
                            }
                            if flat_lines.len() > 0 {
                                // TODO: more, could be 2, 3, and then use Y, or X2
                                match flat_lines[0] {
                                    1 => print!("L1,"),
                                    2 => print!("L2,"),
                                    3 => print!("L3,"),
                                    _ => {}
                                }
                            }
                            if flat_curves.len() > 0 {
                                print!("A{:?}", flat_curves)
                            }
                            println!("");
                            last_perm_curves_len = perm_curves_len;
                        }
                    }
                }
            }
        }
    }
}
