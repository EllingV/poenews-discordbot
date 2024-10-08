# Poenews-Discordbot
This is a simple bot written in rust that periodically fetches news from the [Path of Exile rss-feed](https://www.pathofexile.com/news/rss), and post updates in relevant discord channels.  

It's intended to make it easier keeping up to date with news related to the game Path of Exile.

------------------------
- [Features](#features)
- [Getting Started](#getting-started)
	- [Prerequisites](#prerequisites)
	- [Installation](#installation)
- [Usage](#usage)
- [File Structure](#file-structure)
- [External Crates](#external-crates)
- [Additional Notes](#Additional-notes)
------------------------

## Features
The discord bot comes with a few commands:  
#### !help
The bot will respond by listing the available commands with a short description.  
Can be usefull for when you forget the available commands.
#### !poenews
The bot will respond with more information about **!poenews start** and **!poenews stop**.  
#### !poenews start
The bot will start to post updates regarding the path of exile news feed in the current channel.  
There will also be a response from the bot informing about whether or not it's currently listening for news.
#### !poenews stop
The bot will stop posting updates regarding the path of exile news feed in the current channel.  
There will also be a response from the bot informing about whether or not it's currently listening for news.

## Getting Started
### Prerequisites  
- Cargo version 1.76.0 or newer
	- Cargo is Rust's build system and package manager, so we need it for building and running our application

- A Discord bot application with all gateway intents
	- A discord bot application is needed to acces the Discord API, so we need a corresponding discord token for the application to work
	- These can be set up easily at https://discord.com/developers/applications

### Installation

#### Step 1: Clone the project  
Example using SSH:  
```git clone git@github.com:EllingV/poenews-discordbot.git```  

#### Step 2: Set up the DISCORD_TOKEN environment variable
Make sure the value of this variable is set to the token of your discord bot application.  
Example of setting the env in Windows PowerShell:  
```$env:DISCORD_TOKEN="myToken"```  

#### Step 3: Build the application
To build our Rust application we use Cargo:  
```cargo build --release```  
Note: The *--release* flag is optional, and omitting it will result in building the debug version [unoptimized + debuginfo] as opposed to the release version [optimized].

#### Step 4: Run the application
And to run our application, we also use Cargo::  
```cargo run --release```

You should see the line: *rustbot is connected!* once the application is up and running

## Usage
To use the bot once it's up and running, any user can write any of the commands listed under [Features](#Features) in a discord server with the discord bot application.  
Keep in mind that the discord bot application needs permission to use the text channel commands are posted in.

When writing a command in discord, it should look something like this:  
![poenews start image](https://cdn.discordapp.com/attachments/1212370898062016543/1280513577148747859/image.png?ex=66d85ab3&is=66d70933&hm=14a0963d61a9ccacafa268c881aadf8e46700ff192f9c0a478b332bc85a536d6&)

And that's really it!  
The bot is now going to send a message in the text channel(s) where ```!poenews start``` has been written, whenever a new news post is detected in the [Path of Exile rss-feed](https://www.pathofexile.com/news/rss).  
Here's an example of what that looks like:  
![news update example](https://cdn.discordapp.com/attachments/1212370898062016543/1280517600203505736/image.png?ex=66d85e72&is=66d70cf2&hm=cd1339bbc224990a88abac2179cd418b752dd7daa159fc383d3af18df68e8a24&)   

## File structure
``` bash
poenews-discordbot/
├── src/
│   ├── cms.rs		# This is where commands and messages are stored
│   ├── main.rs		# Entry point and Event Handler for our application
│   ├── poenews.rs	# Commands and structs related to the path of exile rss feed
│   └── rsstestdata.rs	# (unused) dummy data used in testing
├── .gitignore
├── Cargo.toml	# The project manifest, containing data needed for compilation
└── Readme.md	# Information about the project
```

## External crates
- **[Serenity](https://crates.io/crates/serenity)** is a library we use for interacting with the Discord API.  
- **[Tokio](https://crates.io/crates/tokio)** is a library serving as a platform for writing asynchronous applications, and it's a required dependency for using [Serenity](https://crates.io/crates/serenity).
- **[Rss](https://crates.io/crates/rss)** is a library we use for parsing the Path of Exile rss feed
- **[Reqwest](https://crates.io/crates/reqwest)** is an HTTP client library, and we use it for sending get requests to the Path of Exile rss feed
- **[Regex](https://crates.io/crates/regex)** is an implementation of regular expression, and we use it for formatting the update messages sent by the discord bot
- **[Lazy_static](https://crates.io/crates/lazy_static)** is a macro for declaring lazily evaluated statics, and we use it for global variables used by asynchronous functions
- **[Chrono](https://crates.io/crates/chrono)** is a date and time library, and we use it for formatting the local server time

## Additional notes

#### News posts can be rare
Besides "Weekend sales" that occur every week, news posts typically come in the time surrounding "League Starts".  
So do keep in mind that even if you install the bot and start listening for news updates, it could take multiple days before one gets posted.

#### Issue regarding "teaser season"
The 2 week period leading up to a new league is refered to as "teaser season" by the community, as daily teasers regarding content/changes will be revealed until the league starts.

The issue regarding this, is that all the teasers are compiled in a single blog post that get continuously updated without being able to see these updates on the posts entry in the RSS-feed.  

As of right now, there isn't a consistent way to tell when new content is added to a post through the RSS feed, and so this bot doesn't notify about them either.

#### Unused files and commented out code
In ```src/poenews.rs``` there are two functions commented out, and all references to ```rsstestdata.rs``` are also commented out.

The two functions would load different "dummy feeds" saved in ```rsstestdata.rs```, letting me test and make changes regarding the full functionality of the project without needlessly sending requests to the real feed while waiting for updates.

While I know it's incorrect to keep unused functionality in the project, I wanted to show how I did my testing even though it's done poorly.

#### This was my first Rust project
While I did want to recieve news updates regarding Path of Exile through discord, the main reason this project exists is because I wanted to learn more about the Rust programming language.  

The code itself is probably sub-optimal at best, and I already know of multiple changes I would make if I redid the project, but in the end the project works and does what it's supposed to do.

I just wanted to put a disclaimer here in case someone wonders "why did he do x instead of y".
