extern crate secstr;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate rpassword;
extern crate clap;

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
