pub fn format_source(source: &str) -> String {
    let mut output = String::new();
    let mut indent_level: usize = 0;
    
    let mut current_line = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut in_comment = false;
    
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if in_comment {
            if c == '\n' {
                in_comment = false;
                if !current_line.trim().is_empty() {
                    output.push_str(&format_line(&current_line, indent_level));
                    current_line.clear();
                }
            } else {
                current_line.push(c);
            }
        } else if in_string {
            current_line.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
        } else {
            match c {
                '"' => {
                    in_string = true;
                    current_line.push(c);
                }
                '/' => {
                    // Check for // comment
                    if i + 1 < chars.len() && chars[i+1] == '/' {
                        in_comment = true;
                        current_line.push(c);
                        current_line.push(chars[i+1]);
                        i += 1; // Skip next /
                    } else {
                        current_line.push(c);
                    }
                }
                '{' => {
                    current_line.push(c);
                    output.push_str(&format_line(&current_line, indent_level));
                    current_line.clear();
                    indent_level += 1; 
                }
                '}' => {
                    if !current_line.trim().is_empty() {
                         output.push_str(&format_line(&current_line, indent_level));
                         current_line.clear();
                    }
                    indent_level = indent_level.saturating_sub(1);
                    current_line.push(c);
                    output.push_str(&format_line(&current_line, indent_level));
                    current_line.clear();
                }
                ';' => {
                    current_line.push(c);
                    output.push_str(&format_line(&current_line, indent_level));
                    current_line.clear();
                }
                '\n' => {
                    if !current_line.trim().is_empty() {
                        output.push_str(&format_line(&current_line, indent_level));
                        current_line.clear();
                    }
                }
                _ => {
                    current_line.push(c);
                }
            }
        }
        i += 1;
    }
    
    if !current_line.trim().is_empty() {
        output.push_str(&format_line(&current_line, indent_level));
    }
    
    // Remove trailing newline if it wasn't there? Or ensure one?
    // Usually ensure one.
    if !output.ends_with('\n') {
        output.push('\n');
    }
    
    output
}

fn format_line(line: &str, indent_level: usize) -> String {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    
    // Spacing fixes
    let mut formatted = trimmed.to_string();
    formatted = formatted.replace("if(", "if (");
    formatted = formatted.replace("){", ") {");
    formatted = formatted.replace("else{", "else {");
    
    // Indent
    let indent = "    ".repeat(indent_level);
    format!("{}{}\n", indent, formatted)
}
