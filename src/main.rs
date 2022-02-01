extern crate clap;
extern crate serenity;
extern crate dirs;
extern crate futures;

use std::process;

use clap::{App, Arg, ArgMatches};
use futures::executor::block_on;
use serenity::model::prelude::Ready;
use serenity::{Client, model::id::ChannelId, client::EventHandler};
use serenity::prelude::*;
fn main() {
    let send = App::new("send")
        .about("Send a message")
        .arg(Arg::new("message text")
            .required(true)
            .long("text")
            .short('t')
            .takes_value(true));
    let edit = App::new("edit")
        .about("Edit a message")
        .arg(Arg::new("message id")
            .required(true)
            .long("message-id")
            .short('m')
            .takes_value(true))
        .arg(Arg::new("message text")
            .long("text")
            .short('t')
            .required(true)
            .takes_value(true));
    let delete = App::new("delete")
        .about("Delete a message")
        .arg(Arg::new("message id")
            .required(true)
            .long("message-id")
            .short('m')
            .takes_value(true));
    let app = App::new("Message edit shim")
        .version("0.1")
        .author("Pagwin <dev@pagwin.xyz>")
        .arg(Arg::new("config file")
            .long("config")
            .takes_value(true))
        .arg(Arg::new("api token")
            .short('a')
            .long("api-token")
            .takes_value(true))
        .arg(Arg::new("channel id")
            .short('c')
            .long("channel-id")
            .required(true)
            .takes_value(true))
        .subcommand(send)
        .subcommand(edit)
        .subcommand(delete);
    let matches = app.get_matches();
    //pretty sure I want to redo this so I use the derive method so I can just use an enum for the
    //subcommands or maybe there's a way to do it here and I'm dumb
    let handler:Handler = match matches.subcommand(){
        Some(("send",submatch)) =>{
            Some(Handler {
                channel_id: matches.value_of("channel id").unwrap().parse().expect("invalid channel id"),
                action: Action::Send(submatch.value_of("message text").unwrap().to_string())
            })
        },
        Some(("edit",submatch)) =>{
            Some(Handler {
                channel_id: matches.value_of("channel id").unwrap().parse().expect("invalid channel id"),
                action: Action::Edit(
                    submatch.value_of("message id").unwrap().parse().expect("invalid message id"),
                    submatch.value_of("message text").unwrap().to_string())
            })
        },
        Some(("delete",submatch)) => {
            Some(Handler {
                channel_id: matches.value_of("channel id").unwrap().parse().expect("invalid channel id"),
                action: Action::Delete(submatch.value_of("message id").unwrap().parse().expect("invalid message id"))
            })
        },
        Some(_)=>{None}
        None => {None}
    }.expect("Sub command not provided");
    let mut client = block_on(Client::builder(get_token(&matches))
        .event_handler(handler)).unwrap();
    block_on(client.start()).unwrap();
}
fn get_token(matches:&ArgMatches)->&str{
    matches.value_of("api token").unwrap_or_else(||{matches.value_of("config file").expect("api token or config file not provided")})
}
enum Action{Send(String), Edit(u64,String), Delete(u64)}
struct Handler{
    channel_id:u64,
    action:Action
}
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, _ready:Ready){
        let channel = ChannelId(self.channel_id);
        match &self.action {
            Action::Send(message) => {
                channel.say(context, message).await.unwrap();
            },
            //messy clones ick
            Action::Edit(message_id, new_message) => {
                channel.edit_message(context, message_id.clone(), |m| m.content(new_message)).await.unwrap();
            },
            Action::Delete(message_id) => {
                channel.delete_message(context, message_id.clone()).await.unwrap();
            }
        }
        process::exit(0);
    }
}
