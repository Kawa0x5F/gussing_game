use std::io;            // 標準ライブラリである 入出力ライブラリ を スコープに入れる
use std::cmp::Ordering; // 標準ライブラリから列挙子を持つ比較enumをスコープに入れる
use rand::Rng;          // 乱数生成メソッドの実装されたトレイトをスコープに入れる

// プログラムのエントリーポイント main を宣言
// 引数はないことがわかる
fn main() {

    // println!マクロを利用して数字の予想を促す出力をする
    println!("Guessing the Number!");

    // rand::thread_rng関数から乱数生成器を取得している
    // gen_rangeメソッドはRngトレイトで定義されており，範囲式を引数に取り，その範囲内の乱数を生成する
    // 今回の範囲式は下限値は含むが上限値は含まないため， 1~100 は 1..101 と表現する必要がある
    // 1..=100 と表現することもできる
    let secret_number = rand::thread_rng().gen_range(1..101);   // 不変変数 secret_number を 1~100までの範囲の乱数 で 束縛する

    // 無限ループを作成する
    loop {
        // println!マクロを利用して数字の入力を促す出力をする
        println!("Please input your guess!");


        // 変数の作成をする
        // letで不変な変数の作成，mutをつけることで作成した変数は可変となる
        // 統合記号(=)は ある変数を束縛する ことを表す
        // Stringは文字列型を表す．newはStringの関連関数であり，空の文字列を作成する
        let mut guess = String::new();  // 可変(mutable)な変数 guess を 空のString型インスタンス で 束縛する


        // 入出力ライブラリ ioモジュール の stdin関数 を呼び出している
        // *なお，最初にioライブラリのインポートを行なっていなくとも
        //      std::io::stdin. ...
        // と書くことによって関数を呼び出すこともできる
        // stdin関数はターミナルの標準入力へのハンドルを表す方である std::io::Stdinのインスタンス を返す
        io::stdin()
            // 標準入力ハンドルからread_lineを呼び出し，ユーザからの入力を受けている
            // 引数として &mut guess を渡し，ユーザの入力した文字列を guess に追加している
            // 同時に io::Result も返す．
            // 変更をするので渡す変数は可変で履くてはならない
            // また，&は変数が参照であることを示す．引数もデフォルトが不変であるため &guess では不十分で mut が必要となる
            .read_line(&mut guess)

            // io::Resultは，汎用のResultなどと同様に列挙型(enum)である
            // Resultの列挙子は Ok か Err で，それぞれ中にその理由や値が格納されている
            // io::Resultのメソッドである expect は io::ResultインスタンスがErr出会った時に，プログラムをクラッシュさせる
            // その後，引数の文字列を出力する． Ok の場合は列挙子が保持する値を取り出し返す．
            // この処理を行わない時，Result値を利用しておらず，エラーの可能性に対応していないことからコンパイル時に警告が出る
            .expect("Failed to read line");

        // io::stdin().read_line(&mut guess).expect("Failied to read line");


        // guess に対してある型から別の型に値を変換する時によく使用されるシャドーイングをする
        // 入力が文字列として格納された guess は Stringインスタンスである
        // このインスタンスの trimメソッド は文字列の先頭と末尾の空白，改行文字などをすべて削除する
        // パースは文字列を解析して何らかの数値にする．さまざまな数値型へパースできるため，変数に注釈をつける必要がある．

        // エラー処理へと移行し，parseの返り値であるResult型が数値への変換に成功したならOkに格納したnumを，
        // 失敗したならErr値を返す．match のアームは，Err(_)のパターンに合致する（_がどのような値でも受け付けるため）
        // したがって，２番目のアームコードであるcontinueを実行する
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        //  {} は値を所定の場所に出力するためのもの
        // 複数の変数の値を出力したい時は以下のように記述する
        //      let first = 1; let second = 2;
        //      println!("1st number is {}, 2nd number is {}", first, second);
        println!("Your guess is {}", guess); // println!マクロのプレースホルダーを利用して値を出力する


        // cmpメソッドは二つの値の比較を行う．比較できるものなら何に対しても呼び出せる．
        // Ordering列挙型の列挙子を返す．match式を利用している．
        // match式はアームで構成されており，パターンにマッチした時に実行されるコードで構成される．
        // 例えば，cmpは50と38を比較すると，50が大きいためOrdering::Greaterを返す．
        // match式はこれを各アームのパターンで吟味し，マッチしたものを実行する．
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   // 小さかった時
            Ordering::Greater => println!("Too big!"),  // 大きかった時
            Ordering::Equal => {                        // 同じだった時
                println!("You win!");
                break;      // ループを抜ける
            },

        }
    }
}