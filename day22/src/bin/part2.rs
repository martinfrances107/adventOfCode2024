fn main() {
    // let input = include_str!("./input1.txt");
    let mut sum = 0;
    // for line in input.lines() {
    // let mut sn = u64::from_str_radix(line, 10).unwrap();
    let mut sn = 123;
    // println!("parsed {sn}");
    turn_n::<2000>(&mut sn);
    sum += sn;
    // }

    println!("{:?}", sum);
}

static PRUNE: u64 = 0x100_0000;

fn turn(sn: &mut u64) {
    let a = *sn * 64;
    *sn = (*sn ^ a) % PRUNE;

    let a = *sn / 32;
    *sn = (*sn ^ a) % PRUNE;

    let a = *sn * 2048;
    *sn = (*sn ^ a) % PRUNE
}

fn turn_n<const N: u64>(input: &mut u64) {
    for _ in 0..N {
        println!("{}", (*input) % 10);
        turn(input);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let mut sn = 123u64;
        turn_n::<1>(&mut sn);
        assert_eq!(sn, 15887950);
        let mut sn = 123u64;
        turn_n::<2>(&mut sn);
        assert_eq!(sn, 16495136);
        turn(&mut sn);
        assert_eq!(sn, 527345);
        turn(&mut sn);
        assert_eq!(sn, 704524);
        turn(&mut sn);
        assert_eq!(sn, 1553684);
        turn(&mut sn);
        assert_eq!(sn, 12683156);
        turn(&mut sn);
        assert_eq!(sn, 11100544);
        turn(&mut sn);
        assert_eq!(sn, 12249484);
        turn(&mut sn);
        assert_eq!(sn, 7753432);
        turn(&mut sn);
        assert_eq!(sn, 5908254);
    }

    #[test]
    fn f() {
        let mut sn = 1;
        turn_n::<2000>(&mut sn);
        assert_eq!(sn, 8685429);

        let mut sn = 10;
        turn_n::<2000>(&mut sn);
        assert_eq!(sn, 4700978);

        let mut sn = 100;
        turn_n::<2000>(&mut sn);
        assert_eq!(sn, 15273692);

        let mut sn = 2024;
        turn_n::<2000>(&mut sn);
        assert_eq!(sn, 8667524);
    }
}
