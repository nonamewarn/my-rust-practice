use rand::Rng;

/// Генерує випадковий вектор з `n` елементів у діапазоні [10..=99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

/// Знаходить індекси пари з мінімальною сумою серед сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32, i32) {
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1], min_sum)
}

/// Форматований вивід у стилі з прикладу
fn print_min_adjacent_sum(data: &[i32]) {
    // 1. Індекси
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // 2. Дані
    print!("data:  [");
    for (i, val) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{:>2}", val);
    }
    println!("]");

    // 3. Стрілочка-покажчик
    let (i, a, b, sum) = min_adjacent_sum(data);
    print!("indexes:");
    for j in 0..data.len() {
        if j == i {
            print!("\\__");
        } else if j == i + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    // 4. Пояснення
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a, b, sum, i, i + 1
    );
}

fn main() {
    let data = gen_random_vector(20);
    print_min_adjacent_sum(&data);
}
