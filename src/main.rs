use std::{env, path::Path};
use walkdir::WalkDir;
//use rand::Rng;
use std::time::SystemTime;

use serenity::{
    async_trait,
    http::AttachmentType,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!no" {
            println!("got a message");
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it./c/Users/taylo/Pictures/b
            let mut i = 0;
            let mut picpath: String = "/media/pics".to_owned();
            let count = WalkDir::new(&picpath).into_iter().count();
            //let count: u32 = filecount.into();// as u64;
            //let counttmp: u64 = count.into();
            println!("no {}",count);
            let secret_number = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize) % count;// rand::thread_rng().gen_range(1, count );
            println!("yes");
            let mut filename: String = "attachment://".to_owned();
            //let mut filerelpath: String = "./B/".to_owned();

            println!("got a message, {} files, {} secret",count, secret_number);

            for entry in WalkDir::new(&picpath).into_iter().filter_map(|entry| entry.ok()) {
                i = i + 1;
                if i == secret_number { 
                    if let Ok(metadata) = entry.metadata() {//.unwrap().is_file() {
                        if metadata.is_file() {
                            let tmp = entry.file_name().to_str().unwrap();

                       // match tmp {
                        //    Some(x) => {
                            filename.push_str(tmp);
                            println!("file {}",tmp);
                            picpath.push_str(tmp);
                          //  }
                        //}
                        }
                    }
                    break
                }
                    //filename.push_str(entry.file_name().unwrap().to_str());
                    //filerelpath.push_str(entry.file_name().unwrap().to_str());
            }

            let msg = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("Sneed's Feed and Seed, Formerly Chucks.");
                    m.embed(|e| {
                        e.title("Random Shitpost");
                        e.description("no");
                        e.image(&filename);

                        e
                    });
                    m.add_file(AttachmentType::Path(Path::new(&picpath)));
                    m
                })
                .await;
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
