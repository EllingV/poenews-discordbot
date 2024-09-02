# Poenews-Discordbot
This is a simple bot written in rust that periodically fetches news from the path of exile news rss-feed, and post updates in relevant discord channels.  

It's intended to make it easier keeping up to date with news related to the game Path of Exile.

## Features
The discord bot comes with a few commands:  
### !help
The bot will respond by listing the available commands with a short description.  
Can be usefull for when you forget the available commands.
### !poenews
The bot will respond with more information about ```!poenews start``` and ```!poenews stop```.  
### !poenews start
The bot will start to post updates regarding the path of exile news feed in the current channel.  
There will also be a response from the bot informing about whether or not it's currently listening for news.
### !poenews stop
The bot will stop posting updates regarding the path of exile news feed in the current channel.  
There will also be a response from the bot informing about whether or not it's currently listening for news.