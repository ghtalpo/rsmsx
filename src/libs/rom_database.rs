use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::fs;

const XML_DATABASE: &str = "softwaredb.xml";

pub fn search_in_rom_database(rom_to_search: &str) -> Result<String, String> {
    let content = fs::read_to_string(XML_DATABASE).expect("something went wrong reading the file");
    let xml = content.as_str();
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut dict: HashMap<String, String> = HashMap::new();
    let mut is_in_rom = false;
    let mut is_in_megarom = false;
    let mut is_in_hash = false;
    let mut is_in_type = false;
    let mut d_hash = String::new();
    let mut d_type = String::new();
    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"rom" => is_in_rom = true,
                b"megarom" => is_in_megarom = true,
                b"hash" => is_in_hash = true,
                b"type" => is_in_type = true,
                _ => {}
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"dump" => {
                    dict.insert(
                        d_hash.clone(),
                        if d_type.is_empty() {
                            "NORMAL".to_string()
                        } else {
                            d_type.clone()
                        },
                    );
                    d_hash.clear();
                    d_type.clear();
                }
                b"rom" => is_in_rom = false,
                b"megarom" => is_in_megarom = false,
                b"hash" => is_in_hash = false,
                b"type" => is_in_type = false,
                _ => {}
            },
            Ok(Event::Text(e)) => {
                if is_in_rom && is_in_hash {
                    d_hash = e.unescape().unwrap().into_owned();
                }
                if is_in_megarom {
                    if is_in_hash {
                        d_hash = e.unescape().unwrap().into_owned();
                    } else if is_in_type {
                        d_type = e.unescape().unwrap().into_owned();
                    }
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    if dict.contains_key(rom_to_search) {
        return Ok(dict.get(rom_to_search).unwrap().clone());
    }
    Err("Not found".to_string())
}
