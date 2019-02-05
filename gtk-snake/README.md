# GTK-SNAKE

## Requirements
* [Gtk-rs: Requirements](http://gtk-rs.org/docs/requirements.html)  
If you are under windows operate system, you would have to execute complex commands to build GTK environment. I cannot build successfully under windows environment. The following is the error message `error: linking with link.exe failed: exit code: 1181`.  
  
Anyway, I recommend execute under linux environment.  

## GTK Example Project
* [dansnow/rust-intro - gtk-tic-tac-toe](https://github.com/DanSnow/rust-intro/tree/master/gtk-tic-tac-toe/src) ([article](https://ithelp.ithome.com.tw/articles/10206443))  
A gtk-tic-tac-toe game  

* [sprang/marmoset](https://github.com/sprang/marmoset)  
A card game  

* [kirjavascript/snake-rs](https://github.com/kirjavascript/snake-rs)  
A snake game and you can play on his website  

* [dhad05/GoL](https://github.com/dhad05/GoL)  
Conway's Game of Life in Rust. I learn how to use `Arc` to pass variable into the closure here.

* [gtx-rs/examples](https://github.com/gtk-rs/examples)  
Official examples  

* [mmstick/gtkrs-tutorials](https://github.com/mmstick/gtkrs-tutorials)  
Good programming structure for gtk

## Useful Command
```shell
$ GTK_DEBUG=interactive cargo run
```

## Programming Issue
* What is the better way to clone into closures?  

Reference
* [Clone into closures · Issue #2407 · rust-lang/rfcs](https://github.com/rust-lang/rfcs/issues/2407)  
* [How to move data into multiple Rust closures?](https://stackoverflow.com/questions/52464653/how-to-move-data-into-multiple-rust-closures)  

Three solutions:  
1. Rc, RefCell  
```
```

2. Macro clone  
```
```

3. Arc, Mutex  
```
```