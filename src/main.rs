extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("引数の解析中にエラーが発生しました:{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("アプリケーションエラー:{}",e);
        process::exit(1);
    }

}
