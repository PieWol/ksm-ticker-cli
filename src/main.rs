use serde::Deserialize;
use std::fmt;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Line {
    //last trade at
    a: String,
    //ask
    b: String,
    //bid
    c: String,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(last: {}, ask: {}, bid: {})", self.a, self.b, self.c)
    }
}

fn main() {
    loop {
        println!("{}", get_line().unwrap());
        std::thread::sleep(Duration::from_millis(1000))
    }
    // println!("{}", get_line().unwrap());

    fn get_line() -> Option<Line> {
        let body: Result<String, reqwest::Error> = call();
        match body {
            Ok(x) => {
                let l: serde_json::Value = serde_json::from_str(&x).unwrap();

                let ask = &l["result"]["KSMUSD"]["a"][0];
                let bid = &l["result"]["KSMUSD"]["b"][0];
                let last = &l["result"]["KSMUSD"]["c"][0];
                let line = Line {
                    a: last.to_string(),
                    b: ask.to_string(),
                    c: bid.to_string(),
                };
                Some(line)
            }
            Err(_) => None,
        }
    }

    fn call() -> Result<String, reqwest::Error> {
        let body =
            reqwest::blocking::get("https://api.kraken.com/0/public/Ticker?pair=KSMUSD")?.text()?;
        Ok(body)
    }
}
