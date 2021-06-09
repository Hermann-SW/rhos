# rhos
Rust Pollard Rho implementation using ethnum 256bit crate, for benchmarking. 

In case Rust is not already installed:  

0) curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2) cd rhos

3) run, passing composite number (code will run indefinitely for a prime).  
1st example was run on Pi400 with 32bit Raspberry Pi OS (0.48 seconds).  
2nd example was run on Intel i7 64bit Ubuntu OS (5040 seconds or 1.4 hours).  

    $ cargo run --release 1329227995165945853261116920683298817  
        Updating crates.io index  
       Compiling libc v0.2.96  
       Compiling ethnum v1.0.3  
       Compiling time v0.1.44  
       Compiling fine_grained v0.1.2  
       Compiling rhos v0.1.0 (/home/pi/rhos)  
        Finished release [optimized] target(s) in 12.99s  
         Running `target/release/rhos 1329227995165945853261116920683298817`  
    [27081] 478027289ns  
    1329227995165945853261116920683298817: 2147483647 618970019642690137449562111  
    $

    $ cargo run --release 5316911983139663487003542222693990401  
        Finished release [optimized] target(s) in 0.00s  
         Running `target/release/rhos 5316911983139663487003542222693990401`  
    [2163790305] 5080385247327ns  
    5316911983139663487003542222693990401: 2305843009213693951 2305843009213693951  
    $
