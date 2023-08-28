# rust-sais
Suffix Array (接尾辞配列) を Induced Sorting によって構築するアルゴリズム SA-IS (入力に対し線形時間) の Rust による実装です。  
`sais`モジュールの`sais`関数に `String` 型の文字列の参照を与えると、`Vec<usize>`型の接尾辞配列が返ります。  
`src/main.rs` にはファイルを読み込んで接尾辞配列を構築する例があります。
