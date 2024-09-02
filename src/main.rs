use std::env;
use lazy_static::lazy_static; // 1.4.0

use chrono::{Datelike, Timelike, Local};
use regex::Regex;
use tokio::time;
use tokio::sync::Mutex;

// Serenity is a library for the Discord API
use serenity::{
    async_trait, model::{channel::Message, gateway::Ready}, prelude::*
};

// Module containing strings of "commands" and "messages" for the discord bot
mod cms;
// Module containing functions and structs relating to the path of exile news rss-feed
mod poenews;
// Module containing saved rss feed data used in testing
mod rsstestdata;

lazy_static! {
    /// This list contains copies of messages initializing news with the `cms::POE_NEWS_START` command,
    /// and are removed when a message with the `cms:POE_NEWS_STOP` command is sent in the same channel
    /// 
    /// The messages contain `channel_id`'s which we use when posting news updates
    static ref MESSAGE_LIST: Mutex<Vec<Message>> = Mutex::new(vec![]);
    /// List of latest news posts
    static ref POST_LIST: Mutex<Vec<poenews::Post>> = Mutex::new(vec![]);
    /// This `boolean` makes sure the loop sending out news updates isn't run multiple times
    /// 
    /// Without this, there would be a new instances created every time the discord bot beacomes `ready`
    /// (every time the bot connects/reconnects to discord)
    static ref RUN_ONCE: Mutex<bool> = Mutex::new(false);
}

struct Handler;

#[async_trait]
// Message handler
impl EventHandler for Handler {

    // Send a discord message
    async fn message(&self, ctx: Context, msg: Message) {
        // Help messsage
        if msg.content == cms::HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, cms::HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
        
        // Poe News message
        if msg.content == cms::POE_NEWS_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, cms::POE_NEWS_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }

        // Poe News start message
        if msg.content == cms::POE_NEWS_START {

            // Check global message list for current channel (we don't want to run multiple news threads in same channel)
            let mes_list = &mut *MESSAGE_LIST.lock().await;
            if mes_list.iter().any(|x| x.channel_id == msg.channel_id) {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Already listening for news in this channel").await {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }
            // Add a copy of current message to global message list and post about it on discord
            mes_list.push(msg.clone());
            if let Err(why) = msg.channel_id.say(&ctx.http, "Started listening for news in this channel!").await {
                println!("Error sending message: {:?}", why);
            }
            
        }

        // Poe News stop message
        if msg.content == cms::POE_NEWS_STOP {

            // Check global message list for current channel (we stop listening for news by removing saved reference of starting message)
            let mes_list = &mut *MESSAGE_LIST.lock().await;
            if let Some(pos) = mes_list.iter().position(|x| x.channel_id == msg.channel_id) {
                mes_list.remove(pos);
                if let Err(why) = msg.channel_id.say(&ctx.http, "Ok, Stopped listening for news in this channel").await {
                    println!("Error sending message: {:?}", why);
                }
            } else if let Err(why) = msg.channel_id.say(&ctx.http, "I don't listen to news here").await {
                println!("Error sending message: {:?}", why);
            }

        }

    }

    // Bot startup condition (bot connected)
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Just terminal print to tell us startup succeeded
        println!("{} is connected!", ready.user.name);

        // Making sure everything past this point isn't run multiple times
        {
            if *RUN_ONCE.lock().await {
                return;
            }
            *RUN_ONCE.lock().await = true;
        }

        // Setting up regex outside loop since they will never need to be updated
        let reg = Regex::new(r#"<a href="(?<link>[^"]*)">(?<text>[^"]*)</a>"#).unwrap();
        let reg_date = Regex::new(r#":\d\d \+0000"#).unwrap();

        // Set timer for how often we want to loop over 
        let mut news_timer = time::interval(time::Duration::from_secs(1800));
        
        {
            // NB! first time we wait for a tick it is ignored (it's just how tokio works)
            // Therefore we have an additional "tick()" prior to the entering the loop
            news_timer.tick().await;
            *POST_LIST.lock().await = poenews::load_feed().await;
        }


        loop {
            news_timer.tick().await;
            {
                // list of new posts (if we find any)
                let mut new_posts: Vec<&poenews::Post> = Vec::new();
                // list of latest posts
                let new_list = poenews::load_feed().await;
                if !new_list.is_empty(){
                    {
                        let old_list = &*POST_LIST.lock().await;
                        // Counters so we can see when we have reached end of loop
                        // (making the lists peekable and using an iterator could be a cleaner approach)
                        let mut p_counter: i8;
                        let p_max: i8 = 30;
                        // Loop over old post list, compare posts in new post list, add new posts to new_posts
                        'outer: for np in &new_list {
                            // Make sure counter is reset for each post
                            p_counter = 0;
                            for op in old_list {
                                p_counter += 1;
    
                                // post are sorted by date, so once we reach a post in the new list that matches, we know there are
                                // no more new posts to be found, and no reason to continue looping
                                if op.pub_date == np.pub_date {
                                    break 'outer;
                                }
    
                                // we reached the end of our loops without breaking
                                if p_counter == p_max {
                                    new_posts.push(np);
                                }
                            }
                        }
    
                        // Loop over all "messages" saved in message list (list of channel_id's we want to post news to)
                        let mess_list = &*MESSAGE_LIST.lock().await;
                        for m in mess_list {
                            for p in &new_posts {
                                // Replace html-hyperlink with discord hyperlink using regex
                                let post_description = reg.replace_all(&p.description, "[$text]($link)");
                                // Remove last 5 characters from pub_date using regex
                                let post_date = reg_date.replace_all(&p.pub_date, "");
                                // Format the message we want the bot to send in discord             
                                let message = format!("## {}\n{}\n\n{}", &p.title, post_date, post_description);             
                                // Post the message
                                if let Err(why) = m.channel_id.say(&ctx.http, message).await {
                                    println!("Error sending message: {:?}", why);
                                }
                            }
                        }
                    }
                
                    // Update global list of posts with new list of posts
                    *POST_LIST.lock().await = new_list;
                    // Print date and time for loop completion in local time
                    let now = Local::now();
                    println!(
                        "list of news posts updated! {:02}.{:02}.{} {:02}:{:02}:{:02}",
                        now.day(),
                        now.month(),
                        now.year(),
                        now.hour(),
                        now.minute(),
                        now.second()
                    );
                } else {
                    let now = Local::now();
                    println!(
                        "Update failed! Could not fetch new posts {:02}.{:02}.{} {:02}:{:02}:{:02}",
                        now.day(),
                        now.month(),
                        now.year(),
                        now.hour(),
                        now.minute(),
                        now.second()
                    );
                }
            }
        }
    }
}

// macro saying this is where our main is
#[tokio::main]
async fn main() {
    
    // Get discord token
    let token = env::var("DISCORD_TOKEN")
        .expect("Err: Did not find discord token in environment");
    
    // Create a discord client
    let mut discord_client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Err creating client");


    // Await command usage of discord client
    if let Err(why) = discord_client.start().await {
        println!("Client error: {:?}", why);
    }

}



