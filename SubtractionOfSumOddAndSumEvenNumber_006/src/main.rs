fn main() {
    const AMOUNT_OF_NUM: i32 = 5;
    const NUMS:[i32; AMOUNT_OF_NUM as usize] = [7, 4, 6, 8, 3];

    let mut _odd_number_sum: i32 = 0;
    let mut _even_number_sum: i32 = 0;

    for num in 0..NUMS.len() {
        if num % 2 == 0 {
            _even_number_sum += NUMS[num];
        }
        else{
            _odd_number_sum += NUMS[num];
        }
    }

    let result: i32 = _odd_number_sum - _even_number_sum;
    print!("Result: {}", result);
}
