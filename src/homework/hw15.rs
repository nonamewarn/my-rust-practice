use itertools::Itertools;

fn main() {
    let letters = ['M', 'U', 'X', 'A', 'S', 'L', 'O', 'N'];
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    for perm in digits.iter().permutations(8).unique() {
        let m = *perm[0];
        let u = *perm[1];
        let x = *perm[2];
        let a = *perm[3];
        let s = *perm[4];
        let l = *perm[5];
        let o = *perm[6];
        let n = *perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            count += 1;
            println!("{:>4}\n×   {}\n-----\n{:>4}\n", muxa, a, slon);
        }
    }

    println!("Кількість рішень: {}", count);
}
