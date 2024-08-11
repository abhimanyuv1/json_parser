// Tutorial link https://www.freecodecamp.org/news/rust-tutorial-build-a-json-parser/

use std::fs::File;

use parser::JsonParser;

mod parser;
mod reader;
mod token;
mod value;

fn main() {
    let file = File::open("test.json").unwrap();
    let parser = JsonParser::parse(file).unwrap();

    dbg!(parser);

    let input_json_string = r#"
    {
        "pairs": [
            {
                "x0": 95.26235434764715,
                "y0": -33.78221816487377,
                "x1": 41.844453001935875,
                "y1": -78.10213222087448
            },
            {
                "x0": 115.42029308864215,
                "y0": 1.20021873e-5,
                "x1": 83.39640643072113,
                "y1": 28.643090267505812
            },
            {
                "sample": "string sample",
                "nullable": null,
                "isWorking": true,
                "isNotWorking": false
            }
        ],
        "utf8": {
            "key1": "ࠄࠀࠆࠄࠀࠁࠃ",
            "key2": "value2"
        }
    }
    "#;

    let byte_parser = JsonParser::parse_from_bytes(input_json_string.as_bytes()).unwrap();
    dbg!(byte_parser);
}
