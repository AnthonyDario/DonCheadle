extern crate gemini_client;
use crate::gemini_client::go;

#[test]
fn torture_one() {
    let response = go("gemini.circumlunar.space").unwrap();
    assert_eq!(response.header.code, 20);
}
