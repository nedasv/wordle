use reqwest::blocking;
use serde_json::Value;

pub fn random_word(len: u8) -> Result<String, ()> {
    let res = blocking::get(format!("https://random-word-api.herokuapp.com/word?length={}", len));

    if let Ok(res) = res {
        if res.status().is_success() {
            let text = res.text().unwrap();
            let v: Value = serde_json::from_str(text.as_str()).unwrap();
            let e = v.get(0).unwrap().to_string();

            return Ok(e)
        }
    }

   return Err(())
}