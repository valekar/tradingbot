use binance::account::*;
use binance::api::*;
use dotenv::dotenv;
use std::env;

use binance::userstream::*;
use binance::websockets::*;
use std::sync::atomic::AtomicBool;

fn main() {
    load_env();

    k_line_for_pair("solusdt");
}

fn k_line_for_pair(pair: &str) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline: String = format!("{}{}", pair, "@kline_1m");
    println!("kliner is {}", &kline);
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                if kline_event.kline.is_final_bar == true {
                    println!("candle Close at {} ", kline_event.kline.close);
                }

                //println!("candle Close at {} ", kline_event.kline.close);
                /*println!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                )*/
            }
            _ => (),
        };
        Ok(())
    });
    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => {
                println!("Error: {:?}", err);
            }
        }
    }
    web_socket.disconnect().unwrap();
}

fn load_env() {
    println!("Loading .env variables!!");
    dotenv().ok();
}

fn get_user_account_details() {
    let use_binance_details = UserBinanceDetails::new();
    let account: Account =
        Binance::new(use_binance_details.api_key, use_binance_details.api_secret);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn web_socket_example() {
    load_env();
    let use_binance_details = UserBinanceDetails::new();
    let api_key = use_binance_details.api_key;
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let user_stream: UserStream = Binance::new(api_key, None);

    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::AccountUpdate(account_update) => {
                    for balance in &account_update.balance {
                        println!(
                            "Asset: {}, free: {}, locked: {}",
                            balance.asset, balance.free, balance.locked
                        );
                    }
                }
                WebsocketEvent::OrderTrade(trade) => {
                    println!(
                        "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                        trade.symbol, trade.side, trade.price, trade.execution_type
                    );
                }
                _ => (),
            };
            Ok(())
        });

        web_socket.connect(&listen_key).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            match e {
                err => {
                    println!("Error: {:?}", err);
                }
            }
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

struct UserBinanceDetails {
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl UserBinanceDetails {
    fn new() -> Self {
        let api_keys = env::var("BINANCE_CLIENT_API").ok();
        let api_secrets = env::var("BINANCE_CLIENT_SECRET").ok();

        UserBinanceDetails {
            api_key: api_keys,
            api_secret: api_secrets,
        }
    }
}
