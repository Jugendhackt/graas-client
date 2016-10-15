#[macro_use(mime,__mime__ident_or_ext)] extern crate mime;
#[macro_use(object, array)] extern crate json;
extern crate hyper;
extern crate getopts;

use std::env;
use std::str::FromStr;
use hyper::client;
use hyper::header;
use std::io::Read;
use getopts::Options;

/// DataEntry contains all the data
/// that is measured by the Geiger-Müller
/// unit.
struct DataEntry {
  dosage:f64,
  timestamp:i64,
  lat:f64,
  long:f64
}

/// Returns a String that is used by the API to insert
/// a location. Needs latitude and longitude.
fn location_to_string(lat:f64, long:f64) -> String {
  format!("POINT({} {})", lat, long)
}

/// Generates a JSON object for a DataEntry
/// that can be used to insert a value using
/// the API.
fn to_json(data:DataEntry) -> json::JsonValue {
  let json = object!{
    "uSv" => data.dosage,
    "time" => data.timestamp,
    "location" => location_to_string(data.lat, data.long)
  };
  return json;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let progname = args[0].clone();

  let brief = format!("Usage: {} add --time <timestamp> --dosage <dosage> --lat <latitude> --long <longitude>", progname);

  let mut opts = Options::new();
  opts.optflag("h", "help", "print help menu");
  opts.optopt("", "time", "unix timestamp of the measurement", "TIMESTAMP");
  opts.optopt("", "dosage", "dosage in microsievert", "MICROSIEVERT");
  opts.optopt("", "lat", "latitude of the measurement", "LATITUDE");
  opts.optopt("", "long", "longitude of the measurement", "LONGITUDE");

  let matches = match opts.parse(args) {
    Ok(v) => v,
    Err(e) => panic!(e.to_string()),
  };

  if matches.opt_present("help") {
    print!("{}", opts.usage(&brief));
    return;
  }

  let dosage = match matches.opt_str("dosage") {
    Some(s) => match f64::from_str(&s) {
      Ok(f) => f,
      Err(e) => panic!("Couldn't parse \"{}\" as float: {}", s, e.to_string())
    },
    None => panic!("Option \"dosage\" with argument needed but not provided!")
  };
  let time = match matches.opt_str("time") {
    Some(s) => match i64::from_str(&s) {
      Ok(i) => i,
      Err(e) => panic!("Couldn't parse \"{}\" as integer: {}", s, e.to_string()),
    },
    None => panic!("Option \"time\" with argument needed but not provided!")
  };
  let lat = match matches.opt_str("lat") {
    Some(s) => match f64::from_str(&s) {
      Ok(f) => f,
      Err(e) => panic!("Couldn't \"{}\" as float: {}", s, e.to_string())
    },
    None => panic!("Option \"lat\" with argument needed but not provided!")
  };
  let long = match matches.opt_str("long") {
    Some(s) => match f64::from_str(&s) {
      Ok(f) => f,
      Err(e) => panic!("Couldn't \"{}\" as float: {}", s, e.to_string())
    },
    None => panic!("Option \"long\" with argument needed but not provided!")
  };

  let data = DataEntry {
    dosage : dosage,
    timestamp : time,
    lat: lat,
    long: long
  };


  let mut response_body = String::new();
  let mut headers = header::Headers::new();
  let client = client::Client::new();

  let json = json::stringify(to_json(data));

  headers.set(header::ContentType(mime!(Application/Json)));

  let mut res = client.post("http://tmp.pajowu.de/api/data/")
                  .headers(headers)
                  .body(&json)
                  .send().unwrap();

  println!("Data sent: {}", json);
  println!("Response Status: {}", res.status);

  let _ = res.read_to_string(&mut response_body);

  println!("Body: {}", response_body);
}
