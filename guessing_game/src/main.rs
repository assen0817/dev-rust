use std::cmp::Ordering;
use std::io;
use rand::Rng;

// documents
// https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

// 使用しているものlibraryドキュメントを作成
// cargo doc --open
// 数字当てゲーム
fn main() {
    println!("Guess the number!");

    // １～１００の間でランダムな整数を作成
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    // ループ
    loop {
        println!("Please input your guess.");

        // 変数 変数名 = 型
        let mut guess = String::new();

        // コマンドの入力待ち、何かしら起きたときのエラー
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // 入力値を数値に変換(u32)
        // 前後の空白を消す
        // 数値以外の入力をされたかどうかをマッチングする、数値以外が入力されたらループの最初に戻る
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // 入力値とランダム値を比較
        // 入力値がランダム値より小さいとLessとマッチング
        // 入力値がランダム値より大きいとGreaterとマッチング
        // 入力値がランダム値と一緒の場合には、Equalとマッチング、ループを抜ける　
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; },
        }
    }
}
