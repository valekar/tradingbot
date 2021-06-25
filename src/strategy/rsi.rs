extern crate ta_lib_wrapper;
use crate::utils::constants::{RSI_OVERBOUGHT, RSI_OVERSOLD, RSI_PERIOD};
use crate::utils::util::display_contents;
use binance::model::KlineEvent;
use std::error::Error;
use ta_lib_wrapper::{TA_RetCode, TA_RSI};

trait Strategy {
    fn rsi(close_prices: &Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>>;
}
pub struct Me;
impl Strategy for Me {
    fn rsi(close_prices: &Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>> {
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
        Ok(out)
    }
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

pub fn use_rsi(kline_event: KlineEvent, closes: &mut Vec<f64>, in_position: &mut bool) {
    if kline_event.kline.is_final_bar == true {
        println!("candle Close at {} ", kline_event.kline.close);
        closes.push(kline_event.kline.close.parse().unwrap());
        println!("closes");
        display_contents(&closes);

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

pub fn use_rsi_example(closes: &Vec<f64>, in_position: &mut bool) {
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
