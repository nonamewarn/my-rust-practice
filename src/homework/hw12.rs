use rand::Rng;

/// 1. Мінімальна кількість переміщень (небезпечна: повертає usize::MAX, якщо не можна)
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return usize::MAX; // Неможливо зрівняти
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut diff = 0;

    for &load in shipments {
        diff += load as i32 - avg as i32;
        moves += diff.abs() as usize;
    }

    moves
}

/// 2. Безпечний варіант з Result
fn count_permutation_safe(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return Err("Impossible to balance cargo");
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut diff = 0;

    for &load in shipments {
        diff += load as i32 - avg as i32;
        moves += diff.abs() as usize;
    }

    Ok(moves)
}

/// 3. Генерує абсолютно однакові вантажі
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..100);
    (0..n).map(|_| avg).collect()
}

/// 4. Генерує балансовані дані, які можна вирівняти
fn gen_balanced_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut base: Vec<u32> = (0..n).map(|_| rng.gen_range(10..100)).collect();
    let total: u32 = base.iter().sum();
    let avg = total / n as u32;
    vec![avg; n]
}

/// 5. Форматований друк
fn print_shipments_info(shipments: &Vec<u32>) {
    println!("Shipments: {:?}", shipments);
    match count_permutation_safe(shipments) {
        Ok(moves) => println!("✅ Minimum moves: {}", moves),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn main() {
    println!("🔹 Приклад 1 (з картинки): [1, 1, 1, 1, 6]");
    let example1 = vec![1, 1, 1, 1, 6];
    print_shipments_info(&example1);
    println!();

    println!("🔹 Приклад 2 (рандомно збалансовані)");
    let example2 = gen_balanced_shipments(5);
    print_shipments_info(&example2);
    println!();

    println!("🔹 Приклад 3 (однакові вантажі)");
    let example3 = gen_shipments(5);
    print_shipments_info(&example3);
    println!();

    println!("🔹 Приклад 4 (неможливо зрівняти): [10, 10, 10, 11]");
    let example4 = vec![10, 10, 10, 11];
    print_shipments_info(&example4);
}
