# Timer
It's a timer. It's nice and simple.  
<img src="https://media.giphy.com/media/EIXRXCW9azCxYRV2xS/giphy.gif">  
**NOTE**: The background colour shifting is due to my configs. 

# Installation
The pre-requisite is having rust, and rustup installed. Go lookup how
to install it.

If you prefer to build through source:
```
git clone https://github.com/YJH16120/timer
cd timer
cargo build --release
sudo mv target/release/timer /usr/local/bin
```
If you do not want to install rust, and rustup you can download the precompiled
binary from the [releases](https://github.com/YJH16120/timer/releases/tag/1.0.0) page.

# Usage
`timer <message> <time>`  
`message`: A string, used to display a simple message.  
`time`: How long you want the timer to go for. `hours:minutes:seconds`. 
Separate it with with colons.


