fn parse_http(buffer: &[u8]) -> Result<u64, String> {
  let request = String::from_utf8_lossy(buffer);
  let first_line = request.lines().next().ok_or("Empty request")?;
  let words: Vec<&str> = first_line.split_whitespace().collect();

  if words.len() < 3 {
    return Err(String::from("Invalid request format"));
  }

  let uri = String::from(words[1]);
  let i_param = uri.strip_prefix("/pi/").ok_or(format!("Invalid route: {}", uri))?;

  let i = i_param.parse::<u64>().map_err(|_| format!("Invalid value for i: {}", i_param))?;

  Ok(i)
}

pub fn get_i_value(buffer: &[u8]) -> Result<u64, String> {
  let output = parse_http(buffer);
  match output {
    Ok(data) => { Ok(data) }
    Err(e) => { Err(e) }
  }
}