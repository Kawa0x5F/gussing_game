use std::io;    // 標準ライブラリである 入出力ライブラリ を スコープに入れる

// プログラムのエントリーポイント main を宣言
// 引数はないことがわかる
fn main() {

    // println!マクロを利用して数字を予想して入力を促す出力をする
    println!("Guessing the Number!");
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

    // 24行目からの一連のメソッドは次のように１行に収めることもできる
    // io::stdin().read_line(&mut guess).expect("Failied to read line");


    //  {} は値を所定の場所に出力するためのもの
    // 複数の変数の値を出力したい時は以下のように記述する
    //      let first = 1; let second = 2;
    //      println!("1st number is {}, 2nd number is {}", first, second);
    println!("Your guess is {}", guess); // println!マクロのプレースホルダーを利用して値を出力する


}