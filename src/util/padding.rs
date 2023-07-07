pub fn padding(name_size: usize) -> usize {
	
	let size_ctime_to_namesize = 38;

  let remainder = (name_size + size_ctime_to_namesize) % 8;
  let padding = 8 - remainder;
  return padding;
}