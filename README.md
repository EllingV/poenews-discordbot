# Poenews-Discordbot
This is a simple bot written in rust that periodically fetches news from the path of exile news rss-feed, and post updates in relevant discord channels.  

It's intended to make it easier keeping up to date with news related to the game Path of Exile.

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
Note: The __--release__ flag is optional, and omitting it will result in building the debug version [unoptimized + debuginfo] as opposed to the release version [optimized].

#### Step 4: Run the application
And to run our application, we also use Cargo::  
```cargo run --release```

You should see the line: __rustbot is connected!__ once the application is up and running
