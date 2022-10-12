use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: Prices,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
    eur: f32,
}

fn get_price(coin: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin.to_lowercase()
    ))
    .call()?
    .into_string()?;

    let coin_data: CoinData = serde_json::from_str(&body).unwrap();

    Ok(coin_data.market_data.current_price.usd.to_string())
}

fn main() {
    loop {
        let mut coin = String::new();
        println!("Enter coin name: ");

        let _ = std::io::stdin()
            .read_line(&mut coin)
            .expect("An error occurred");

        let result_price = get_price(&coin);

        if coin.trim() == "exit" {
            break;
        }

        match result_price {
            Ok(price) => println!("The price of {} is {} USD", coin, price),
            Err(e) => println!("Error: {}", e),
        }
    }
}
