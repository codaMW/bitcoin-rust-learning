/* Define a struct named BitcoinTransaction with fields:
 * txid: String a hex like "abd123"
 * amount: u64
 * sender: String
 * receiver: String
 * Write a function new_transaction that takes parameters, returns a new instance (handle ownership
 * by passing references where posible)
 * Add a method display to print the tx details
 *
 * NOTE: Do not clone unnecessarily
 *
 * Expected Output: A main function that creates & displays a tx
 */

#[derive(Debug)]
#[allow(unused)]
struct BitTx {
    txid: [u8; 8],
    amount: u64,
    sender: String,
    receiver: String,
}

impl BitTx {
    fn new_transaction(txid: [u8; 8], amount: u64, sender: String, receiver: String) -> Self {
        
        Self {
            txid,
            amount,
            sender,
            receiver,
        }
    }

    fn display(&self) {

        println!("{:?}", self);
    }
}

fn main() {
    let tx1 = BitTx::new_transaction(*b"12345678", 2500,String::from("Satoshi"),String::from("Nakamoto"));

   tx1.display();
}
