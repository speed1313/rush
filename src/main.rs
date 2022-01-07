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

        // 改行文字を除く
        let command = input.trim();

        let mut child = Command::new(command).spawn().unwrap();

        // wait a child process if this process bear it.
        child.wait();
    }
}
