// Basic module just so we can move commands and messages out of main

// --------------------------------------------
// ----------------- COMMANDS -----------------
// --------------------------------------------

/// Command for listing available commands
pub const HELP_COMMAND: &str = "!help";

/// Command for information on the `poe_news` commands
pub const POE_NEWS_COMMAND: &str = "!poenews";

/// Command for starting news listening in current channel
pub const POE_NEWS_START: &str = "!poenews start";

/// Command for stopping news listening in current channel
pub const POE_NEWS_STOP: &str = "!poenews stop";


// --------------------------------------------
// ----------------- MESSAGES -----------------
// --------------------------------------------

pub const HELP_MESSAGE: &str = "
Available bot commands:

!poenews        -   returns some more information on \"!poenews start/stop\" commands

!poenews start  -   bot starts listening for poe news updates in current channel

!poenews stop   -   bot stops listening for news in current channel
";

pub const POE_NEWS_MESSAGE: &str = "
To start litening for news updates, use write \"!poenews start\"

If you want to stop listening for news, write \"!poenews stop\" 

Regardless of which command you use, there will be a reply indicating the status of news listening in the channel
(Example: attempting to start listening when already listening will cause the bot to respond \"Already listening for news in this channel\")

Listening for news is also channel specific, so \"!poenews stop\" will only stop news listening in the channel it's written in
Conversely, this also means you can listen for news in multiple channels on the same server if you like
";
