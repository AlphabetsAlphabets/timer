# Timer
It's a timer. It's nice and simple.  
<img src="https://media.giphy.com/media/3XV4H8N5vVSHO6fEit/giphy.gif" width="300">

# Installation
The pre-requisite is having rust, and rustup installed. Go lookup how
to install it.

```
git clone https://github.com/YJH16120/timer
cd timer
cargo build --release
sudo mv target/release/timer /usr/local/bin
```

# Usage
`timer <message> <time>`  
`message`: A string, used to display a simple message.  
`time`: How long you want the timer to go for. `hours:minutes:seconds`. 
Separate it with with colons.


