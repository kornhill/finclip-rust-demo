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
use qrcodegen::*;
use base64::{encode};

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

#[no_mangle]
pub unsafe extern "C" fn generate_qrcode_svg(input: *const c_char) -> *const c_char {
    let s = CStr::from_ptr(input);
    let sp = s.to_str().unwrap();

    let qr = QrCode::encode_text(sp, QrCodeEcc::Medium).unwrap();
    let svg = to_svg_string_base64(&qr, 4); 

    let result = CString::new(svg).unwrap();
    let c_result: *mut c_char = result.into_raw();

    c_result
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

fn to_svg_string_base64(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    //result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    //result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr.size().checked_add(border.checked_mul(2).unwrap()).unwrap();
    result += &format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0 .. qr.size() {
            for x in 0 .. qr.size() {
                    if qr.get_module(x, y) {
                            if x != 0 || y != 0 {
                                    result += " ";
                            }
                            result += &format!("M{},{}h1v1h-1z", x + border, y + border);
                    }
            }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    encode(result)
}