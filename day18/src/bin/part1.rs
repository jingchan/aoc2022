use std::io;

const grid_size: usize = 30;
// 2,2,2
// 1,2,2
// 3,2,2
// 2,1,2
// 2,3,2
// 2,2,1
// 2,2,3
// 2,2,4
// 2,2,6
// 1,2,5
// 3,2,5
// 2,1,5
// 2,3,5
fn main() -> io::Result<()> {
    let mut grid = [[[false; grid_size]; grid_size]; grid_size];
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut coords = line.split(",");
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        let z: usize = coords.next().unwrap().parse().unwrap();

        grid[x][y][z] = true;
    }

    // step, start, first axis, second axis
    let dirs: [[(i32, i32, i32); 4]; 3] = [
        [(1, 0, 0), (0, 0, 0), (0, 1, 0), (0, 0, 1)],
        // [
        //     (-1, 0, 0),
        //     ((grid_size - 1) as i32, 0, 0),
        //     (0, 1, 0),
        //     (0, 0, 1),
        // ],
        [(0, 1, 0), (0, 0, 0), (1, 0, 0), (0, 0, 1)],
        // [
        //     (0, -1, 0),
        //     (0, (grid_size - 1) as i32, 0),
        //     (1, 0, 0),
        //     (0, 0, 1),
        // ],
        [(0, 0, 1), (0, 0, 0), (1, 0, 0), (0, 1, 0)],
        // [
        //     (0, 0, -1),
        //     (0, 0, (grid_size - 1) as i32),
        //     (1, 0, 0),
        //     (0, 1, 0),
        // ],
    ];

    let mut total = 0;
    for d in dirs {
        let start = d[1];
        for i in 0..grid_size {
            let second_axis = mul(i as i32, d[3]);
            for j in 0..grid_size {
                let first_axis = mul(j as i32, d[2]);
                for k in 0..grid_size {
                    let ds = d[0];
                    let step = mul(k as i32, d[0]);

                    let x = start.0 + step.0 + first_axis.0 + second_axis.0;
                    let y = start.1 + step.1 + first_axis.1 + second_axis.1;
                    let z = start.2 + step.2 + first_axis.2 + second_axis.2;
                    // dbg!(d, i, j, k);
                    // dbg!(x, y, z);

                    let cur = grid[(x) as usize][(y) as usize][(z) as usize];
                    if k == 0 {
                        if cur {
                            total += 1;
                        }
                    }

                    if k == grid_size - 1 {
                        if cur {
                            total += 1;
                        }
                    } else {
                        let next = grid[(x + ds.0) as usize]
                            [(y + ds.1) as usize]
                            [(z + ds.2) as usize];
                        if cur && !next || next && !cur {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn mul(x: i32, a: (i32, i32, i32)) -> (i32, i32, i32) {
    (x * a.0, x * a.1, x * a.2)
}
