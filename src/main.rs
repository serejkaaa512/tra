extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate clap;
extern crate reqwest;


#[derive(Deserialize)]
struct Resp {
    code: i32,
    lang: String,
    text: Vec<String>,
}

const KEY: &str = "PUT YOUR KEY HERE!!!!";

fn main() {
    let matches = clap_app!(tra =>
        (version: "1.0")
        (author: "serejkaaa512 <5125402@gmail.com>")
        (about: "Simple text translator. It uses Yandex API.")
        (@arg lang: -l --lang +takes_value "Sets a language pair: en-ru, ru-es, ..")
        (@arg INPUT: +required "Text to translate.")
    ).get_matches();
    let lang = matches.value_of("lang").unwrap_or("en-ru");
    let input = matches.value_of("INPUT").unwrap();
    let client = reqwest::Client::new();
    let params = [("key", KEY), ("text", input), ("lang", lang)];
    let mut res = client
        .post("https://translate.yandex.net/api/v1.5/tr.json/translate")
        .form(&params)
        .send()
        .unwrap();
    let content: Resp = res.json().unwrap();

    for s in content.text {
        println!("{}", s);
    }
}
