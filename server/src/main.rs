#![feature(generic_const_exprs)]
#![feature(iter_collect_into)]
#![feature(if_let_guard)]

mod db;
mod lang;
mod objects;
mod proto;
mod types;
mod error;
mod update;
mod events;
mod query;
mod cache;
mod values;
mod permissions;
mod tags;
mod constraints;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use deadpool::Runtime;
use deadpool_sqlite::{Config, Pool};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use byte_reader::Cursor;
use tokio::spawn;
use tokio::sync::{mpsc, Mutex};
use crate::constraints::Constraint;
use crate::tags::{BuiltInTagId, TagId};
use crate::values::{DateTime, Value};
use crate::proto::{Decode, IncomingMessage};


struct AppState {
    active_sessions: Mutex<HashMap<i64, [u8; 32]>>,
}

#[tokio::main]
async fn main() {
    let pool = Config::new("../dev.db")
        .create_pool(Runtime::Tokio1)
        .unwrap();

    /*
    {
        let conn = pool.get().await.unwrap();
        let locked = conn.lock().unwrap();
        
        // Created + Favourite
        let q = db::objects::query(
            locked.as_ref(),
            Some(Constraint::And(
                Box::new(Constraint::Tag {
                    id: TagId::BuiltIn(BuiltInTagId::Created),
                    match_value: None,
                }),
                Box::new(Constraint::Tag {
                    id: TagId::BuiltIn(BuiltInTagId::Favourite),
                    match_value: None,
                })
            )),
            100,
            0,
        );
        
        println!("{q:?}");
    }*/

    // let (send_channel, _receive_channel) = tokio::sync::broadcast::channel::<String>(10);

    let app = Router::new()
        .route("/", get(async || Html(include_str!("../index.html"))))
        .route(
            "/connect",
            get(|ws: WebSocketUpgrade, State(pool): State<Pool>| async {
                ws.on_upgrade(|socket| ws_channel(socket, pool))
            }),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("Shutting down...");
        })
        .await
        .unwrap();
}

fn transform(slice: &[u8]) -> Vec<u8> {
    
}

async fn ws_channel(ws: WebSocket, pool: Pool) {
    let (mut sender, mut receiver) = ws.split();
    let (message_out, mut message_collect) = mpsc::channel::<Vec<u8>>(128); // Magic number

    spawn(async move {
        while let Some(msg) = message_collect.recv().await {
            sender.send(Message::Binary(msg.into())).await.unwrap();
        }
    });

    loop {
        let message_out = message_out.clone();
        match receiver.next().await {
            Some(Ok(Message::Binary(payload))) => {
                println!("Got a payload: {:?}", payload.as_ref());
                
                let pool = pool.clone();

                spawn(async move {
                    let mut cursor = Cursor::new(payload.as_ref());
                    
                    let in_msg = Decode::<IncomingMessage>::next(&mut cursor)
                        .await
                    
                    println!("sent: {:?}", rb.as_slice());
                    
                    message_out.send(rb.into()).await.unwrap();
                });
            }
            Some(Ok(Message::Close(_))) => break,
            Some(Ok(_)) => {
                spawn(async move {
                    message_out.send(vec![1]).await.unwrap();
                });
            }
            Some(Err(e)) => todo!("error: {e:?}"),
            None => break,
        }
    }

    println!("Disconnected");
}

/*

let user_name = loop {
    if let Message::Text(name) = receiver.next().await.unwrap().unwrap() {
        let mut users = state.user_set.lock().await;

        if users.contains(name.as_str()) {
            let _ = sender
                .send(Message::Text(Utf8Bytes::from_static("Username already taken.")))
                .await;

            continue;
        }

        users.insert(name.to_string());
        break name.to_string();
    }
};

let mut rx = state.sender.subscribe();

// Now send the "joined" message to all subscribers.
let msg = format!("{user_name} joined.");
tracing::debug!("{msg}");
let _ = state.sender.send(msg);

// Spawn the first task that will receive broadcast messages and send text
// messages over the websocket to our client.
let mut send_task = tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
        // In any websocket error, break loop.
        if sender.send(Message::text(msg)).await.is_err() {
            break;
        }
    }
});

let mut recv_task = tokio::spawn({
    let sender = state.sender.clone();
    let name = user_name.clone();

    async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = sender.send(format!("{name}: {text}"));
        }
    }
});

tokio::select! {
    _ = &mut send_task => recv_task.abort(),
    _ = &mut recv_task => send_task.abort(),
}

let msg = format!("{user_name} left.");
tracing::debug!("{msg}");
let _ = state.sender.send(msg);

state.user_set.lock().await.remove(&user_name); */
