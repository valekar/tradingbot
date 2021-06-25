use binance::websockets::*;
use std::sync::atomic::AtomicBool;
extern crate ta_lib_wrapper;
use strategy::rsi::use_rsi;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};
use utils::util::{display_contents, load_env};

mod example;
mod strategy;
mod utils;

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
                use_rsi(kline_event, closes, &mut in_position);

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
