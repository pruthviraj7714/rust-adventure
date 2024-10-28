use tokio::join;

async fn fetch_data() -> String {
    "Hi from async fn".to_string()
}

async fn get_balance(address: &str) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    4
}

async fn get_token_supply() -> i32 {
    4
}

async fn get_token_mint() -> String {
    "asfas2132".to_string()
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("{}",data);

    let wallet_address = "0x42412131";
    let wallet_address2 = "0x4123121";

    let tast1 = tokio::spawn(get_balance(wallet_address));
    let task2 = tokio::spawn(get_balance(wallet_address2));

    let balance_of_wallet_1 = tast1.await.unwrap();
    let balance_of_wallet_2 = task2.await.unwrap();

    println!(
        "The balance of 1 is {} and 2 is {}",
        balance_of_wallet_1, balance_of_wallet_2
    );

    //Handling Tasks and Futures

    let (mint, supply) = join!(get_token_mint(), get_token_supply());

    println!("The mint {} and supply {}", mint, supply);
}
