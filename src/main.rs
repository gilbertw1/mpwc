extern crate libc;
extern crate secstr;
extern crate byteorder;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate rpassword;
extern crate clap;
extern crate libsodium_sys as ffi;

use libc::size_t;

mod cli;
mod conf;

use std::io::Read;
use secstr::*;
use rusterpassword::*;

fn main() {
  let mut cli_app = cli::create_mpwc_cli_app();
  let matches = cli_app.clone().get_matches();
  let config = conf::create_config(&matches);

  let master_pass = if config.prompt {
    SecStr::from(rpassword::prompt_password_stdout("Master Password: ").unwrap())
  } else {
    let mut master_pass = String::new();
    std::io::stdin().read_to_string(&mut master_pass).expect("Failed to read master password from stdin.");
    SecStr::from(master_pass)
  };

  let template = match config.pass_type.as_str() {
    "max" => TEMPLATES_MAXIMUM,
    "long" => TEMPLATES_LONG,
    "medium" => TEMPLATES_MEDIUM,
    "short" => TEMPLATES_SHORT,
    "basic" => TEMPLATES_BASIC,
    "pin" => TEMPLATES_PIN,
    _ => panic!("Invalid password type: {}", config.pass_type)
  };

  sodiumoxide::init();
  let master_key = gen_master_key(master_pass.clone(), &config.name).unwrap();
  let site_seed = gen_site_seed(&master_key, &config.site, config.counter).unwrap();
  let password = gen_site_password(&site_seed, template);
  let identicon = create_identicon(&master_pass, &config.name);
  print!("[{}{}{}{}] ", identicon.left_arm, identicon.body, identicon.right_arm, identicon.accessory);
  print!("{}", String::from_utf8_lossy(password.unsecure()));
}

use std::io;
use std::mem::uninitialized;
use byteorder::{BigEndian, WriteBytesExt};

const SALT_PREFIX : &'static [u8] = b"com.lyndir.masterpassword";
const left_arms: &'static [&'static str] = &["╔", "╚", "╰", "═"];
const right_arms: &'static [&'static str] = &["╗", "╝", "╯", "═"];
const bodies: &'static [&'static str] = &["█", "░", "▒", "▓", "☺", "☻"];
const accessories: &'static [&'static str] = &[
  "◈", "◎", "◐", "◑", "◒", "◓", "☀", "☁", "☂", "☃", "", "★", "☆", "☎", "☏", "⎈", "⌂", "☘", "☢", "☣",
  "☕", "⌚", "⌛", "⏰", "⚡", "⛄", "⛅", "☔", "♔", "♕", "♖", "♗", "♘", "♙", "♚", "♛", "♜", "♝", "♞", "♟",
  "♨", "♩", "♪", "♫", "⚐", "⚑", "⚔", "⚖", "⚙", "⚠", "⌘", "⏎", "✄", "✆", "✈", "✉", "✌"
];

pub fn gen_identicon_seed(master_pass: &SecStr, name: &str) -> io::Result<SecStr> {
  let mut dst = Vec::<u8>::with_capacity(32);
  if unsafe {
    let mut state = uninitialized::<ffi::crypto_auth_hmacsha256_state>();
    let mut ret = 0;
    ret += ffi::crypto_auth_hmacsha256_init(
      &mut state,
      master_pass.unsecure().as_ptr() as *const u8,
      master_pass.unsecure().len() as size_t);
    ret += ffi::crypto_auth_hmacsha256_update(
      &mut state,
      name.as_ptr(),
      name.len() as u64);
    ret += ffi::crypto_auth_hmacsha256_final(
      &mut state,
      dst.as_mut_ptr() as *mut [u8; 32]);
    ret
  } == 0 {
    unsafe { dst.set_len(32); }
    Ok(SecStr::new(dst))
  } else {
    Err(io::Error::new(io::ErrorKind::Other, "HMAC-SHA-256 failed"))
  }
}

fn create_identicon(master_pass: &SecStr, name: &str) -> Identicon {
  let seed = gen_identicon_seed(master_pass, name).expect("Failed to generate identicon seed");
  Identicon {
    left_arm: left_arms[seed.unsecure()[0] as usize % (left_arms.len())],
    body: bodies[seed.unsecure()[1] as usize % (bodies.len())],
    right_arm: right_arms[seed.unsecure()[2] as usize % (right_arms.len())],
    accessory: accessories[seed.unsecure()[3] as usize % (accessories.len())],
    color: (seed.unsecure()[4] % 7 + 1) as u8,
  }
}

struct Identicon {
  left_arm: &'static str,
  right_arm: &'static str,
  body: &'static str,
  accessory: &'static str,
  color: u8,
}
