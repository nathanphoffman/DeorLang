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
    variant_reg: StrList,
    shape_reg: StrList,
    struct_reg: StrList,
    enum_reg: StrList,
    mut_names: StrList,
    type_reg: StrList,
    using_type: String,
    using_var: String,
    var_type_reg: StrList,
}

#[derive(Clone, PartialEq, Debug)]
struct TokenCursor {
    token_count: i32,
    pos: i32,
    current: Token,
}

fn f_read(path: String) -> String {
    std::fs::read_to_string(path.as_str()).expect("cannot read input file")
}

fn f_write(path: String, content: String) {
    std::fs::write(path.as_str(), content.as_str()).expect("cannot write output file");
}

fn f_args() -> Vec<String> {
    std::env::args().skip(1).collect()
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

fn s_from(source: String, start: i32) -> String {
    source.get(start as usize..).unwrap_or_default().to_string()
}

fn s_rtrim(source: String) -> String {
    source.trim_end().to_string()
}

fn s_trim(source: String) -> String {
    source.trim().to_string()
}

fn s_starts_with(source: String, prefix: String) -> bool {
    source.starts_with(prefix.as_str())
}

fn s_split(source: String, delimiter: String) -> Vec<String> {
    source.split(delimiter.as_str()).map(|s| s.to_string()).collect()
}

fn s_repeat(source: String, count: i32) -> String {
    source.repeat(count as usize)
}

fn s_pascal(source: String) -> String {
    {
    	let mut chars = source.chars();
    	match chars.next() {
    		None => String::new(),
    		Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    	}
    }
}

fn s_debug(source: String) -> String {
    format!("{:?}", source)
}

fn c_chars(source: String) -> Vec<String> {
    source.chars().map(|c| c.to_string()).collect()
}

fn c_alpha(character: String) -> bool {
    character.chars().next().map(|ch| ch.is_alphabetic() || ch == '_').unwrap_or(false)
}

fn c_digit(character: String) -> bool {
    character.chars().next().map(|ch| ch.is_ascii_digit()).unwrap_or(false)
}

fn c_alnum(character: String) -> bool {
    character.chars().next().map(|ch| ch.is_alphanumeric() || ch == '_').unwrap_or(false)
}

fn n_parse(source: String) -> i32 {
    source.parse::<i32>().unwrap_or(0)
}

fn n_to_str(number: i32) -> String {
    number.to_string()
}

fn is_upper_char(ch: String) -> bool {
    ch.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn is_lower_char(ch: String) -> bool {
    ch.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
}

fn is_pascal(name: String) -> bool {
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    if name_len == 0 {
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    return is_upper_char(first.clone());
}

fn is_camel(name: String) -> bool {
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    if name_len == 0 {
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    if !is_lower_char(first.clone()) {
        return false;
    }
    let mut idx: i32 = 0;
    while idx < name_len {
        let mut chr: String = chars[idx as usize].clone();
        if chr == "_" {
            return false;
        }
        idx = idx + 1;
    }
    return true;
}

fn is_snake(name: String) -> bool {
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    let mut idx: i32 = 0;
    while idx < name_len {
        let mut chr: String = chars[idx as usize].clone();
        if is_upper_char(chr.clone()) {
            return false;
        }
        idx = idx + 1;
    }
    return true;
}

fn val_err(line_num: i32, label: String, name: String, rule: String) -> String {
    return s_cat("[validation] line ".to_string(), s_cat(n_to_str(line_num.clone()), s_cat(": ".to_string(), s_cat(label.clone(), s_cat(" '".to_string(), s_cat(name.clone(), s_cat("' - ".to_string(), rule.clone())))))));
}

fn validate_tokens(tokens: Vec<Token>) {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut errors: Vec<String> = Vec::new();
    let mut pos: i32 = 0;
    while pos < token_count {
        let mut tok: Token = tokens[pos as usize].clone();
        let kind = tok.kind.clone();
        let mut cur_kind: String = kind.clone();
        let line = tok.line.clone();
        let mut cur_line: i32 = line.clone();
        if cur_kind == "KW_STRUCT" {
            let mut name_pos: i32 = pos + 1.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(cur_line.clone(), "struct".to_string(), name_val.clone(), "name must be at least 3 characters".to_string()).clone());
                    }
                    if !is_pascal(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), "struct".to_string(), name_val.clone(), "name must be PascalCase (start with uppercase letter)".to_string()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "KW_ENUM" {
            let mut name_pos: i32 = pos + 1.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(cur_line.clone(), "enum".to_string(), name_val.clone(), "name must be at least 3 characters".to_string()).clone());
                    }
                    if !is_pascal(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), "enum".to_string(), name_val.clone(), "name must be PascalCase (start with uppercase letter)".to_string()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "KW_SHAPE" {
            let mut name_pos: i32 = pos + 1.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(cur_line.clone(), "shape".to_string(), name_val.clone(), "name must be at least 3 characters".to_string()).clone());
                    }
                    if !is_camel(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), "shape".to_string(), name_val.clone(), "name must be camelCase (start lowercase, no underscores)".to_string()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "KW_TYPE" {
            let mut name_pos: i32 = pos + 1.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(cur_line.clone(), "type".to_string(), name_val.clone(), "name must be at least 3 characters".to_string()).clone());
                    }
                    if !is_camel(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), "type".to_string(), name_val.clone(), "name must be camelCase (start lowercase, no underscores)".to_string()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "KW_FN" {
            let mut name_pos: i32 = pos + 2.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(cur_line.clone(), "fn".to_string(), name_val.clone(), "name must be at least 3 characters".to_string()).clone());
                    }
                    if !is_snake(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), "fn".to_string(), name_val.clone(), "name must be lower_snake_case (no uppercase letters)".to_string()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "IDENT" {
            let mut next1: i32 = pos + 1.clone();
            let mut next2: i32 = pos + 2.clone();
            if next2 < token_count {
                let mut tok_one: Token = tokens[next1 as usize].clone();
                let mut tok_two: Token = tokens[next2 as usize].clone();
                let kind = tok_one.kind.clone();
                let mut one_kind: String = kind.clone();
                let kind = tok_two.kind.clone();
                let mut two_kind: String = kind.clone();
                if one_kind == "IDENT" && two_kind == "EQUALS" {
                    let value = tok_one.value.clone();
                    let line = tok_one.line.clone();
                    let mut var_name: String = value.clone();
                    let mut var_line: i32 = line.clone();
                    if (var_name.len() as i32) < 3 {
                        errors.push(val_err(var_line.clone(), "variable".to_string(), var_name.clone(), "name must be at least 3 characters".to_string()).clone());
                    }
                    if !is_snake(var_name.clone()) {
                        errors.push(val_err(var_line.clone(), "variable".to_string(), var_name.clone(), "name must be lower_snake_case (no uppercase letters)".to_string()).clone());
                    }
                }
            }
        }
        pos = pos + 1;
    }
    let mut error_count: i32 = (errors.len() as i32);
    if error_count > 0 {
        let mut err_idx: i32 = 0;
        while err_idx < error_count {
            let mut err_msg: String = errors[err_idx as usize].clone();
            println!("{}", err_msg.clone());
            err_idx = err_idx + 1;
        }
        std::process::exit(1);
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

fn adv_nl(pos: i32, tokens: Vec<Token>) -> i32 {
    let mut token_count: i32 = (tokens.len() as i32);
    if pos < token_count {
        let mut cur_token: Token = tokens[pos as usize].clone();
        let kind = cur_token.kind.clone();
        if kind == "NEWLINE" {
            return pos + 1;
        }
    }
    return pos;
}

fn adv_indent(pos: i32, tokens: Vec<Token>) -> i32 {
    let mut token_count: i32 = (tokens.len() as i32);
    if pos < token_count {
        let mut cur_token: Token = tokens[pos as usize].clone();
        let kind = cur_token.kind.clone();
        if kind == "INDENT" {
            return pos + 1;
        }
    }
    return pos;
}

fn skip_to_body(tokens: Vec<Token>, pos: i32) -> i32 {
    let mut cur: i32 = adv_nl(pos.clone(), tokens.clone());
    cur = adv_indent(cur.clone(), tokens.clone());
    return cur;
}

fn is_empty(source: String) -> bool {
    let mut length: i32 = (source.len() as i32);
    return length == 0;
}

fn str_eq(left: String, right: String) -> bool {
    return left == right;
}

fn reg_get_stride(pairs: Vec<String>, key: String, stride: i32) -> String {
    let mut pairs_count: i32 = (pairs.len() as i32);
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
    let mut pairs_count: i32 = (pairs.len() as i32);
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
    let mut item_count: i32 = (items.len() as i32);
    for index in 0..item_count {
        let mut item: String = items[index as usize].clone();
        if item == val {
            return true;
        }
    }
    return false;
}

fn skip_to_block_start(tokens: Vec<Token>, start: i32) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = start.clone();
    for skip_index in start..token_count {
        let mut skip_token: Token = tokens[skip_index as usize].clone();
        let kind = skip_token.kind.clone();
        cur = skip_index + 1;
        if kind == "INDENT" {
            break;
        }
    }
    return make_result("".to_string(), cur.clone());
}

fn collect_struct_fields(tokens: Vec<Token>, start: i32) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i32 = start.clone();
    for field_index in start..token_count {
        let mut field_token: Token = tokens[field_index as usize].clone();
        let kind = field_token.kind.clone();
        if kind == "DEDENT" {
            cur = field_index + 1;
            break;
        } else if kind == "IDENT" {
            let mut fname_pos: i32 = field_index + 1.clone();
            if fname_pos < token_count {
                let mut fname_token: Token = tokens[fname_pos as usize].clone();
                let kind = fname_token.kind.clone();
                let value = fname_token.value.clone();
                if kind == "IDENT" {
                    fields.push(value.clone());
                    cur = fname_pos + 1;
                }
            }
        }
    }
    return make_result(s_join_with(fields.clone(), ",".to_string()), cur.clone());
}

fn build_struct_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
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
                let kind = name_token.kind.clone();
                let value = name_token.value.clone();
                if kind == "IDENT" {
                    let mut block_r: ParseResult = skip_to_block_start(tokens.clone(), name_pos + 1.clone());
                    let mut fields_r: ParseResult = collect_struct_fields(tokens.clone(), pr_pos(block_r.clone()));
                    result.push(value.clone());
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
    let mut token_count: i32 = (tokens.len() as i32);
    for index in 0..token_count {
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_SHAPE" {
            let mut name_pos: i32 = index + 1.clone();
            let mut form_pos: i32 = index + 3.clone();
            let mut elem_pos: i32 = index + 5.clone();
            if elem_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut form_token: Token = tokens[form_pos as usize].clone();
                let mut elem_token: Token = tokens[elem_pos as usize].clone();
                let value = name_token.value.clone();
                let mut shape_name: String = value.clone();
                let kind = form_token.kind.clone();
                let value = elem_token.value.clone();
                let mut elem_type: String = value.clone();
                if kind == "KW_LIST" {
                    result.push(shape_name.clone());
                    result.push(elem_type.clone());
                } else {
                    let mut out_pos: i32 = index + 7.clone();
                    if out_pos < token_count {
                        let mut out_token: Token = tokens[out_pos as usize].clone();
                        let value = out_token.value.clone();
                        let mut out_type: String = value.clone();
                        result.push(shape_name.clone());
                        result.push(s_join(vec!["fn:".to_string(), elem_type.clone(), ":".to_string(), out_type.clone()]).clone());
                    }
                }
            }
        }
    }
    return result;
}

fn build_enum_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    for index in 0..token_count {
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            let mut name_pos: i32 = index + 1.clone();
            if name_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let value = name_token.value.clone();
                let mut enum_name: String = value.clone();
                let mut rust_name: String = s_pascal(enum_name.clone());
                result.push(enum_name.clone());
                result.push(rust_name.clone());
            }
        }
    }
    return result;
}

fn build_variant_reg(tokens: Vec<Token>, enum_reg: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
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
                let value = name_token.value.clone();
                let mut enum_name: String = value.clone();
                let mut rust_name: String = reg_get(enum_reg.clone(), enum_name.clone());
                raw_i = name_pos + 1;
                while raw_i < token_count {
                    let mut skip_token: Token = tokens[raw_i as usize].clone();
                    let kind = skip_token.kind.clone();
                    raw_i = raw_i + 1;
                    if kind == "INDENT" {
                        break;
                    }
                }
                while raw_i < token_count {
                    let mut variant_token: Token = tokens[raw_i as usize].clone();
                    let kind = variant_token.kind.clone();
                    let value = variant_token.value.clone();
                    raw_i = raw_i + 1;
                    if kind == "DEDENT" {
                        break;
                    } else if kind == "IDENT" {
                        result.push(value.clone());
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
    let mut token_count: i32 = (tokens.len() as i32);
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
                let value = name_token.value.clone();
                result.push(value.clone());
                let value = param_type_token.value.clone();
                result.push(value.clone());
                let value = param_name_token.value.clone();
                result.push(value.clone());
            }
        }
    }
    return result;
}

fn build_var_type_reg(tokens: Vec<Token>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut tok_idx: i32 = 0;
    while tok_idx < token_count {
        let mut tok: Token = tokens[tok_idx as usize].clone();
        let kind = tok.kind.clone();
        let value = tok.value.clone();
        if kind == "IDENT" {
            let mut next_i: i32 = tok_idx + 1.clone();
            let mut eq_i: i32 = tok_idx + 2.clone();
            if eq_i < token_count {
                let mut next_tok: Token = tokens[next_i as usize].clone();
                let mut eq_tok: Token = tokens[eq_i as usize].clone();
                let kind = next_tok.kind.clone();
                let mut next_is_ident: bool = kind == "IDENT".clone();
                let value = next_tok.value.clone();
                let mut var_name: String = value.clone();
                let kind = eq_tok.kind.clone();
                let mut eq_is_equals: bool = kind == "EQUALS".clone();
                if next_is_ident && eq_is_equals {
                    let mut type_tok: Token = tokens[tok_idx as usize].clone();
                    let value = type_tok.value.clone();
                    let mut type_name: String = value.clone();
                    let mut is_int: bool = type_name == "int".clone();
                    let mut is_string: bool = type_name == "string".clone();
                    let mut is_bool: bool = type_name == "bool".clone();
                    let mut is_float: bool = type_name == "float".clone();
                    let mut is_void: bool = type_name == "void".clone();
                    let mut is_primitive: bool = is_int || is_string || is_bool || is_float || is_void.clone();
                    if !is_primitive {
                        result.push(var_name.clone());
                        result.push(type_name.clone());
                    }
                }
            }
        }
        tok_idx = tok_idx + 1;
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
        if s_starts_with(elem_type.clone(), "fn:".to_string()) {
            let mut parts: Vec<String> = s_split(elem_type.clone(), ":".to_string());
            let mut in_type: String = parts[1 as usize].clone();
            let mut out_type: String = parts[2 as usize].clone();
            let mut rust_in: String = render_rust_type(in_type.clone());
            let mut rust_out: String = render_rust_type(out_type.clone());
            if is_empty(rust_out.clone()) {
                return s_join(vec!["fn(".to_string(), rust_in.clone(), ")".to_string()]);
            }
            return s_join(vec!["fn(".to_string(), rust_in.clone(), ") -> ".to_string(), rust_out.clone()]);
        }
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        return s_join(vec!["Vec<".to_string(), rust_elem.clone(), ">".to_string()]);
    }
    return render_rust_type(type_name.clone());
}

fn find_block_end(tokens: Vec<Token>, indent_pos: i32) -> i32 {
    let mut token_count: i32 = (tokens.len() as i32);
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
                let kind = prev_token.kind.clone();
                let value = prev_token.value.clone();
                if kind == "IDENT" {
                    if !list_has(result.clone(), value.clone()) {
                        result.push(value.clone());
                    }
                }
            }
        }
        if kind == "KW_USING" {
            let mut using_var_pos: i32 = raw_i + 1.clone();
            if using_var_pos < end_pos {
                let mut using_var_token: Token = tokens[using_var_pos as usize].clone();
                let value = using_var_token.value.clone();
                if !list_has(result.clone(), value.clone()) {
                    result.push(value.clone());
                }
            }
        }
    }
    return result;
}

fn cur_at(tokens: Vec<Token>, pos: i32) -> TokenCursor {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut current: Token = tokens[pos as usize].clone();
    let c = TokenCursor { token_count, pos, current };
    return c;
}

fn cur_next(c: TokenCursor, tokens: Vec<Token>) -> TokenCursor {
    let token_count = c.token_count.clone();
    let mut pos = c.pos.clone();
    let mut current = c.current.clone();
    let mut pos: i32 = pos + 1.clone();
    if pos < token_count {
        let mut current: Token = tokens[pos as usize].clone();
        return TokenCursor { token_count, pos, current };
    }
    return TokenCursor { token_count, pos, current };
}

fn c_at_end(c: TokenCursor) -> bool {
    let token_count = c.token_count.clone();
    let pos = c.pos.clone();
    return pos >= token_count;
}

fn cur_skip_to_body(c: TokenCursor, tokens: Vec<Token>) -> TokenCursor {
    let pos = c.pos.clone();
    let mut body_pos: i32 = adv_nl(pos.clone(), tokens.clone());
    body_pos = adv_indent(body_pos.clone(), tokens.clone());
    return cur_at(tokens.clone(), body_pos.clone());
}

fn cur_peek(c: TokenCursor, tokens: Vec<Token>, offset: i32) -> Token {
    let pos = c.pos.clone();
    let mut peek_pos: i32 = pos + offset.clone();
    return tokens[peek_pos as usize].clone();
}

fn l_slice(tokens: Vec<Token>, start: i32, end_val: i32) -> Vec<Token> {
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
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
    if word == "with" {
        return "KW_WITH".to_string();
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
    return s_pascal(type_name.clone());
}

fn find_struct_for_fields(struct_reg: Vec<String>, fields: Vec<String>) -> String {
    let mut fields_key: String = s_join_with(fields.clone(), ",".to_string());
    let mut reg_count: i32 = (struct_reg.len() as i32);
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

fn find_struct_for_field(struct_reg: Vec<String>, field: String) -> String {
    let mut reg_count: i32 = (struct_reg.len() as i32);
    let mut next_is_val: bool = false;
    let mut cur_name: String = "".to_string();
    for index in 0..reg_count {
        let mut item: String = struct_reg[index as usize].clone();
        if next_is_val {
            let mut fields: Vec<String> = s_split(item.clone(), ",".to_string());
            let mut has_field: bool = list_has(fields.clone(), field.clone());
            if has_field {
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
    let mut token_count: i32 = (tokens.len() as i32);
    while true {
        if cur >= token_count {
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "RPAREN" {
            break;
        }
        if kind == "COMMA" {
            cur = cur + 1;
            continue;
        }
        let mut arg_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        let mut arg_code: String = pr_code(arg_r.clone());
        let mut arg_pos: i32 = pr_pos(arg_r.clone());
        let mut start_token: Token = tokens[cur as usize].clone();
        let kind = start_token.kind.clone();
        if kind == "STRING" {
            arg_code = s_join(vec![arg_code.clone(), ".to_string()".to_string()]);
        } else if kind == "IDENT" {
            let mut next_cur: i32 = cur + 1.clone();
            let mut peek_is_call: bool = false;
            let mut peek_is_idx: bool = false;
            if next_cur < token_count {
                let mut next_tok: Token = tokens[next_cur as usize].clone();
                let kind = next_tok.kind.clone();
                peek_is_call = kind == "LPAREN";
                peek_is_idx = kind == "KW_AT";
            }
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
    let mut token_count: i32 = (tokens.len() as i32);
    while true {
        if cur >= token_count {
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "RBRACKET" {
            break;
        }
        if kind == "COMMA" {
            cur = cur + 1;
            continue;
        }
        let mut item_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        let mut item_code: String = pr_code(item_r.clone());
        let mut item_pos: i32 = pr_pos(item_r.clone());
        let mut start_token: Token = tokens[cur as usize].clone();
        let kind = start_token.kind.clone();
        if kind == "STRING" {
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

fn gen_primary(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut token: Token = tokens[pos as usize].clone();
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    if kind == "INT" {
        return make_result(value.clone(), pos + 1.clone());
    }
    if kind == "STRING" {
        return make_result(s_debug(value.clone()), pos + 1.clone());
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
            let kind = next_token.kind.clone();
            if kind == "RBRACKET" {
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
            let kind = peek_token.kind.clone();
            if kind == "KW_AVOW" {
                let mut expr_pos: i32 = peek_pos + 1.clone();
                let mut expr_r: ParseResult = gen_expr(tokens.clone(), expr_pos.clone(), ctx.clone());
                let mut expr_code: String = pr_code(expr_r.clone());
                let mut after_rparen: i32 = pr_pos(expr_r.clone()) + 1;
                return make_result(s_join(vec![expr_code.clone(), ".unwrap()".to_string()]), after_rparen.clone());
            }
            let mut fields: Vec<String> = Vec::new();
            let mut cur: i32 = peek_pos.clone();
            while cur < token_count {
                let mut field_token: Token = tokens[cur as usize].clone();
                let kind = field_token.kind.clone();
                let value = field_token.value.clone();
                if kind == "RPAREN" {
                    cur = cur + 1;
                    break;
                } else if kind == "COMMA" {
                    cur = cur + 1;
                } else if kind == "IDENT" {
                    fields.push(value.clone());
                    cur = cur + 1;
                }
            }
            let mut struct_name: String = find_struct_for_fields(struct_reg.clone(), fields.clone());
            let mut fields_code: String = s_join_with(fields.clone(), ", ".to_string());
            return make_result(s_join(vec![struct_name.clone(), " { ".to_string(), fields_code.clone(), " }".to_string()]), cur.clone());
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
            let kind = next_token.kind.clone();
            if kind == "LPAREN" {
                let mut func_name: String = value.clone();
                let mut args_pos: i32 = next_pos + 1.clone();
                if func_name == "len" {
                    let mut len_r: ParseResult = gen_unary_method(tokens.clone(), args_pos.clone(), ".len() as i32".to_string(), ctx.clone());
                    let mut len_code: String = pr_code(len_r.clone());
                    let mut len_end: i32 = pr_pos(len_r.clone());
                    return make_result(s_join(vec!["(".to_string(), len_code.clone(), ")".to_string()]), len_end.clone());
                }
                let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
                let mut args_code: String = pr_code(args_r.clone());
                let mut args_end: i32 = pr_pos(args_r.clone());
                let mut after_paren: i32 = args_end + 1.clone();
                return make_result(s_join(vec![func_name.clone(), "(".to_string(), args_code.clone(), ")".to_string()]), after_paren.clone());
            }
            if kind == "KW_AT" {
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
    let mut token_count: i32 = (tokens.len() as i32);
    let mut first_token: Token = tokens[pos as usize].clone();
    let kind = first_token.kind.clone();
    let mut left_has_str: bool = kind == "STRING".clone();
    while cur_pos < token_count {
        let mut operator_token: Token = tokens[cur_pos as usize].clone();
        let kind = operator_token.kind.clone();
        let value = operator_token.value.clone();
        if !is_binary_op(kind.clone()) {
            break;
        }
        let mut operator_str: String = value.clone();
        let mut after_op: i32 = cur_pos + 1.clone();
        if kind == "KW_IS" {
            if after_op < token_count {
                let mut maybe_not: Token = tokens[after_op as usize].clone();
                let kind = maybe_not.kind.clone();
                if kind == "KW_NOT" {
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
            let kind = left_token.kind.clone();
            let mut left_is_str: bool = kind == "STRING".clone();
            let mut rhs_token: Token = tokens[after_op as usize].clone();
            let kind = rhs_token.kind.clone();
            let mut rhs_is_str: bool = kind == "STRING".clone();
            let mut rhs_is_ident: bool = kind == "IDENT".clone();
            if left_is_str {
                left_code = s_join(vec!["format!(\"{}{}\", ".to_string(), left_code.clone(), ", ".to_string(), rhs_code.clone(), ")".to_string()]);
                left_has_str = true;
                cur_pos = rhs_pos;
                continue;
            }
            if left_has_str {
                if rhs_is_ident {
                    rhs_code = s_join(vec![rhs_code.clone(), ".as_str()".to_string()]);
                }
            }
            if rhs_is_str {
                left_has_str = true;
            }
        }
        left_code = s_join(vec![left_code.clone(), " ".to_string(), rust_op.clone(), " ".to_string(), rhs_code.clone()]);
        cur_pos = rhs_pos;
    }
    return make_result(left_code.clone(), cur_pos.clone());
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

fn make_destruct_code(var_name: String, struct_type: String, struct_reg: Vec<String>, mut_names: Vec<String>, pad: String) -> String {
    let mut fields_str: String = reg_get(struct_reg.clone(), struct_type.clone());
    if is_empty(fields_str.clone()) {
        return "".to_string();
    }
    let mut fields: Vec<String> = s_split(fields_str.clone(), ",".to_string());
    let mut field_count: i32 = (fields.len() as i32);
    let mut lines: Vec<String> = Vec::new();
    for i in 0..field_count {
        let mut field: String = fields[i as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            mut_kw = "mut ".to_string();
        }
        lines.push(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), field.clone(), " = ".to_string(), var_name.clone(), ".".to_string(), field.clone(), ".clone();".to_string()]).clone());
    }
    let mut code: String = s_join_nl(lines.clone());
    return s_cat(code.clone(), "\n".to_string());
}

fn gen_destructure(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let using_type = ctx.using_type.clone();
    let using_var = ctx.using_var.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pad: String = s_repeat("    ".to_string(), depth.clone());
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i32 = pos + 1.clone();
    while cur < token_count {
        let mut field_token: Token = tokens[cur as usize].clone();
        let kind = field_token.kind.clone();
        let value = field_token.value.clone();
        if kind == "RPAREN" {
            cur = cur + 1;
            break;
        } else if kind == "COMMA" {
            cur = cur + 1;
        } else if kind == "IDENT" {
            fields.push(value.clone());
            cur = cur + 1;
        }
    }
    let mut src_pos: i32 = cur + 1.clone();
    let mut src_r: ParseResult = gen_expr(tokens.clone(), src_pos.clone(), ctx.clone());
    let mut src_code: String = pr_code(src_r.clone());
    let mut src_end: i32 = pr_pos(src_r.clone());
    let mut after: i32 = adv_nl(src_end.clone(), tokens.clone());
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = (fields.len() as i32);
    for field_index in 0..field_count {
        let mut field: String = fields[field_index as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            mut_kw = "mut ".to_string();
        }
        dest_lines.push(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), field.clone(), " = ".to_string(), src_code.clone(), ".".to_string(), field.clone(), ".clone();".to_string()]).clone());
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
    let mut using_type = ctx.using_type.clone();
    let mut using_var = ctx.using_var.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut token: Token = tokens[pos as usize].clone();
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    let mut pad: String = s_repeat("    ".to_string(), depth.clone());
    if kind == "KW_RUST" {
        let mut block_pos: i32 = pos + 2.clone();
        let mut block_token: Token = tokens[block_pos as usize].clone();
        let value = block_token.value.clone();
        let mut block_content: String = value.clone();
        let mut rust_lines: Vec<String> = s_split(block_content.clone(), "\n".to_string());
        let mut padded: Vec<String> = Vec::new();
        let mut rust_line_count: i32 = (rust_lines.len() as i32);
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
        let kind = val_token.kind.clone();
        let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let mut val_code: String = pr_code(val_r.clone());
        let mut val_end: i32 = pr_pos(val_r.clone());
        let mut ret_suffix: String = "".to_string();
        if kind == "STRING" {
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
            let kind = peek_token.kind.clone();
            if kind == "KW_AVOW" {
                let mut expr_pos: i32 = peek_pos + 1.clone();
                let mut expr_r: ParseResult = gen_expr(tokens.clone(), expr_pos.clone(), ctx.clone());
                let mut expr_code: String = pr_code(expr_r.clone());
                let mut after_rparen: i32 = pr_pos(expr_r.clone()) + 1;
                return make_result(s_join(vec![pad.clone(), expr_code.clone(), ".unwrap();\n".to_string()]), adv_nl(after_rparen.clone(), tokens.clone()));
            }
        }
        return gen_destructure(tokens.clone(), pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_USING" {
        let mut var_pos: i32 = pos + 1.clone();
        let mut var_token: Token = tokens[var_pos as usize].clone();
        let value = var_token.value.clone();
        let mut using_var: String = value.clone();
        let mut struct_type: String = reg_get(var_type_reg.clone(), using_var.clone());
        let mut using_type: String = struct_type.clone();
        let mut init_destruct: String = make_destruct_code(using_var.clone(), struct_type.clone(), struct_reg.clone(), mut_names.clone(), pad.clone());
        let mut body_start: i32 = skip_to_body(tokens.clone(), var_pos + 1.clone());
/* unhandled(IDENT) */
        let using_ctx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, using_type, using_var, var_type_reg };
        let mut block_r: ParseResult = gen_block(tokens.clone(), body_start.clone(), depth.clone(), using_ctx.clone());
        let mut full_code: String = s_cat(init_destruct.clone(), pr_code(block_r.clone()));
        return make_result(full_code.clone(), pr_pos(block_r.clone()));
    }
    if kind == "IDENT" {
        let mut ident_name: String = value.clone();
        let mut next_pos: i32 = pos + 1.clone();
        if next_pos >= token_count {
            return make_result("/* eof */\n".to_string(), pos + 1.clone());
        }
        let mut next_token: Token = tokens[next_pos as usize].clone();
        let kind = next_token.kind.clone();
        if kind == "KW_AS" {
            let mut after_as: i32 = next_pos + 1.clone();
            let mut after_as_token: Token = tokens[after_as as usize].clone();
            let kind = after_as_token.kind.clone();
            let value = after_as_token.value.clone();
            let mut after_as_value: String = value.clone();
            if kind == "LPAREN" {
                let mut fields: Vec<String> = Vec::new();
                let mut fend: i32 = after_as + 1.clone();
                while fend < token_count {
                    let mut field_token: Token = tokens[fend as usize].clone();
                    let kind = field_token.kind.clone();
                    let value = field_token.value.clone();
                    if kind == "RPAREN" {
                        fend = fend + 1;
                        break;
                    } else if kind == "COMMA" {
                        fend = fend + 1;
                    } else if kind == "IDENT" {
                        fields.push(value.clone());
                        fend = fend + 1;
                    }
                }
                let mut struct_name: String = find_struct_for_fields(struct_reg.clone(), fields.clone());
                let mut is_mut: bool = list_has(mut_names.clone(), ident_name.clone());
                let mut mut_kw: String = "".to_string();
                if is_mut {
                    mut_kw = "mut ".to_string();
                }
                let mut fields_code: String = s_join_with(fields.clone(), ", ".to_string());
                let mut stmt_code: String = s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), ident_name.clone(), " = ".to_string(), struct_name.clone(), " { ".to_string(), fields_code.clone(), " };\n".to_string()]);
                return make_result(stmt_code.clone(), adv_nl(fend.clone(), tokens.clone()));
            }
            if kind == "KW_EMPTY" {
                return make_result(s_join(vec![pad.clone(), "let mut ".to_string(), ident_name.clone(), " = Vec::new();\n".to_string()]), adv_nl(after_as + 1.clone(), tokens.clone()));
            }
            if kind == "IDENT" {
                let mut maybe_with_pos: i32 = after_as + 1.clone();
                if maybe_with_pos < token_count {
                    let mut maybe_with_token: Token = tokens[maybe_with_pos as usize].clone();
                    let kind = maybe_with_token.kind.clone();
                    if kind == "KW_WITH" {
                        let mut source_name: String = after_as_value.clone();
                        let mut lparen_pos: i32 = maybe_with_pos + 1.clone();
                        let mut override_fields: Vec<String> = Vec::new();
                        let mut fend: i32 = lparen_pos + 1.clone();
                        while fend < token_count {
                            let mut field_tok: Token = tokens[fend as usize].clone();
                            let kind = field_tok.kind.clone();
                            let value = field_tok.value.clone();
                            if kind == "RPAREN" {
                                fend = fend + 1;
                                break;
                            }
                            if kind == "COMMA" {
                                fend = fend + 1;
                                continue;
                            }
                            if kind == "IDENT" {
                                override_fields.push(value.clone());
                                fend = fend + 1;
                            }
                        }
                        let mut first_field: String = override_fields[0 as usize].clone();
                        let mut struct_name: String = find_struct_for_field(struct_reg.clone(), first_field.clone());
                        let mut fields_code: String = s_join_with(override_fields.clone(), ", ".to_string());
                        let mut is_mut: bool = list_has(mut_names.clone(), ident_name.clone());
                        let mut mut_kw: String = "".to_string();
                        if is_mut {
                            mut_kw = "mut ".to_string();
                        }
                        let mut with_code: String = s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), ident_name.clone(), " = ".to_string(), struct_name.clone(), " { ".to_string(), fields_code.clone(), ", ..".to_string(), source_name.clone(), " };\n".to_string()]);
                        return make_result(with_code.clone(), adv_nl(fend.clone(), tokens.clone()));
                    }
                }
            }
            let kind = after_as_token.kind.clone();
            let mut val_r: ParseResult = gen_expr(tokens.clone(), after_as.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut is_str: bool = kind == "STRING".clone();
            let mut is_mut: bool = list_has(mut_names.clone(), ident_name.clone());
            let mut mut_kw: String = "".to_string();
            if is_mut {
                mut_kw = "mut ".to_string();
            }
            let mut suffix: String = "".to_string();
            if is_str {
                suffix = ".to_string()".to_string();
            }
            return make_result(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), ident_name.clone(), " = ".to_string(), val_code.clone(), suffix.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
        }
        if kind == "LPAREN" {
            let mut args_pos: i32 = next_pos + 1.clone();
            let mut peek_rparen: Token = tokens[args_pos as usize].clone();
            let kind = peek_rparen.kind.clone();
            let mut is_zero_arg: bool = kind == "RPAREN".clone();
            let mut has_using: bool = !is_empty(using_var.clone());
            if is_zero_arg && has_using {
                let mut after_rparen: i32 = args_pos + 1.clone();
                let mut has_with_arg: bool = false;
                let mut extra_arg: String = "".to_string();
                let mut after_with: i32 = after_rparen.clone();
                if after_rparen < token_count {
                    let mut with_token: Token = tokens[after_rparen as usize].clone();
                    let kind = with_token.kind.clone();
                    if kind == "KW_WITH" {
                        let mut extra_pos: i32 = after_rparen + 1.clone();
                        if extra_pos < token_count {
                            let mut extra_token: Token = tokens[extra_pos as usize].clone();
                            let kind = extra_token.kind.clone();
                            let value = extra_token.value.clone();
                            if kind == "IDENT" {
                                has_with_arg = true;
                                extra_arg = value;
                                after_with = extra_pos + 1;
                            }
                        }
                    }
                }
                let mut shim_code: String = "".to_string();
                if has_with_arg {
                    shim_code = s_join(vec![pad.clone(), using_var.clone(), " = ".to_string(), ident_name.clone(), "(".to_string(), using_var.clone(), ".clone(), ".to_string(), extra_arg.clone(), ".clone());\n".to_string()]);
                } else {
                    shim_code = s_join(vec![pad.clone(), using_var.clone(), " = ".to_string(), ident_name.clone(), "(".to_string(), using_var.clone(), ".clone());\n".to_string()]);
                }
                let mut re_destruct: String = make_destruct_code(using_var.clone(), using_type.clone(), struct_reg.clone(), mut_names.clone(), pad.clone());
                shim_code = s_cat(shim_code.clone(), re_destruct.clone());
                return make_result(shim_code.clone(), adv_nl(after_with.clone(), tokens.clone()));
            }
            let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
            let mut args_code: String = pr_code(args_r.clone());
            let mut args_end: i32 = pr_pos(args_r.clone());
            let mut after_paren: i32 = args_end + 1.clone();
            let mut call_code: String = "".to_string();
            if ident_name == "print" {
                call_code = s_join(vec![pad.clone(), "println!(\"{}\", ".to_string(), args_code.clone(), ");\n".to_string()]);
            } else {
                call_code = s_join(vec![pad.clone(), ident_name.clone(), "(".to_string(), args_code.clone(), ");\n".to_string()]);
            }
            return make_result(call_code.clone(), adv_nl(after_paren.clone(), tokens.clone()));
        }
        if kind == "KW_INSERT" {
            let mut val_pos: i32 = next_pos + 1.clone();
            let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut val_tok: Token = tokens[val_pos as usize].clone();
            let kind = val_tok.kind.clone();
            return make_result(s_join(vec![pad.clone(), ident_name.clone(), ".push(".to_string(), emit_val(val_code.clone(), kind.clone()).clone(), ");\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
        }
        if kind == "KW_AT" {
            let mut after_at: i32 = next_pos + 1.clone();
            if after_at < token_count {
                let mut at_next_token: Token = tokens[after_at as usize].clone();
                let kind = at_next_token.kind.clone();
                let value = at_next_token.value.clone();
                if kind == "IDENT" && value == "end" {
                    let mut val_pos: i32 = after_at + 2.clone();
                    let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                    let mut val_code: String = pr_code(val_r.clone());
                    let mut val_end: i32 = pr_pos(val_r.clone());
                    let mut val_tok: Token = tokens[val_pos as usize].clone();
                    let kind = val_tok.kind.clone();
                    return make_result(s_join(vec![pad.clone(), ident_name.clone(), ".push(".to_string(), emit_val(val_code.clone(), kind.clone()).clone(), ");\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
                }
                let mut idx_r: ParseResult = gen_expr(tokens.clone(), after_at.clone(), ctx.clone());
                let mut idx_code: String = pr_code(idx_r.clone());
                let mut idx_end: i32 = pr_pos(idx_r.clone());
                let mut val_pos: i32 = idx_end + 1.clone();
                let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                let mut val_code: String = pr_code(val_r.clone());
                let mut val_end: i32 = pr_pos(val_r.clone());
                let mut val_tok: Token = tokens[val_pos as usize].clone();
                let kind = val_tok.kind.clone();
                return make_result(s_join(vec![pad.clone(), ident_name.clone(), "[".to_string(), idx_code.clone(), " as usize] = ".to_string(), emit_val(val_code.clone(), kind.clone()).clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
            }
        }
        if kind == "KW_REMOVE" {
            let mut idx_pos: i32 = next_pos + 2.clone();
            let mut idx_r: ParseResult = gen_expr(tokens.clone(), idx_pos.clone(), ctx.clone());
            let mut idx_code: String = pr_code(idx_r.clone());
            let mut idx_end: i32 = pr_pos(idx_r.clone());
            return make_result(s_join(vec![pad.clone(), ident_name.clone(), ".remove(".to_string(), idx_code.clone(), " as usize);\n".to_string()]), adv_nl(idx_end.clone(), tokens.clone()));
        }
        if kind == "EQUALS" {
            let mut val_pos: i32 = next_pos + 1.clone();
            let mut eq_val_token: Token = tokens[val_pos as usize].clone();
            let kind = eq_val_token.kind.clone();
            let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut assign_suffix: String = "".to_string();
            if kind == "STRING" {
                assign_suffix = ".to_string()".to_string();
            }
            return make_result(s_join(vec![pad.clone(), ident_name.clone(), " = ".to_string(), val_code.clone(), assign_suffix.clone(), ";\n".to_string()]), adv_nl(val_end.clone(), tokens.clone()));
        }
        if kind == "IDENT" {
            let mut eq_pos: i32 = next_pos + 1.clone();
            if eq_pos < token_count {
                let mut eq_token: Token = tokens[eq_pos as usize].clone();
                let kind = eq_token.kind.clone();
                if kind == "EQUALS" {
                    let mut var_type: String = ident_name.clone();
                    let mut next_tok2: Token = tokens[next_pos as usize].clone();
                    let value = next_tok2.value.clone();
                    let mut var_name: String = value.clone();
                    let mut val_pos: i32 = eq_pos + 1.clone();
                    let mut rust_type: String = resolve_type(var_type.clone(), shape_reg.clone(), variant_reg.clone());
                    let mut val_token: Token = tokens[val_pos as usize].clone();
                    let kind = val_token.kind.clone();
                    if kind == "KW_EMPTY" {
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
                    if kind == "LPAREN" {
                        let mut peek_pos: i32 = val_pos + 1.clone();
                        let mut peek_token: Token = tokens[peek_pos as usize].clone();
                        let kind = peek_token.kind.clone();
                        let mut is_avow_expr: bool = kind == "KW_AVOW".clone();
                        if !is_avow_expr {
                            let mut struct_fields_str: String = reg_get(struct_reg.clone(), var_type.clone());
                            let mut field_names: Vec<String> = s_split(struct_fields_str.clone(), ",".to_string());
                            let mut field_pairs: Vec<String> = Vec::new();
                            let mut fend: i32 = val_pos + 1.clone();
                            let mut fni: i32 = 0;
                            let mut fn_count: i32 = (field_names.len() as i32);
                            while fend < token_count {
                                let mut field_tok: Token = tokens[fend as usize].clone();
                                let kind = field_tok.kind.clone();
                                if kind == "RPAREN" {
                                    fend = fend + 1;
                                    break;
                                }
                                if kind == "COMMA" {
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
                            let mut is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                            let mut mut_kw: String = "".to_string();
                            if is_mut {
                                mut_kw = "mut ".to_string();
                            }
                            return make_result(s_join(vec![pad.clone(), "let ".to_string(), mut_kw.clone(), var_name.clone(), " = ".to_string(), var_type.clone(), " { ".to_string(), fields_code.clone(), " };\n".to_string()]), adv_nl(fend.clone(), tokens.clone()));
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
                    if kind == "LBRACKET" {
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
                    if kind == "KW_NONE" {
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
                    if kind == "STRING" {
                        suffix = ".to_string()".to_string();
                    } else if kind == "IDENT" {
                        let mut value_next_index: i32 = val_pos + 1.clone();
                        if value_next_index < token_count {
                            let mut next_val_token: Token = tokens[value_next_index as usize].clone();
                            let kind = next_val_token.kind.clone();
                            let mut val_is_call: bool = kind == "LPAREN".clone();
                            let mut val_is_idx: bool = kind == "KW_AT".clone();
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
    let mut token_count: i32 = (tokens.len() as i32);
    while true {
        if cur >= token_count {
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "DEDENT" || kind == "EOF" {
            if kind == "DEDENT" {
                cur = cur + 1;
            }
            break;
        }
        if kind == "NEWLINE" {
            cur = cur + 1;
            continue;
        }
        let mut stmt_r: ParseResult = gen_stmt(tokens.clone(), cur.clone(), depth.clone(), ctx.clone());
        stmts.push(pr_code(stmt_r.clone()).clone());
        cur = pr_pos(stmt_r.clone());
    }
    return make_result(s_join(stmts.clone()), cur.clone());
}

fn gen_if_branch(tokens: Vec<Token>, cond_pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let mut body_start: i32 = skip_to_body(tokens.clone(), pr_pos(cond_r.clone()));
    let mut body_r: ParseResult = gen_block(tokens.clone(), body_start.clone(), depth + 1.clone(), ctx.clone());
    let mut combined: String = s_join(vec![pr_code(cond_r.clone()).clone(), " {\n".to_string(), pr_code(body_r.clone()).clone()]);
    return make_result(combined.clone(), pr_pos(body_r.clone()));
}

fn gen_if(tokens: Vec<Token>, pos: i32, depth: i32, ctx: GenCtx) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pad: String = s_repeat("    ".to_string(), depth.clone());
    let mut then_r: ParseResult = gen_if_branch(tokens.clone(), pos + 1.clone(), depth.clone(), ctx.clone());
    let mut result_code: String = s_join(vec![pad.clone(), "if ".to_string(), pr_code(then_r.clone()).clone(), pad.clone(), "}".to_string()]);
    let mut cur: i32 = pr_pos(then_r.clone());
    while true {
        if cur >= token_count {
            break;
        }
        let mut else_token: Token = tokens[cur as usize].clone();
        let kind = else_token.kind.clone();
        let mut is_else: bool = kind == "KW_ELSE".clone();
        if !is_else {
            break;
        }
        let mut after_else: i32 = cur + 1.clone();
        if after_else >= token_count {
            break;
        }
        let mut after_else_token: Token = tokens[after_else as usize].clone();
        let kind = after_else_token.kind.clone();
        if kind == "KW_IF" {
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
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pad: String = s_repeat("    ".to_string(), depth.clone());
    let mut next_pos: i32 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_IF" {
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
    if kind == "KW_IN" {
        iter_pos = next_pos + 1;
    } else {
        let value = next_token.value.clone();
        var_name = value;
        let mut in_pos: i32 = next_pos + 1.clone();
        iter_pos = in_pos + 1;
    }
    let mut iter_token: Token = tokens[iter_pos as usize].clone();
    let kind = iter_token.kind.clone();
    let value = iter_token.value.clone();
    let mut range_expr: String = "".to_string();
    let mut body_tok_pos: i32 = 0;
    if kind == "IDENT" && value == "range" {
        let mut lparen: i32 = iter_pos + 1.clone();
        let mut first_pos: i32 = lparen + 1.clone();
        let mut first_r: ParseResult = gen_expr(tokens.clone(), first_pos.clone(), ctx.clone());
        let mut first_code: String = pr_code(first_r.clone());
        let mut first_p: i32 = pr_pos(first_r.clone());
        let mut comma_token: Token = tokens[first_p as usize].clone();
        let kind = comma_token.kind.clone();
        let mut has_start: bool = kind == "COMMA".clone();
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
    } else if kind == "LPAREN" {
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

fn gen_struct_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut cur: TokenCursor = cur_at(tokens.clone(), pos + 1.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut struct_name: String = value.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_skip_to_body(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut field_lines: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "DEDENT" {
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            break;
        } else if kind == "NEWLINE" {
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else if kind == "IDENT" {
            let mut field_type: String = value.clone();
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut field_name: String = value.clone();
            let mut rust_type: String = render_rust_type(field_type.clone());
            field_lines.push(s_join(vec!["    ".to_string(), field_name.clone(), ": ".to_string(), rust_type.clone(), ",".to_string()]).clone());
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        }
    }
    let mut fields_code: String = s_join_nl(field_lines.clone());
    let mut decl: String = s_join(vec!["#[derive(Clone, PartialEq, Debug)]\nstruct ".to_string(), struct_name.clone(), " {\n".to_string(), fields_code.clone(), "\n}\n\n".to_string()]);
    let pos = cur.pos.clone();
    return make_result(decl.clone(), pos.clone());
}

fn gen_shape_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut name_pos: i32 = pos + 1.clone();
    let mut form_pos: i32 = pos + 3.clone();
    let mut elem_pos: i32 = pos + 5.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut form_token: Token = tokens[form_pos as usize].clone();
    let mut elem_token: Token = tokens[elem_pos as usize].clone();
    let value = name_token.value.clone();
    let mut shape_name: String = value.clone();
    let kind = form_token.kind.clone();
    let value = elem_token.value.clone();
    let mut elem_type: String = value.clone();
    let mut rust_name: String = s_pascal(shape_name.clone());
    if kind == "KW_LIST" {
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        let mut decl: String = s_join(vec!["type ".to_string(), rust_name.clone(), " = Vec<".to_string(), rust_elem.clone(), ">;\n\n".to_string()]);
        let mut after: i32 = elem_pos + 1.clone();
        return make_result(decl.clone(), adv_nl(after.clone(), tokens.clone()));
    }
    let mut out_pos: i32 = pos + 7.clone();
    let mut out_token: Token = tokens[out_pos as usize].clone();
    let value = out_token.value.clone();
    let mut out_type: String = value.clone();
    let mut rust_in: String = render_rust_type(elem_type.clone());
    let mut rust_out: String = render_rust_type(out_type.clone());
    let mut out_suffix: String = s_join(vec![" -> ".to_string(), rust_out.clone()]);
    if is_empty(rust_out.clone()) {
        out_suffix = "".to_string();
    }
    let mut decl: String = s_join(vec!["type ".to_string(), rust_name.clone(), " = fn(".to_string(), rust_in.clone(), ")".to_string(), out_suffix.clone(), ";\n\n".to_string()]);
    let mut after: i32 = out_pos + 1.clone();
    return make_result(decl.clone(), adv_nl(after.clone(), tokens.clone()));
}

fn gen_enum_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut cur: TokenCursor = cur_at(tokens.clone(), pos + 1.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut enum_name: String = value.clone();
    let mut rust_name: String = s_pascal(enum_name.clone());
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_skip_to_body(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut variant_lines: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        cur = cur_next(cur.clone(), tokens.clone());
        let token_count = cur.token_count.clone();
        let pos = cur.pos.clone();
        let current = cur.current.clone();
        if kind == "DEDENT" {
            break;
        }
        if kind == "IDENT" {
            variant_lines.push(s_join(vec!["    ".to_string(), value.clone(), ",".to_string()]).clone());
        }
    }
    let mut variants_code: String = s_join_nl(variant_lines.clone());
    let mut decl: String = s_join(vec!["#[derive(Clone, Copy, PartialEq, Debug)]\nenum ".to_string(), rust_name.clone(), " {\n".to_string(), variants_code.clone(), "\n}\n\n".to_string()]);
    let pos = cur.pos.clone();
    return make_result(decl.clone(), pos.clone());
}

fn gen_type_decl(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let mut cur: TokenCursor = cur_at(tokens.clone(), pos + 1.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut type_name: String = value.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut param_type: String = value.clone();
    let mut rust_param_type: String = render_rust_type(param_type.clone());
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut param_name: String = value.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_skip_to_body(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let pos = cur.pos.clone();
    let mut pred_r: ParseResult = gen_expr(tokens.clone(), pos.clone(), ctx.clone());
    let mut pred_code: String = pr_code(pred_r.clone());
    let mut pred_end: i32 = pr_pos(pred_r.clone());
    let mut cur2: TokenCursor = cur_at(tokens.clone(), pred_end.clone());
    let token_count = cur2.token_count.clone();
    let pos = cur2.pos.clone();
    let current = cur2.current.clone();
    while !c_at_end(cur2.clone()) {
        let current = cur2.current.clone();
        let kind = current.kind.clone();
        cur2 = cur_next(cur2.clone(), tokens.clone());
        let token_count = cur2.token_count.clone();
        let pos = cur2.pos.clone();
        let current = cur2.current.clone();
        if kind == "DEDENT" {
            break;
        }
    }
    let pos = cur2.pos.clone();
    let mut struct_code: String = s_join(vec!["#[derive(Clone, Copy, PartialEq, Debug)]\nstruct ".to_string(), type_name.clone(), "(".to_string(), rust_param_type.clone(), ");\n\n".to_string()]);
    let mut impl_code: String = s_join(vec!["impl ".to_string(), type_name.clone(), " {\n    fn new(".to_string(), param_name.clone(), ": ".to_string(), rust_param_type.clone(), ") -> Option<Self> {\n        if ".to_string(), pred_code.clone(), " {\n            Some(".to_string(), type_name.clone(), "(".to_string(), param_name.clone(), "))\n        } else {\n            None\n        }\n    }\n}\n\n".to_string()]);
    return make_result(s_cat(struct_code.clone(), impl_code.clone()), pos.clone());
}

fn gen_fn_decl(tokens: Vec<Token>, pos: i32, ctx: GenCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut using_type = ctx.using_type.clone();
    let mut using_var = ctx.using_var.clone();
    let mut var_type_reg = ctx.var_type_reg.clone();
    let mut cur: TokenCursor = cur_at(tokens.clone(), pos + 1.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut ret_type: String = resolve_type(value.clone(), shape_reg.clone(), enum_reg.clone());
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut fn_name: String = value.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut param_strs: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "RPAREN" {
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            break;
        } else if kind == "COMMA" {
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else if kind == "IDENT" {
            let mut param_type: String = value.clone();
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut param_name: String = value.clone();
            let mut rust_param_type: String = resolve_type(param_type.clone(), shape_reg.clone(), enum_reg.clone());
            param_strs.push(s_join(vec![param_name.clone(), ": ".to_string(), rust_param_type.clone()]).clone());
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        }
    }
    let pos = cur.pos.clone();
    let mut indent_pos: i32 = pos + 1.clone();
    cur = cur_skip_to_body(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut body_end_pos: i32 = find_block_end(tokens.clone(), indent_pos.clone());
    let mut body_start: i32 = pos.clone();
    let mut body_tokens: Vec<Token> = l_slice(tokens.clone(), body_start.clone(), body_end_pos + 1.clone());
    let mut body_len: i32 = (body_tokens.len() as i32);
    let mut mut_names: Vec<String> = collect_mut_names(body_tokens.clone(), 0, body_len - 1.clone());
    let mut var_type_reg: Vec<String> = build_var_type_reg(body_tokens.clone());
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
/* unhandled(IDENT) */
    let body_ctx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, using_type, using_var, var_type_reg };
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

fn count_tabs(line: String) -> i32 {
    let mut chars: Vec<String> = c_chars(line.clone());
    let mut char_count: i32 = (chars.len() as i32);
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
    let mut lines: Vec<String> = s_split(source.clone(), "\n".to_string());
    let mut n_lines: i32 = (lines.len() as i32);
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
        let mut line: String = s_rtrim(raw.clone());
        let mut stripped: String = s_trim(line.clone());
        if is_empty(stripped.clone()) {
            continue;
        }
        let mut indent: i32 = count_tabs(line.clone());
        let mut content: String = s_from(line.clone(), indent.clone());
        let mut slen: i32 = (indent_stack.len() as i32);
        let mut top_idx: i32 = slen - 1.clone();
        let mut top: i32 = n_parse(indent_stack[top_idx as usize].clone());
        if indent > top {
            tokens.push(make_token("INDENT".to_string(), "".to_string(), cur_line.clone()).clone());
            indent_stack.push(n_to_str(indent.clone()).clone());
        } else {
            let mut dedenting: bool = indent < top.clone();
            while dedenting {
                let mut new_slen: i32 = (indent_stack.len() as i32);
                let mut new_top_idx: i32 = new_slen - 1.clone();
                let mut cur_top: i32 = n_parse(indent_stack[new_top_idx as usize].clone());
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
                let mut rl_stripped: String = s_trim(rust_line.clone());
                if is_empty(rl_stripped.clone()) {
                    rust_lines.push("".to_string());
                    skip = skip + 1;
                } else if rl_indent >= rust_base {
                    let mut rl_content: String = s_from(rust_line.clone(), rust_base.clone());
                    rust_lines.push(s_rtrim(rl_content.clone()).clone());
                    skip = skip + 1;
                } else {
                    break;
                }
            }
            let mut trimming: bool = true;
            while trimming {
                let mut rl_len: i32 = (rust_lines.len() as i32);
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
        let mut chars: Vec<String> = c_chars(content.clone());
        let mut char_count: i32 = (chars.len() as i32);
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
            if c_digit(character.clone()) {
                let mut num: String = s_cat("".to_string(), character.clone());
                let mut num_start: i32 = char_index + 1.clone();
                char_index = char_index + 1;
                for number_index in num_start..char_count {
                    let mut number_char: String = chars[number_index as usize].clone();
                    if c_digit(number_char.clone()) {
                        num = s_cat(num.clone(), number_char.clone());
                        char_index = number_index + 1;
                    } else {
                        break;
                    }
                }
                tokens.push(make_token("INT".to_string(), num.clone(), cur_line.clone()).clone());
                continue;
            }
            if c_alpha(character.clone()) {
                let mut word: String = s_cat("".to_string(), character.clone());
                let mut word_start: i32 = char_index + 1.clone();
                char_index = char_index + 1;
                for word_index in word_start..char_count {
                    let mut word_char: String = chars[word_index as usize].clone();
                    if c_alnum(word_char.clone()) {
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
    let mut final_stack_len: i32 = (indent_stack.len() as i32);
    for _ in 1..final_stack_len {
        tokens.push(make_token("DEDENT".to_string(), "".to_string(), cur_line.clone()).clone());
    }
    tokens.push(make_token("EOF".to_string(), "".to_string(), cur_line.clone()).clone());
    return tokens;
}

fn filter_by_names(tokens: Vec<Token>, names: Vec<String>) -> Vec<Token> {
    let mut names_count: i32 = (names.len() as i32);
    let mut result: Vec<Token> = Vec::new();
    let mut cur: i32 = 0;
    let mut token_count: i32 = (tokens.len() as i32);
    while cur < token_count {
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
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
                let value = name_tok.value.clone();
                decl_name = value;
            }
            let mut include: bool = names_count == 0 || list_has(names.clone(), decl_name.clone()).clone();
            let mut depth: i32 = 0;
            let mut entered: bool = false;
            while cur < token_count {
                let mut cur_tok: Token = tokens[cur as usize].clone();
                let kind = cur_tok.kind.clone();
                if include {
                    result.push(cur_tok.clone());
                }
                cur = cur + 1;
                if kind == "INDENT" {
                    depth = depth + 1;
                    entered = true;
                } else if kind == "DEDENT" {
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
                let value = name_tok.value.clone();
                decl_name = value;
            }
            let mut include: bool = names_count == 0 || list_has(names.clone(), decl_name.clone()).clone();
            while cur < token_count {
                let mut cur_tok: Token = tokens[cur as usize].clone();
                let kind = cur_tok.kind.clone();
                if include {
                    result.push(cur_tok.clone());
                }
                cur = cur + 1;
                if kind == "NEWLINE" {
                    break;
                }
            }
        } else {
            cur = cur + 1;
        }
    }
    return result;
}

fn collect_imports_from_raw(source_tokens: Vec<Token>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = (source_tokens.len() as i32);
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    while true {
        if pos >= token_count {
            break;
        }
        let mut token: Token = source_tokens[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "EOF" {
            break;
        }
        if kind == "NEWLINE" {
            pos = pos + 1;
            continue;
        }
        if kind == "INDENT" {
            depth = depth + 1;
            pos = pos + 1;
            continue;
        }
        if kind == "DEDENT" {
            depth = depth - 1;
            pos = pos + 1;
            continue;
        }
        let mut at_root: bool = depth == 0.clone();
        if kind == "LPAREN" && at_root {
            let mut sub_names: Vec<String> = Vec::new();
            let mut scan: i32 = pos + 1.clone();
            while scan < token_count {
                let mut scan_token: Token = source_tokens[scan as usize].clone();
                let kind = scan_token.kind.clone();
                scan = scan + 1;
                if kind == "RPAREN" {
                    break;
                }
                if kind == "IDENT" {
                    let value = scan_token.value.clone();
                    sub_names.push(value.clone());
                }
            }
            let mut path_pos: i32 = scan + 1.clone();
            if path_pos < token_count {
                let mut in_token: Token = source_tokens[scan as usize].clone();
                let kind = in_token.kind.clone();
                let mut is_in: bool = kind == "KW_IN".clone();
                if is_in {
                    let mut path_token: Token = source_tokens[path_pos as usize].clone();
                    let value = path_token.value.clone();
                    let mut sub_path: String = value.clone();
                    let mut name_count: i32 = (sub_names.len() as i32);
                    if name_count > 0 {
                        let mut sub_tokens: Vec<Token> = load_named(sub_path.clone(), sub_names.clone());
                        let mut sub_len: i32 = (sub_tokens.len() as i32);
                        for sub_i in 0..sub_len {
                            let mut sub_tok: Token = sub_tokens[sub_i as usize].clone();
                            let kind = sub_tok.kind.clone();
                            let mut sub_is_eof: bool = kind == "EOF".clone();
                            if !sub_is_eof {
                                result.push(sub_tok.clone());
                            }
                        }
                    }
                    pos = path_pos + 1;
                    continue;
                }
            }
        }
        pos = pos + 1;
    }
    return result;
}

fn load_named(path: String, names: Vec<String>) -> Vec<Token> {
    let mut source: String = f_read(path.clone());
    let mut raw: Vec<Token> = tokenize(source.clone());
    let mut support: Vec<Token> = collect_imports_from_raw(raw.clone());
    let mut own: Vec<Token> = filter_by_names(raw.clone(), Vec::new());
    let mut result: Vec<Token> = Vec::new();
    let mut support_len: i32 = (support.len() as i32);
    for i in 0..support_len {
        result.push(support[i as usize].clone().clone());
    }
    let mut own_len: i32 = (own.len() as i32);
    for i in 0..own_len {
        result.push(own[i as usize].clone().clone());
    }
    return result;
}

fn deduplicate_decls(tokens: Vec<Token>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pos: i32 = 0;
    while pos < token_count {
        let mut token: Token = tokens[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "EOF" {
            result.push(token.clone());
            pos = pos + 1;
            continue;
        }
        if kind == "NEWLINE" {
            result.push(token.clone());
            pos = pos + 1;
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
            let mut name_pos: i32 = pos + name_offset.clone();
            let mut decl_name: String = "".to_string();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let value = name_tok.value.clone();
                decl_name = value;
            }
            let mut already_seen: bool = list_has(seen.clone(), decl_name.clone());
            if !already_seen {
                seen.push(decl_name.clone());
            }
            let mut depth: i32 = 0;
            let mut entered: bool = false;
            while pos < token_count {
                let mut cur_tok: Token = tokens[pos as usize].clone();
                let kind = cur_tok.kind.clone();
                if !already_seen {
                    result.push(cur_tok.clone());
                }
                pos = pos + 1;
                if kind == "INDENT" {
                    depth = depth + 1;
                    entered = true;
                } else if kind == "DEDENT" {
                    depth = depth - 1;
                    if depth == 0 && entered {
                        break;
                    }
                }
            }
        } else if is_shape {
            let mut name_pos: i32 = pos + 1.clone();
            let mut decl_name: String = "".to_string();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let value = name_tok.value.clone();
                decl_name = value;
            }
            let mut already_seen: bool = list_has(seen.clone(), decl_name.clone());
            if !already_seen {
                seen.push(decl_name.clone());
            }
            while pos < token_count {
                let mut cur_tok: Token = tokens[pos as usize].clone();
                let kind = cur_tok.kind.clone();
                if !already_seen {
                    result.push(cur_tok.clone());
                }
                pos = pos + 1;
                if kind == "NEWLINE" {
                    break;
                }
            }
        } else {
            result.push(token.clone());
            pos = pos + 1;
        }
    }
    return result;
}

fn collect_all_tokens_with_all_imports(path: String) -> Vec<Token> {
    let mut source: String = f_read(path.clone());
    let mut source_tokens: Vec<Token> = tokenize(source.clone());
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = (source_tokens.len() as i32);
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    while true {
        if pos >= token_count {
            break;
        }
        let mut token: Token = source_tokens[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "EOF" {
            break;
        }
        if kind == "INDENT" {
            depth = depth + 1;
            result.push(token.clone());
            pos = pos + 1;
            continue;
        }
        if kind == "DEDENT" {
            depth = depth - 1;
            result.push(token.clone());
            pos = pos + 1;
            continue;
        }
        let mut at_root: bool = depth == 0.clone();
        if kind == "LPAREN" && at_root {
            let mut import_names: Vec<String> = Vec::new();
            let mut scan: i32 = pos + 1.clone();
            while scan < token_count {
                let mut scan_token: Token = source_tokens[scan as usize].clone();
                let kind = scan_token.kind.clone();
                scan = scan + 1;
                if kind == "RPAREN" {
                    break;
                }
                if kind == "IDENT" {
                    let value = scan_token.value.clone();
                    import_names.push(value.clone());
                }
            }
            let mut path_pos: i32 = scan + 1.clone();
            if path_pos < token_count {
                let mut in_token: Token = source_tokens[scan as usize].clone();
                let kind = in_token.kind.clone();
                let mut is_in: bool = kind == "KW_IN".clone();
                if is_in {
                    let mut path_token: Token = source_tokens[path_pos as usize].clone();
                    let value = path_token.value.clone();
                    let mut imp_path: String = value.clone();
                    let mut name_count: i32 = (import_names.len() as i32);
                    if name_count > 0 {
                        let mut imp_tokens: Vec<Token> = load_named(imp_path.clone(), import_names.clone());
                        let mut import_len: i32 = (imp_tokens.len() as i32);
                        for import_index in 0..import_len {
                            let mut import_token: Token = imp_tokens[import_index as usize].clone();
                            let kind = import_token.kind.clone();
                            let mut import_is_eof: bool = kind == "EOF".clone();
                            if !import_is_eof {
                                result.push(import_token.clone());
                            }
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
    return deduplicate_decls(result.clone());
}

fn generate_rust_from_tokens(tokens: Vec<Token>) -> String {
    let mut struct_reg: Vec<String> = build_struct_reg(tokens.clone());
    let mut shape_reg: Vec<String> = build_shape_reg(tokens.clone());
    let mut enum_reg: Vec<String> = build_enum_reg(tokens.clone());
    let mut variant_reg: Vec<String> = build_variant_reg(tokens.clone(), enum_reg.clone());
    let mut type_reg: Vec<String> = build_type_reg(tokens.clone());
    let mut mut_names: Vec<String> = Vec::new();
    let mut var_type_reg: Vec<String> = build_var_type_reg(tokens.clone());
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
/* unhandled(IDENT) */
    let ctx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, using_type, using_var, var_type_reg };
    let mut output: String = "".to_string();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pos: i32 = 0;
    while true {
        if pos >= token_count {
            break;
        }
        let mut token: Token = tokens[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "EOF" {
            break;
        }
        if kind == "NEWLINE" {
            pos = pos + 1;
            continue;
        }
        if kind == "KW_STRUCT" {
            let mut result: ParseResult = gen_struct_decl(tokens.clone(), pos.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_SHAPE" {
            let mut result: ParseResult = gen_shape_decl(tokens.clone(), pos.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_ENUM" {
            let mut result: ParseResult = gen_enum_decl(tokens.clone(), pos.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_TYPE" {
            let mut result: ParseResult = gen_type_decl(tokens.clone(), pos.clone(), ctx.clone());
            output = s_cat(output.clone(), pr_code(result.clone()));
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_FN" {
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
    let mut args: Vec<String> = f_args();
    let mut arg_count: i32 = (args.len() as i32);
    if arg_count < 2 {
        println!("{}", "usage: deor input.deor output.rs".to_string());
    } else {
        let mut input_path: String = args[0 as usize].clone();
        let mut output_path: String = args[1 as usize].clone();
        let mut tokens: Vec<Token> = collect_all_tokens_with_all_imports(input_path.clone());
        validate_tokens(tokens.clone());
        let mut rust_code: String = generate_rust_from_tokens(tokens.clone());
        f_write(output_path.clone(), rust_code.clone());
    }
}

