use std::io;

fn main() -> io::Result<()> {
    let (_, fives) = (0..24).fold((1, Vec::new()), |(mut t, mut acc), i| {
        acc.push(t);
        t *= 5;

        (t, acc)
    });

    let mut total_sum: i64 = 0;
    for line in io::stdin().lines() {
        let mut sum: i64 = 0;
        let line = line.unwrap();
        let chars: Vec<_> = line.chars().collect();
        let length = chars.len();
        for (i, c) in chars.iter().enumerate() {
            let d = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };

            sum += d * fives[length - i - 1]
        }

        println!("Sum: {}", sum);
        total_sum += sum;
    }
    println!("Total sum: {}", total_sum);
    println!();
    println!("Snafu: {}", to_snafu(total_sum));

    Ok(())
}

fn to_snafu(c: i64) -> String {
    let mut acc = c;
    let digits = c.ilog(5) as i64;
    let fives: Vec<_> = (0..=digits).map(|d| 5_i64.pow(d as _)).collect();
    fives.iter().for_each(|b| acc += b * 2);
    fives
        .iter()
        .rev()
        .map(|b| {
            let r = acc / b;
            acc %= b;
            r
        })
        .map(|d| match d {
            6 => "1-",
            5 => "1=",
            4 => "2",
            3 => "1",
            2 => "0",
            1 => "-",
            0 => "=",
            _ => panic!(),
        })
        .collect()
}
// [1, 4, 1, 2, 4, 1, 1, 2, 0, 4, 3, 2, 1, 4, 2, 0, 0, 0, 3, 1, 0, 0, 0, 0]
// 30535047052796
// 30535047052797
// 30535047052797
// 30535047052797
// 30535047052797

// 19073486328125
// 95367431640625

// [day25/src/bin/part1.rs:10] &fives.iter().enumerate().collect::<Vec<_>>() = [
//     ( 0, 1,),
//     ( 1, 5,),
//     ( 2, 25,),
//     ( 3, 125,),
//     ( 4, 625,),
//     ( 5, 3125,),
//     ( 6, 15625,),
//     ( 7, 78125,),
//     ( 8, 390625,),
//     ( 9, 1953125,),
//     ( 10, 9765625,),
//     ( 11, 48828125,),
//     ( 12, 244140625,),
//     ( 13, 1220703125,),
//     ( 14, 6103515625,),
//     ( 15, 30517578125,),
//     ( 16, 152587890625,),
//     ( 17, 762939453125,),
//     ( 18, 3814697265625,),
//     ( 19, 19073486328125,),
//     ( 20, 95367431640625,),
//     ( 21, 476837158203125,),
//     ( 22, 2384185791015625,),
//     ( 23, 11920928955078125,),
// ]
