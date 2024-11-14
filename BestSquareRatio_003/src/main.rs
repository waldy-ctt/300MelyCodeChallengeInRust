fn main() {
    const AMOUNT_OF_CASE: i32 = 3;
    const CASES: [i32; AMOUNT_OF_CASE as usize] = [2, 49, 10000000];
    let mut _final_result: [f32; 2 * AMOUNT_OF_CASE as usize] = [0.0; 2 * AMOUNT_OF_CASE as usize];

    let mut index: usize = 0;
    println!("Result: ");
    for _case in CASES {
        let amount_of_block: i32 = _case;
        let _root: f32 = (amount_of_block as f32).sqrt();
        if check_if_number_is_whole_number(_root) && _root * _root == amount_of_block as f32 {
            _final_result[index] = _root;
            _final_result[index + 1] = _root;
            index += 2;
        } else {
            let mut _x: f32 = (_root as i32) as f32;
            let mut _y: f32 = amount_of_block as f32 / _x;

            while {
                _x <= 0.0
                    || _y <= 0.0
                    || !check_if_number_is_whole_number(_x)
                    || !check_if_number_is_whole_number(_y)
                    || _x * _y != amount_of_block as f32
            } {
                _x += 1.0;
                _y = amount_of_block as f32 / _x;
            }
            _final_result[index] = _x;
            _final_result[index + 1] = _y;
            index += 2;
        }
    }
    println!("size: {}", _final_result.len());

    for item in _final_result{
        println!("item: {}", item);
    }
}

fn check_if_number_is_whole_number(num: f32) -> bool {
    //True = whole number, False = have decimal point
    if num % 1.0 == 0.0 {
        return true;
    }
    return false;
}
