pub fn first_or_last(index: usize, line: usize) -> bool {
    index == 0 || index == line - 1
}
