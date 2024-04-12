use std::{fs::File};

use marc::{Records, Record};

fn main() {
  let input = File::open("tests/fixtures/marc_data.mrc").unwrap();
  let mut count = 0;

  for (i, record) in Records::new(input).enumerate() {
    let record = record.unwrap();
    match i {
      0 => assert_eq!(record.field(b"001")[0].get_data::<str>(), "020598225"),
      1 => assert_eq!(record.field(b"001")[0].get_data::<str>(), "021175603"),
      2 => assert_eq!(record.field(b"001")[0].get_data::<str>(), "021641563"),
      _ => println!("ID: {}", get_id(record)),
    }
    count += 1;
  }

  assert_eq!(count, 536);
}

fn get_id(record: Record) -> String {
  if record.field(b"001").is_empty() {
    String::from("No ID")
  } else {
    String::from(record.field(b"001")[0].get_data::<str>())
  }
}
