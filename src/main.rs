use std::process::Command;
extern crate redis;

use redis::{Commands};
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params, Error};
use jsonrpc_http_server::{ServerBuilder};

fn parse_arguments (p: Params) -> Result<Vec<String>, Error> {
  let mut result = Vec::new();
  match p {
    Params::Array(array) => {
      for s in &array {
        match s {
          Value::String(s) => result.push(s.clone()),
          _ => return Err(Error::invalid_params("expecting strings"))
        }
      }
    }
    _ => return Err(Error::invalid_params("expecting an array of strings"))
  }
  if result.len() < 1 {
    return Err(Error::invalid_params("missing api key"));
  }

  return Ok(result[0..].to_vec());
}

fn get_bg_pls() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("bg_pls")
}

fn get_js_bgpls() -> redis::RedisResult<String> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("js_bgpls")
}


fn get_bg_pls_by_id(id: &str) -> redis::RedisResult<String> {
  let mut echo_hello = Command::new("sh");
  echo_hello.arg("-c").arg("/home/uid0001/scripts/get_bgpls_id_alias_by_order_id.sh ".to_owned() + id).status()?;

  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.get("bgpls".to_owned()+id)
}

fn set_bg_pls(create_bg_pls: &str) -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  let _ : () = con.set("newbgpls", create_bg_pls)?;

  let mut echo_hello = Command::new("sh");
  echo_hello.arg("-c").arg("/home/uid0001/scripts/wget_newbgpls.sh").status()?;

  con.get("newbgpls")
}

fn set_first_run() -> redis::RedisResult<isize> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  //let _ : () = con.set("all_bg_pls", "")?;
  let _ : () = con.set("js_bgpls", "
  var $table = $('<table>').attr('id', 'bgpls');
  $table.append('<thead>').children('thead').append('<tr />').children('tr').append('<th>#</th><th>Дата создания</th><th>Описание</th>');
  ")?;

  //  var $tbody = $table.append('<tbody />').children('tbody');
  //   $tbody.append('<tr />').children('tr:last').append('<td>val</td>').append('<td>val</td>').append('<td>val</td>').append('<td>val</td>');
  //   $table.appendTo('body');
  con.get("bg_pls")
}

fn main() {
  let mut io = IoHandler::new();

  let _ = set_first_run();

  io.add_method("set_bg_pls",  move |params: Params| {
    let json = parse_arguments(params)?;
    let _ = set_bg_pls(&json[0]);

    Ok(Value::String("Ok".to_string()))
  });

  io.add_method("get_bg_pls_by_id",  move |params: Params| {
    let id = parse_arguments(params)?;
    let bg_pls = get_bg_pls_by_id(&id[0]).unwrap();

    Ok(Value::String(bg_pls))
  });

  io.add_method("get_bg_pls",  | _params | {
    let text = get_bg_pls().unwrap();

    Ok(Value::String(text))
  });

  io.add_method("get_js_bgpls",  | _params | {
    let text = get_js_bgpls().unwrap();

    Ok(Value::String(text))
  });

  let server = ServerBuilder::new(io)
    .threads(3)
    .start_http(&"127.0.0.1:3032".parse().unwrap())
    .unwrap();

  server.wait();
}

