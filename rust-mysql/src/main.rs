use mysql::*;
use mysql::Opts;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    match Opts::from_url("mysql://root:root@127.0.0.1:3307/test") {
        Ok(opts) => {
            let pool = Pool::new(opts).unwrap();
            let mut conn = pool.get_conn().unwrap();

            conn.query_drop(
                r"CREATE TABLE IF NOT EXISTS payment (
                        customer_id int not null,
                        amount int not null,
                        account_name text
                        )").unwrap();

            let payments = vec![
                Payment { customer_id: 1, amount: 2, account_name: None },
                Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
                Payment { customer_id: 5, amount: 6, account_name: None },
                Payment { customer_id: 7, amount: 8, account_name: None },
                Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
            ];

            // Now let's insert payments to the database
            conn.exec_batch(
                r"INSERT INTO payment (customer_id, amount, account_name)
                      VALUES (:customer_id, :amount, :account_name)",
                payments.iter().map(|p| params! {
                        "customer_id" => p.customer_id,
                        "amount" => p.amount,
                        "account_name" => &p.account_name,
    }),
            ).unwrap();
        }
        Err(_) => println!("err")
    }
    // let opts = Opts::from_url("mysql://root:root@127.0.0.1:3307/test")?;
}
