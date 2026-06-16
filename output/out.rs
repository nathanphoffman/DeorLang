fn chars_of(source: String) -> Vec<String> {
    source.chars().map(|c| c.to_string()).collect()
}

fn str_from(source: String, start: i32) -> String {
    source.get(start as usize..).unwrap_or_default().to_string()
}

fn rtrim(source: String) -> String {
    source.trim_end().to_string()
}

fn s_cat(left: String, right: String) -> String {
    left + right.as_str()
}

fn s_join(parts: Vec<String>) -> String {
    parts.join("")
}

fn s_join_nl(parts: Vec<String>) -> String {
    parts.join("\n")
}

fn s_join_with(parts: Vec<String>, sep: String) -> String {
    parts.join(sep.as_str())
}

fn read_file(path: String) -> String {
    std::fs::read_to_string(path.as_str()).expect("cannot read input file")
}

fn write_file(path: String, content: String) {
    std::fs::write(path.as_str(), content.as_str()).expect("cannot write output file");
}

fn get_args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

fn is_alpha(character: String) -> bool {
    character.chars().next().map(|ch| ch.is_alphabetic() || ch == '_').unwrap_or(false)
}

fn is_digit(character: String) -> bool {
    character.chars().next().map(|ch| ch.is_ascii_digit()).unwrap_or(false)
}

fn is_alnum(character: String) -> bool {
    character.chars().next().map(|ch| ch.is_alphanumeric() || ch == '_').unwrap_or(false)
}

fn str_to_int(source: String) -> i32 {
    source.parse::<i32>().unwrap_or(0)
}

fn int_to_str(number: i32) -> String {
    number.to_string()
}

fn str_repeat(source: String, count: i32) -> String {
    source.repeat(count as usize)
}

fn pascal_case(source: String) -> String {
    {
    	let mut chars = source.chars();
    	match chars.next() {
    		None => String::new(),
    		Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    	}
    }
}

fn to_debug_str(source: String) -> String {
    format!("{:?}", source)
}

fn token_slice(tokens: Vec<Token>, start: i32, end_val: i32) -> Vec<Token> {
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
}

fn pr_code(result: ParseResult) -> String {
    let code = result.code.clone();
    let new_pos = result.new_pos.clone();
    return code;
}

fn pr_pos(result: ParseResult) -> i32 {
    let code = result.code.clone();
    let new_pos = result.new_pos.clone();
    return new_pos;
}

fn make_result(code: String, new_pos: i32) -> ParseResult {
    let result = ParseResult { code, new_pos };
    return result;
}

fn make_token(kind: String, value: String, line: i32) -> Token {
    let token = Token { kind, value, line };
    return token;
}

fn tok_kind(token: Token) -> String {
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    return kind;
}

fn tok_value(token: Token) -> String {
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    return value;
}

type TokenList = Vec<Token>;

type StrList = Vec<String>;

#[derive(Clone, PartialEq, Debug)]
struct Token {
    kind: String,
    value: String,
    line: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct ParseResult {
    code: String,
    new_pos: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct GenCtx {
    variant_reg: strList,
    shape_reg: strList,
    struct_reg: strList,
    enum_reg: strList,
    mut_names: strList,
    type_reg: strList,
}

fn is_empty(source: String) -> bool {
    let mut length: i32 = source.len() as i32;
    return length == 0;
}

fn str_eq(left: String, right: String) -> bool {
    return left == right;
}

fn reg_get_stride(pairs: Vec<String>, key: String, stride: i32) -> String {
    let mut pairs_count: i32 = pairs.len() as i32;
    let mut index: i32 = 0;
    while index < pairs_count {
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            let mut val_index: i32 = index + 1.clone();
            return pairs[val_index as usize].clone();
        }
        index = index + stride;
    }
    return "".to_string();
}

fn reg_has_stride(pairs: Vec<String>, key: String, stride: i32) -> bool {
    let mut pairs_count: i32 = pairs.len() as i32;
    let mut index: i32 = 0;
    while index < pairs_count {
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            return true;
        }
        index = index + stride;
    }
    return false;
}

fn reg_get(pairs: Vec<String>, key: String) -> String {
    return reg_get_stride(pairs.clone(), key.clone(), 2);
}

fn reg_has(pairs: Vec<String>, key: String) -> bool {
    return reg_has_stride(pairs.clone(), key.clone(), 2);
}

fn reg3_get(pairs: Vec<String>, key: String) -> String {
    return reg_get_stride(pairs.clone(), key.clone(), 3);
}

fn reg3_has(pairs: Vec<String>, key: String) -> bool {
    return reg_has_stride(pairs.clone(), key.clone(), 3);
}

fn list_has(items: Vec<String>, val: String) -> bool {
    let mut item_count: i32 = items.len() as i32;
    for index in 0..item_count {
        let mut item: String = items[index as usize].clone();
        if item == val {
            return true;
        }
    }
    return false;
}

fn word_to_kind(word: String) -> String {
    if word == "fn" {
        return "KW_FN".to_string();
    }
    if word == "as" {
        return "KW_AS".to_string();
    }
    if word == "return" {
        return "KW_RETURN".to_string();
    }
    if word == "if" {
        return "KW_IF".to_string();
    }
    if word == "else" {
        return "KW_ELSE".to_string();
    }
    if word == "for" {
        return "KW_FOR".to_string();
    }
    if word == "in" {
        return "KW_IN".to_string();
    }
    if word == "break" {
        return "KW_BREAK".to_string();
    }
    if word == "continue" {
        return "KW_CONTINUE".to_string();
    }
    if word == "and" {
        return "KW_AND".to_string();
    }
    if word == "or" {
        return "KW_OR".to_string();
    }
    if word == "not" {
        return "KW_NOT".to_string();
    }
    if word == "is" {
        return "KW_IS".to_string();
    }
    if word == "true" {
        return "KW_TRUE".to_string();
    }
    if word == "false" {
        return "KW_FALSE".to_string();
    }
    if word == "none" {
        return "KW_NONE".to_string();
    }
    if word == "avow" {
        return "KW_AVOW".to_string();
    }
    if word == "empty" {
        return "KW_EMPTY".to_string();
    }
    if word == "type" {
        return "KW_TYPE".to_string();
    }
    if word == "struct" {
        return "KW_STRUCT".to_string();
    }
    if word == "shape" {
        return "KW_SHAPE".to_string();
    }
    if word == "list" {
        return "KW_LIST".to_string();
    }
    if word == "of" {
        return "KW_OF".to_string();
    }
    if word == "insert" {
        return "KW_INSERT".to_string();
    }
    if word == "enum" {
        return "KW_ENUM".to_string();
    }
    if word == "at" {
        return "KW_AT".to_string();
    }
    if word == "remove" {
        return "KW_REMOVE".to_string();
    }
    if word == "rust" {
        return "KW_RUST".to_string();
    }
    if word == "void" {
        return "KW_VOID".to_string();
    }
    if word == "using" {
        return "KW_USING".to_string();
    }
    return "IDENT".to_string();
}

fn is_binary_op(kind: String) -> bool {
    if kind == "PLUS" {
        return true;
    }
    if kind == "MINUS" {
        return true;
    }
    if kind == "STAR" {
        return true;
    }
    if kind == "SLASH" {
        return true;
    }
    if kind == "PERCENT" {
        return true;
    }
    if kind == "GT" {
        return true;
    }
    if kind == "LT" {
        return true;
    }
    if kind == "GTE" {
        return true;
    }
    if kind == "LTE" {
        return true;
    }
    if kind == "KW_IS" {
        return true;
    }
    if kind == "KW_AND" {
        return true;
    }
    if kind == "KW_OR" {
        return true;
    }
    return false;
}

fn map_op(operator: String) -> String {
    if operator == "is" {
        return "==".to_string();
    }
    if operator == "is not" {
        return "!=".to_string();
    }
    if operator == "and" {
        return "&&".to_string();
    }
    if operator == "or" {
        return "||".to_string();
    }
    if operator == ">" {
        return ">".to_string();
    }
    if operator == "<" {
        return "<".to_string();
    }
    if operator == ">=" {
        return ">=".to_string();
    }
    if operator == "<=" {
        return "<=".to_string();
    }
    return operator;
}

fn render_rust_type(type_name: String) -> String {
    if type_name == "void" {
        return "".to_string();
    }
    if type_name == "int" {
        return "i32".to_string();
    }
    if type_name == "string" {
        return "String".to_string();
    }
    if type_name == "bool" {
        return "bool".to_string();
    }
    if type_name == "float" {
        return "f64".to_string();
    }
    return type_name;
}

fn count_tabs(line: String) -> i32 {
    let mut chars: Vec<String> = chars_of(line.clone());
    let mut char_count: i32 = chars.len() as i32;
    let mut count: i32 = 0;
    for index in 0..char_count {
        let mut character: String = chars[index as usize].clone();
        if character == "\t" {
            count = count + 1;
        } else {
            break;
        }
    }
    return count;
}

fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut lines: Vec<String> = source.split("\n").map(|s| s.to_string()).collect();
    let mut n_lines: i32 = lines.len() as i32;
    let mut indent_stack: Vec<String> = Vec::new();
    indent_stack.push("0".to_string());
    let mut cur_line: i32 = 0;
    let mut skip: i32 = 0;
    for raw_li in 0..n_lines {
        cur_line = cur_line + 1;
        if skip > 0 {
            skip = skip - 1;
            continue;
        }
        let mut raw: String = lines[raw_li as usize].clone();
        let mut line: String = rtrim(raw.clone());
        let mut stripped: String = line.trim().to_string();
        if is_empty(stripped.clone()) {
            continue;
        }
        let mut indent: i32 = count_tabs(line.clone());
        let mut content: String = str_from(line.clone(), indent.clone());
        let mut slen: i32 = indent_stack.len() as i32;
        let mut top_idx: i32 = slen - 1.clone();
        let mut top: i32 = str_to_int(indent_stack[top_idx as usize].clone());
        if indent > top {
            tokens.push(make_token("INDENT".to_string(), "".to_string(), cur_line.clone()).clone());
            indent_stack.push(int_to_str(indent.clone()).clone());
        } else {
            let mut dedenting: bool = indent < top.clone();
            while dedenting {
                let mut new_slen: i32 = indent_stack.len() as i32;
                let mut new_top_idx: i32 = new_slen - 1.clone();
                let mut cur_top: i32 = str_to_int(indent_stack[new_top_idx as usize].clone());
                if indent < cur_top {
                    tokens.push(make_token("DEDENT".to_string(), "".to_string(), cur_line.clone()).clone());
                    indent_stack.remove(new_top_idx as usize);
                } else {
                    dedenting = false;
                }
            }
        }
        if content == "rust" {
            tokens.push(make_token("KW_RUST".to_string(), "rust".to_string(), cur_line.clone()).clone());
            tokens.push(make_token("NEWLINE".to_string(), "".to_string(), cur_line.clone()).clone());
            let mut rust_base: i32 = indent + 1.clone();
            let mut rust_lines: Vec<String> = Vec::new();
            let mut rli_start: i32 = raw_li + 1.clone();
            for rli in rli_start..n_lines {
                let mut rust_line: String = lines[rli as usize].clone();
                let mut rl_indent: i32 = count_tabs(rust_line.clone());
                let mut rl_stripped: String = rust_line.trim().to_string();
                if is_empty(rl_stripped.clone()) {
                    rust_lines.push("".to_string());
                    skip = skip + 1;
                } else if rl_indent >= rust_base {
                    let mut rl_content: String = str_from(rust_line.clone(), rust_base.clone());
                    rust_lines.push(rtrim(rl_content.clone()).clone());
                    skip = skip + 1;
                } else {
                    break;
                }
            }
            let mut trimming: bool = true;
            while trimming {
                let mut rl_len: i32 = rust_lines.len() as i32;
                if rl_len > 0 {
                    let mut last_rl: i32 = rl_len - 1.clone();
                    let mut last_line: String = rust_lines[last_rl as usize].clone();
                    if last_line == "" {
                        rust_lines.remove(last_rl as usize);
                    } else {
                        trimming = false;
                    }
                } else {
                    trimming = false;
                }
            }
            tokens.push(make_token("RUST_BLOCK".to_string(), s_join_nl(rust_lines.clone()), cur_line.clone()).clone());
            continue;
        }
        let mut chars: Vec<String> = chars_of(content.clone());
        let mut char_count: i32 = chars.len() as i32;
        let mut char_index: i32 = 0;
        while char_index < char_count {
            let mut character: String = chars[char_index as usize].clone();
            if character == " " {
                char_index = char_index + 1;
                continue;
            }
            if character == "#" {
                break;
            }
            if character == "\"" {
                let mut val: String = "".to_string();
                let mut escape_next: bool = false;
                let mut str_start: i32 = char_index + 1.clone();
                char_index = char_index + 1;
                for string_index in str_start..char_count {
                    let mut string_char: String = chars[string_index as usize].clone();
                    if escape_next {
                        if string_char == "n" {
                            val = s_cat(val.clone(), "\n".to_string());
                        } else if string_char == "t" {
                            val = s_cat(val.clone(), "\t".to_string());
                        } else if string_char == "\\" {
                            val = s_cat(val.clone(), "\\".to_string());
                        } else if string_char == "\"" {
                            val = s_cat(val.clone(), "\"".to_string());
                        } else {
                            val = s_cat(val.clone(), "\\".to_string());
                            val = s_cat(val.clone(), string_char.clone());
                        }
                        escape_next = false;
                        char_index = string_index + 1;
                    } else if string_char == "\\" {
                        escape_next = true;
                        char_index = string_index + 1;
                    } else if string_char == "\"" {
                        char_index = string_index + 1;
                        break;
                    } else {
                        val = s_cat(val.clone(), string_char.clone());
                        char_index = string_index + 1;
                    }
                }
                tokens.push(make_token("STRING".to_string(), val.clone(), cur_line.clone()).clone());
                continue;
            }
            if is_digit(character.clone()) {
                let mut num: String = s_cat("".to_string(), character.clone());
                let mut num_start: i32 = char_index + 1.clone();
                char_index = char_index + 1;
                for number_index in num_start..char_count {
                    let mut number_char: String = chars[number_index as usize].clone();
                    if is_digit(number_char.clone()) {
                        num = s_cat(num.clone(), number_char.clone());
                        char_index = number_index + 1;
                    } else {
                        break;
                    }
                }
                tokens.push(make_token("INT".to_string(), num.clone(), cur_line.clone()).clone());
                continue;
            }
            if is_alpha(character.clone()) {
                let mut word: String = s_cat("".to_string(), character.clone());
                let mut word_start: i32 = char_index + 1.clone();
                char_index = char_index + 1;
                for word_index in word_start..char_count {
                    let mut word_char: String = chars[word_index as usize].clone();
                    if is_alnum(word_char.clone()) {
                        word = s_cat(word.clone(), word_char.clone());
                        char_index = word_index + 1;
                    } else {
                        break;
                    }
                }
                tokens.push(make_token(word_to_kind(word.clone()), word.clone(), cur_line.clone()).clone());
                continue;
            }
            let mut peek_idx: i32 = char_index + 1.clone();
            let mut peek: String = "".to_string();
            if peek_idx < char_count {
                peek = chars[peek_idx as usize].clone();
            }
            if character == ">" && peek == "=" {
                tokens.push(make_token("GTE".to_string(), ">=".to_string(), cur_line.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "<" && peek == "=" {
                tokens.push(make_token("LTE".to_string(), "<=".to_string(), cur_line.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "+" {
                tokens.push(make_token("PLUS".to_string(), "+".to_string(), cur_line.clone()).clone());
            } else if character == "-" {
                tokens.push(make_token("MINUS".to_string(), "-".to_string(), cur_line.clone()).clone());
            } else if character == "*" {
                tokens.push(make_token("STAR".to_string(), "*".to_string(), cur_line.clone()).clone());
            } else if character == "/" {
                tokens.push(make_token("SLASH".to_string(), "/".to_string(), cur_line.clone()).clone());
            } else if character == "%" {
                tokens.push(make_token("PERCENT".to_string(), "%".to_string(), cur_line.clone()).clone());
            } else if character == "=" {
                tokens.push(make_token("EQUALS".to_string(), "=".to_string(), cur_line.clone()).clone());
            } else if character == ">" {
                tokens.push(make_token("GT".to_string(), ">".to_string(), cur_line.clone()).clone());
            } else if character == "<" {
                tokens.push(make_token("LT".to_string(), "<".to_string(), cur_line.clone()).clone());
            } else if character == "(" {
                tokens.push(make_token("LPAREN".to_string(), "(".to_string(), cur_line.clone()).clone());
            } else if character == ")" {
                tokens.push(make_token("RPAREN".to_string(), ")".to_string(), cur_line.clone()).clone());
            } else if character == "[" {
                tokens.push(make_token("LBRACKET".to_string(), "[".to_string(), cur_line.clone()).clone());
            } else if character == "]" {
                tokens.push(make_token("RBRACKET".to_string(), "]".to_string(), cur_line.clone()).clone());
            } else if character == "," {
                tokens.push(make_token("COMMA".to_string(), ",".to_string(), cur_line.clone()).clone());
            }
            char_index = char_index + 1;
        }
        tokens.push(make_token("NEWLINE".to_string(), "".to_string(), cur_line.clone()).clone());
    }
    let mut final_stack_len: i32 = indent_stack.len() as i32;
    for _ in 1..final_stack_len {
        tokens.push(make_token("DEDENT".to_string(), "".to_string(), cur_line.clone()).clone());
    }
    tokens.push(make_token("EOF".to_string(), "".to_string(), cur_line.clone()).clone());
    return tokens;
}

fn skip_to_block_start(tokens: Vec<Token>, start: i32) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut cur: i32 = start.clone();
    for skip_index in start..token_count {
        let mut skip_token: Token = tokens[skip_index as usize].clone();
        let mut skip_kind: String = tok_kind(skip_token.clone());
        cur = skip_index + 1;
        if skip_kind == "INDENT" {
            break;
        }
    }
    return make_result("".to_string(), cur.clone());
}

fn collect_struct_fields(tokens: Vec<Token>, start: i32) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i32 = start.clone();
    for field_index in start..token_count {
        let mut field_token: Token = tokens[field_index as usize].clone();
        let mut field_kind: String = tok_kind(field_token.clone());
        if field_kind == "DEDENT" {
            cur = field_index + 1;
            break;
        } else if field_kind == "IDENT" {
            let mut fname_pos: i32 = field_index + 1.clone();
            if fname_pos < token_count {
                let mut fname_token: Token = tokens[fname_pos as usize].clone();
                if tok_kind(fname_token.clone()) == "IDENT" {
                    fields.push(tok_value(fname_token.clone()).clone());
                    cur = fname_pos + 1;
                }
            }
        }
    }
    return make_result(s_join_with(fields.clone(), ",".to_string()), cur.clone());
}

fn build_struct_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = tokens.len() as i32;
    let mut raw_i: i32 = 0;
    while raw_i < token_count {
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_STRUCT" {
            let mut name_pos: i32 = raw_i + 1.clone();
            if name_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                if tok_kind(name_token.clone()) == "IDENT" {
                    let mut block_r: ParseResult = skip_to_block_start(tokens.clone(), name_pos + 1.clone());
                    let mut fields_r: ParseResult = collect_struct_fields(tokens.clone(), pr_pos(block_r.clone()));
                    result.push(tok_value(name_token.clone()).clone());
                    result.push(pr_code(fields_r.clone()).clone());
                    raw_i = pr_pos(fields_r.clone());
                    continue;
                }
            }
        }
        raw_i = raw_i + 1;
    }
    return result;
}

fn build_shape_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = tokens.len() as i32;
    for index in 0..token_count {
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_SHAPE" {
            let mut name_pos: i32 = index + 1.clone();
            let mut elem_pos: i32 = index + 5.clone();
            if elem_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut elem_token: Token = tokens[elem_pos as usize].clone();
                let mut shape_name: String = tok_value(name_token.clone());
                let mut elem_type: String = tok_value(elem_token.clone());
                result.push(shape_name.clone());
                result.push(elem_type.clone());
            }
        }
    }
    return result;
}

fn build_enum_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = tokens.len() as i32;
    for index in 0..token_count {
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            let mut name_pos: i32 = index + 1.clone();
            if name_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut enum_name: String = tok_value(name_token.clone());
                let mut rust_name: String = pascal_case(enum_name.clone());
                result.push(enum_name.clone());
                result.push(rust_name.clone());
            }
        }
    }
    return result;
}

fn build_variant_reg(tokens: Vec<Token>, enum_reg: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = tokens.len() as i32;
    let mut raw_i: i32 = 0;
    while raw_i < token_count {
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            let mut name_pos: i32 = raw_i + 1.clone();
            if name_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut enum_name: String = tok_value(name_token.clone());
                let mut rust_name: String = reg_get(enum_reg.clone(), enum_name.clone());
                raw_i = name_pos + 1;
                while raw_i < token_count {
                    let mut skip_token: Token = tokens[raw_i as usize].clone();
                    let mut skip_kind: String = tok_kind(skip_token.clone());
                    raw_i = raw_i + 1;
                    if skip_kind == "INDENT" {
                        break;
                    }
                }
                while raw_i < token_count {
                    let mut variant_token: Token = tokens[raw_i as usize].clone();
                    let mut variant_kind: String = tok_kind(variant_token.clone());
                    let mut variant_value: String = tok_value(variant_token.clone());
                    raw_i = raw_i + 1;
                    if variant_kind == "DEDENT" {
                        break;
                    } else if variant_kind == "IDENT" {
                        result.push(variant_value.clone());
                        result.push(rust_name.clone());
                    }
                }
                continue;
            }
        }
        raw_i = raw_i + 1;
    }
    return result;
}

fn build_type_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = tokens.len() as i32;
    for index in 0..token_count {
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_TYPE" {
            let mut name_pos: i32 = index + 1.clone();
            let mut param_type_pos: i32 = index + 3.clone();
            let mut param_name_pos: i32 = index + 4.clone();
            if param_name_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut param_type_token: Token = tokens[param_type_pos as usize].clone();
                let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                result.push(tok_value(name_token.clone()).clone());
                result.push(tok_value(param_type_token.clone()).clone());
                result.push(tok_value(param_name_token.clone()).clone());
            }
        }
    }
    return result;
}

fn resolve_type(type_name: String, shape_reg: Vec<String>, enum_reg: Vec<String>) -> String {
    let mut enum_rust: String = reg_get(enum_reg.clone(), type_name.clone());
    if !is_empty(enum_rust.clone()) {
        return enum_rust;
    }
    let mut elem_type: String = reg_get(shape_reg.clone(), type_name.clone());
    if !is_empty(elem_type.clone()) {
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        return s_join(vec!["Vec<".to_string(), rust_elem.clone(), ">".to_string()]);
    }
    return render_rust_type(type_name.clone());
}

fn find_block_end(tokens: Vec<Token>, indent_pos: i32) -> i32 {
    let mut token_count: i32 = tokens.len() as i32;
    let mut depth: i32 = 1;
    let mut result: i32 = indent_pos.clone();
    let mut start: i32 = indent_pos + 1.clone();
    for raw_i in start..token_count {
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "INDENT" {
            depth = depth + 1;
        } else if kind == "DEDENT" {
            depth = depth - 1;
            if depth == 0 {
                result = raw_i;
                break;
            }
        }
    }
    return result;
}

fn collect_mut_names(tokens: Vec<Token>, start: i32, end_pos: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for raw_i in start..end_pos {
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "EQUALS" {
            let mut prev_pos: i32 = raw_i - 1.clone();
            if prev_pos >= start {
                let mut prev_token: Token = tokens[prev_pos as usize].clone();
                let mut prev_kind: String = tok_kind(prev_token.clone());
                let mut prev_value: String = tok_value(prev_token.clone());
                if prev_kind == "IDENT" {
                    if !list_has(result.clone(), prev_value.clone()) {
                        result.push(prev_value.clone());
                    }
                }
            }
        }
    }
    return result;
}

fn find_struct_for_fields(struct_reg: Vec<String>, fields: Vec<String>) -> String {
    let mut fields_key: String = s_join_with(fields.clone(), ",".to_string());
    let mut reg_count: i32 = struct_reg.len() as i32;
    let mut next_is_val: bool = false;
    let mut cur_name: String = "".to_string();
    for index in 0..reg_count {
        let mut item: String = struct_reg[index as usize].clone();
        if next_is_val {
            if item == fields_key {
                return cur_name;
            }
            next_is_val = false;
        } else {
            cur_name = item;
            next_is_val = true;
        }
    }
    return "Unknown".to_string();
}

fn gen_call_args(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let mut arg_codes: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = tokens.len() as i32;
    while true {
        if cur >= token_count {
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let mut current_kind: String = tok_kind(token.clone());
        if current_kind == "RPAREN" {
            break;
        }
        if current_kind == "COMMA" {
            cur = cur + 1;
            continue;
        }
        let mut arg_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        let mut arg_code: String = pr_code(arg_r.clone());
        let mut arg_pos: i32 = pr_pos(arg_r.clone());
        let mut start_token: Token = tokens[cur as usize].clone();
        let mut start_kind: String = tok_kind(start_token.clone());
        if start_kind == "STRING" {
            arg_code = s_join(vec![arg_code.clone(), ".to_string()".to_string()]);
        } else if start_kind == "IDENT" {
            let mut next_cur: i32 = cur + 1.clone();
            let mut next_kind: String = "".to_string();
            if next_cur < token_count {
                next_kind = tok_kind(tokens[next_cur as usize].clone());
            }
            let mut peek_is_call: bool = next_kind == "LPAREN".clone();
            let mut peek_is_idx: bool = next_kind == "KW_AT".clone();
            if !peek_is_call {
                if !peek_is_idx {
                    arg_code = s_join(vec![arg_code.clone(), ".clone()".to_string()]);
                }
            }
        }
        arg_codes.push(arg_code.clone());
        cur = arg_pos;
    }
    return make_result(s_join_with(arg_codes.clone(), ", ".to_string()), cur.clone());
}

fn gen_list_items(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let mut item_codes: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = tokens.len() as i32;
    while true {
        if cur >= token_count {
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let mut current_kind: String = tok_kind(token.clone());
        if current_kind == "RBRACKET" {
            break;
        }
        if current_kind == "COMMA" {
            cur = cur + 1;
            continue;
        }
        let mut item_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        let mut item_code: String = pr_code(item_r.clone());
        let mut item_pos: i32 = pr_pos(item_r.clone());
        let mut start_token: Token = tokens[cur as usize].clone();
        let mut start_kind: String = tok_kind(start_token.clone());
        if start_kind == "STRING" {
            item_code = s_join(vec![item_code.clone(), ".to_string()".to_string()]);
        } else {
            item_code = s_join(vec![item_code.clone(), ".clone()".to_string()]);
        }
        item_codes.push(item_code.clone());
        cur = item_pos;
    }
    return make_result(s_join_with(item_codes.clone(), ", ".to_string()), cur.clone());
}

fn gen_unary_method(tokens: Vec<Token>, args_pos: i32, suffix: String, ctx: GenCtx) -> ParseResult {
    let mut inner_r: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
    let mut inner_code: String = pr_code(inner_r.clone());
    let mut close: i32 = pr_pos(inner_r.clone()) + 1;
    return make_result(s_join(vec![inner_code.clone(), suffix.clone()]), close.clone());
}

fn gen_binary_str_method(tokens: Vec<Token>, args_pos: i32, method_name: String, ctx: GenCtx) -> ParseResult {
    let mut a0_r: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
    let mut a0c: String = pr_code(a0_r.clone());
    let mut sep_pos: i32 = pr_pos(a0_r.clone()) + 1;
    let mut a1_r: ParseResult = gen_expr(tokens.clone(), sep_pos.clone(), ctx.clone());
    let mut a1c: String = pr_code(a1_r.clone());
    let mut close_pos: i32 = pr_pos(a1_r.clone()) + 1;
    let mut a1_token: Token = tokens[sep_pos as usize].clone();
    let mut a1_kind: String = tok_kind(a1_token.clone());
    let mut a1_arg: String = a1c.clone();
    let mut a1_is_str: bool = a1_kind == "STRING".clone();
    if !a1_is_str {
        a1_arg = s_join(vec![a1c.clone(), ".as_str()".to_string()]);
    }
    return make_result(s_join(vec![a0c.clone(), ".".to_string(), method_name.clone(), "(".to_string(), a1_arg.clone(), ")".to_string()]), close_pos.clone());
}

fn gen_primary(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut token_count: i32 = tokens.len() as i32;
    let mut token: Token = tokens[pos as usize].clone();
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    if kind == "INT" {
        return make_result(value.clone(), pos + 1.clone());
    }
    if kind == "STRING" {
        return make_result(to_debug_str(value.clone()), pos + 1.clone());
    }
    if kind == "KW_TRUE" {
        return make_result("true".to_string(), pos + 1.clone());
    }
    if kind == "KW_FALSE" {
        return make_result("false".to_string(), pos + 1.clone());
    }
    if kind == "KW_NONE" {
        return make_result("None".to_string(), pos + 1.clone());
    }
    if kind == "LBRACKET" {
        let mut inner_pos: i32 = pos + 1.clone();
        if inner_pos < token_count {
            let mut next_token: Token = tokens[inner_pos as usize].clone();
            let mut next_kind: String = tok_kind(next_token.clone());
            if next_kind == "RBRACKET" {
                let mut after: i32 = inner_pos + 1.clone();
                return make_result("Vec::new()".to_string(), after.clone());
            }
        }
        let mut items_r: ParseResult = gen_list_items(tokens.clone(), inner_pos.clone(), ctx.clone());
        let mut items_code: String = pr_code(items_r.clone());
        let mut items_pos: i32 = pr_pos(items_r.clone());
        let mut list_code: String = s_join(vec!["vec![".to_string(), items_code.clone(), "]".to_string()]);
        let mut after_bracket: i32 = items_pos + 1.clone();
        return make_result(list_code.clone(), after_bracket.clone());
    }
    if kind == "LPAREN" {
        let mut peek_pos: i32 = pos + 1.clone();
        if peek_pos < token_count {
            let mut peek_token: Token = tokens[peek_pos as usize].clone();
            if tok_kind(peek_token.clone()) == "KW_AVOW" {
                let mut expr_pos: i32 = peek_pos + 1.clone();
                let mut expr_r: ParseResult = gen_expr(tokens.clone(), expr_pos.clone(), ctx.clone());
                let mut expr_code: String = pr_code(expr_r.clone());
                let mut after_rparen: i32 = pr_pos(expr_r.clone()) + 1;
                return make_result(s_join(vec![expr_code.clone(), ".unwrap()".to_string()]), after_rparen.clone());
            }
        }
    }
    if kind == "KW_NOT" {
        let mut operand_pos: i32 = pos + 1.clone();
        let mut operand_r: ParseResult = gen_primary(tokens.clone(), operand_pos.clone(), ctx.clone());
        let mut operand_code: String = pr_code(operand_r.clone());
        let mut operand_pos_end: i32 = pr_pos(operand_r.clone());
        return make_result(s_join(vec!["!".to_string(), operand_code.clone()]), operand_pos_end.clone());
    }
    if kind == "IDENT" {
        let mut next_pos: i32 = pos + 1.clone();
        if next_pos < token_count {
            let mut next_token: Token = tokens[next_pos as usize].clone();
            let mut next_kind: String = tok_kind(next_token.clone());
            if next_kind == "LPAREN" {
                let mut func_name: String = value.clone();
                let mut args_pos: i32 = next_pos + 1.clone();
                if func_name == "len" {
                    return gen_unary_method(tokens.clone(), args_pos.clone(), ".len() as i32".to_string(), ctx.clone());
                }
                if func_name == "trim" {
                    return gen_unary_method(tokens.clone(), args_pos.clone(), ".trim().to_string()".to_string(), ctx.clone());
                }
                if func_name == "to_upper" {
                    return gen_unary_method(tokens.clone(), args_pos.clone(), ".to_uppercase()".to_string(), ctx.clone());
                }
                if func_name == "to_lower" {
                    return gen_unary_method(tokens.clone(), args_pos.clone(), ".to_lowercase()".to_string(), ctx.clone());
                }
                if func_name == "contains" {
                    return gen_binary_str_method(tokens.clone(), args_pos.clone(), "contains".to_string(), ctx.clone());
                }
                if func_name == "starts_with" {
                    return gen_binary_str_method(tokens.clone(), args_pos.clone(), "starts_with".to_string(), ctx.clone());
                }
                if func_name == "ends_with" {
                    return gen_binary_str_method(tokens.clone(), args_pos.clone(), "ends_with".to_string(), ctx.clone());
                }
                if func_name == "split" {
                    let mut a0_r: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
                    let mut a0c: String = pr_code(a0_r.clone());
                    let mut a0p: i32 = pr_pos(a0_r.clone());
                    let mut sep_pos: i32 = a0p + 1.clone();
                    let mut a1_r: ParseResult = gen_expr(tokens.clone(), sep_pos.clone(), ctx.clone());
                    let mut a1c: String = pr_code(a1_r.clone());
                    let mut a1p: i32 = pr_pos(a1_r.clone());
                    let mut a1_token: Token = tokens[sep_pos as usize].clone();
                    let mut a1_kind: String = tok_kind(a1_token.clone());
                    let mut a1_arg: String = a1c.clone();
                    let mut a1_is_str: bool = a1_kind == "STRING".clone();
                    if !a1_is_str {
                        a1_arg = s_join(vec![a1c.clone(), ".as_str()".to_string()]);
                    }
                    let mut close_pos: i32 = a1p + 1.clone();
                    return make_result(s_join(vec![a0c.clone(), ".split(".to_string(), a1_arg.clone(), ").map(|s| s.to_string()).collect()".to_string()]), close_pos.clone());
                }
                let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
                let mut args_code: String = pr_code(args_r.clone());
                let mut args_end: i32 = pr_pos(args_r.clone());
                let mut after_paren: i32 = args_end + 1.clone();
                return make_result(s_join(vec![func_name.clone(), "(".to_string(), args_code.clone(), ")".to_string()]), after_paren.clone());
            }
            if next_kind == "KW_AT" {
                let mut idx_pos: i32 = next_pos + 1.clone();
                let mut idx_r: ParseResult = gen_primary(tokens.clone(), idx_pos.clone(), ctx.clone());
                let mut idx_code: String = pr_code(idx_r.clone());
                let mut idx_end: i32 = pr_pos(idx_r.clone());
                return make_result(s_join(vec![value.clone(), "[".to_string(), idx_code.clone(), " as usize].clone()".to_string()]), idx_end.clone());
            }
        }
        let mut variant_enum: String = reg_get(variant_reg.clone(), value.clone());
        if !is_empty(variant_enum.clone()) {
            return make_result(s_join(vec![variant_enum.clone(), "::".to_string(), value.clone()]), next_pos.clone());
        }
        return make_result(value.clone(), next_pos.clone());
    }
    return make_result("/* unknown_primary */".to_string(), pos + 1.clone());
}

fn gen_expr(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let mut primary_r: ParseResult = gen_primary(tokens.clone(), pos.clone(), ctx.clone());
    let mut left_code: String = pr_code(primary_r.clone());
    let mut cur_pos: i32 = pr_pos(primary_r.clone());
    let mut token_count: i32 = tokens.len() as i32;
    let mut first_token: Token = tokens[pos as usize].clone();
    let mut left_has_str: bool = tok_kind(first_token.clone()) == "STRING";
    while cur_pos < token_count {
        let mut operator_token: Token = tokens[cur_pos as usize].clone();
        let mut operator_kind: String = tok_kind(operator_token.clone());
        let mut operator_value: String = tok_value(operator_token.clone());
        if !is_binary_op(operator_kind.clone()) {
            break;
        }
        let mut operator_str: String = operator_value.clone();
        let mut after_op: i32 = cur_pos + 1.clone();
        if operator_kind == "KW_IS" {
            if after_op < token_count {
                let mut maybe_not: Token = tokens[after_op as usize].clone();
                let mut next_kind: String = tok_kind(maybe_not.clone());
                if next_kind == "KW_NOT" {
                    operator_str = "is not".to_string();
                    after_op = after_op + 1;
                }
            }
        }
        let mut rhs_r: ParseResult = gen_primary(tokens.clone(), after_op.clone(), ctx.clone());
        let mut rhs_code: String = pr_code(rhs_r.clone());
        let mut rhs_pos: i32 = pr_pos(rhs_r.clone());
        let mut rust_op: String = map_op(operator_str.clone());
        if operator_str == "+" {
            let mut left_token: Token = tokens[pos as usize].clone();
            let mut left_kind: String = tok_kind(left_token.clone());
            let mut rhs_token: Token = tokens[after_op as usize].clone();
            let mut rhs_kind: String = tok_kind(rhs_token.clone());
            if left_kind == "STRING" {
                left_code = s_join(vec!["format!(\"{}{}\", ".to_string(), left_code.clone(), ", ".to_string(), rhs_code.clone(), ")".to_string()]);
                left_has_str = true;
                cur_pos = rhs_pos;
                continue;
            }
            if left_has_str {
                if rhs_kind == "IDENT" {
                    rhs_code = s_join(vec![rhs_code.clone(), ".as_str()".to_string()]);
                }
            }
            if rhs_kind == "STRING" {
                left_has_str = true;
            }
        }
        left_code = s_join(vec![left_code.clone(), " ".to_string(), rust_op.clone(), " ".to_string(), rhs_code.clone()]);
        cur_pos = rhs_pos;
    }
    return make_result(left_code.clone(), cur_pos.clone());
}

fn gen_if_branch(tokens: Vec<Token>, cond_pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let mut body_start: i32 = skip_to_body(tokens.clone(), pr_pos(cond_r.clone()));
    let mut body_r: ParseResult = gen_block(tokens.clone(), body_start.clone(), depth + 1.clone(), ctx.clone());
    let mut combined: String = s_join(vec![pr_code(cond_r.clone()).clone(), " {\n".to_string(), pr_code(body_r.clone()).clone()]);
    return make_result(combined.clone(), pr_pos(body_r.clone()));
}

fn gen_if(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut pad: String = str_repeat("    ".to_string(), depth.clone());
    let mut then_r: ParseResult = gen_if_branch(tokens.clone(), pos + 1.clone(), depth.clone(), ctx.clone());
    let mut result_code: String = s_join(vec![pad.clone(), "if ".to_string(), pr_code(then_r.clone()).clone(), pad.clone(), "}".to_string()]);
    let mut cur: i32 = pr_pos(then_r.clone());
    while true {
        if cur >= token_count {
            break;
        }
        let mut else_token: Token = tokens[cur as usize].clone();
        let mut else_kind: String = tok_kind(else_token.clone());
        let mut is_else: bool = else_kind == "KW_ELSE".clone();
        if !is_else {
            break;
        }
        let mut after_else: i32 = cur + 1.clone();
        if after_else >= token_count {
            break;
        }
        let mut after_else_token: Token = tokens[after_else as usize].clone();
        let mut after_else_kind: String = tok_kind(after_else_token.clone());
        if after_else_kind == "KW_IF" {
            let mut ei_r: ParseResult = gen_if_branch(tokens.clone(), after_else + 1.clone(), depth.clone(), ctx.clone());
            result_code = s_join(vec![result_code.clone(), " else if ".to_string(), pr_code(ei_r.clone()).clone(), pad.clone(), "}".to_string()]);
            cur = pr_pos(ei_r.clone());
        } else {
            let mut else_body_start: i32 = skip_to_body(tokens.clone(), after_else.clone());
            let mut else_r: ParseResult = gen_block(tokens.clone(), else_body_start.clone(), depth + 1.clone(), ctx.clone());
            result_code = s_join(vec![result_code.clone(), " else {\n".to_string(), pr_code(else_r.clone()).clone(), pad.clone(), "}".to_string()]);
            cur = pr_pos(else_r.clone());
            break;
        }
    }
    result_code = s_cat(result_code.clone(), "\n".to_string());
    return make_result(result_code.clone(), cur.clone());
}

fn gen_for(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut pad: String = str_repeat("    ".to_string(), depth.clone());
    let mut next_pos: i32 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let mut next_kind: String = tok_kind(next_token.clone());
    if next_kind == "KW_IF" {
        let mut cond_pos: i32 = next_pos + 1.clone();
        let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
        let mut cond_code: String = pr_code(cond_r.clone());
        let mut cond_end: i32 = pr_pos(cond_r.clone());
        let mut while_body_start: i32 = skip_to_body(tokens.clone(), cond_end.clone());
        let mut while_body_r: ParseResult = gen_block(tokens.clone(), while_body_start.clone(), depth + 1.clone(), ctx.clone());
        let mut while_body_code: String = pr_code(while_body_r.clone());
        let mut while_body_end: i32 = pr_pos(while_body_r.clone());
        let mut while_code: String = s_join(vec![pad.clone(), "while ".to_string(), cond_code.clone(), " {\n".to_string(), while_body_code.clone(), pad.clone(), "}\n".to_string()]);
        return make_result(while_code.clone(), while_body_end.clone());
    }
    let mut var_name: String = "_".to_string();
    let mut iter_pos: i32 = 0;
    if next_kind == "KW_IN" {
        iter_pos = next_pos + 1;
    } else {
        var_name = tok_value(next_token.clone());
        let mut in_pos: i32 = next_pos + 1.clone();
        iter_pos = in_pos + 1;
    }
    let mut iter_token: Token = tokens[iter_pos as usize].clone();
    let mut iter_kind: String = tok_kind(iter_token.clone());
    let mut iter_value: String = tok_value(iter_token.clone());
    let mut range_expr: String = "".to_string();
    let mut body_tok_pos: i32 = 0;
    if iter_kind == "IDENT" && iter_value == "range" {
        let mut lparen: i32 = iter_pos + 1.clone();
        let mut first_pos: i32 = lparen + 1.clone();
        let mut first_r: ParseResult = gen_expr(tokens.clone(), first_pos.clone(), ctx.clone());
        let mut first_code: String = pr_code(first_r.clone());
        let mut first_p: i32 = pr_pos(first_r.clone());
        let mut comma_token: Token = tokens[first_p as usize].clone();
        let mut has_start: bool = tok_kind(comma_token.clone()) == "COMMA";
        if has_start {
            let mut second_pos: i32 = first_p + 1.clone();
            let mut second_r: ParseResult = gen_expr(tokens.clone(), second_pos.clone(), ctx.clone());
            let mut second_code: String = pr_code(second_r.clone());
            let mut second_p: i32 = pr_pos(second_r.clone());
            range_expr = s_join(vec![first_code.clone(), "..".to_string(), second_code.clone()]);
            body_tok_pos = second_p + 1;
        } else {
            range_expr = s_join(vec!["0..".to_string(), first_code.clone()]);
            body_tok_pos = first_p + 1;
        }
    } else if iter_kind == "LPAREN" {
        let mut start_pos: i32 = iter_pos + 1.clone();
        let mut start_r: ParseResult = gen_expr(tokens.clone(), start_pos.clone(), ctx.clone());
        let mut start_code: String = pr_code(start_r.clone());
        let mut start_p: i32 = pr_pos(start_r.clone());
        let mut end_pos: i32 = start_p + 1.clone();
        let mut end_r: ParseResult = gen_expr(tokens.clone(), end_pos.clone(), ctx.clone());
        let mut end_code: String = pr_code(end_r.clone());
        let mut end_p: i32 = pr_pos(end_r.clone());
        range_expr = s_join(vec![start_code.clone(), "..".to_string(), end_code.clone()]);
        body_tok_pos = end_p + 1;
    } else {
        let mut src_r: ParseResult = gen_expr(tokens.clone(), iter_pos.clone(), ctx.clone());
        let mut src_code: String = pr_code(src_r.clone());
        let mut src_p: i32 = pr_pos(src_r.clone());
        range_expr = src_code;
        body_tok_pos = src_p;
    }
    body_tok_pos = skip_to_body(tokens.clone(), body_tok_pos.clone());
    let mut body_r: ParseResult = gen_block(tokens.clone(), body_tok_pos.clone(), depth + 1.clone(), ctx.clone());
    let mut body_code: String = pr_code(body_r.clone());
    let mut body_end: i32 = pr_pos(body_r.clone());
    let mut for_code: String = s_join(vec![pad.clone(), "for ".to_string(), var_name.clone(), " in ".to_string(), range_expr.clone(), " {\n".to_string(), body_code.clone(), pad.clone(), "}\n".to_string()]);
    return make_result(for_code.clone(), body_end.clone());
}

fn skip_newline(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    if pos < token_count {
        let mut current_kind: String = tok_kind(tokens[pos as usize].clone());
        if current_kind == "NEWLINE" {
            return make_result("".to_string(), pos + 1.clone());
        }
    }
    return make_result("".to_string(), pos.clone());
}

fn adv_nl(pos: i32, tokens: Vec<Token>) -> i32 {
    let mut token_count: i32 = tokens.len() as i32;
    if pos < token_count {
        let mut current_kind: String = tok_kind(tokens[pos as usize].clone());
        if current_kind == "NEWLINE" {
            return pos + 1;
        }
    }
    return pos;
}

fn adv_indent(pos: i32, tokens: Vec<Token>) -> i32 {
    let mut token_count: i32 = tokens.len() as i32;
    if pos < token_count {
        let mut current_kind: String = tok_kind(tokens[pos as usize].clone());
        if current_kind == "INDENT" {
            return pos + 1;
        }
    }
    return pos;
}

fn skip_to_body(tokens: Vec<Token>, pos: i32) -> i32 {
    let mut _state = pos.clone();
    _state = adv_nl(_state.clone(), tokens.clone());
    _state = adv_indent(_state.clone(), tokens.clone());
    let mut cur = _state;
    return cur;
}

fn emit_val(val_code: String, val_kind: String) -> String {
    if val_kind == "STRING" {
        return s_join(vec![val_code.clone(), ".to_string()".to_string()]);
    }
    if val_kind == "IDENT" {
        return s_join(vec![val_code.clone(), ".clone()".to_string()]);
    }
    return val_code;
}

fn gen_destructure(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut pad: String = str_repeat("    ".to_string(), depth.clone());
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i32 = pos + 1.clone();
    while cur < token_count {
        let mut field_token: Token = tokens[cur as usize].clone();
        let mut field_kind: String = tok_kind(field_token.clone());
        let mut field_value: String = tok_value(field_token.clone());
        if field_kind == "RPAREN" {
            cur = cur + 1;
            break;
        } else if field_kind == "COMMA" {
            cur = cur + 1;
        } else if field_kind == "IDENT" {
            fields.push(field_value.clone());
            cur = cur + 1;
        }
    }
    let mut src_pos: i32 = cur + 1.clone();
    let mut src_r: ParseResult = gen_expr(tokens.clone(), src_pos.clone(), ctx.clone());
    let mut src_code: String = pr_code(src_r.clone());
    let mut src_end: i32 = pr_pos(src_r.clone());
    let mut after: i32 = adv_nl(src_end.clone(), tokens.clone());
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = fields.len() as i32;
    for field_index in 0..field_count {
        let mut field: String = fields[field_index as usize].clone();
        dest_lines.push(s_join(vec![pad.clone(), "let ".to_string(), field.clone(), " = ".to_string(), src_code.clone(), ".".to_string(), field.clone(), ".clone();".to_string()]).clone());
    }
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    dest_code = s_cat(dest_code.clone(), "\n".to_string());
    return make_result(dest_code.clone(), after.clone());
}

fn gen_stmt(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut token_count: i32 = tokens.len() as i32;
    let mut token: Token = tokens[pos as usize].clone();
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    let mut pad: String = str_repeat("    ".to_string(), depth.clone());
    if kind == "KW_RUST" {
        let mut block_pos: i32 = pos + 2.clone();
        let mut block_token: Token = tokens[block_pos as usize].clone();
        let mut block_content: String = tok_value(block_token.clone());
        let mut rust_lines: Vec<String> = block_content.split("\n").map(|s| s.to_string()).collect();
        let mut padded: Vec<String> = Vec::new();
        let mut rust_line_count: i32 = rust_lines.len() as i32;
        for rust_index in 0..rust_line_count {
            let mut rust_line: String = rust_lines[rust_index as usize].clone();
            if is_empty(rust_line.clone()) {
                padded.push("".to_string());
            } else {
                padded.push(s_join(vec![pad.clone(), rust_line.clone()]).clone());
            }
        }
        let mut block_code: String = s_join_nl(padded.clone());
        block_code = s_cat(block_code.clone(), "\n".to_string());
        return make_result(block_code.clone(), block_pos + 1.clone());
    }
    if kind == "KW_RETURN" {
        let mut val_pos: i32 = pos + 1.clone();
        let mut val_token: Token = tokens[val_pos as usize].clone();
        let mut val_kind: String = tok_kind(val_token.clone());
        let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let mut val_code: String = pr_code(val_r.clone());
        let mut val_end: i32 = pr_pos(val_r.clone());
        let mut ret_suffix: String = "".to_string();
        if val_kind == "STRING" {
            ret_suffix = ".to_string()".to_string();
        }
        return make_result(s_join(vec![pad.clone(), "return ".to_string(), val_code.clone(), ret_suffix.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
    }
    if kind == "KW_BREAK" {
        return make_result(s_join(vec![pad.clone(), "break;\n".to_string()]), adv_nl(pos + 1.clone(), tokens.clone()));
    }
    if kind == "KW_CONTINUE" {
        return make_result(s_join(vec![pad.clone(), "continue;\n".to_string()]), adv_nl(pos + 1.clone(), tokens.clone()));
    }
    if kind == "KW_IF" {
        return gen_if(tokens.clone(), pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_FOR" {
        return gen_for(tokens.clone(), pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "LPAREN" {
        let mut peek_pos: i32 = pos + 1.clone();
        if peek_pos < token_count {
            let mut peek_token: Token = tokens[peek_pos as usize].clone();
            if tok_kind(peek_token.clone()) == "KW_AVOW" {
                let mut expr_pos: i32 = peek_pos + 1.clone();
                let mut expr_r: ParseResult = gen_expr(tokens.clone(), expr_pos.clone(), ctx.clone());
                let mut expr_code: String = pr_code(expr_r.clone());
                let mut after_rparen: i32 = pr_pos(expr_r.clone()) + 1;
                return make_result(s_join(vec![pad.clone(), expr_code.clone(), ".unwrap();\n".to_string()]), adv_nl(after_rparen.clone(), tokens.clone()));
            }
        }
        return gen_destructure(tokens.clone(), pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "IDENT" {
        let mut next_pos: i32 = pos + 1.clone();
        if next_pos >= token_count {
            return make_result("/* eof */\n".to_string(), pos + 1.clone());
        }
        let mut next_token: Token = tokens[next_pos as usize].clone();
        let mut next_kind: String = tok_kind(next_token.clone());
        let mut next_value: String = tok_value(next_token.clone());
        if next_kind == "KW_AS" {
            let mut after_as: i32 = next_pos + 1.clone();
            let mut after_as_token: Token = tokens[after_as as usize].clone();
            let mut after_as_kind: String = tok_kind(after_as_token.clone());
            if after_as_kind == "LPAREN" {
                let mut fields: Vec<String> = Vec::new();
                let mut fend: i32 = after_as + 1.clone();
                while fend < token_count {
                    let mut field_token: Token = tokens[fend as usize].clone();
                    let mut field_kind: String = tok_kind(field_token.clone());
                    let mut field_value: String = tok_value(field_token.clone());
                    if field_kind == "RPAREN" {
                        fend = fend + 1;
                        break;
                    } else if field_kind == "COMMA" {
                        fend = fend + 1;
                    } else if field_kind == "IDENT" {
                        fields.push(field_value.clone());
                        fend = fend + 1;
                    }
                }
                let mut struct_name: String = find_struct_for_fields(struct_reg.clone(), fields.clone());
                let mut is_mut: bool = list_has(mut_names.clone(), value.clone());
                let mut mut_kw: String = "".to_string();
                if is_mut {
                    mut_kw = "mut ".to_string();
                }
                let mut fields_code: String = s_join_with(fields.clone(), ", ".to_string());
                let mut stmt_code: String = s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), value.clone(), " = ".to_string(), struct_name.clone(), " { ".to_string(), fields_code.clone(), " };\n".to_string()]);
                return make_result(stmt_code.clone(), adv_nl(fend.clone(), tokens.clone()));
            }
            if after_as_kind == "KW_EMPTY" {
                return make_result(s_join(vec![pad.clone(), "let mut ".to_string(), value.clone(), " = Vec::new();\n".to_string()]), adv_nl(after_as + 1.clone(), tokens.clone()));
            }
            let mut val_r: ParseResult = gen_expr(tokens.clone(), after_as.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut is_str: bool = after_as_kind == "STRING".clone();
            let mut is_mut: bool = list_has(mut_names.clone(), value.clone());
            let mut mut_kw: String = "".to_string();
            if is_mut {
                mut_kw = "mut ".to_string();
            }
            let mut suffix: String = "".to_string();
            if is_str {
                suffix = ".to_string()".to_string();
            }
            return make_result(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), value.clone(), " = ".to_string(), val_code.clone(), suffix.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
        }
        if next_kind == "LPAREN" {
            let mut args_pos: i32 = next_pos + 1.clone();
            let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
            let mut args_code: String = pr_code(args_r.clone());
            let mut args_end: i32 = pr_pos(args_r.clone());
            let mut after_paren: i32 = args_end + 1.clone();
            let mut call_code: String = "".to_string();
            if value == "print" {
                call_code = s_join(vec![pad.clone(), "println!(\"{}\", ".to_string(), args_code.clone(), ");\n".to_string()]);
            } else {
                call_code = s_join(vec![pad.clone(), value.clone(), "(".to_string(), args_code.clone(), ");\n".to_string()]);
            }
            return make_result(call_code.clone(), adv_nl(after_paren.clone(), tokens.clone()));
        }
        if next_kind == "KW_INSERT" {
            let mut val_pos: i32 = next_pos + 1.clone();
            let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut value_kind: String = tok_kind(tokens[val_pos as usize].clone());
            return make_result(s_join(vec![pad.clone(), value.clone(), ".push(".to_string(), emit_val(val_code.clone(), value_kind.clone()).clone(), ");\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
        }
        if next_kind == "KW_AT" {
            let mut after_at: i32 = next_pos + 1.clone();
            if after_at < token_count {
                let mut at_next_token: Token = tokens[after_at as usize].clone();
                let mut at_next_kind: String = tok_kind(at_next_token.clone());
                let mut at_next_value: String = tok_value(at_next_token.clone());
                if at_next_kind == "IDENT" && at_next_value == "end" {
                    let mut val_pos: i32 = after_at + 2.clone();
                    let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                    let mut val_code: String = pr_code(val_r.clone());
                    let mut val_end: i32 = pr_pos(val_r.clone());
                    let mut value_kind: String = tok_kind(tokens[val_pos as usize].clone());
                    return make_result(s_join(vec![pad.clone(), value.clone(), ".push(".to_string(), emit_val(val_code.clone(), value_kind.clone()).clone(), ");\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
                }
                let mut idx_r: ParseResult = gen_expr(tokens.clone(), after_at.clone(), ctx.clone());
                let mut idx_code: String = pr_code(idx_r.clone());
                let mut idx_end: i32 = pr_pos(idx_r.clone());
                let mut val_pos: i32 = idx_end + 1.clone();
                let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                let mut val_code: String = pr_code(val_r.clone());
                let mut val_end: i32 = pr_pos(val_r.clone());
                let mut value_kind: String = tok_kind(tokens[val_pos as usize].clone());
                return make_result(s_join(vec![pad.clone(), value.clone(), "[".to_string(), idx_code.clone(), " as usize] = ".to_string(), emit_val(val_code.clone(), value_kind.clone()).clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
            }
        }
        if next_kind == "KW_REMOVE" {
            let mut idx_pos: i32 = next_pos + 2.clone();
            let mut idx_r: ParseResult = gen_expr(tokens.clone(), idx_pos.clone(), ctx.clone());
            let mut idx_code: String = pr_code(idx_r.clone());
            let mut idx_end: i32 = pr_pos(idx_r.clone());
            return make_result(s_join(vec![pad.clone(), value.clone(), ".remove(".to_string(), idx_code.clone(), " as usize);\n".to_string()]), adv_nl(idx_end.clone(), tokens.clone()));
        }
        if next_kind == "EQUALS" {
            let mut val_pos: i32 = next_pos + 1.clone();
            let mut eq_val_token: Token = tokens[val_pos as usize].clone();
            let mut eq_val_kind: String = tok_kind(eq_val_token.clone());
            let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut assign_suffix: String = "".to_string();
            if eq_val_kind == "STRING" {
                assign_suffix = ".to_string()".to_string();
            }
            return make_result(s_join(vec![pad.clone(), value.clone(), " = ".to_string(), val_code.clone(), assign_suffix.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
        }
        if next_kind == "IDENT" {
            let mut eq_pos: i32 = next_pos + 1.clone();
            if eq_pos < token_count {
                let mut eq_token: Token = tokens[eq_pos as usize].clone();
                if tok_kind(eq_token.clone()) == "EQUALS" {
                    let mut var_type: String = value.clone();
                    let mut var_name: String = next_value.clone();
                    let mut val_pos: i32 = eq_pos + 1.clone();
                    let mut rust_type: String = resolve_type(var_type.clone(), shape_reg.clone(), variant_reg.clone());
                    let mut val_token: Token = tokens[val_pos as usize].clone();
                    let mut val_kind: String = tok_kind(val_token.clone());
                    if val_kind == "KW_USING" {
                        let mut state_pos: i32 = val_pos + 1.clone();
                        let mut state_r: ParseResult = gen_expr(tokens.clone(), state_pos.clone(), ctx.clone());
                        let mut state_code: String = pr_code(state_r.clone());
                        let mut body_start: i32 = skip_to_body(tokens.clone(), pr_pos(state_r.clone()));
                        let mut lines: Vec<String> = Vec::new();
                        lines.push(s_join(vec![pad.clone(), "let mut _state = ".to_string(), state_code.clone(), ".clone();\n".to_string()]).clone());
                        let mut ucur: i32 = body_start.clone();
                        while ucur < token_count {
                            let mut ut: Token = tokens[ucur as usize].clone();
                            let mut ut_kind: String = tok_kind(ut.clone());
                            if ut_kind == "DEDENT" {
                                ucur = ucur + 1;
                                break;
                            }
                            if ut_kind == "NEWLINE" {
                                ucur = ucur + 1;
                                continue;
                            }
                            if ut_kind == "IDENT" {
                                let mut ufn: String = tok_value(ut.clone());
                                let mut extra_start: i32 = ucur + 2.clone();
                                let mut extra_r: ParseResult = gen_call_args(tokens.clone(), extra_start.clone(), ctx.clone());
                                let mut extra_code: String = pr_code(extra_r.clone());
                                let mut after_rparen: i32 = pr_pos(extra_r.clone()) + 1;
                                let mut full_args: String = "_state.clone()".to_string();
                                if !is_empty(extra_code.clone()) {
                                    full_args = s_join(vec!["_state.clone(), ".to_string(), extra_code.clone()]);
                                }
                                lines.push(s_join(vec![pad.clone(), "_state = ".to_string(), ufn.clone(), "(".to_string(), full_args.clone(), ");\n".to_string()]).clone());
                                ucur = adv_nl(after_rparen.clone(), tokens.clone());
                            } else {
                                ucur = ucur + 1;
                            }
                        }
                        let mut umut: bool = list_has(mut_names.clone(), var_name.clone());
                        let mut umut_kw: String = "".to_string();
                        if umut {
                            umut_kw = "mut ".to_string();
                        }
                        lines.push(s_join(vec![pad.clone(), "let ".to_string(), umut_kw.clone(), var_name.clone(), " = _state;\n".to_string()]).clone());
                        return make_result(s_join(lines.clone()), ucur.clone());
                    }
                    if val_kind == "KW_EMPTY" {
                        let mut is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
                        let mut is_shape: bool = reg_has(shape_reg.clone(), var_type.clone());
                        let mut after_empty: i32 = adv_nl(val_pos + 1.clone(), tokens.clone());
                        if is_validator {
                            return make_result(s_join(vec![pad.clone(), "let mut ".to_string(), var_name.clone(), ": Option<".to_string(), var_type.clone(), "> = None;\n".to_string()]), after_empty.clone());
                        }
                        if is_shape {
                            return make_result(s_join(vec![pad.clone(), "let mut ".to_string(), var_name.clone(), ": ".to_string(), rust_type.clone(), " = Vec::new();\n".to_string()]), after_empty.clone());
                        }
                        return make_result(s_join(vec![pad.clone(), "/* error: empty only valid for validator types and list shapes */\n".to_string()]), after_empty.clone());
                    }
                    if val_kind == "LPAREN" {
                        let mut peek_pos: i32 = val_pos + 1.clone();
                        let mut peek_token: Token = tokens[peek_pos as usize].clone();
                        let mut is_avow_expr: bool = tok_kind(peek_token.clone()) == "KW_AVOW";
                        if !is_avow_expr {
                            let mut struct_fields_str: String = reg_get(struct_reg.clone(), var_type.clone());
                            let mut field_names: Vec<String> = struct_fields_str.split(",").map(|s| s.to_string()).collect();
                            let mut field_pairs: Vec<String> = Vec::new();
                            let mut fend: i32 = val_pos + 1.clone();
                            let mut fni: i32 = 0;
                            let mut fn_count: i32 = field_names.len() as i32;
                            while fend < token_count {
                                let mut ft: Token = tokens[fend as usize].clone();
                                let mut fk: String = tok_kind(ft.clone());
                                if fk == "RPAREN" {
                                    fend = fend + 1;
                                    break;
                                }
                                if fk == "COMMA" {
                                    fend = fend + 1;
                                    continue;
                                }
                                let mut fv_r: ParseResult = gen_expr(tokens.clone(), fend.clone(), ctx.clone());
                                let mut fv_code: String = pr_code(fv_r.clone());
                                if fni < fn_count {
                                    let mut fname: String = field_names[fni as usize].clone();
                                    field_pairs.push(s_join(vec![fname.clone(), ": ".to_string(), fv_code.clone()]).clone());
                                    fni = fni + 1;
                                }
                                fend = pr_pos(fv_r.clone());
                            }
                            let mut fields_code: String = s_join_with(field_pairs.clone(), ", ".to_string());
                            return make_result(s_join(vec![pad.clone(), "let ".to_string(), var_name.clone(), " = ".to_string(), var_type.clone(), " { ".to_string(), fields_code.clone(), " };\n".to_string()]), adv_nl(fend.clone(), tokens.clone()));
                        }
                        let mut inner_pos: i32 = peek_pos + 1.clone();
                        let mut inner_r: ParseResult = gen_expr(tokens.clone(), inner_pos.clone(), ctx.clone());
                        let mut inner_code: String = pr_code(inner_r.clone());
                        let mut after_rparen: i32 = pr_pos(inner_r.clone()) + 1;
                        let mut unwrap_expr: String = s_cat(inner_code.clone(), ".unwrap()".to_string());
                        if var_type == "int" {
                            unwrap_expr = s_cat(inner_code.clone(), ".unwrap().0".to_string());
                        }
                        if var_type == "string" {
                            unwrap_expr = s_cat(inner_code.clone(), ".unwrap().0".to_string());
                        }
                        if var_type == "bool" {
                            unwrap_expr = s_cat(inner_code.clone(), ".unwrap().0".to_string());
                        }
                        if var_type == "float" {
                            unwrap_expr = s_cat(inner_code.clone(), ".unwrap().0".to_string());
                        }
                        return make_result(s_join(vec![pad.clone(), "let ".to_string(), var_name.clone(), ": ".to_string(), rust_type.clone(), " = ".to_string(), unwrap_expr.clone(), ";\n".to_string()]), adv_nl(after_rparen.clone(), tokens.clone()));
                    }
                    if val_kind == "LBRACKET" {
                        let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                        let mut val_code: String = pr_code(val_r.clone());
                        let mut val_end: i32 = pr_pos(val_r.clone());
                        return make_result(s_join(vec![pad.clone(), "let mut ".to_string(), var_name.clone(), ": ".to_string(), rust_type.clone(), " = ".to_string(), val_code.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
                    }
                    let mut is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
                    if is_validator {
                        let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                        let mut val_code: String = pr_code(val_r.clone());
                        let mut val_end: i32 = pr_pos(val_r.clone());
                        let mut is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                        let mut mut_kw: String = "".to_string();
                        if is_mut {
                            mut_kw = "mut ".to_string();
                        }
                        return make_result(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), var_name.clone(), ": Option<".to_string(), var_type.clone(), "> = ".to_string(), var_type.clone(), "::new(".to_string(), val_code.clone(), ");\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
                    }
                    if val_kind == "KW_NONE" {
                        return make_result(s_join(vec![pad.clone(), "let mut ".to_string(), var_name.clone(), ": Option<".to_string(), rust_type.clone(), "> = None;\n".to_string()]), adv_nl(val_pos + 1.clone(), tokens.clone()));
                    }
                    let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                    let mut val_code: String = pr_code(val_r.clone());
                    let mut val_end: i32 = pr_pos(val_r.clone());
                    let mut is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                    let mut mut_kw: String = "".to_string();
                    if is_mut {
                        mut_kw = "mut ".to_string();
                    }
                    let mut suffix: String = "".to_string();
                    if val_kind == "STRING" {
                        suffix = ".to_string()".to_string();
                    } else if val_kind == "IDENT" {
                        let mut value_next_index: i32 = val_pos + 1.clone();
                        if value_next_index < token_count {
                            let mut next_val_token: Token = tokens[value_next_index as usize].clone();
                            let mut next_val_kind: String = tok_kind(next_val_token.clone());
                            let mut val_is_call: bool = next_val_kind == "LPAREN".clone();
                            let mut val_is_idx: bool = next_val_kind == "KW_AT".clone();
                            if !val_is_call {
                                if !val_is_idx {
                                    suffix = ".clone()".to_string();
                                }
                            }
                        }
                    }
                    return make_result(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), var_name.clone(), ": ".to_string(), rust_type.clone(), " = ".to_string(), val_code.clone(), suffix.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
                }
            }
        }
    }
    return make_result(s_join(vec!["/* unhandled(".to_string(), kind.clone(), ") */\n".to_string()]), pos + 1.clone());
}

fn gen_block(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut stmts: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = tokens.len() as i32;
    while true {
        if cur >= token_count {
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let mut current_kind: String = tok_kind(token.clone());
        if current_kind == "DEDENT" || current_kind == "EOF" {
            if current_kind == "DEDENT" {
                cur = cur + 1;
            }
            break;
        }
        if current_kind == "NEWLINE" {
            cur = cur + 1;
            continue;
        }
        let mut stmt_r: ParseResult = gen_stmt(tokens.clone(), cur.clone(), depth.clone(), ctx.clone());
        stmts.push(pr_code(stmt_r.clone()).clone());
        cur = pr_pos(stmt_r.clone());
    }
    return make_result(s_join(stmts.clone()), cur.clone());
}

fn gen_struct_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut name_pos: i32 = pos + 1.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut struct_name: String = tok_value(name_token.clone());
    let mut cur: i32 = name_pos + 1.clone();
    cur = skip_to_body(tokens.clone(), cur.clone());
    let mut field_lines: Vec<String> = Vec::new();
    while cur < token_count {
        let mut field_token: Token = tokens[cur as usize].clone();
        let mut field_kind: String = tok_kind(field_token.clone());
        let mut field_value: String = tok_value(field_token.clone());
        if field_kind == "DEDENT" {
            cur = cur + 1;
            break;
        } else if field_kind == "NEWLINE" {
            cur = cur + 1;
        } else if field_kind == "IDENT" {
            let mut field_type: String = field_value.clone();
            let mut fname_pos: i32 = cur + 1.clone();
            let mut fname_token: Token = tokens[fname_pos as usize].clone();
            let mut field_name: String = tok_value(fname_token.clone());
            let mut rust_type: String = render_rust_type(field_type.clone());
            field_lines.push(s_join(vec!["    ".to_string(), field_name.clone(), ": ".to_string(), rust_type.clone(), ",".to_string()]).clone());
            cur = fname_pos + 1;
        }
    }
    let mut fields_code: String = s_join_nl(field_lines.clone());
    let mut decl: String = s_join(vec!["#[derive(Clone, PartialEq, Debug)]\nstruct ".to_string(), struct_name.clone(), " {\n".to_string(), fields_code.clone(), "\n}\n\n".to_string()]);
    return make_result(decl.clone(), cur.clone());
}

fn gen_shape_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut name_pos: i32 = pos + 1.clone();
    let mut elem_pos: i32 = pos + 5.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut elem_token: Token = tokens[elem_pos as usize].clone();
    let mut shape_name: String = tok_value(name_token.clone());
    let mut elem_type: String = tok_value(elem_token.clone());
    let mut rust_name: String = pascal_case(shape_name.clone());
    let mut rust_elem: String = render_rust_type(elem_type.clone());
    let mut decl: String = s_join(vec!["type ".to_string(), rust_name.clone(), " = Vec<".to_string(), rust_elem.clone(), ">;\n\n".to_string()]);
    let mut after: i32 = elem_pos + 1.clone();
    return make_result(decl.clone(), adv_nl(after.clone(), tokens.clone()));
}

fn gen_enum_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut name_pos: i32 = pos + 1.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut enum_name: String = tok_value(name_token.clone());
    let mut rust_name: String = pascal_case(enum_name.clone());
    let mut cur: i32 = name_pos + 1.clone();
    cur = skip_to_body(tokens.clone(), cur.clone());
    let mut variant_lines: Vec<String> = Vec::new();
    while cur < token_count {
        let mut variant_token: Token = tokens[cur as usize].clone();
        let mut variant_kind: String = tok_kind(variant_token.clone());
        let mut variant_value: String = tok_value(variant_token.clone());
        if variant_kind == "DEDENT" {
            cur = cur + 1;
            break;
        } else if variant_kind == "NEWLINE" {
            cur = cur + 1;
        } else if variant_kind == "IDENT" {
            variant_lines.push(s_join(vec!["    ".to_string(), variant_value.clone(), ",".to_string()]).clone());
            cur = cur + 1;
        }
    }
    let mut variants_code: String = s_join_nl(variant_lines.clone());
    let mut decl: String = s_join(vec!["#[derive(Clone, Copy, PartialEq, Debug)]\nenum ".to_string(), rust_name.clone(), " {\n".to_string(), variants_code.clone(), "\n}\n\n".to_string()]);
    return make_result(decl.clone(), cur.clone());
}

fn gen_type_decl(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let mut token_count: i32 = tokens.len() as i32;
    let mut name_pos: i32 = pos + 1.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut type_name: String = tok_value(name_token.clone());
    let mut param_type_pos: i32 = pos + 3.clone();
    let mut param_type_token: Token = tokens[param_type_pos as usize].clone();
    let mut param_type: String = tok_value(param_type_token.clone());
    let mut rust_param_type: String = render_rust_type(param_type.clone());
    let mut param_name_pos: i32 = pos + 4.clone();
    let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
    let mut param_name: String = tok_value(param_name_token.clone());
    let mut body_pos: i32 = pos + 8.clone();
    let mut pred_r: ParseResult = gen_expr(tokens.clone(), body_pos.clone(), ctx.clone());
    let mut pred_code: String = pr_code(pred_r.clone());
    let mut pred_end: i32 = pr_pos(pred_r.clone());
    let mut cur: i32 = pred_end.clone();
    while cur < token_count {
        let mut t: Token = tokens[cur as usize].clone();
        cur = cur + 1;
        if tok_kind(t.clone()) == "DEDENT" {
            break;
        }
    }
    let mut struct_code: String = s_join(vec!["#[derive(Clone, Copy, PartialEq, Debug)]\nstruct ".to_string(), type_name.clone(), "(".to_string(), rust_param_type.clone(), ");\n\n".to_string()]);
    let mut impl_code: String = s_join(vec!["impl ".to_string(), type_name.clone(), " {\n    fn new(".to_string(), param_name.clone(), ": ".to_string(), rust_param_type.clone(), ") -> Option<Self> {\n        if ".to_string(), pred_code.clone(), " {\n            Some(".to_string(), type_name.clone(), "(".to_string(), param_name.clone(), "))\n        } else {\n            None\n        }\n    }\n}\n\n".to_string()]);
    return make_result(s_cat(struct_code.clone(), impl_code.clone()), cur.clone());
}

fn gen_fn_decl(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut token_count: i32 = tokens.len() as i32;
    let mut cur: i32 = pos + 1.clone();
    let mut ret_token: Token = tokens[cur as usize].clone();
    let mut ret_type: String = resolve_type(tok_value(ret_token.clone()), shape_reg.clone(), enum_reg.clone());
    cur = cur + 1;
    let mut name_token: Token = tokens[cur as usize].clone();
    let mut fn_name: String = tok_value(name_token.clone());
    cur = cur + 1;
    cur = cur + 1;
    let mut param_strs: Vec<String> = Vec::new();
    while cur < token_count {
        let mut param_token: Token = tokens[cur as usize].clone();
        let mut param_kind: String = tok_kind(param_token.clone());
        let mut param_value: String = tok_value(param_token.clone());
        if param_kind == "RPAREN" {
            cur = cur + 1;
            break;
        } else if param_kind == "COMMA" {
            cur = cur + 1;
        } else if param_kind == "IDENT" {
            let mut param_type: String = param_value.clone();
            let mut pname_pos: i32 = cur + 1.clone();
            let mut pname_token: Token = tokens[pname_pos as usize].clone();
            let mut param_name: String = tok_value(pname_token.clone());
            let mut rust_param_type: String = resolve_type(param_type.clone(), shape_reg.clone(), enum_reg.clone());
            param_strs.push(s_join(vec![param_name.clone(), ": ".to_string(), rust_param_type.clone()]).clone());
            cur = pname_pos + 1;
        }
    }
    cur = adv_nl(cur.clone(), tokens.clone());
    let mut indent_pos: i32 = cur.clone();
    if cur < token_count {
        let mut indent_token: Token = tokens[cur as usize].clone();
        if tok_kind(indent_token.clone()) == "INDENT" {
            cur = cur + 1;
        }
    }
    let mut body_end_pos: i32 = find_block_end(tokens.clone(), indent_pos.clone());
    let mut body_start: i32 = cur.clone();
    let mut body_tokens: Vec<Token> = token_slice(tokens.clone(), body_start.clone(), body_end_pos + 1.clone());
    let mut body_len: i32 = body_tokens.len() as i32;
    let mut fn_mut_names: Vec<String> = collect_mut_names(body_tokens.clone(), 0, body_len - 1.clone());
/* unhandled(IDENT) */
    let body_ctx = Unknown { variant_reg, shape_reg, struct_reg, enum_reg, fn_mut_names, type_reg };
    let mut body_r: ParseResult = gen_block(body_tokens.clone(), 0, 1, body_ctx.clone());
    let mut body_code: String = pr_code(body_r.clone());
    let mut body_end: i32 = body_start + pr_pos(body_r.clone()).clone();
    let mut params_code: String = s_join_with(param_strs.clone(), ", ".to_string());
    let mut ret_suffix: String = "".to_string();
    if !is_empty(ret_type.clone()) {
        ret_suffix = s_join(vec![" -> ".to_string(), ret_type.clone()]);
    }
    let mut fn_code: String = s_join(vec!["fn ".to_string(), fn_name.clone(), "(".to_string(), params_code.clone(), ")".to_string(), ret_suffix.clone(), " {\n".to_string(), body_code.clone(), "}\n\n".to_string()]);
    return make_result(fn_code.clone(), body_end.clone());
}

fn filter_by_names(tokens: Vec<Token>, names: Vec<String>) -> Vec<Token> {
    let mut names_count: i32 = names.len() as i32;
    if names_count == 0 {
        return tokens;
    }
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = tokens.len() as i32;
    let mut cur: i32 = 0;
    while cur < token_count {
        let mut token: Token = tokens[cur as usize].clone();
        let mut kind: String = tok_kind(token.clone());
        if kind == "EOF" {
            break;
        }
        if kind == "NEWLINE" {
            cur = cur + 1;
            continue;
        }
        let mut is_fn: bool = kind == "KW_FN".clone();
        let mut is_struct: bool = kind == "KW_STRUCT".clone();
        let mut is_enum: bool = kind == "KW_ENUM".clone();
        let mut is_shape: bool = kind == "KW_SHAPE".clone();
        let mut is_block_decl: bool = is_fn || is_struct || is_enum.clone();
        if is_block_decl {
            let mut name_offset: i32 = 1;
            if is_fn {
                name_offset = 2;
            }
            let mut name_pos: i32 = cur + name_offset.clone();
            let mut decl_name: String = "".to_string();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                decl_name = tok_value(name_tok.clone());
            }
            let mut include: bool = list_has(names.clone(), decl_name.clone());
            let mut depth: i32 = 0;
            let mut entered: bool = false;
            while cur < token_count {
                let mut t: Token = tokens[cur as usize].clone();
                let mut tk: String = tok_kind(t.clone());
                if include {
                    result.push(t.clone());
                }
                cur = cur + 1;
                if tk == "INDENT" {
                    depth = depth + 1;
                    entered = true;
                } else if tk == "DEDENT" {
                    depth = depth - 1;
                    if depth == 0 && entered {
                        break;
                    }
                }
            }
        } else if is_shape {
            let mut name_pos: i32 = cur + 1.clone();
            let mut decl_name: String = "".to_string();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                decl_name = tok_value(name_tok.clone());
            }
            let mut include: bool = list_has(names.clone(), decl_name.clone());
            while cur < token_count {
                let mut t: Token = tokens[cur as usize].clone();
                let mut tk: String = tok_kind(t.clone());
                if include {
                    result.push(t.clone());
                }
                cur = cur + 1;
                if tk == "NEWLINE" {
                    break;
                }
            }
        } else {
            cur = cur + 1;
        }
    }
    return result;
}

fn collect_all_tokens_with_all_imports(path: String) -> Vec<Token> {
    let mut source: String = read_file(path.clone());
    let mut source_tokens: Vec<Token> = tokenize(source.clone());
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = source_tokens.len() as i32;
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    while true {
        if pos >= token_count {
            break;
        }
        let mut token: Token = source_tokens[pos as usize].clone();
        let mut current_kind: String = tok_kind(token.clone());
        if current_kind == "EOF" {
            break;
        }
        if current_kind == "INDENT" {
            depth = depth + 1;
            result.push(token.clone());
            pos = pos + 1;
            continue;
        }
        if current_kind == "DEDENT" {
            depth = depth - 1;
            result.push(token.clone());
            pos = pos + 1;
            continue;
        }
        let mut at_root: bool = depth == 0.clone();
        if current_kind == "LPAREN" && at_root {
            let mut import_names: Vec<String> = Vec::new();
            let mut scan: i32 = pos + 1.clone();
            while scan < token_count {
                let mut scan_token: Token = source_tokens[scan as usize].clone();
                let mut scan_kind: String = tok_kind(scan_token.clone());
                scan = scan + 1;
                if scan_kind == "RPAREN" {
                    break;
                }
                if scan_kind == "IDENT" {
                    import_names.push(tok_value(scan_token.clone()).clone());
                }
            }
            let mut path_pos: i32 = scan + 1.clone();
            if path_pos < token_count {
                let mut in_token: Token = source_tokens[scan as usize].clone();
                let mut is_in: bool = tok_kind(in_token.clone()) == "KW_IN";
                if is_in {
                    let mut path_token: Token = source_tokens[path_pos as usize].clone();
                    let mut imp_path: String = tok_value(path_token.clone());
                    let mut imp_tokens: Vec<Token> = collect_all_tokens_with_all_imports(imp_path.clone());
                    let mut filtered: Vec<Token> = filter_by_names(imp_tokens.clone(), import_names.clone());
                    let mut import_len: i32 = filtered.len() as i32;
                    for import_index in 0..import_len {
                        let mut import_token: Token = filtered[import_index as usize].clone();
                        let mut import_kind: String = tok_kind(import_token.clone());
                        let mut import_is_eof: bool = import_kind == "EOF".clone();
                        if !import_is_eof {
                            result.push(import_token.clone());
                        }
                    }
                    pos = path_pos + 1;
                    continue;
                }
            }
        }
        result.push(token.clone());
        pos = pos + 1;
    }
    return result;
}

fn generate(tokens: Vec<Token>) -> String {
    let mut struct_reg: Vec<String> = build_struct_reg(tokens.clone());
    let mut shape_reg: Vec<String> = build_shape_reg(tokens.clone());
    let mut enum_reg: Vec<String> = build_enum_reg(tokens.clone());
    let mut variant_reg: Vec<String> = build_variant_reg(tokens.clone(), enum_reg.clone());
    let mut type_reg: Vec<String> = build_type_reg(tokens.clone());
    let mut mut_names: Vec<String> = Vec::new();
/* unhandled(IDENT) */
    let ctx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg };
    let mut output: String = "".to_string();
    let mut token_count: i32 = tokens.len() as i32;
    let mut pos: i32 = 0;
    while true {
        if pos >= token_count {
            break;
        }
        let mut token: Token = tokens[pos as usize].clone();
        let mut current_kind: String = tok_kind(token.clone());
        if current_kind == "EOF" {
            break;
        }
        if current_kind == "NEWLINE" {
            pos = pos + 1;
            continue;
        }
        if current_kind == "KW_STRUCT" {
            let mut result: ParseResult = gen_struct_decl(tokens.clone(), pos.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if current_kind == "KW_SHAPE" {
            let mut result: ParseResult = gen_shape_decl(tokens.clone(), pos.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if current_kind == "KW_ENUM" {
            let mut result: ParseResult = gen_enum_decl(tokens.clone(), pos.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if current_kind == "KW_TYPE" {
            let mut result: ParseResult = gen_type_decl(tokens.clone(), pos.clone(), ctx.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if current_kind == "KW_FN" {
            let mut result: ParseResult = gen_fn_decl(tokens.clone(), pos.clone(), ctx.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        pos = pos + 1;
    }
    return output;
}

fn main() {
    let mut args: Vec<String> = get_args();
    let mut arg_count: i32 = args.len() as i32;
    if arg_count < 2 {
        println!("{}", "usage: deor input.deor output.rs".to_string());
    } else {
        let mut input_path: String = args[0 as usize].clone();
        let mut output_path: String = args[1 as usize].clone();
        let mut tokens: Vec<Token> = collect_all_tokens_with_all_imports(input_path.clone());
        let mut rust_code: String = generate(tokens.clone());
        write_file(output_path.clone(), rust_code.clone());
    }
}

