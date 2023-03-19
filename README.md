# wordle


This is a CLI version of wordle. To run it you must have Rust installed. Ensure you have Rust installed with ```rustc --version```. If you don't, [follow the docs to install it](https://www.rust-lang.org/tools/install)


To clone and build the program run the following commands:
1. ```git clone https://github.com/c3pko/wordle.git```
2. ```cd wordle```
3. ```cargo build```
<br>

To run the program run the following commands from the wordle/src directory:
1. Run program with ```cargo run```
2. Run program without explainer text with ```cargo run no-help-text```
<br>  
<br>

About the game:

Welcome to the poor man's CLI version of wordle: where the words aren't random and the points are all made up!

You have 6 tries to guess the 5 letter word of the day. A valid guess consists of a 5 letter English word that is not a proper noun. You will get color coded feedback on each guess like so:
- A correctly guessed character will be printed as green
- A correctly guessed character in the wrong place will be printed as yellow
- All other characters will be printed as black

We recommend increasing the font size on your command line terminal to 18 for the best user experience.
