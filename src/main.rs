// Structures (models) && Ownership
struct BankAcount  {
    balance: i32,
    verified: bool
}

// Add & to allow for ownership to be borrowed
fn print_balance(account: &BankAcount) {
    println!("{:?}", account.balance);
}
// Add & to allow for ownership to be borrowed
fn print_verified(account: &BankAcount) {
    println!("{:?}", account.verified);
}

// Results
fn is_verified(account: &BankAcount) -> Result<bool, bool> {
    let result = match account.verified {
        true => Ok(true),
        false => Err(false)
    };
    return result;
}

fn main() {
    let my_account = BankAcount {
        balance: 20,
        verified: false
    };
    let verification_status = is_verified(&my_account)
        .expect("Unable to unwrap result.");

    // println!("{:?}", my_account.balance);
    // println!("{:?}", my_account.verified);

    // Add & to allow for ownership to be borrowed
    print_balance(&my_account);
    print_verified(&my_account);
    println!("{:?}", verification_status);
    
}
