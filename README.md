# Timer
It's a timer. It's nice and simple.
<div style='position:relative; padding-bottom:calc(56.25% + 44px)'><iframe src='https://gfycat.com/ifr/OptimalEssentialBlowfish' frameborder='0' scrolling='no' width='100%' height='100%' style='position:absolute;top:0;left:0;' allowfullscreen></iframe></div>

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


