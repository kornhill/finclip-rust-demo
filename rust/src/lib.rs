// lib.rs
//
// 此部分代码主要负责Rust-C两侧的数据内存结构转换，提供了C侧的函数接口。注意命名规范：
// 在C侧使用时，凡是函数名带有 '_cwallet'的，调用过之后都必须用'free_cwallet'释放内存，
// 否则导致内存泄漏
// 

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod wallet;

use wallet_impl::Wallet;

use crate::wallet::*;

#[cfg(target_os = "android")]
mod android;


#[repr(C)]
pub struct CWallet {
    pub public_key: *mut c_char,
    pub private_key: *mut c_char,
    pub public_addr: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn generate_cwallet() -> CWallet {
    println!("generating wallet");
    let (secret_key, pub_key) = wallet_impl::generate_keypair();

    // println!("secret key: {}", &secret_key.to_string());
    // println!("public key: {}", &pub_key.to_string());

    //let pub_address = eth_wallet::public_key_address(&pub_key);
    //println!("public address: {:?}", pub_address);

    let rust_wallet = wallet_impl::Wallet::new(&secret_key, &pub_key);
    println!("rust_wallet: {:?}", &rust_wallet);
 
    convert_to_cwallet(rust_wallet)
}

#[no_mangle]
pub unsafe extern "C" fn free_cwallet(cw: CWallet) {
    drop(CString::from_raw(cw.public_key));
    drop(CString::from_raw(cw.private_key));
    drop(CString::from_raw(cw.public_addr));
}

#[no_mangle]
pub unsafe extern "C" fn save_wallet(cw: &CWallet) {
    let rwallet = convert_to_rwallet(cw);
    wallet_impl::Wallet::save_keys(&rwallet, "wallet.json").unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn fetch_cwallet() -> CWallet {
    match wallet_impl::Wallet::retrieve_keys("wallet.json") {
        Err(_) => {
            let wallet = wallet_impl::Wallet {
                secret_key: "".to_string(),
                public_key: "".to_string(),
                public_address: "".to_string(),
            };
            return convert_to_cwallet(wallet)
        }
        Ok(w) => return convert_to_cwallet(w)
    };
}

unsafe fn convert_to_cwallet(rwallet: Wallet) -> CWallet {
   // 转换Rust字符串数据为C的字符串并移交ownership
   let pubkey = CString::new(rwallet.public_key).unwrap();
   let c_pubkey: *mut c_char = pubkey.into_raw();
   let seckey = CString::new(rwallet.secret_key).unwrap();
   let c_seckey: *mut c_char = seckey.into_raw();
   let pubaddr = CString::new(rwallet.public_address).unwrap();
   let c_pubaddr: *mut c_char = pubaddr.into_raw();
   
   //println!("crypto wallet address: {}", CStr::from_ptr(c_pubaddr).to_str().unwrap());

   let cw = CWallet {
       public_key: c_pubkey,
       private_key: c_seckey,
       public_addr: c_pubaddr,
   };


   //println!("crypto_wallet addr: {}", CStr::from_ptr(cw.public_addr).to_str().unwrap());

   cw
}

unsafe fn convert_to_rwallet(cwallet: &CWallet) -> Wallet {

    let a = CStr::from_ptr(cwallet.public_addr);
    let pa = a.to_str().unwrap();

    let pk = CStr::from_ptr(cwallet.public_key);
    let ppk = pk.to_str().unwrap();

    let sk = CStr::from_ptr(cwallet.private_key);
    let psk = sk.to_str().unwrap();
    
    Wallet {
        secret_key: psk.to_string(),
        public_key: ppk.to_string(),
        public_address: pa.to_string(),
    }
}