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

const APIURL : &'static str = "http://tmp.pajowu.de/api/data/";

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

/// Adds data to the dataset via the API
fn insert_data(data:DataEntry) {
  let mut response_body = String::new();
  let mut headers = header::Headers::new();
  let client = client::Client::new();

  let json = json::stringify(to_json(data));

  headers.set(header::ContentType(mime!(Application/Json)));

  let mut res = client.post(APIURL)
                  .headers(headers)
                  .body(&json)
                  .send().unwrap();

  println!("Data sent: {}", json);
  println!("Response Status: {}", res.status);

  let _ = res.read_to_string(&mut response_body);

  println!("Body: {}", response_body);
}

fn query_data() {
  let client = client::Client::new();

  let mut response_body = String::new();
  let mut res = client.get(APIURL).send().unwrap();

  let _ = res.read_to_string(&mut response_body);

  let measurements = match json::parse(response_body.as_str()) {
    Err(e) => panic!(e.to_string()),
    Ok(j) => j
  };

  if !measurements.is_array() {
    panic!("Unexpected data format!");
  }


  for m in measurements.members() {
    println!("Dosage: {} Time: {} Lat: {} Long: {}", 
             m["uSv"],
             m["time"],
             m["location"]["coordinates"][0],
             m["location"]["coordinates"][1]);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let progname = args[0].clone();

  let brief = format!("Usage: {} add --time <timestamp> --dosage <dosage> --lat <latitude> --long <longitude>
     | {} query", progname, progname);

  let command = args[1].clone();

  match command.as_str() {
    "add" => {
      let mut opts = Options::new();
      opts.optopt("", "time", "unix timestamp of the measurement", "TIMESTAMP");
      opts.optopt("", "dosage", "dosage in microsievert", "MICROSIEVERT");
      opts.optopt("", "lat", "latitude of the measurement", "LATITUDE");
      opts.optopt("", "long", "longitude of the measurement", "LONGITUDE");

      let matches = match opts.parse(args) {
        Ok(v) => v,
        Err(e) => panic!(e.to_string()),
      };

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

      insert_data(data);
    },
    "query" => query_data(),
    _ => print!("{}", brief)
  };
}
