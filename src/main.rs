use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
fn main() {
    loop {
        // promptを出力
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        // 改行されるまで受け取りinputに格納.
        stdin().read_line(&mut input).unwrap();
        // 空白で分割
        let mut parts = input.trim().split_whitespace();

        // nextはpartsの先頭を返し, イテレータを次に進める.
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                //argumentがなければデフォルトは'/'へ移動.
                //peek() Returns a reference to the next() value without advancing the iterator.
                // map_orで, argsがSomeなら関数適用, Noneなら"/".
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root){
                    eprintln!("{}",e);
                }
            },
            // spawn()でプロセスを生成する.
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();
                // wait a child process if this process bear it.
                child.wait();
            }
        }

    }
}
