/*Transaction data type owns the variables inside it
 * Inorder to fix the bug, we need to borrow/ reference the Transaction type in our print_tx
 * funtion to avoid moving ownership to the function
 */

#[derive(Debug)]
#[allow(unused)]
struct Transaction {
    txid: String,
    amount: u64,
}

fn print_tx(tx: &Transaction) {
    println!("{:?}", tx);
}

fn main() {
    let tx = Transaction {
        txid: String::from("abc123"),
        amount: 50_000,
    };

    print_tx(&tx);

    // ❌ FIX THIS LINE WITHOUT CLONING
    println!("{:?}", tx);
}
