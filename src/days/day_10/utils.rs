pub fn calculate_signal_strength(i: usize, r: i32) -> i32 {
  return match i {
      20 | 60 | 100 | 140 | 180 | 220 => i as i32 * r,
      _ => 0,
  };
}

pub fn draw_pixel(pixel: i32, sprite: i32) -> String {
  if sprite - 1 == pixel || sprite == pixel || sprite + 1 == pixel {
      return "#".to_string();
  }
  return ".".to_string();
}
