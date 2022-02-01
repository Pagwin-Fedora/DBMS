extern crate clap;
#[macro_use]
extern crate serenity;
extern crate dirs;
extern crate futures;

use clap::{App, Arg, ArgMatches};
use futures::executor::block_on;
use serenity::model::prelude::Ready;
use serenity::{Client, model::id::ChannelId, client::EventHandler};
use serenity::prelude::*;
fn main() {
    let send = App::new("send")
        .about("Send a message");
    let edit = App::new("edit")
        .about("Edit a message");
    let delete = App::new("delete")
        .about("Delete a message");
    let matches = App::new("Message edit shim")
        .version("0.1")
        .author("Pagwin <dev@pagwin.xyz>")
        .arg(Arg::new("config file")
            .short('c')
            .long("config")
            )
        .arg(Arg::new("api token")
            .short('a')
            .long("api-token"))
        .subcommand(send)
        .subcommand(edit)
        .subcommand(delete)
        .get_matches();
    let mut client = block_on(Client::builder(get_token(&matches))).unwrap();
    //how I'm going to do this
    let handler:Handler = match matches.subcommand(){
        Some(("send",_)) =>{
            Handler {
                
            }
        },
        Some(("edit",_)) =>{},
        Some(("delete",_)) => {},
        Some(_)=>{}
        None => {}
    }
    block_on(client.start());
}
fn get_token(matches:&ArgMatches)->&str{
    matches.value_of("api token").unwrap_or_else(||{matches.value_of("config file").expect("api token or config file not provided")})
}
enum Action{Send, Edit, Delete}
struct Handler{
    channelId:u64,
    messageId:Option<u64>,
    action:Action
}
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready:Ready){
        let test_channel = ChannelId(self.channelId);
        //test_channel.say(context,"test")
        
    }
}
