fn return_a_number_or_none(num: Option<i8>) -> Option<i8> {
    match num {
        Some(val) if val > 10 => Some(val),
        _ => None,
    }
}

fn main() {
    let num = return_a_number_or_none(Some(20));
    let num_less = return_a_number_or_none(Some(8));

    let num_null = return_a_number_or_none(None);

    println!(
        "Number greater than 10: {:?}",
        match num {
            Some(i) => i.to_string(),
            None => "None".to_string(),
        }
    );
    println!("Number less than 10: {num_less:?}");
    println!("Number null: {num_null:?}");
}
