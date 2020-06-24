# roller_bot
A simple rust discord bot to roll dice used with serenity https://github.com/serenity-rs/serenity
used https://medium.com/better-programming/writing-a-discord-bot-in-rust-2d0e50869f64 as a guide to get going.



Currently the bot works by looking for the "~" sign in discord chat.

Current commands implemented
  ~roll - rolls dice that you input
  
    notes
      will currently only let you roll one set of die at a time eg: cannot do ~roll 1D20 2D7
      additions/subtrations cannot have a whitespace between number eg: do not do - 7
      
    args
      -a : instead of printing each roll as its own line, will add them all together and print out total alongside average
           this will be done automatically without -a if the number of dice goes over 128.
           
  ~ping - bot will respond with pong. basically used to check if the bot is alive.
  
  ~help - not useful yet. 

  example ~roll commands:
    ~roll 1D20  -> will roll one D20.
    ~roll 1D20 -7  -> will roll one D20 and also display rol lresult with a -7.
    ~roll 20D7 +2  -> will roll twenty D7's each with its roll result and a total added with 2.
    ~roll 1D20 -7 +2 -> will roll one D20 and add the result with -5.
    ~roll 100D100 -a -> will show a line with the total added result of all rolls and another line showing the average.
  
  
  
TODO
  features
    fix the help command
    NAT MIN/MAx announcement
    
  tech debt
    async
    config (RON, JSON, etc)
    unit testing
    CI
    auto release
    Commenting/Wiki
   
    
    
