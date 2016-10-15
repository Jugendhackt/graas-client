#[macro_use(object, array)] extern crate json;
extern crate hyper;
#[macro_use(mime,__mime__ident_or_ext)] extern crate mime;
use std::env;
use std::process;
use std::str::FromStr;
use hyper::client;
use hyper::header;
use std::io::Read;

struct DataEntry {
  dosage:f64,
  timestamp:i64,
  lat:f64,
  long:f64
}

fn location_to_string(lat:f64, long:f64) -> String {
  format!("POINT({} {})", lat, long)
}
fn to_json(data:DataEntry) -> json::JsonValue {
  let json = object!{
    "uSv" => data.dosage,
    "time" => data.timestamp,
    "location" => location_to_string(data.lat, data.long)
  };
  return json;
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  if args.len() != 4 {
    process::exit(1);
  }
  let dosage = f64::from_str(&args[0]).unwrap();
  let timestamp = i64::from_str(&args[1]).unwrap();
  let lat = f64::from_str(&args[2]).unwrap();
  let long = f64::from_str(&args[3]).unwrap();
  let data = DataEntry {
    dosage : dosage,
    timestamp : timestamp,
    lat: lat,
    long: long
  };


  let json = json::stringify(to_json(data));

  let mut headers = header::Headers::new();
  headers.set(header::ContentType(mime!(Application/Json)));

  let mut response_body = String::new();

  let client = client::Client::new();
  let mut res = client.post("http://tmp.pajowu.de/api/data/")
                  .headers(headers)
                  .body(&json)
                  .send().unwrap();
  println!("Data sent: {}", json);
  println!("Response Status: {}", res.status);
  println!("Headers: {}", res.headers);

  let response_size = res.read_to_string(&mut response_body);

  println!("Body: {}", response_body);
}
