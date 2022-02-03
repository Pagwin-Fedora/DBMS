extern crate clap;
extern crate serenity;
extern crate futures;
extern crate dirs;
#[macro_use]
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate lazy_static;
use clap::{App, Arg, ArgMatches};
use futures::executor::block_on;
use serenity::model::prelude::Ready;
use serenity::{Client, model::id::ChannelId, client::EventHandler};
use serenity::prelude::*;
use std::{process, fs};
lazy_static!{
    static ref CONFIG_FILE_LOCATION:String = 
        dirs::config_dir().unwrap()
        .join("DBMS")
        .join("config.yaml")
        .into_os_string().into_string().unwrap();
}
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
            .takes_value(true)
            .default_value(CONFIG_FILE_LOCATION.as_str())
            )
        .arg(Arg::new("api token")
            .short('a')
            .long("api-token")
            .takes_value(true))
        .arg(Arg::new("channel id")
            .short('c')
            .long("channel-id")
            .takes_value(true))
        .subcommand(send)
        .subcommand(edit)
        .subcommand(delete);
    let matches = app.get_matches();
    let (channel_id, api_token) = gather_init_info(&matches);

    //pretty sure I want to redo this so I use the derive method so I can just use an enum for the
    //subcommands or maybe there's a way to do it here and I'm dumb
    let handler:Handler = 
    match matches.subcommand(){
        Some(("send",submatch)) =>{
            Some(Handler {
                channel_id,
                action: Action::Send(submatch.value_of("message text").unwrap().to_string())
            })
        },
        Some(("edit",submatch)) =>{
            Some(Handler {
                channel_id,
                action: Action::Edit(
                    submatch.value_of("message id").unwrap().parse().expect("invalid message id"),
                    submatch.value_of("message text").unwrap().to_string())
            })
        },
        Some(("delete",submatch)) => {
            Some(Handler {
                channel_id,
                action: Action::Delete(submatch.value_of("message id").unwrap().parse().expect("invalid message id"))
            })
        },
        Some(_)=>{None}
        None => {None}
    }.expect("Sub command not provided");
    let mut client = block_on(Client::builder(api_token)
        .event_handler(handler)).unwrap();
    block_on(client.start()).unwrap();
}
//grabs channel id and api token from the cli args or config file
fn gather_init_info(matches:&ArgMatches) -> (u64,String) {
    let config:Option<Config> = if matches.value_of("channel id").is_none() || matches.value_of("api token").is_none() {
        let config_file_loc = matches.value_of("config file").unwrap();
        Some(serde_yaml::from_reader(
            fs::File::open(config_file_loc)
                .expect(format!("config file doesn't exist at {}",config_file_loc).as_str())
            ).expect("invalid yaml config file"))
    } else {None};
    
    let channel_id:u64 = 
        match matches.value_of("channel id") {
            Some(id)=> id.parse().expect("invalid channel id"),
            None => config.clone().unwrap().channel_id
                
        };
    let api_token:String = 
        match matches.value_of("api token") {
            Some(token)=>token.to_string(),
            None => config.unwrap().token
        };
    (channel_id,api_token)
}
enum Action{Send(String), Edit(u64,String), Delete(u64)}
struct Handler{
    channel_id:u64,
    action:Action
}
#[derive(Serialize,Deserialize,Clone)]
struct Config{
    channel_id:u64,
    token:String
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
