use std::{io::Read, path::Path};
use bytes::Bytes;
use multer::Multipart;
use yarte::Serialize;
use futures::stream;
use std::convert::Infallible;
use may_minihttp::{Request, Response};
use crate::{constants::http::{self}, types::{item::TItemSubmissionData, message::TMessageOk}};

/**
# NOTE:
* this controller doesn't check if file already exists
* some e from Err is not processing correctly
* `callbackData` data meant to be:
    - use multiple times as a message
    - alternative when my parsing message
    - otherwise use something else that suite the case

### example to test with curl
```sh
curl -X POST \
  -F "data={\"items\":[{\"item_name\":\"Test\",\"item_image\":\"test.png\",\"quantity\":42}]}" \
  -F "test.png=@//path/to/test.png" \
  http://localhost:8080/api/submit/item
```
*/
#[allow(non_snake_case)]
pub fn item(req: Request, resp: &mut Response) {
    resp.header(http::content_type::CT_APPLICATION_JSON);

    let mut callbackData = TMessageOk {
        ok: false,
        message: "n/a"
    };

    // forcing one method
    if req.method() != http::method::POST {
        resp.status_code(405, "not allowed");
        return;
    }

    // extract content-type & boundry
    let (boundary, has_multipart) = req.headers().iter()
        .find(|h| h.name.eq_ignore_ascii_case("Content-Type"))
        .map(|ct| {
            let content_type = String::from_utf8_lossy(ct.value);
            let is_multipart = content_type.starts_with("multipart/form-data");
            
            let boundary = ct.value.split(|&b| b == b'=')
                .nth(1)
                .map(|b| {
                    String::from_utf8_lossy(b)
                        .trim_matches(|c| c == '"' || c == ' ')
                        .to_string()
                })
                .unwrap_or_default();
                
            (boundary, is_multipart)
        })
        .unwrap_or_default();
    if !has_multipart || boundary.is_empty() {
        resp.status_code(400, "Bad Request")
            .body(r#"{"error":"Missing or invalid Content-Type"}"#);
        return;
    }

    // parse multipart data
    let body = match {
        let mut body_data = Vec::new();
        req.body().read_to_end(&mut body_data)
            .map(|_| bytes::Bytes::from(body_data))
    } {
        Ok(b) => b,
        Err(e) => {
            eprintln!("ERROR: parsing data \"{e}\"");
            resp.status_code(500, "Internal Server Error")
                .body(r#"{"error":"Body read failed"}"#);
            return;
        }
    };
    let body_stream = stream::iter(vec![Ok::<Bytes, Infallible>(body)]);
    let mut multipart = Multipart::new(body_stream, &boundary);

    // process each part of the multipart data & process the json
    let mut items = Vec::new();
    let mut files = Vec::new();
    loop {
        let field = futures::executor::block_on(multipart.next_field());
        match field {
            Ok(Some(field)) => {
                let field_name = field.name().unwrap_or_default().to_string();

                match field_name.as_str() {
                    "data" => {
                        // process json data
                        let data = futures::executor::block_on(field.bytes());
                        match data {
                            Ok(bytes) => {
                                match serde_json::from_slice::<TItemSubmissionData>(&bytes) {
                                    Ok(req_data) => items = req_data.items,
                                    Err(_) => {
                                        resp.status_code(400, "Bad Request")
                                            .body(r#"{{"error":"Parse json error"}}"#);
                                        return;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("ERROR: data field \"{e}\"");
                                resp.status_code(400, "Bad Request")
                                    .body(r#"{{"error":"Data field error"}}"#);
                                return;
                            }
                        }
                    }
                    _ => {
                        // process file
                        let filename = field.file_name()
                            .map(|f| crate::functions::utility::sanitize::fileName(f))
                            .unwrap_or_default();

                        if filename.is_empty() {
                            resp.status_code(400, "Bad Request")
                                .body(r#"{"error":"Invalid filename"}"#);
                            return;
                        }

                        let data = futures::executor::block_on(field.bytes());
                        match data {
                            Ok(bytes) => files.push((filename, bytes.to_vec())),
                            Err(e) => {
                                eprintln!("ERROR: file read error \"{e}\"");
                                resp.status_code(500, "Internal Server Error")
                                    .body(r#"{{"error":"File read error"}}"#);
                                return;
                            }
                        }
                    }
                }
            }
            Ok(None) => break, // no more fields
            Err(e) => {
                eprintln!("ERROR: field processing error \"{e}\"");
                resp.status_code(400, "Bad Request")
                    .body(r#"{{"error":"Field processing error"}}"#);
                return;
            }
        }
    }

    // make it sure data exists?
    if items.is_empty() {
        resp.status_code(400, "Bad Request")
            .body(r#"{"error":"Missing data field"}"#);
        return;
    }

    // create pub dir, already created when server start
    // save data for each item and save those files
    for item in &items {
        let expected_file = crate::functions::utility::sanitize::fileName(&item.item_image);
        if expected_file.is_empty() {
            resp.status_code(400, "Bad Request")
                .body(r#"{"error":"Invalid image name"}"#);
            return;
        }

        if !files.iter().any(|(fname, _)| fname == &expected_file) {
            resp.status_code(400, "Bad Request")
                .body(r#"{{"error":"Missing file"}}"#);
            return;
        }
    }
    for (filename, data) in files {
        let path = Path::new("../../public/upload").join(&filename);
        if let Err(e) = std::fs::write(&path, &data) {
            eprintln!("ERROR: failed to save file \"{e}\"");
            resp.status_code(500, "Internal Server Error")
                .body(r#"{{"error":"File save failed"}}"#);
            return;
        }
    }

    // double check
    if !callbackData.ok {
        resp.status_code(400, "bad request");

        callbackData.to_bytes_mut(resp.body_mut());
        return;
    }

    // finally, it's clear
    callbackData.ok = true;
    callbackData.message = "ok";

    resp.status_code(200, "ok");
    callbackData.to_bytes_mut(resp.body_mut());
}