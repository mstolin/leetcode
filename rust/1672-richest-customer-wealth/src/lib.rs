#![allow(dead_code)]

fn approach_a(accounts: &[&[u8]]) -> Option<u8> {
    let mut max = None;
    for customer in accounts {
        let wealth: u8 = (*customer).iter().sum();
        if let Some(current) = max {
            max = Some(std::cmp::max(current, wealth));
        } else if wealth > 0 {
            max = Some(wealth);
        }
    }
    max
}

fn approach_b(accounts: &[&[u8]]) -> Option<u8> {
    accounts
        .iter()
        .map(|customer| {
            if customer.len() <= 50 {
                customer.iter().sum()
            } else {
                0
            }
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [[u8; 3]; 3] = [[7, 1, 3], [2, 8, 7], [1, 9, 5]];

    #[test]
    fn test_approach_a() {
        let data = DATA.iter().map(|v| v.as_slice()).collect::<Vec<&[u8]>>();
        let result = approach_a(data.as_slice());
        assert_eq!(Some(17), result);
    }

    #[test]
    fn test_approach_b() {
        let data = DATA.iter().map(|v| v.as_slice()).collect::<Vec<&[u8]>>();
        let result = approach_b(data.as_slice());
        assert_eq!(Some(17), result);
    }
}
