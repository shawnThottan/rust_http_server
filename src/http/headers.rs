use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.trim().split("\r\n") {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find(": ") {
                key = &sub_str[..i];
                val = &sub_str[i+2..];
            }

            if key.len() == 0 || val.len() == 0 {
                continue;
            }

            data.entry(key)
                .and_modify(|e| *e = val)
                .or_insert(val);
        }

        Headers { data }
    }
}
