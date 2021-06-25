use crate::strategy::rsi::use_rsi_example;
use crate::utils::util::load_env;
use binance::account::*;
use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use std::env;
use std::sync::atomic::AtomicBool;

fn sample_rsi_test() {
    let closes: Vec<f64> = vec![
        34.017, 34.011, 34.02, 33.96, 33.938, 33.928, 33.972, 33.951, 33.798, 33.787, 33.752,
        33.665, 33.688, 33.611, 33.646, 33.612, 33.661, 33.745, 33.785, 33.769, 33.7, 33.859,
        33.908, 33.925, 33.891, 33.907, 33.879, 33.758, 33.759, 33.724, 33.649, 33.701, 33.649,
        33.67, 33.721, 33.662,
    ];
    let mut in_position = false;
    use_rsi_example(&closes, &mut in_position);
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
