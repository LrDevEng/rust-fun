pub fn fibonacci (n: u64) -> u64 {

  if n == 1 || n == 2 {
    return 1;
  }

  let mut parent = 1 ;
  let mut grandparent = 1;
  let mut current = 0;

  let mut i = 3;
  while i <= n {
    current = parent + grandparent;
    grandparent = parent;
    parent = current;
    i += 1;
  }

  return current;
}
