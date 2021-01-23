// Rustに入門してみるぞい！！
fn main() {
    // 変数作って{}を使って出力
    let name = "aya";
    println!("Hello, {}", name);

    println!("-----------");

    // let time = 0;で変数を宣言するとイミュータブルだから変更できない
    // 変数にmutをつけると変更できるようになる
    let mut time = 0;
    while time < 3 {
        println!("hoge");
        time = time + 1;
    }

    println!("-----------");

    // なんでifの中でprintln!()を呼ぶときはセミコロンなくて平気なの？
    if true {
        println!("hoge")
    }
    // こんな感じにifを変数にすることができて、その場合は複数の処理を書かない限りはセミコロンがいらない
    let aaa = true;
    let bbb = if aaa {
        format!("aaa")
    } else {
        format!("bbb")
    };
    println!("{}", bbb);
    // これを関数に書き出す時、return if aaa { format!("aaa") } else { format!("bbb") };みたいにしなくても、
    // returnがいらないからif aaa { format!("aaa") } else { format!("bbb") }だけで値が返せる
    println!("{}", msg());

    println!("-----------");

    // FizzBuzzしてみる（rangeで）
    let range = 1..10;  // 1以上10未満
    for time in range {
        println!("{}", fizzbuzz(time))
    }

    println!("-----------");

    // FizzBuzzしてみる（mapで）
    let range = 1..10;
    let messages = range.map(fizzbuzz);
    for line in messages {
        println!("{}", line);
    }

    println!("-----------");

    // 値として受け取っているため、所有権が変数cccのoutput関数のmessage引数に行ってしまう
    let ccc = format!("test1");
    output(ccc);
    // println!("{}", ccc); output関数のmessage引数に所有権がいってるので怒られる

    // &をつければ参照になる（Goみたいだね）
    let ccc = format!("test2");
    output2(&ccc);
    println!("{}", &ccc);
}

// 関数にしてみた
fn fizzbuzz(time: u32) -> String {
    if time % 15 == 0 {
        // str型とstring型があって、string型にするためにformatのマクロを使う
        format!("FizzBuzz")
    } else if time % 5 == 0 {
        format!("Buzz")
    } else if time % 3 == 0 {
        format!("Fizz")
    } else {
        format!("{}", time)
    }
}

fn msg() -> String {
    let aaa = true;
    if aaa {
        format!("aaa")
    } else {
        format!("bbb")
    }
}

fn output(message: String){
    println!("{}", message)
}

fn output2(message: &String) {
    println!("{}", message)
}
