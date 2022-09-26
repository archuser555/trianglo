//Here, Im Using One Of The World Fatest Way's To Calculate Square Root...
pub fn sqrt(x: f32) -> f32 {
  if (x == 0.0) || (x == 1.0) {
    return x
  }
  else if x < 0.0 {
    return f32::MAX
  }
  
  let mut res: f32 = x * 0.25;
  
  loop {
    let pre_res: f32 = res;
    res = 0.5 * (res + (x / res));
    
    if pre_res == res { break }
  }
  res
}
