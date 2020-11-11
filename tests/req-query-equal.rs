use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_not_map_when_invalid_key() {
    let server = mount("req/query/equal/string");
    let uri = format!("{}?not-age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_exact_string_query() {
    let server = mount("req/query/equal/string");
    let uri = format!("{}?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_when_incorrect_string_value() {
    let server = mount("req/query/equal/string");
    let uri = format!("{}?age=old", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_exact_int_query() {
    let server = mount("req/query/equal/int");
    let uri = format!("{}?age=42", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_when_incorrect_int_value() {
    let server = mount("req/query/equal/int");
    let uri = format!("{}?age=43", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_not_map_when_not_an_int_value() {
    let server = mount("req/query/equal/int");
    let uri = format!("{}?age=string", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_exact_bool_query() {
    let server = mount("req/query/equal/bool");
    let uri = format!("{}?age=true", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_when_incorrect_bool_value() {
    let server = mount("req/query/equal/bool");
    let uri = format!("{}?age=false", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_not_map_when_not_a_bool() {
    let server = mount("req/query/equal/bool");
    let uri = format!("{}?age=42", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}