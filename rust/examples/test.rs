//test.rs
// 
// 测试在Rust侧生成钱包密钥对，转换成C侧的数据结构。
// 测试钱包在C侧调用接口存储和重新读出钱包密钥
//

use std::ffi::{CStr};
//use rustylib::gen::{CWallet};
use rustywallet::{CWallet, generate_cwallet, free_cwallet, save_wallet, fetch_cwallet};

fn main() {

    unsafe {
        let wallet: CWallet = generate_cwallet();
        println!("---- generated a wallet to be used on C-side ----");
        print_wallet(&wallet);

        println!("---- saving the wallet to wallet.json ----");
        save_wallet(&wallet);
        println!("---- saved! ----");

        println!("---- fetching the saved wallet to be exposed to C-side ----");
        let fetched = fetch_cwallet();
        print_wallet(&fetched);

        free_cwallet(wallet);  // 对应 generate_cwallet()
        free_cwallet(fetched); // 对应 fetch_wallet()
    }
}

unsafe fn print_wallet(wallet: &CWallet) {
    let a = CStr::from_ptr(wallet.public_addr);
    let pa = a.to_str().unwrap();
    println!("public address=> {}", pa);

    let pk = CStr::from_ptr(wallet.public_key);
    let ppk = pk.to_str().unwrap();
    println!("public key=> {}", ppk);

    let sk = CStr::from_ptr(wallet.private_key);
    let psk = sk.to_str().unwrap();
    println!("private key=> {}", psk);
}
