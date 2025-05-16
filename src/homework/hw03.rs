const WIDTH: usize = 30;   // має бути парне число
const HEIGHT: usize = 14;  // кількість рядків (парна для симетрії)

fn main() {
    let mut result = String::new();

    for row in 0..HEIGHT {
        for col in 0..=WIDTH {
            let left_diag = row;
            let right_diag = WIDTH - row;

            let mid = HEIGHT / 2;
            let is_border = row == 0 || row == HEIGHT - 1 || col == 0 || col == WIDTH;

            let is_upper_x = row < mid && (col == row * 2 || col == WIDTH - row * 2);
            let is_lower_x = row >= mid && (col == (HEIGHT - 1 - row) * 2 || col == WIDTH - (HEIGHT - 1 - row) * 2);

            if is_border || is_upper_x || is_lower_x {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}
