pub fn parse_command(cmd: &str) -> Option<(&str, &str)> {
    if cmd.as_bytes()[0] as char != '$' {
        return None;
    }

    Some(parse_content(cmd, 2))
}

pub fn parse_content(line: &str, start: usize) -> (&str, &str) {
    // If command doesn't have an argument, return empty string
    let line: Vec<&str> = line[start..].split(' ').collect();
    if line.len() == 1 {
        return (line[0], " ");
    }
    (line[0], line[1])
}
