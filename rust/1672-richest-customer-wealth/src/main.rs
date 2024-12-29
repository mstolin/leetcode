const DATA: [[u32; 3]; 3] = [[7, 1, 3], [2, 8, 7], [1, 9, 5]];

fn main() {
    if DATA.len() < 1 {
        panic!("No data given");
    } else if DATA.len() > 100 {
        panic!("Given data is too big");
    }

    let richest: Option<u32> = DATA
        .iter()
        .map(|account| {
            if account.len() <= 50 {
                account.iter().sum()
            } else {
                0
            }
        })
        .max();
    println!(
        "{:?}",
        richest.expect("couldn't find the richest customer wealth")
    );
}
