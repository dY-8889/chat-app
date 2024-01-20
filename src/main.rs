use std::{
    io::{stdin, stdout, Write},
    thread::{self, sleep},
    time::Duration,
};

use reqwest::Client;
use serde::{Deserialize, Serialize};

const SERVER_URL: &str = "http://127.0.0.1:9999/";

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Search {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct SqlResult<T> {
    message: String,
    data: Option<T>,
}

#[derive(Debug, Serialize)]
struct ChatRoom {
    id: u16,
    name: String,
    password: String,
    message: Option<Vec<String>>,
    user_list: Vec<i16>,
}

#[derive(Debug, Clone, Serialize)]
struct EnterRoom {
    room_id: i32,
    room_name: String,
    password: String,
    user_id: i32,
}

#[derive(Debug, Serialize)]
struct Message {
    text: String,
    room_id: i32,
}

impl Message {
    fn new(text: String, room_id: i32) -> Message {
        Message { text, room_id }
    }
}

impl EnterRoom {
    fn new(room_name: String, password: String) -> EnterRoom {
        EnterRoom {
            room_id: 0,
            room_name,
            password,
            user_id: 0,
        }
    }
}

struct App {
    user: User,
    room: Option<ChatRoom>,
}

fn draw() {
    clear();
}

fn help() {
    println!(
        "
help | h -> ヘルプ
new      -> 新しくユーザーを作る
search   -> ユーザーを検索
del      -> ユーザーを削除
create   -> 新しく部屋を作る
enter    -> 部屋に入る
quit | q -> 終了
    "
    );
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    //clear();
    //let login: String = pinput(Some("ログイン"), "~>");

    loop {
        let inp: &str = &pinput::<String>(None, "> ");

        match inp {
            "new" => new_user().await,
            "search" => {
                let id = pinput(Some("idを入力"), "=> ");
                let name = pinput(Some("名前を入力"), "=> ");

                let res = search_user(client.clone(), Search { id, name }).await;

                println!("{:?}", res.message);
                if let Some(ref user) = res.data {
                    println!("{:#?}", user)
                }
            }
            // ユーザーの削除
            "del" => delete().await,
            "create" => {
                let name = pinput(Some("部屋の名前"), "=> ");
                let password = pinput(Some("部屋のパスワード"), "=> ");

                let res = create_room(client.clone(), EnterRoom::new(name, password)).await;
                println!("{:#?}", res);
            }
            "enter" => {
                let room_id = pinput(Some("部屋のidを入力"), "=> ");
                let room_name = pinput(Some("部屋の名前を入力"), "=> ");
                let password = pinput(Some("部屋のパスワードを入力"), "=> ");
                let user_id = pinput(Some("ユーザーのidを入力"), "=> ");

                let room = EnterRoom {
                    room_id,
                    room_name,
                    password,
                    user_id,
                };

                let res = enter_room(client.clone(), room.clone()).await;
                println!("{:#?}", res);

                if res.data.is_some() {
                    chat(room).await;
                }
            }
            // 終了
            "quit" | "q" => break,
            "help" | "h" => help(),
            _ => {}
        }
    }

    quit();
}

async fn message_get(client: Client, room_id: i32) -> SqlResult<Vec<String>> {
    let res = client
        .post(SERVER_URL.to_owned() + "message/get")
        .json(&room_id)
        .send()
        .await
        .unwrap();
    res.json().await.unwrap()
}

async fn message_send(client: Client, message: Message) -> SqlResult<bool> {
    let res = client
        .post(SERVER_URL.to_owned() + "message/send")
        .json(&message)
        .send()
        .await
        .unwrap();
    res.json().await.unwrap()
}

async fn chat(room: EnterRoom) {
    tokio::spawn(async move {
        let client = Client::new();
        loop {
            let res = message_get(client.clone(), room.room_id).await;
            println!("{:#?}", res.data);
            sleep(Duration::from_secs_f32(2.));
        }
    });

    let client = Client::new();
    loop {
        let text = pinput(None, ">> ");
        message_send(client.clone(), Message::new(text, room.room_id)).await;
        // println!("{:#?}", res);
    }
}

async fn new_user() {
    println!("名前を入力");
    let name = input();
    println!("パスワードを入力");
    let password = input();

    let user = User {
        id: 0,
        name,
        password,
    };

    let res = create_user(Client::new(), user).await;
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

async fn create_user(client: Client, data: User) -> SqlResult<bool> {
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

async fn create_room(client: Client, new_room: EnterRoom) -> SqlResult<bool> {
    let res = client
        .post(SERVER_URL.to_owned() + "room/create")
        .json(&new_room)
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}

async fn enter_room(client: Client, room: EnterRoom) -> SqlResult<bool> {
    let res = client
        .post(SERVER_URL.to_owned() + "room/enter")
        .json(&room)
        .send()
        .await
        .unwrap();

    res.json().await.unwrap()
}

fn pinput<T: std::str::FromStr>(message: Option<&str>, str: &str) -> T {
    if let Some(m) = message {
        println!("{}", m)
    }
    print!("{}", str);
    stdout().flush().unwrap();
    input()
}

fn input<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse().ok().unwrap()
}

fn clear() {
    println!("\x1B[2J\x1B[1;1H");
}

fn quit() {
    println!("終了...");
}
