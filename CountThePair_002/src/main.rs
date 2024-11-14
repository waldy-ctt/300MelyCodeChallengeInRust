fn main() {
    const N: i32 = 10;
    const X: i32 = 12;
    const A: [i32; N as usize] = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
    let mut result: i32 = 0;

    for i in 1..N + 1 {
        let num_i: i32 = A[(i - 1) as usize];

        for j in i..N + 1 {
            let num_j: i32 = A[(j - 1) as usize];

            if check_if_number_is_suit(num_i, num_j, X) {
                result += 1;
            }
        }
    }
    print!("Result: {}", result);
}

fn check_if_number_is_suit(i: i32, j: i32, x: i32) -> bool {
    if 1 <= i && i <= j && j <= x {
        if ((i32::pow(i, 2)) + j) == x {
            return true;
        }
    }

    return false;
}
