//use binance::account::*;
//use binance::api::*;
use dotenv::dotenv;
//use std::env;

//use binance::userstream::*;
use binance::websockets::*;
//use std::fmt::Display;
use std::sync::atomic::AtomicBool;

extern crate ta_lib_wrapper;

use ta_lib_wrapper::{TA_RetCode, TA_RSI};

const RSI_PERIOD: u8 = 14;
const RSI_OVERBOUGHT: f64 = 70.0;
const RSI_OVERSOLD: f64 = 30.0;
const TRADE_SYMBOL: &str = "solusdt";
fn main() {
    load_env();
    let mut closes: Vec<f64> = Vec::new();
    k_line_for_pair(TRADE_SYMBOL, &mut closes);
}

fn k_line_for_pair(pair: &str, closes: &mut Vec<f64>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline: String = format!("{}{}", pair, "@kline_1m");
    let mut in_position: bool = false;
    println!("k_liner is {}", &kline);
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                if kline_event.kline.is_final_bar == true {
                    println!("candle Close at {} ", kline_event.kline.close);
                    closes.push(kline_event.kline.close.parse().unwrap());
                    println!("closes");
                    display_contents(closes);
                    find_rsi(closes, &mut in_position);
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

fn display_contents(elements: &Vec<f64>) {
    println!("Contents of array ::");
    for element in elements {
        print!(" {}", element)
    }
    println!(" ")
}

fn rsi(close_prices: &Vec<f64>) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::with_capacity(close_prices.len());
    let mut out_begin: i32 = 0;
    let mut out_size: i32 = 0;

    unsafe {
        let ret_code = TA_RSI(
            0,
            close_prices.len() as i32 - 1,
            close_prices.as_ptr(),
            RSI_PERIOD.into(),
            &mut out_begin,
            &mut out_size,
            out.as_mut_ptr(),
        );

        match ret_code {
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }
    out
}

fn find_rsi(closes: &Vec<f64>, in_position: &mut bool) {
    if closes.len() > RSI_PERIOD.into() {
        let result = rsi(&closes);
        display_contents(&result);
        let last_rsi = result.last();

        match last_rsi {
            Some(res) => {
                println!("the current RSI is {}", res);
                enter_into_position(res, in_position);
            }
            None => {
                println!("no RSI result");
            }
        }
    }
}

fn enter_into_position(last_rsi: &f64, in_position: &mut bool) {
    if *last_rsi > RSI_OVERBOUGHT {
        println!("Sell! Sell! Sell!");
    }

    if *last_rsi < RSI_OVERSOLD {
        if *in_position {
            println!("We have already bought, no need to do anything :) ");
        } else {
            println!("BUY! BUY! BUY! ");
            //order logic
            *in_position = true;
        }
    }
}

/*fn sample_rsi_test() {
    let closes: Vec<f64> = vec![
        34.017, 34.011, 34.02, 33.96, 33.938, 33.928, 33.972, 33.951, 33.798, 33.787, 33.752,
        33.665, 33.688, 33.611, 33.646, 33.612, 33.661, 33.745, 33.785, 33.769, 33.7, 33.859,
        33.908, 33.925, 33.891, 33.907, 33.879, 33.758, 33.759, 33.724, 33.649, 33.701, 33.649,
        33.67, 33.721, 33.662,
    ];
    let mut in_position = false;
    find_rsi(&closes, &mut in_position);
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
}*/
