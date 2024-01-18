use std::io::{stdin, stdout, Write};

use reqwest::Client;
use serde::{Deserialize, Serialize};

const SERVER_URL: &str = "http://192.168.11.6:9999/";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Search {
    id: u64,
}

#[derive(Debug, Deserialize)]
struct SqlResult<T> {
    message: String,
    data: Option<T>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let inp: String = input();

        match inp.as_str() {
            "new" => new_user().await,
            "search" => {
                println!("idを入力");
                let id = input();
                let res = search_user(client.clone(), Search { id }).await;

                println!("{:?}", res.message);
                while let Some(ref user) = res.data {
                    println!("{:#?}", user)
                }
            }
            // ユーザーの削除
            "del" => delete().await,
            "conec" => {}
            // 終了
            "q" => break,
            _ => {}
        }
    }

    quit();
}

fn quit() {
    println!("終了します...");
}

async fn new_user() {
    println!("名前を入力");
    let name = input();
    println!("idを入力");
    let id = input();
    println!("パスワードを入力");
    let password = input();

    let user = User { id, name, password };

    let res = add_user(Client::new(), user).await;
    println!("{:#?}", res);
}

async fn delete() {
    println!("名前を入力");
    let name = input();
    println!("idを入力");
    let id = input();
    println!("パスワードを入力");
    let password = input();

    let user = User { id, name, password };

    let res = delete_user(Client::new(), user).await;
    println!("{:#?}", res);
}

async fn add_user(client: Client, data: User) -> SqlResult<char> {
    let res = client
        .post(SERVER_URL.to_owned() + "user/add")
        .json(&data)
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}

async fn search_user(client: Client, data: Search) -> SqlResult<Vec<User>> {
    let res = client
        .post(SERVER_URL.to_owned() + "user/search")
        .json(&data)
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}

async fn delete_user(client: Client, user: User) -> SqlResult<u64> {
    let res = client
        .post(SERVER_URL.to_owned() + "user/delete")
        .json(&user)
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}

fn input<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse().ok().unwrap()
}
