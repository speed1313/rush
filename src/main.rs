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
        // pipeの場合に備えて"|"で一旦分割.
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            // nextはpartsの先頭を返し, イテレータを次に進める.
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    //argumentがなければデフォルトは'/'へ移動.
                    //peek() Returns a reference to the next() value without          advancing the iterator.
                    // map_orで, argsがSomeなら関数適用, Noneなら"/".
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                "exit" => return,

                // spawn()でプロセスを生成する.
                command => {
                    // pipeがあれば,
                    // stdinを前のcommandのoutputにつなげる.
                    // inheritで親のioを受け継ぐ
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                            
                        );
                    // 次のコマンドがあれば,
                    // stdoutを次のコマンドに準備するように準備
                    let stdout = if commands.peek().is_some(){
                        Stdio::piped()
                    } else {
                        //次に送るコマンドがないため, outputをshellに送る.
                        Stdio::inherit()
                    };
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output{
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }
        // loopを抜けたとき, previous_commandには最後のコマンドが入っているため, 
        if let Some(mut final_command) = previous_command {
            final_command.wait();
        }
    }
}
