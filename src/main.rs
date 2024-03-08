use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let args:Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let query:&String  = &args[1];
    let filename:&String = &args[2];
    // println!("{}を探しています", query);
    // println!("{}というファイルの中から", filename);

    let mut f:File = File::open(filename)
    .expect("ファイルが見つかりませんでした");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("ファイルの読み込みに失敗しました");

    println!("テキストは\n{}です", contents);
}
