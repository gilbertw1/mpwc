use clap::ArgMatches;

pub fn create_config(matches: &ArgMatches) -> MpwcConfig {
  MpwcConfig {
    stdin: matches.is_present("stdin"),
    quiet: matches.is_present("quite"),
    name: get_string_value(matches, "name").unwrap(),
    site: get_string_value(matches, "site").unwrap(),
    counter: get_int_value(matches, "counter").unwrap(),
    pass_type: get_string_value(matches, "type").unwrap()
  }
}

fn get_string_value(matches: &ArgMatches, key: &str) -> Option<String> {
  matches.value_of(key).map(|m| m.to_string())
}

fn get_int_value(matches: &ArgMatches, key: &str) -> Option<u32> {
  matches.value_of(key).map(|m| m.parse::<u32>().unwrap())
}

#[derive(Debug)]
pub struct MpwcConfig {
  pub quiet: bool,
  pub stdin: bool,
  pub name: String,
  pub site: String,
  pub counter: u32,
  pub pass_type: String,
}
