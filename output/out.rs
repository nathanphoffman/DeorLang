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

use std::rc::Rc;
type TokensRef = Rc<Vec<Token>>;
type RcCtx = Rc<GenCtx>;
fn tokens_wrap(t: Vec<Token>) -> TokensRef { Rc::new(t) }
fn make_rctx(ctx: GenCtx) -> RcCtx { Rc::new(ctx) }
fn now_ms() -> i32 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i32 }
fn elapsed_ms(start: i32) -> i32 { now_ms() - start }
fn f_read(path: String) -> String {
    std::fs::read_to_string(path.as_str()).expect("cannot read input file")
}

fn f_write(path: String, content: String) {
    std::fs::write(path.as_str(), content.as_str()).expect("cannot write output file");
}

fn f_args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

fn n_parse(source: String) -> i32 {
    source.parse::<i32>().unwrap_or(0)
}

fn n_to_str(number: i32) -> String {
    number.to_string()
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
    let mut line_str: String = n_to_str(line_num.clone());
    let mut val_pfx: String = "[validation] line ".to_string();
    let mut val_col: String = ": ".to_string();
    let mut val_qt1: String = " '".to_string();
    let mut val_qt2: String = "' - ".to_string();
    let mut val_parts: Vec<String> = vec![val_pfx.clone(), line_str.clone(), val_col.clone(), label.clone(), val_qt1.clone(), name.clone(), val_qt2.clone(), rule.clone()];
    return s_join(val_parts.clone());
}

fn count_call_args(tokens: Vec<Token>, lp_pos: i32) -> i32 {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = lp_pos + 1.clone();
    let mut depth: i32 = 0;
    let mut comma_count: i32 = 0;
    let mut saw_token: bool = false;
    while cur < token_count {
        let mut tok: Token = tokens[cur as usize].clone();
        let kind = tok.kind.clone();
        if kind == "RPAREN" {
            let mut at_root: bool = depth == 0.clone();
            if at_root {
                break;
            }
            depth = depth - 1;
        } else if kind == "LPAREN" {
            depth = depth + 1;
            saw_token = true;
        } else if kind == "COMMA" {
            let mut at_root: bool = depth == 0.clone();
            if at_root {
                comma_count = comma_count + 1;
            }
            saw_token = true;
        } else {
            saw_token = true;
        }
        cur = cur + 1;
    }
    let mut result: i32 = 0;
    if saw_token {
        result = comma_count + 1;
    }
    return result;
}

fn validate_tokens(tokens: Vec<Token>) {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut errors: Vec<String> = Vec::new();
    let mut pos: i32 = 0;
    let mut lbl_struct: String = "struct".to_string();
    let mut lbl_enum: String = "enum".to_string();
    let mut lbl_shape: String = "shape".to_string();
    let mut lbl_type: String = "type".to_string();
    let mut lbl_fn: String = "fn".to_string();
    let mut lbl_var: String = "variable".to_string();
    let mut lbl_call: String = "call to".to_string();
    let mut rule_min3: String = "name must be at least 3 characters".to_string();
    let mut rule_pascal: String = "name must be PascalCase (start with uppercase letter)".to_string();
    let mut rule_camel: String = "name must be camelCase (start lowercase, no underscores)".to_string();
    let mut rule_snake: String = "name must be lower_snake_case (no uppercase letters)".to_string();
    let mut rule_named_arg: String = "each arg must be a named variable when passing 2 or more args".to_string();
    while pos < token_count {
        let mut tok: Token = tokens[pos as usize].clone();
        let kind = tok.kind.clone();
        let value = tok.value.clone();
        let line = tok.line.clone();
        let mut cur_kind: String = kind.clone();
        let mut cur_val: String = value.clone();
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
                        errors.push(val_err(cur_line.clone(), lbl_struct.clone(), name_val.clone(), rule_min3.clone()).clone());
                    }
                    if !is_pascal(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), lbl_struct.clone(), name_val.clone(), rule_pascal.clone()).clone());
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
                        errors.push(val_err(cur_line.clone(), lbl_enum.clone(), name_val.clone(), rule_min3.clone()).clone());
                    }
                    if !is_pascal(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), lbl_enum.clone(), name_val.clone(), rule_pascal.clone()).clone());
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
                        errors.push(val_err(cur_line.clone(), lbl_shape.clone(), name_val.clone(), rule_min3.clone()).clone());
                    }
                    if !is_camel(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), lbl_shape.clone(), name_val.clone(), rule_camel.clone()).clone());
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
                        errors.push(val_err(cur_line.clone(), lbl_type.clone(), name_val.clone(), rule_min3.clone()).clone());
                    }
                    if !is_camel(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), lbl_type.clone(), name_val.clone(), rule_camel.clone()).clone());
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
                        errors.push(val_err(cur_line.clone(), lbl_fn.clone(), name_val.clone(), rule_min3.clone()).clone());
                    }
                    if !is_snake(name_val.clone()) {
                        errors.push(val_err(cur_line.clone(), lbl_fn.clone(), name_val.clone(), rule_snake.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "IDENT" {
            let mut call_lp: i32 = pos + 1.clone();
            if call_lp < token_count {
                let mut call_lp_tok: Token = tokens[call_lp as usize].clone();
                let kind = call_lp_tok.kind.clone();
                if kind == "LPAREN" {
                    let mut arg_count: i32 = count_call_args(tokens.clone(), call_lp.clone());
                    if arg_count >= 2 {
                        let mut scan_pos: i32 = call_lp + 1.clone();
                        let mut scan_depth: i32 = 0;
                        let mut at_arg_start: bool = true;
                        while scan_pos < token_count {
                            let mut scan_tok: Token = tokens[scan_pos as usize].clone();
                            let kind = scan_tok.kind.clone();
                            if kind == "RPAREN" {
                                let mut scan_root: bool = scan_depth == 0.clone();
                                if scan_root {
                                    break;
                                }
                                scan_depth = scan_depth - 1;
                                at_arg_start = false;
                            } else if kind == "LPAREN" {
                                scan_depth = scan_depth + 1;
                                at_arg_start = false;
                            } else if kind == "COMMA" {
                                let mut scan_root: bool = scan_depth == 0.clone();
                                if scan_root {
                                    at_arg_start = true;
                                }
                            } else if at_arg_start {
                                let mut check_pos: i32 = scan_pos.clone();
                                let mut chk_kind: String = kind.clone();
                                if kind == "KW_GIVEUP" {
                                    check_pos = scan_pos + 1;
                                    if check_pos < token_count {
                                        let mut giveup_tok: Token = tokens[check_pos as usize].clone();
                                        let kind = giveup_tok.kind.clone();
                                        chk_kind = kind;
                                    }
                                }
                                let mut arg_is_ident: bool = chk_kind == "IDENT".clone();
                                if !arg_is_ident {
                                    errors.push(val_err(cur_line.clone(), lbl_call.clone(), cur_val.clone(), rule_named_arg.clone()).clone());
                                } else {
                                    let mut peek_pos: i32 = check_pos + 1.clone();
                                    if peek_pos < token_count {
                                        let mut peek_tok: Token = tokens[peek_pos as usize].clone();
                                        let kind = peek_tok.kind.clone();
                                        let mut arg_is_call: bool = kind == "LPAREN".clone();
                                        let mut arg_is_idx: bool = kind == "KW_AT".clone();
                                        let mut arg_is_plus: bool = kind == "PLUS".clone();
                                        let mut arg_is_minus: bool = kind == "MINUS".clone();
                                        let mut arg_is_star: bool = kind == "STAR".clone();
                                        let mut arg_is_slash: bool = kind == "SLASH".clone();
                                        let mut arg_is_pct: bool = kind == "PERCENT".clone();
                                        let mut arg_is_op: bool = arg_is_plus || arg_is_minus || arg_is_star || arg_is_slash || arg_is_pct.clone();
                                        if arg_is_call || arg_is_idx || arg_is_op {
                                            errors.push(val_err(cur_line.clone(), lbl_call.clone(), cur_val.clone(), rule_named_arg.clone()).clone());
                                        }
                                    }
                                }
                                at_arg_start = false;
                            }
                            scan_pos = scan_pos + 1;
                        }
                    }
                }
            }
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
                        errors.push(val_err(var_line.clone(), lbl_var.clone(), var_name.clone(), rule_min3.clone()).clone());
                    }
                    if !is_snake(var_name.clone()) {
                        errors.push(val_err(var_line.clone(), lbl_var.clone(), var_name.clone(), rule_snake.clone()).clone());
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
    let result = ParseResult { code: code.clone(), new_pos: new_pos.clone() };
    return result;
}

fn make_token(kind: String, value: String, line: i32) -> Token {
    let token = Token { kind: kind.clone(), value: value.clone(), line: line.clone() };
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

fn adv_nl_ref(pos: i32, tokens: TokensRef) -> i32 {
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

fn adv_indent_ref(pos: i32, tokens: TokensRef) -> i32 {
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

fn skip_to_body_ref(tokens: TokensRef, pos: i32) -> i32 {
    let mut cur: i32 = adv_nl_ref(pos.clone(), tokens.clone());
    cur = adv_indent_ref(cur.clone(), tokens.clone());
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
    let mut two: i32 = 2;
    return reg_get_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg_has(pairs: Vec<String>, key: String) -> bool {
    let mut two: i32 = 2;
    return reg_has_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg3_get(pairs: Vec<String>, key: String) -> String {
    let mut thr: i32 = 3;
    return reg_get_stride(pairs.clone(), key.clone(), thr.clone());
}

fn reg3_has(pairs: Vec<String>, key: String) -> bool {
    let mut thr: i32 = 3;
    return reg_has_stride(pairs.clone(), key.clone(), thr.clone());
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
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), cur.clone());
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
    let mut comma: String = ",".to_string();
    let mut fields_joined: String = s_join_with(fields.clone(), comma.clone());
    return make_result(fields_joined.clone(), cur.clone());
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
                    let mut blk_start: i32 = name_pos + 1.clone();
                    let mut block_r: ParseResult = skip_to_block_start(tokens.clone(), blk_start.clone());
                    let mut fld_start: i32 = pr_pos(block_r.clone());
                    let mut fields_r: ParseResult = collect_struct_fields(tokens.clone(), fld_start.clone());
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
                        let mut fn_parts: Vec<String> = vec!["fn:".to_string(), elem_type.clone(), ":".to_string(), out_type.clone()];
                        result.push(s_join(fn_parts.clone()).clone());
                    }
                }
            }
        }
        if kind == "KW_RAW" {
            let mut name_pos: i32 = index + 1.clone();
            if name_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let value = name_token.value.clone();
                result.push(value.clone());
                result.push("raw:".to_string());
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
        let mut raw_prefix: String = "raw:".to_string();
        let mut fn_prefix: String = "fn:".to_string();
        let mut colon: String = ":".to_string();
        if s_starts_with(elem_type.clone(), raw_prefix.clone()) {
            return type_name;
        }
        if s_starts_with(elem_type.clone(), fn_prefix.clone()) {
            let mut parts: Vec<String> = s_split(elem_type.clone(), colon.clone());
            let mut in_type: String = parts[1 as usize].clone();
            let mut out_type: String = parts[2 as usize].clone();
            let mut rust_in: String = render_rust_type(in_type.clone());
            let mut rust_out: String = render_rust_type(out_type.clone());
            if is_empty(rust_out.clone()) {
                let mut fn_no_ret: Vec<String> = vec!["fn(".to_string(), rust_in.clone(), ")".to_string()];
                return s_join(fn_no_ret.clone());
            }
            let mut fn_with_ret: Vec<String> = vec!["fn(".to_string(), rust_in.clone(), ") -> ".to_string(), rust_out.clone()];
            return s_join(fn_with_ret.clone());
        }
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        let mut vec_parts: Vec<String> = vec!["Vec<".to_string(), rust_elem.clone(), ">".to_string()];
        return s_join(vec_parts.clone());
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
    let c = TokenCursor { token_count: token_count.clone(), pos: pos.clone(), current: current.clone() };
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
    if word == "giveup" {
        return "KW_GIVEUP".to_string();
    }
    if word == "raw" {
        return "KW_RAW".to_string();
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
    let mut comma: String = ",".to_string();
    let mut fields_key: String = s_join_with(fields.clone(), comma.clone());
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
            let mut comma: String = ",".to_string();
            let mut fields: Vec<String> = s_split(item.clone(), comma.clone());
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

fn gen_call_args(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
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
            let mut str_suf: String = ".to_string()".to_string();
            let mut str_parts: Vec<String> = vec![arg_code.clone(), str_suf.clone()];
            arg_code = s_join(str_parts.clone());
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
                    let mut cln_suf: String = ".clone()".to_string();
                    let mut cln_parts: Vec<String> = vec![arg_code.clone(), cln_suf.clone()];
                    arg_code = s_join(cln_parts.clone());
                }
            }
        }
        arg_codes.push(arg_code.clone());
        cur = arg_pos;
    }
    let mut sep: String = ", ".to_string();
    let mut args_str: String = s_join_with(arg_codes.clone(), sep.clone());
    return make_result(args_str.clone(), cur.clone());
}

fn gen_list_items(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
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
            let mut its_suf: String = ".to_string()".to_string();
            let mut its_parts: Vec<String> = vec![item_code.clone(), its_suf.clone()];
            item_code = s_join(its_parts.clone());
        } else {
            let mut itc_suf: String = ".clone()".to_string();
            let mut itc_parts: Vec<String> = vec![item_code.clone(), itc_suf.clone()];
            item_code = s_join(itc_parts.clone());
        }
        item_codes.push(item_code.clone());
        cur = item_pos;
    }
    let mut sep: String = ", ".to_string();
    let mut items_str: String = s_join_with(item_codes.clone(), sep.clone());
    return make_result(items_str.clone(), cur.clone());
}

fn gen_unary_method(tokens: TokensRef, args_pos: i32, suffix: String, ctx: RcCtx) -> ParseResult {
    let mut inner_r: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
    let mut inner_code: String = pr_code(inner_r.clone());
    let mut close: i32 = pr_pos(inner_r.clone()) + 1;
    let mut res_parts: Vec<String> = vec![inner_code.clone(), suffix.clone()];
    let mut result_code: String = s_join(res_parts.clone());
    return make_result(result_code.clone(), close.clone());
}

fn gen_primary(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
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
        let mut next: i32 = pos + 1.clone();
        return make_result(value.clone(), next.clone());
    }
    if kind == "STRING" {
        let mut debug_val: String = s_debug(value.clone());
        let mut next: i32 = pos + 1.clone();
        return make_result(debug_val.clone(), next.clone());
    }
    if kind == "KW_TRUE" {
        let mut true_str: String = "true".to_string();
        let mut next: i32 = pos + 1.clone();
        return make_result(true_str.clone(), next.clone());
    }
    if kind == "KW_FALSE" {
        let mut false_str: String = "false".to_string();
        let mut next: i32 = pos + 1.clone();
        return make_result(false_str.clone(), next.clone());
    }
    if kind == "KW_NONE" {
        let mut none_str: String = "None".to_string();
        let mut next: i32 = pos + 1.clone();
        return make_result(none_str.clone(), next.clone());
    }
    if kind == "LBRACKET" {
        let mut inner_pos: i32 = pos + 1.clone();
        if inner_pos < token_count {
            let mut next_token: Token = tokens[inner_pos as usize].clone();
            let kind = next_token.kind.clone();
            if kind == "RBRACKET" {
                let mut after: i32 = inner_pos + 1.clone();
                let mut empty_vec: String = "Vec::new()".to_string();
                return make_result(empty_vec.clone(), after.clone());
            }
        }
        let mut items_r: ParseResult = gen_list_items(tokens.clone(), inner_pos.clone(), ctx.clone());
        let mut items_code: String = pr_code(items_r.clone());
        let mut items_pos: i32 = pr_pos(items_r.clone());
        let mut vec_open: String = "vec![".to_string();
        let mut vec_close: String = "]".to_string();
        let mut list_parts: Vec<String> = vec![vec_open.clone(), items_code.clone(), vec_close.clone()];
        let mut list_code: String = s_join(list_parts.clone());
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
                let mut unw_suf: String = ".unwrap()".to_string();
                let mut unw_parts: Vec<String> = vec![expr_code.clone(), unw_suf.clone()];
                let mut unwrap_code: String = s_join(unw_parts.clone());
                return make_result(unwrap_code.clone(), after_rparen.clone());
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
            let mut sep: String = ", ".to_string();
            let mut fields_code: String = s_join_with(fields.clone(), sep.clone());
            let mut sco: String = " { ".to_string();
            let mut scc: String = " }".to_string();
            let mut sct_parts: Vec<String> = vec![struct_name.clone(), sco.clone(), fields_code.clone(), scc.clone()];
            let mut struct_code: String = s_join(sct_parts.clone());
            return make_result(struct_code.clone(), cur.clone());
        }
    }
    if kind == "KW_GIVEUP" {
        let mut inner_pos: i32 = pos + 1.clone();
        let mut inner_r: ParseResult = gen_primary(tokens.clone(), inner_pos.clone(), ctx.clone());
        return inner_r;
    }
    if kind == "KW_NOT" {
        let mut operand_pos: i32 = pos + 1.clone();
        let mut operand_r: ParseResult = gen_primary(tokens.clone(), operand_pos.clone(), ctx.clone());
        let mut operand_code: String = pr_code(operand_r.clone());
        let mut operand_pos_end: i32 = pr_pos(operand_r.clone());
        let mut not_pfx: String = "!".to_string();
        let mut not_parts: Vec<String> = vec![not_pfx.clone(), operand_code.clone()];
        let mut not_code: String = s_join(not_parts.clone());
        return make_result(not_code.clone(), operand_pos_end.clone());
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
                    let mut len_sfx: String = ".len() as i32".to_string();
                    let mut len_r: ParseResult = gen_unary_method(tokens.clone(), args_pos.clone(), len_sfx.clone(), ctx.clone());
                    let mut len_code: String = pr_code(len_r.clone());
                    let mut len_end: i32 = pr_pos(len_r.clone());
                    let mut lpn: String = "(".to_string();
                    let mut rpn: String = ")".to_string();
                    let mut wpd_parts: Vec<String> = vec![lpn.clone(), len_code.clone(), rpn.clone()];
                    let mut wrapped: String = s_join(wpd_parts.clone());
                    return make_result(wrapped.clone(), len_end.clone());
                } else if func_name == "crash" {
                    let mut crash_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
                    let mut crash_code: String = pr_code(crash_r.clone());
                    let mut crash_end: i32 = pr_pos(crash_r.clone());
                    let mut pan_pfx: String = "panic!(\"{}\", ".to_string();
                    let mut pan_sfx: String = ")".to_string();
                    let mut pan_parts: Vec<String> = vec![pan_pfx.clone(), crash_code.clone(), pan_sfx.clone()];
                    let mut panic_code: String = s_join(pan_parts.clone());
                    let mut after_crash: i32 = crash_end + 1.clone();
                    return make_result(panic_code.clone(), after_crash.clone());
                }
                let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
                let mut args_code: String = pr_code(args_r.clone());
                let mut args_end: i32 = pr_pos(args_r.clone());
                let mut after_paren: i32 = args_end + 1.clone();
                let mut clp: String = "(".to_string();
                let mut crp: String = ")".to_string();
                let mut call_parts: Vec<String> = vec![func_name.clone(), clp.clone(), args_code.clone(), crp.clone()];
                let mut call_code: String = s_join(call_parts.clone());
                return make_result(call_code.clone(), after_paren.clone());
            }
            if kind == "KW_AT" {
                let mut idx_pos: i32 = next_pos + 1.clone();
                let mut idx_r: ParseResult = gen_primary(tokens.clone(), idx_pos.clone(), ctx.clone());
                let mut idx_code: String = pr_code(idx_r.clone());
                let mut idx_end: i32 = pr_pos(idx_r.clone());
                let mut idx_mid: String = "[".to_string();
                let mut idx_sfx: String = " as usize].clone()".to_string();
                let mut idx_parts: Vec<String> = vec![value.clone(), idx_mid.clone(), idx_code.clone(), idx_sfx.clone()];
                let mut idx_expr: String = s_join(idx_parts.clone());
                return make_result(idx_expr.clone(), idx_end.clone());
            }
        }
        let mut variant_enum: String = reg_get(variant_reg.clone(), value.clone());
        if !is_empty(variant_enum.clone()) {
            let mut dbl_colon: String = "::".to_string();
            let mut var_parts: Vec<String> = vec![variant_enum.clone(), dbl_colon.clone(), value.clone()];
            let mut variant_code: String = s_join(var_parts.clone());
            return make_result(variant_code.clone(), next_pos.clone());
        }
        return make_result(value.clone(), next_pos.clone());
    }
    let mut unknown: String = "/* unknown_primary */".to_string();
    let mut next: i32 = pos + 1.clone();
    return make_result(unknown.clone(), next.clone());
}

fn gen_expr(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
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
                let mut fmt_pfx: String = "format!(\"{}{}\", ".to_string();
                let mut fmt_mid: String = ", ".to_string();
                let mut fmt_sfx: String = ")".to_string();
                let mut fmt_parts: Vec<String> = vec![fmt_pfx.clone(), left_code.clone(), fmt_mid.clone(), rhs_code.clone(), fmt_sfx.clone()];
                left_code = s_join(fmt_parts.clone());
                left_has_str = true;
                cur_pos = rhs_pos;
                continue;
            }
            if left_has_str {
                if rhs_is_ident {
                    let mut astr_sfx: String = ".as_str()".to_string();
                    let mut astr_parts: Vec<String> = vec![rhs_code.clone(), astr_sfx.clone()];
                    rhs_code = s_join(astr_parts.clone());
                }
            }
            if rhs_is_str {
                left_has_str = true;
            }
        }
        let mut op_sp: String = " ".to_string();
        let mut bin_parts: Vec<String> = vec![left_code.clone(), op_sp.clone(), rust_op.clone(), op_sp.clone(), rhs_code.clone()];
        left_code = s_join(bin_parts.clone());
        cur_pos = rhs_pos;
    }
    return make_result(left_code.clone(), cur_pos.clone());
}

fn emit_val(val_code: String, val_kind: String) -> String {
    if val_kind == "STRING" {
        let mut tos_sfx: String = ".to_string()".to_string();
        let mut tos_parts: Vec<String> = vec![val_code.clone(), tos_sfx.clone()];
        return s_join(tos_parts.clone());
    }
    if val_kind == "IDENT" {
        let mut cln_sfx: String = ".clone()".to_string();
        let mut cln_parts: Vec<String> = vec![val_code.clone(), cln_sfx.clone()];
        return s_join(cln_parts.clone());
    }
    return val_code;
}

fn make_destruct_code(var_name: String, struct_type: String, struct_reg: Vec<String>, mut_names: Vec<String>, pad: String) -> String {
    let mut fields_str: String = reg_get(struct_reg.clone(), struct_type.clone());
    if is_empty(fields_str.clone()) {
        return "".to_string();
    }
    let mut comma: String = ",".to_string();
    let mut fields: Vec<String> = s_split(fields_str.clone(), comma.clone());
    let mut field_count: i32 = (fields.len() as i32);
    let mut lines: Vec<String> = Vec::new();
    for i in 0..field_count {
        let mut field: String = fields[i as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            mut_kw = "mut ".to_string();
        }
        let mut fld_let: String = "let ".to_string();
        let mut fld_eq: String = " = ".to_string();
        let mut fld_dot: String = ".".to_string();
        let mut fld_cln: String = ".clone();".to_string();
        let mut fld_parts: Vec<String> = vec![pad.clone(), fld_let.clone(), mut_kw.clone(), field.clone(), fld_eq.clone(), var_name.clone(), fld_dot.clone(), field.clone(), fld_cln.clone()];
        lines.push(s_join(fld_parts.clone()).clone());
    }
    let mut code: String = s_join_nl(lines.clone());
    let mut newline: String = "\n".to_string();
    return s_cat(code.clone(), newline.clone());
}

fn gen_destructure(tokens: TokensRef, pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
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
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
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
    let mut after: i32 = adv_nl_ref(src_end.clone(), tokens.clone());
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = (fields.len() as i32);
    for field_index in 0..field_count {
        let mut field: String = fields[field_index as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            mut_kw = "mut ".to_string();
        }
        let mut dst_let: String = "let ".to_string();
        let mut dst_eq: String = " = ".to_string();
        let mut dst_dot: String = ".".to_string();
        let mut dst_cln: String = ".clone();".to_string();
        let mut dst_parts: Vec<String> = vec![pad.clone(), dst_let.clone(), mut_kw.clone(), field.clone(), dst_eq.clone(), src_code.clone(), dst_dot.clone(), field.clone(), dst_cln.clone()];
        dest_lines.push(s_join(dst_parts.clone()).clone());
    }
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    let mut newline: String = "\n".to_string();
    dest_code = s_cat(dest_code.clone(), newline.clone());
    return make_result(dest_code, after.clone());
}

fn gen_stmt(tokens: TokensRef, pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
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
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut newline: String = "\n".to_string();
    if kind == "KW_RUST" {
        let mut block_pos: i32 = pos + 2.clone();
        let mut block_token: Token = tokens[block_pos as usize].clone();
        let value = block_token.value.clone();
        let mut block_content: String = value.clone();
        let mut rust_lines: Vec<String> = s_split(block_content.clone(), newline.clone());
        let mut padded: Vec<String> = Vec::new();
        let mut rust_line_count: i32 = (rust_lines.len() as i32);
        for rust_index in 0..rust_line_count {
            let mut rust_line: String = rust_lines[rust_index as usize].clone();
            if is_empty(rust_line.clone()) {
                let mut empty_line: String = "".to_string();
                padded.push(empty_line.clone());
            } else {
                let mut rsl_parts: Vec<String> = vec![pad.clone(), rust_line.clone()];
                padded.push(s_join(rsl_parts.clone()).clone());
            }
        }
        let mut block_code: String = s_join_nl(padded.clone());
        block_code = s_cat(block_code.clone(), newline.clone());
        let mut block_next: i32 = block_pos + 1.clone();
        return make_result(block_code, block_next.clone());
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
        let mut ret_kw: String = "return ".to_string();
        let mut ret_sc: String = ";\n".to_string();
        let mut ret_parts: Vec<String> = vec![pad.clone(), ret_kw.clone(), val_code.clone(), ret_suffix.clone(), ret_sc.clone()];
        let mut ret_code: String = s_join(ret_parts.clone());
        let mut ret_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
        return make_result(ret_code.clone(), ret_next.clone());
    }
    if kind == "KW_BREAK" {
        let mut brk_kw: String = "break;\n".to_string();
        let mut brk_parts: Vec<String> = vec![pad.clone(), brk_kw.clone()];
        let mut brk_code: String = s_join(brk_parts.clone());
        let mut brk_next: i32 = pos + 1.clone();
        let mut brk_after: i32 = adv_nl_ref(brk_next.clone(), tokens.clone());
        return make_result(brk_code.clone(), brk_after.clone());
    }
    if kind == "KW_CONTINUE" {
        let mut cnt_kw: String = "continue;\n".to_string();
        let mut cnt_parts: Vec<String> = vec![pad.clone(), cnt_kw.clone()];
        let mut cnt_code: String = s_join(cnt_parts.clone());
        let mut cnt_next: i32 = pos + 1.clone();
        let mut cnt_after: i32 = adv_nl_ref(cnt_next.clone(), tokens.clone());
        return make_result(cnt_code.clone(), cnt_after.clone());
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
                let mut avw_sfx: String = ".unwrap();\n".to_string();
                let mut avw_parts: Vec<String> = vec![pad.clone(), expr_code.clone(), avw_sfx.clone()];
                let mut avow_code: String = s_join(avw_parts.clone());
                let mut avow_next: i32 = adv_nl_ref(after_rparen.clone(), tokens.clone());
                return make_result(avow_code.clone(), avow_next.clone());
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
        let mut var_next: i32 = var_pos + 1.clone();
        let mut body_start: i32 = skip_to_body_ref(tokens.clone(), var_next.clone());
/* unhandled(IDENT) */
        let using_ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone() };
        let mut using_ctx: RcCtx = make_rctx(using_ctx_raw);
        let mut block_r: ParseResult = gen_block(tokens.clone(), body_start.clone(), depth.clone(), using_ctx);
        let mut blk_code: String = pr_code(block_r.clone());
        let mut blk_pos: i32 = pr_pos(block_r.clone());
        let mut full_code: String = s_cat(init_destruct.clone(), blk_code.clone());
        return make_result(full_code, blk_pos.clone());
    }
    if kind == "IDENT" {
        let mut ident_name: String = value.clone();
        let mut next_pos: i32 = pos + 1.clone();
        if next_pos >= token_count {
            let mut eof_code: String = "/* eof */\n".to_string();
            let mut eof_next: i32 = pos + 1.clone();
            return make_result(eof_code.clone(), eof_next.clone());
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
                let mut field_count: i32 = (fields.len() as i32);
                let mut field_pairs: Vec<String> = Vec::new();
                for field_idx in 0..field_count {
                    let mut field_name: String = fields[field_idx as usize].clone();
                    let mut fp_sep: String = ": ".to_string();
                    let mut fp_cln: String = ".clone()".to_string();
                    let mut fp_parts: Vec<String> = vec![field_name.clone(), fp_sep.clone(), field_name.clone(), fp_cln.clone()];
                    field_pairs.push(s_join(fp_parts.clone()).clone());
                }
                let mut sep: String = ", ".to_string();
                let mut fields_code: String = s_join_with(field_pairs.clone(), sep.clone());
                let mut sas_let: String = "let ".to_string();
                let mut sas_eq: String = " = ".to_string();
                let mut sas_ob: String = " { ".to_string();
                let mut sas_cb: String = " };\n".to_string();
                let mut sas_parts: Vec<String> = vec![pad.clone(), sas_let.clone(), mut_kw.clone(), ident_name.clone(), sas_eq.clone(), struct_name.clone(), sas_ob.clone(), fields_code.clone(), sas_cb.clone()];
                let mut stmt_code: String = s_join(sas_parts.clone());
                let mut stmt_next: i32 = adv_nl_ref(fend.clone(), tokens.clone());
                return make_result(stmt_code, stmt_next.clone());
            }
            if kind == "KW_EMPTY" {
                let mut emp_pfx: String = "let mut ".to_string();
                let mut emp_sfx: String = " = Vec::new();\n".to_string();
                let mut emp_parts: Vec<String> = vec![pad.clone(), emp_pfx.clone(), ident_name.clone(), emp_sfx.clone()];
                let mut empty_code: String = s_join(emp_parts.clone());
                let mut after_empty_as: i32 = after_as + 1.clone();
                let mut empty_next: i32 = adv_nl_ref(after_empty_as.clone(), tokens.clone());
                return make_result(empty_code.clone(), empty_next.clone());
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
                        let mut sep: String = ", ".to_string();
                        let mut fields_code: String = s_join_with(override_fields.clone(), sep.clone());
                        let mut is_mut: bool = list_has(mut_names.clone(), ident_name.clone());
                        let mut mut_kw: String = "".to_string();
                        if is_mut {
                            mut_kw = "mut ".to_string();
                        }
                        let mut wth_let: String = "let ".to_string();
                        let mut wth_eq: String = " = ".to_string();
                        let mut wth_ob: String = " { ".to_string();
                        let mut wth_sp: String = ", ..".to_string();
                        let mut wth_cb: String = " };\n".to_string();
                        let mut wth_parts: Vec<String> = vec![pad.clone(), wth_let.clone(), mut_kw.clone(), ident_name.clone(), wth_eq.clone(), struct_name.clone(), wth_ob.clone(), fields_code.clone(), wth_sp.clone(), source_name.clone(), wth_cb.clone()];
                        let mut with_code: String = s_join(wth_parts.clone());
                        let mut with_next: i32 = adv_nl_ref(fend.clone(), tokens.clone());
                        return make_result(with_code, with_next.clone());
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
            let mut asc_let: String = "let ".to_string();
            let mut asc_eq: String = " = ".to_string();
            let mut asc_sc: String = ";\n".to_string();
            let mut asc_parts: Vec<String> = vec![pad.clone(), asc_let.clone(), mut_kw.clone(), ident_name.clone(), asc_eq.clone(), val_code.clone(), suffix.clone(), asc_sc.clone()];
            let mut as_code: String = s_join(asc_parts.clone());
            let mut as_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
            return make_result(as_code.clone(), as_next.clone());
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
                    let mut shm_eq: String = " = ".to_string();
                    let mut shm_op: String = "(".to_string();
                    let mut shm_cl: String = ".clone(), ".to_string();
                    let mut shm_ea: String = ".clone());\n".to_string();
                    let mut shm_parts: Vec<String> = vec![pad.clone(), using_var.clone(), shm_eq.clone(), ident_name.clone(), shm_op.clone(), using_var.clone(), shm_cl.clone(), extra_arg.clone(), shm_ea.clone()];
                    shim_code = s_join(shm_parts.clone());
                } else {
                    let mut shm2_eq: String = " = ".to_string();
                    let mut shm2_op: String = "(".to_string();
                    let mut shm2_cl: String = ".clone());\n".to_string();
                    let mut shm2_parts: Vec<String> = vec![pad.clone(), using_var.clone(), shm2_eq.clone(), ident_name.clone(), shm2_op.clone(), using_var.clone(), shm2_cl.clone()];
                    shim_code = s_join(shm2_parts.clone());
                }
                let mut re_destruct: String = make_destruct_code(using_var.clone(), using_type.clone(), struct_reg.clone(), mut_names.clone(), pad.clone());
                shim_code = s_cat(shim_code.clone(), re_destruct.clone());
                let mut shm_next: i32 = adv_nl_ref(after_with.clone(), tokens.clone());
                return make_result(shim_code, shm_next.clone());
            }
            let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
            let mut args_code: String = pr_code(args_r.clone());
            let mut args_end: i32 = pr_pos(args_r.clone());
            let mut after_paren: i32 = args_end + 1.clone();
            let mut call_code: String = "".to_string();
            if ident_name == "print" {
                let mut prt_pfx: String = "println!(\"{}\", ".to_string();
                let mut prt_sfx: String = ");\n".to_string();
                let mut prt_parts: Vec<String> = vec![pad.clone(), prt_pfx.clone(), args_code.clone(), prt_sfx.clone()];
                call_code = s_join(prt_parts.clone());
            } else if ident_name == "crash" {
                let mut crsh_pfx: String = "panic!(\"{}\", ".to_string();
                let mut crsh_sfx: String = ");\n".to_string();
                let mut crsh_parts: Vec<String> = vec![pad.clone(), crsh_pfx.clone(), args_code.clone(), crsh_sfx.clone()];
                call_code = s_join(crsh_parts.clone());
            } else {
                let mut cal_op: String = "(".to_string();
                let mut cal_sfx: String = ");\n".to_string();
                let mut cal_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), cal_op.clone(), args_code.clone(), cal_sfx.clone()];
                call_code = s_join(cal_parts.clone());
            }
            let mut call_next: i32 = adv_nl_ref(after_paren.clone(), tokens.clone());
            return make_result(call_code, call_next.clone());
        }
        if kind == "KW_INSERT" {
            let mut val_pos: i32 = next_pos + 1.clone();
            let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let mut val_code: String = pr_code(val_r.clone());
            let mut val_end: i32 = pr_pos(val_r.clone());
            let mut val_tok: Token = tokens[val_pos as usize].clone();
            let kind = val_tok.kind.clone();
            let mut push_val: String = emit_val(val_code.clone(), kind.clone());
            let mut psh_pfx: String = ".push(".to_string();
            let mut psh_sfx: String = ");\n".to_string();
            let mut psh_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), psh_pfx.clone(), push_val.clone(), psh_sfx.clone()];
            let mut push_code: String = s_join(psh_parts.clone());
            let mut push_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
            return make_result(push_code.clone(), push_next.clone());
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
                    let mut app_val: String = emit_val(val_code.clone(), kind.clone());
                    let mut app_pfx: String = ".push(".to_string();
                    let mut app_sfx: String = ");\n".to_string();
                    let mut app_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), app_pfx.clone(), app_val.clone(), app_sfx.clone()];
                    let mut app_code: String = s_join(app_parts.clone());
                    let mut app_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
                    return make_result(app_code.clone(), app_next.clone());
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
                let mut idx_val: String = emit_val(val_code.clone(), kind.clone());
                let mut idw_op: String = "[".to_string();
                let mut idw_mid: String = " as usize] = ".to_string();
                let mut idw_sfx: String = ";\n".to_string();
                let mut idw_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), idw_op.clone(), idx_code.clone(), idw_mid.clone(), idx_val.clone(), idw_sfx.clone()];
                let mut idx_code2: String = s_join(idw_parts.clone());
                let mut idx_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
                return make_result(idx_code2.clone(), idx_next.clone());
            }
        }
        if kind == "KW_REMOVE" {
            let mut idx_pos: i32 = next_pos + 2.clone();
            let mut idx_r: ParseResult = gen_expr(tokens.clone(), idx_pos.clone(), ctx.clone());
            let mut idx_code: String = pr_code(idx_r.clone());
            let mut idx_end: i32 = pr_pos(idx_r.clone());
            let mut rem_pfx: String = ".remove(".to_string();
            let mut rem_sfx: String = " as usize);\n".to_string();
            let mut rem_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), rem_pfx.clone(), idx_code.clone(), rem_sfx.clone()];
            let mut rem_code: String = s_join(rem_parts.clone());
            let mut rem_next: i32 = adv_nl_ref(idx_end.clone(), tokens.clone());
            return make_result(rem_code.clone(), rem_next.clone());
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
            let mut asg_eq: String = " = ".to_string();
            let mut asg_sc: String = ";\n".to_string();
            let mut asg_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), asg_eq.clone(), val_code.clone(), assign_suffix.clone(), asg_sc.clone()];
            let mut asgn_code: String = s_join(asg_parts.clone());
            let mut asgn_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
            return make_result(asgn_code.clone(), asgn_next.clone());
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
                        let mut val_next_pos: i32 = val_pos + 1.clone();
                        let mut after_empty: i32 = adv_nl_ref(val_next_pos.clone(), tokens.clone());
                        if is_validator {
                            let mut vv_pfx: String = "let mut ".to_string();
                            let mut vv_mid: String = ": Option<".to_string();
                            let mut vv_sfx: String = "> = None;\n".to_string();
                            let mut vv_parts: Vec<String> = vec![pad.clone(), vv_pfx.clone(), var_name.clone(), vv_mid.clone(), var_type.clone(), vv_sfx.clone()];
                            let mut vv_code: String = s_join(vv_parts.clone());
                            return make_result(vv_code.clone(), after_empty.clone());
                        }
                        if is_shape {
                            let mut sh_pfx: String = "let mut ".to_string();
                            let mut sh_mid: String = ": ".to_string();
                            let mut sh_sfx: String = " = Vec::new();\n".to_string();
                            let mut sh_parts: Vec<String> = vec![pad.clone(), sh_pfx.clone(), var_name.clone(), sh_mid.clone(), rust_type.clone(), sh_sfx.clone()];
                            let mut sh_code: String = s_join(sh_parts.clone());
                            return make_result(sh_code.clone(), after_empty.clone());
                        }
                        let mut err_msg: String = "/* error: empty only valid for validator types and list shapes */\n".to_string();
                        let mut err_parts: Vec<String> = vec![pad.clone(), err_msg.clone()];
                        let mut err_code: String = s_join(err_parts.clone());
                        return make_result(err_code.clone(), after_empty.clone());
                    }
                    if kind == "LPAREN" {
                        let mut peek_pos: i32 = val_pos + 1.clone();
                        let mut peek_token: Token = tokens[peek_pos as usize].clone();
                        let kind = peek_token.kind.clone();
                        let mut is_avow_expr: bool = kind == "KW_AVOW".clone();
                        if !is_avow_expr {
                            let mut struct_fields_str: String = reg_get(struct_reg.clone(), var_type.clone());
                            let mut comma: String = ",".to_string();
                            let mut field_names: Vec<String> = s_split(struct_fields_str.clone(), comma.clone());
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
                                    let mut sfp_sep: String = ": ".to_string();
                                    let mut sfp_cln: String = ".clone()".to_string();
                                    let mut sfp_parts: Vec<String> = vec![fname.clone(), sfp_sep.clone(), fv_code.clone(), sfp_cln.clone()];
                                    field_pairs.push(s_join(sfp_parts.clone()).clone());
                                    fni = fni + 1;
                                }
                                fend = pr_pos(fv_r.clone());
                            }
                            let mut sep: String = ", ".to_string();
                            let mut fields_code: String = s_join_with(field_pairs.clone(), sep.clone());
                            let mut is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                            let mut mut_kw: String = "".to_string();
                            if is_mut {
                                mut_kw = "mut ".to_string();
                            }
                            let mut scc_let: String = "let ".to_string();
                            let mut scc_eq: String = " = ".to_string();
                            let mut scc_ob: String = " { ".to_string();
                            let mut scc_cb: String = " };\n".to_string();
                            let mut scc_parts: Vec<String> = vec![pad.clone(), scc_let.clone(), mut_kw.clone(), var_name.clone(), scc_eq.clone(), var_type.clone(), scc_ob.clone(), fields_code.clone(), scc_cb.clone()];
                            let mut sc_code: String = s_join(scc_parts.clone());
                            let mut sc_next: i32 = adv_nl_ref(fend.clone(), tokens.clone());
                            return make_result(sc_code.clone(), sc_next.clone());
                        }
                        let mut inner_pos: i32 = peek_pos + 1.clone();
                        let mut inner_r: ParseResult = gen_expr(tokens.clone(), inner_pos.clone(), ctx.clone());
                        let mut inner_code: String = pr_code(inner_r.clone());
                        let mut after_rparen: i32 = pr_pos(inner_r.clone()) + 1;
                        let mut suf_unwrap: String = ".unwrap()".to_string();
                        let mut suf_unwrap0: String = ".unwrap().0".to_string();
                        let mut unwrap_expr: String = s_cat(inner_code.clone(), suf_unwrap.clone());
                        if var_type == "int" {
                            unwrap_expr = s_cat(inner_code.clone(), suf_unwrap0.clone());
                        }
                        if var_type == "string" {
                            unwrap_expr = s_cat(inner_code.clone(), suf_unwrap0.clone());
                        }
                        if var_type == "bool" {
                            unwrap_expr = s_cat(inner_code.clone(), suf_unwrap0.clone());
                        }
                        if var_type == "float" {
                            unwrap_expr = s_cat(inner_code.clone(), suf_unwrap0.clone());
                        }
                        let mut awc_let: String = "let ".to_string();
                        let mut awc_col: String = ": ".to_string();
                        let mut awc_eq: String = " = ".to_string();
                        let mut awc_sc: String = ";\n".to_string();
                        let mut awc_parts: Vec<String> = vec![pad.clone(), awc_let.clone(), var_name.clone(), awc_col.clone(), rust_type.clone(), awc_eq.clone(), unwrap_expr.clone(), awc_sc.clone()];
                        let mut aw_code: String = s_join(awc_parts.clone());
                        let mut aw_next: i32 = adv_nl_ref(after_rparen.clone(), tokens.clone());
                        return make_result(aw_code.clone(), aw_next.clone());
                    }
                    if kind == "LBRACKET" {
                        let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                        let mut val_code: String = pr_code(val_r.clone());
                        let mut val_end: i32 = pr_pos(val_r.clone());
                        let mut lst_pfx: String = "let mut ".to_string();
                        let mut lst_col: String = ": ".to_string();
                        let mut lst_eq: String = " = ".to_string();
                        let mut lst_sc: String = ";\n".to_string();
                        let mut lst_parts: Vec<String> = vec![pad.clone(), lst_pfx.clone(), var_name.clone(), lst_col.clone(), rust_type.clone(), lst_eq.clone(), val_code.clone(), lst_sc.clone()];
                        let mut lst_code: String = s_join(lst_parts.clone());
                        let mut lst_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
                        return make_result(lst_code.clone(), lst_next.clone());
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
                        let mut vld_let: String = "let ".to_string();
                        let mut vld_opt: String = ": Option<".to_string();
                        let mut vld_new: String = "> = ".to_string();
                        let mut vld_nop: String = "::new(".to_string();
                        let mut vld_sc: String = ");\n".to_string();
                        let mut vld_parts: Vec<String> = vec![pad.clone(), vld_let.clone(), mut_kw.clone(), var_name.clone(), vld_opt.clone(), var_type.clone(), vld_new.clone(), var_type.clone(), vld_nop.clone(), val_code.clone(), vld_sc.clone()];
                        let mut vld_code: String = s_join(vld_parts.clone());
                        let mut vld_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
                        return make_result(vld_code.clone(), vld_next.clone());
                    }
                    if kind == "KW_NONE" {
                        let mut non_pfx: String = "let mut ".to_string();
                        let mut non_mid: String = ": Option<".to_string();
                        let mut non_sfx: String = "> = None;\n".to_string();
                        let mut non_parts: Vec<String> = vec![pad.clone(), non_pfx.clone(), var_name.clone(), non_mid.clone(), rust_type.clone(), non_sfx.clone()];
                        let mut none_code: String = s_join(non_parts.clone());
                        let mut none_pos_next: i32 = val_pos + 1.clone();
                        let mut none_next: i32 = adv_nl_ref(none_pos_next.clone(), tokens.clone());
                        return make_result(none_code.clone(), none_next.clone());
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
                    let mut bnd_let: String = "let ".to_string();
                    let mut bnd_col: String = ": ".to_string();
                    let mut bnd_eq: String = " = ".to_string();
                    let mut bnd_sc: String = ";\n".to_string();
                    let mut bnd_parts: Vec<String> = vec![pad.clone(), bnd_let.clone(), mut_kw.clone(), var_name.clone(), bnd_col.clone(), rust_type.clone(), bnd_eq.clone(), val_code.clone(), suffix.clone(), bnd_sc.clone()];
                    let mut bind_code: String = s_join(bnd_parts.clone());
                    let mut bind_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
                    return make_result(bind_code.clone(), bind_next.clone());
                }
            }
        }
    }
    let mut unh_pfx: String = "/* unhandled(".to_string();
    let mut unh_sfx: String = ") */\n".to_string();
    let mut unh_parts: Vec<String> = vec![unh_pfx.clone(), kind.clone(), unh_sfx.clone()];
    let mut unhand: String = s_join(unh_parts.clone());
    let mut unhand_next: i32 = pos + 1.clone();
    return make_result(unhand.clone(), unhand_next.clone());
}

fn gen_block(tokens: TokensRef, pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
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
    let mut block_joined: String = s_join(stmts.clone());
    return make_result(block_joined.clone(), cur.clone());
}

fn gen_if_branch(tokens: TokensRef, cond_pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let mut cond_end: i32 = pr_pos(cond_r.clone());
    let mut body_start: i32 = skip_to_body_ref(tokens.clone(), cond_end.clone());
    let mut body_depth: i32 = depth + 1.clone();
    let mut body_r: ParseResult = gen_block(tokens.clone(), body_start.clone(), body_depth.clone(), ctx.clone());
    let mut cond_code: String = pr_code(cond_r.clone());
    let mut body_code: String = pr_code(body_r.clone());
    let mut brc_open: String = " {\n".to_string();
    let mut cmb_parts: Vec<String> = vec![cond_code.clone(), brc_open.clone(), body_code.clone()];
    let mut combined: String = s_join(cmb_parts.clone());
    let mut branch_end: i32 = pr_pos(body_r.clone());
    return make_result(combined, branch_end.clone());
}

fn gen_if(tokens: TokensRef, pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut if_cond_pos: i32 = pos + 1.clone();
    let mut then_r: ParseResult = gen_if_branch(tokens.clone(), if_cond_pos.clone(), depth.clone(), ctx.clone());
    let mut then_code: String = pr_code(then_r.clone());
    let mut if_kw: String = "if ".to_string();
    let mut brc_cl: String = "}".to_string();
    let mut if_parts: Vec<String> = vec![pad.clone(), if_kw.clone(), then_code.clone(), pad.clone(), brc_cl.clone()];
    let mut result_code: String = s_join(if_parts.clone());
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
            let mut ei_cond: i32 = after_else + 1.clone();
            let mut ei_r: ParseResult = gen_if_branch(tokens.clone(), ei_cond.clone(), depth.clone(), ctx.clone());
            let mut ei_code: String = pr_code(ei_r.clone());
            let mut eli_kw: String = " else if ".to_string();
            let mut eli_cl: String = "}".to_string();
            let mut eli_parts: Vec<String> = vec![result_code.clone(), eli_kw.clone(), ei_code.clone(), pad.clone(), eli_cl.clone()];
            result_code = s_join(eli_parts.clone());
            cur = pr_pos(ei_r.clone());
        } else {
            let mut else_body_start: i32 = skip_to_body_ref(tokens.clone(), after_else.clone());
            let mut else_depth: i32 = depth + 1.clone();
            let mut else_r: ParseResult = gen_block(tokens.clone(), else_body_start.clone(), else_depth.clone(), ctx.clone());
            let mut else_code: String = pr_code(else_r.clone());
            let mut els_kw: String = " else {\n".to_string();
            let mut els_cl: String = "}".to_string();
            let mut els_parts: Vec<String> = vec![result_code.clone(), els_kw.clone(), else_code.clone(), pad.clone(), els_cl.clone()];
            result_code = s_join(els_parts.clone());
            cur = pr_pos(else_r.clone());
            break;
        }
    }
    let mut if_newline: String = "\n".to_string();
    result_code = s_cat(result_code.clone(), if_newline.clone());
    return make_result(result_code, cur.clone());
}

fn gen_for(tokens: TokensRef, pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut next_pos: i32 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_IF" {
        let mut cond_pos: i32 = next_pos + 1.clone();
        let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
        let mut cond_code: String = pr_code(cond_r.clone());
        let mut cond_end: i32 = pr_pos(cond_r.clone());
        let mut while_body_start: i32 = skip_to_body_ref(tokens.clone(), cond_end.clone());
        let mut whl_depth: i32 = depth + 1.clone();
        let mut while_body_r: ParseResult = gen_block(tokens.clone(), while_body_start.clone(), whl_depth.clone(), ctx.clone());
        let mut while_body_code: String = pr_code(while_body_r.clone());
        let mut while_body_end: i32 = pr_pos(while_body_r.clone());
        let mut whl_kw: String = "while ".to_string();
        let mut whl_ob: String = " {\n".to_string();
        let mut whl_cb: String = "}\n".to_string();
        let mut whl_parts: Vec<String> = vec![pad.clone(), whl_kw.clone(), cond_code.clone(), whl_ob.clone(), while_body_code.clone(), pad.clone(), whl_cb.clone()];
        let mut while_code: String = s_join(whl_parts.clone());
        return make_result(while_code, while_body_end.clone());
    }
    if kind == "KW_GIVEUP" {
        let mut lparen_pos: i32 = next_pos + 1.clone();
        let mut var_pos: i32 = lparen_pos + 1.clone();
        let mut var_tok: Token = tokens[var_pos as usize].clone();
        let value = var_tok.value.clone();
        let mut giveup_var: String = value.clone();
        let mut in_pos: i32 = var_pos + 1.clone();
        let mut iter_pos: i32 = in_pos + 1.clone();
        let mut iter_r: ParseResult = gen_expr(tokens.clone(), iter_pos.clone(), ctx.clone());
        let mut iter_code: String = pr_code(iter_r.clone());
        let mut iter_end: i32 = pr_pos(iter_r.clone());
        let mut iter_next: i32 = iter_end + 1.clone();
        let mut body_start: i32 = skip_to_body_ref(tokens.clone(), iter_next.clone());
        let mut body_depth: i32 = depth + 1.clone();
        let mut body_r: ParseResult = gen_block(tokens.clone(), body_start.clone(), body_depth.clone(), ctx.clone());
        let mut body_code: String = pr_code(body_r.clone());
        let mut body_end: i32 = pr_pos(body_r.clone());
        let mut gfr_kw: String = "for ".to_string();
        let mut gfr_in: String = " in ".to_string();
        let mut gfr_ob: String = " {\n".to_string();
        let mut gfr_cb: String = "}\n".to_string();
        let mut gfr_parts: Vec<String> = vec![pad.clone(), gfr_kw.clone(), giveup_var.clone(), gfr_in.clone(), iter_code.clone(), gfr_ob.clone(), body_code.clone(), pad.clone(), gfr_cb.clone()];
        let mut for_code: String = s_join(gfr_parts.clone());
        return make_result(for_code, body_end.clone());
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
            let mut rng_dot: String = "..".to_string();
            let mut rng_parts: Vec<String> = vec![first_code.clone(), rng_dot.clone(), second_code.clone()];
            range_expr = s_join(rng_parts.clone());
            body_tok_pos = second_p + 1;
        } else {
            let mut rng0_pfx: String = "0..".to_string();
            let mut rng0_parts: Vec<String> = vec![rng0_pfx.clone(), first_code.clone()];
            range_expr = s_join(rng0_parts.clone());
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
        let mut rng2_dot: String = "..".to_string();
        let mut rng2_parts: Vec<String> = vec![start_code.clone(), rng2_dot.clone(), end_code.clone()];
        range_expr = s_join(rng2_parts.clone());
        body_tok_pos = end_p + 1;
    } else {
        let mut src_r: ParseResult = gen_expr(tokens.clone(), iter_pos.clone(), ctx.clone());
        let mut src_code: String = pr_code(src_r.clone());
        let mut src_p: i32 = pr_pos(src_r.clone());
        range_expr = src_code;
        body_tok_pos = src_p;
    }
    body_tok_pos = skip_to_body_ref(tokens.clone(), body_tok_pos.clone());
    let mut for_body_depth: i32 = depth + 1.clone();
    let mut body_r: ParseResult = gen_block(tokens.clone(), body_tok_pos.clone(), for_body_depth.clone(), ctx.clone());
    let mut body_code: String = pr_code(body_r.clone());
    let mut body_end: i32 = pr_pos(body_r.clone());
    let mut frc_kw: String = "for ".to_string();
    let mut frc_in: String = " in ".to_string();
    let mut frc_ob: String = " {\n".to_string();
    let mut frc_cb: String = "}\n".to_string();
    let mut frc_parts: Vec<String> = vec![pad.clone(), frc_kw.clone(), var_name.clone(), frc_in.clone(), range_expr.clone(), frc_ob.clone(), body_code.clone(), pad.clone(), frc_cb.clone()];
    let mut for_code: String = s_join(frc_parts.clone());
    return make_result(for_code.clone(), body_end.clone());
}

fn gen_struct_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at(tokens.clone(), start_pos.clone());
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
            let mut fln_ind: String = "    ".to_string();
            let mut fln_sep: String = ": ".to_string();
            let mut fln_com: String = ",".to_string();
            let mut fln_parts: Vec<String> = vec![fln_ind.clone(), field_name.clone(), fln_sep.clone(), rust_type.clone(), fln_com.clone()];
            field_lines.push(s_join(fln_parts.clone()).clone());
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        }
    }
    let mut fields_code: String = s_join_nl(field_lines.clone());
    let mut sdcl_pfx: String = "#[derive(Clone, PartialEq, Debug)]\nstruct ".to_string();
    let mut sdcl_ob: String = " {\n".to_string();
    let mut sdcl_cb: String = "\n}\n\n".to_string();
    let mut sdcl_parts: Vec<String> = vec![sdcl_pfx.clone(), struct_name.clone(), sdcl_ob.clone(), fields_code.clone(), sdcl_cb.clone()];
    let mut decl: String = s_join(sdcl_parts.clone());
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
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
        let mut shp_pfx: String = "type ".to_string();
        let mut shp_mid: String = " = Vec<".to_string();
        let mut shp_sfx: String = ">;\n\n".to_string();
        let mut shp_parts: Vec<String> = vec![shp_pfx.clone(), rust_name.clone(), shp_mid.clone(), rust_elem.clone(), shp_sfx.clone()];
        let mut decl: String = s_join(shp_parts.clone());
        let mut after: i32 = elem_pos + 1.clone();
        let mut shape_next: i32 = adv_nl(after.clone(), tokens.clone());
        return make_result(decl, shape_next.clone());
    }
    let mut out_pos: i32 = pos + 7.clone();
    let mut out_token: Token = tokens[out_pos as usize].clone();
    let value = out_token.value.clone();
    let mut out_type: String = value.clone();
    let mut rust_in: String = render_rust_type(elem_type.clone());
    let mut rust_out: String = render_rust_type(out_type.clone());
    let mut ost_pfx: String = " -> ".to_string();
    let mut ost_parts: Vec<String> = vec![ost_pfx.clone(), rust_out.clone()];
    let mut out_suffix: String = s_join(ost_parts.clone());
    if is_empty(rust_out.clone()) {
        out_suffix = "".to_string();
    }
    let mut fns_pfx: String = "type ".to_string();
    let mut fns_mid: String = " = fn(".to_string();
    let mut fns_rp: String = ")".to_string();
    let mut fns_sfx: String = ";\n\n".to_string();
    let mut fns_parts: Vec<String> = vec![fns_pfx.clone(), rust_name.clone(), fns_mid.clone(), rust_in.clone(), fns_rp.clone(), out_suffix.clone(), fns_sfx.clone()];
    let mut decl: String = s_join(fns_parts.clone());
    let mut after: i32 = out_pos + 1.clone();
    let mut fn_shape_next: i32 = adv_nl(after.clone(), tokens.clone());
    return make_result(decl, fn_shape_next.clone());
}

fn gen_enum_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at(tokens.clone(), start_pos.clone());
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
            let mut vln_ind: String = "    ".to_string();
            let mut vln_com: String = ",".to_string();
            let mut vln_parts: Vec<String> = vec![vln_ind.clone(), value.clone(), vln_com.clone()];
            variant_lines.push(s_join(vln_parts.clone()).clone());
        }
    }
    let mut variants_code: String = s_join_nl(variant_lines.clone());
    let mut enm_pfx: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nenum ".to_string();
    let mut enm_ob: String = " {\n".to_string();
    let mut enm_cb: String = "\n}\n\n".to_string();
    let mut enm_parts: Vec<String> = vec![enm_pfx.clone(), rust_name.clone(), enm_ob.clone(), variants_code.clone(), enm_cb.clone()];
    let mut decl: String = s_join(enm_parts.clone());
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
}

fn gen_type_decl(tokens: Vec<Token>, pos: i32, ctx: RcCtx) -> ParseResult {
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at(tokens.clone(), start_pos.clone());
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
    let mut tokens_ref: TokensRef = tokens_wrap(tokens.clone());
    let mut pred_r: ParseResult = gen_expr(tokens_ref.clone(), pos.clone(), ctx.clone());
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
    let mut tsc_pfx: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nstruct ".to_string();
    let mut tsc_op: String = "(".to_string();
    let mut tsc_sfx: String = ");\n\n".to_string();
    let mut tsc_parts: Vec<String> = vec![tsc_pfx.clone(), type_name.clone(), tsc_op.clone(), rust_param_type.clone(), tsc_sfx.clone()];
    let mut struct_code: String = s_join(tsc_parts.clone());
    let mut imp_pfx: String = "impl ".to_string();
    let mut imp_fn: String = " {\n    fn new(".to_string();
    let mut imp_col: String = ": ".to_string();
    let mut imp_ret: String = ") -> Option<Self> {\n        if ".to_string();
    let mut imp_som: String = " {\n            Some(".to_string();
    let mut imp_inn: String = "(".to_string();
    let mut imp_sfx: String = "))\n        } else {\n            None\n        }\n    }\n}\n\n".to_string();
    let mut imp_parts: Vec<String> = vec![imp_pfx.clone(), type_name.clone(), imp_fn.clone(), param_name.clone(), imp_col.clone(), rust_param_type.clone(), imp_ret.clone(), pred_code.clone(), imp_som.clone(), type_name.clone(), imp_inn.clone(), param_name.clone(), imp_sfx.clone()];
    let mut impl_code: String = s_join(imp_parts.clone());
    let mut type_code: String = s_cat(struct_code, impl_code);
    return make_result(type_code.clone(), pos.clone());
}

fn gen_fn_decl(tokens: Vec<Token>, pos: i32, ctx: RcCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut using_type = ctx.using_type.clone();
    let mut using_var = ctx.using_var.clone();
    let mut var_type_reg = ctx.var_type_reg.clone();
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at(tokens.clone(), start_pos.clone());
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
            let mut prm_sep: String = ": ".to_string();
            let mut prm_parts: Vec<String> = vec![param_name.clone(), prm_sep.clone(), rust_param_type.clone()];
            param_strs.push(s_join(prm_parts.clone()).clone());
            cur = cur_next(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else {
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
    let mut body_slice_end: i32 = body_end_pos + 1.clone();
    let mut body_tokens_raw: Vec<Token> = l_slice(tokens.clone(), body_start.clone(), body_slice_end.clone());
    let mut body_len: i32 = (body_tokens_raw.len() as i32);
    let mut zero: i32 = 0;
    let mut body_last: i32 = body_len - 1.clone();
    let mut mut_names: Vec<String> = collect_mut_names(body_tokens_raw.clone(), zero.clone(), body_last.clone());
    let mut var_type_reg: Vec<String> = build_var_type_reg(body_tokens_raw.clone());
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
/* unhandled(IDENT) */
    let body_ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone() };
    let mut body_ctx: RcCtx = make_rctx(body_ctx_raw);
    let mut body_tokens: TokensRef = tokens_wrap(body_tokens_raw);
    let mut body_pos: i32 = 0;
    let mut body_depth: i32 = 1;
    let mut body_r: ParseResult = gen_block(body_tokens, body_pos.clone(), body_depth.clone(), body_ctx);
    let mut body_code: String = pr_code(body_r.clone());
    let mut body_end: i32 = body_start + pr_pos(body_r.clone()).clone();
    let mut sep: String = ", ".to_string();
    let mut params_code: String = s_join_with(param_strs.clone(), sep.clone());
    let mut ret_suffix: String = "".to_string();
    if !is_empty(ret_type.clone()) {
        let mut rts_pfx: String = " -> ".to_string();
        let mut rts_parts: Vec<String> = vec![rts_pfx.clone(), ret_type.clone()];
        ret_suffix = s_join(rts_parts.clone());
    }
    let mut fnc_kw: String = "fn ".to_string();
    let mut fnc_op: String = "(".to_string();
    let mut fnc_rp: String = ")".to_string();
    let mut fnc_ob: String = " {\n".to_string();
    let mut fnc_cb: String = "}\n\n".to_string();
    let mut fnc_parts: Vec<String> = vec![fnc_kw.clone(), fn_name.clone(), fnc_op.clone(), params_code.clone(), fnc_rp.clone(), ret_suffix.clone(), fnc_ob.clone(), body_code.clone(), fnc_cb.clone()];
    let mut fn_code: String = s_join(fnc_parts.clone());
    return make_result(fn_code, body_end.clone());
}

fn gen_raw_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut name_pos: i32 = pos + 1.clone();
    let mut after: i32 = name_pos + 1.clone();
    let mut emp: String = "".to_string();
    let mut raw_next: i32 = adv_nl(after.clone(), tokens.clone());
    return make_result(emp.clone(), raw_next.clone());
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
    let mut newline: String = "\n".to_string();
    let mut lines: Vec<String> = s_split(source.clone(), newline.clone());
    let mut n_lines: i32 = (lines.len() as i32);
    let mut indent_stack: Vec<String> = Vec::new();
    let mut zero_str: String = "0".to_string();
    indent_stack.push(zero_str.clone());
    let mut cur_line: i32 = 0;
    let mut skip: i32 = 0;
    let mut empty_val: String = "".to_string();
    let mut kind_indent: String = "INDENT".to_string();
    let mut kind_dedent: String = "DEDENT".to_string();
    let mut kind_newline: String = "NEWLINE".to_string();
    let mut kind_kw_rust: String = "KW_RUST".to_string();
    let mut kw_rust_val: String = "rust".to_string();
    let mut kind_rust_block: String = "RUST_BLOCK".to_string();
    let mut kind_string: String = "STRING".to_string();
    let mut kind_int: String = "INT".to_string();
    let mut kind_eof: String = "EOF".to_string();
    let mut kind_gte: String = "GTE".to_string();
    let mut kind_lte: String = "LTE".to_string();
    let mut kind_plus: String = "PLUS".to_string();
    let mut kind_minus: String = "MINUS".to_string();
    let mut kind_star: String = "STAR".to_string();
    let mut kind_slash: String = "SLASH".to_string();
    let mut kind_pct: String = "PERCENT".to_string();
    let mut kind_eq: String = "EQUALS".to_string();
    let mut kind_gt: String = "GT".to_string();
    let mut kind_lt: String = "LT".to_string();
    let mut kind_lp: String = "LPAREN".to_string();
    let mut kind_rp: String = "RPAREN".to_string();
    let mut kind_lb: String = "LBRACKET".to_string();
    let mut kind_rb: String = "RBRACKET".to_string();
    let mut kind_cm: String = "COMMA".to_string();
    let mut val_gte: String = ">=".to_string();
    let mut val_lte: String = "<=".to_string();
    let mut val_plus: String = "+".to_string();
    let mut val_minus: String = "-".to_string();
    let mut val_star: String = "*".to_string();
    let mut val_slash: String = "/".to_string();
    let mut val_pct: String = "%".to_string();
    let mut val_eq: String = "=".to_string();
    let mut val_gt: String = ">".to_string();
    let mut val_lt: String = "<".to_string();
    let mut val_lp: String = "(".to_string();
    let mut val_rp: String = ")".to_string();
    let mut val_lb: String = "[".to_string();
    let mut val_rb: String = "]".to_string();
    let mut val_cm: String = ",".to_string();
    let mut ch_nl: String = "\n".to_string();
    let mut ch_tab: String = "\t".to_string();
    let mut ch_bs: String = "\\".to_string();
    let mut ch_qt: String = "\"".to_string();
    for raw_li in 0..n_lines {
        cur_line = cur_line + 1;
        if skip > 0 {
            skip = skip - 1;
            continue;
        }
        let mut raw_line: String = lines[raw_li as usize].clone();
        let mut line: String = s_rtrim(raw_line.clone());
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
            tokens.push(make_token(kind_indent.clone(), empty_val.clone(), cur_line.clone()).clone());
            let mut indent_str: String = n_to_str(indent.clone());
            indent_stack.push(indent_str.clone());
        } else {
            let mut dedenting: bool = indent < top.clone();
            while dedenting {
                let mut new_slen: i32 = (indent_stack.len() as i32);
                let mut new_top_idx: i32 = new_slen - 1.clone();
                let mut cur_top: i32 = n_parse(indent_stack[new_top_idx as usize].clone());
                if indent < cur_top {
                    tokens.push(make_token(kind_dedent.clone(), empty_val.clone(), cur_line.clone()).clone());
                    indent_stack.remove(new_top_idx as usize);
                } else {
                    dedenting = false;
                }
            }
        }
        if content == "rust" {
            tokens.push(make_token(kind_kw_rust.clone(), kw_rust_val.clone(), cur_line.clone()).clone());
            tokens.push(make_token(kind_newline.clone(), empty_val.clone(), cur_line.clone()).clone());
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
            let mut block_content: String = s_join_nl(rust_lines.clone());
            tokens.push(make_token(kind_rust_block.clone(), block_content.clone(), cur_line.clone()).clone());
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
                            val = s_cat(val.clone(), ch_nl.clone());
                        } else if string_char == "t" {
                            val = s_cat(val.clone(), ch_tab.clone());
                        } else if string_char == "\\" {
                            val = s_cat(val.clone(), ch_bs.clone());
                        } else if string_char == "\"" {
                            val = s_cat(val.clone(), ch_qt.clone());
                        } else {
                            val = s_cat(val.clone(), ch_bs.clone());
                            val = s_cat(val.clone(), string_char.clone());
                        }
                        escape_next = false;
                        char_index = string_index + 1;
                    } else if string_char == ch_bs {
                        escape_next = true;
                        char_index = string_index + 1;
                    } else if string_char == ch_qt {
                        char_index = string_index + 1;
                        break;
                    } else {
                        val = s_cat(val.clone(), string_char.clone());
                        char_index = string_index + 1;
                    }
                }
                tokens.push(make_token(kind_string.clone(), val.clone(), cur_line.clone()).clone());
                continue;
            }
            if c_digit(character.clone()) {
                let mut num: String = s_cat(empty_val.clone(), character.clone());
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
                tokens.push(make_token(kind_int.clone(), num.clone(), cur_line.clone()).clone());
                continue;
            }
            if c_alpha(character.clone()) {
                let mut word: String = s_cat(empty_val.clone(), character.clone());
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
                let mut word_kind: String = word_to_kind(word.clone());
                tokens.push(make_token(word_kind.clone(), word.clone(), cur_line.clone()).clone());
                continue;
            }
            let mut peek_idx: i32 = char_index + 1.clone();
            let mut peek: String = "".to_string();
            if peek_idx < char_count {
                peek = chars[peek_idx as usize].clone();
            }
            if character == ">" && peek == "=" {
                tokens.push(make_token(kind_gte.clone(), val_gte.clone(), cur_line.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "<" && peek == "=" {
                tokens.push(make_token(kind_lte.clone(), val_lte.clone(), cur_line.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "+" {
                tokens.push(make_token(kind_plus.clone(), val_plus.clone(), cur_line.clone()).clone());
            } else if character == "-" {
                tokens.push(make_token(kind_minus.clone(), val_minus.clone(), cur_line.clone()).clone());
            } else if character == "*" {
                tokens.push(make_token(kind_star.clone(), val_star.clone(), cur_line.clone()).clone());
            } else if character == "/" {
                tokens.push(make_token(kind_slash.clone(), val_slash.clone(), cur_line.clone()).clone());
            } else if character == "%" {
                tokens.push(make_token(kind_pct.clone(), val_pct.clone(), cur_line.clone()).clone());
            } else if character == "=" {
                tokens.push(make_token(kind_eq.clone(), val_eq.clone(), cur_line.clone()).clone());
            } else if character == ">" {
                tokens.push(make_token(kind_gt.clone(), val_gt.clone(), cur_line.clone()).clone());
            } else if character == "<" {
                tokens.push(make_token(kind_lt.clone(), val_lt.clone(), cur_line.clone()).clone());
            } else if character == "(" {
                tokens.push(make_token(kind_lp.clone(), val_lp.clone(), cur_line.clone()).clone());
            } else if character == ")" {
                tokens.push(make_token(kind_rp.clone(), val_rp.clone(), cur_line.clone()).clone());
            } else if character == "[" {
                tokens.push(make_token(kind_lb.clone(), val_lb.clone(), cur_line.clone()).clone());
            } else if character == "]" {
                tokens.push(make_token(kind_rb.clone(), val_rb.clone(), cur_line.clone()).clone());
            } else if character == "," {
                tokens.push(make_token(kind_cm.clone(), val_cm.clone(), cur_line.clone()).clone());
            }
            char_index = char_index + 1;
        }
        tokens.push(make_token(kind_newline.clone(), empty_val.clone(), cur_line.clone()).clone());
    }
    let mut final_stack_len: i32 = (indent_stack.len() as i32);
    for _ in 1..final_stack_len {
        tokens.push(make_token(kind_dedent.clone(), empty_val.clone(), cur_line.clone()).clone());
    }
    tokens.push(make_token(kind_eof.clone(), empty_val.clone(), cur_line.clone()).clone());
    return tokens;
}

thread_local! {
	static INCLUDED_FILES: std::cell::RefCell<std::collections::HashSet<String>> = std::cell::RefCell::new(std::collections::HashSet::new());
}
fn file_is_new(path: String) -> bool {
	INCLUDED_FILES.with(|set| {
		let mut s = set.borrow_mut();
		if s.contains(&path) { false } else { s.insert(path); true }
	})
}
fn scan_import(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut scan: i32 = pos + 1.clone();
    while scan < token_count {
        let mut scan_tok: Token = tokens[scan as usize].clone();
        let kind = scan_tok.kind.clone();
        scan = scan + 1;
        if kind == "RPAREN" {
            break;
        }
    }
    let mut path_pos: i32 = scan + 1.clone();
    if path_pos < token_count {
        let mut in_tok: Token = tokens[scan as usize].clone();
        let kind = in_tok.kind.clone();
        let mut is_in: bool = kind == "KW_IN".clone();
        if is_in {
            let mut path_tok: Token = tokens[path_pos as usize].clone();
            let value = path_tok.value.clone();
            let mut after_path: i32 = path_pos + 1.clone();
            return make_result(value.clone(), after_path.clone());
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), pos.clone());
}

fn name_of_decl(tokens: Vec<Token>, pos: i32, is_fn: bool) -> String {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut name_offset: i32 = 1;
    if is_fn {
        name_offset = 2;
    }
    let mut name_pos: i32 = pos + name_offset.clone();
    if name_pos < token_count {
        let mut name_tok: Token = tokens[name_pos as usize].clone();
        let value = name_tok.value.clone();
        return value;
    }
    return "".to_string();
}

fn end_of_block(tokens: Vec<Token>, pos: i32) -> i32 {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = pos.clone();
    let mut depth: i32 = 0;
    let mut entered: bool = false;
    while cur < token_count {
        let mut tok: Token = tokens[cur as usize].clone();
        let kind = tok.kind.clone();
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
    return cur;
}

fn end_of_shape(tokens: Vec<Token>, pos: i32) -> i32 {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = pos.clone();
    while cur < token_count {
        let mut tok: Token = tokens[cur as usize].clone();
        let kind = tok.kind.clone();
        cur = cur + 1;
        if kind == "NEWLINE" {
            break;
        }
    }
    return cur;
}

fn load_file(path: String) -> Vec<Token> {
    let mut source: String = f_read(path.clone());
    let mut tok_raw: Vec<Token> = tokenize(source.clone());
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = (tok_raw.len() as i32);
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    while true {
        let is_at_end_of_file = pos >= token_count;
        if is_at_end_of_file {
            break;
        }
        let mut tok: Token = tok_raw[pos as usize].clone();
        let kind = tok.kind.clone();
        if kind == "EOF" {
            break;
        }
        if kind == "INDENT" {
            depth = depth + 1;
            result.push(tok.clone());
            pos = pos + 1;
            continue;
        }
        if kind == "DEDENT" {
            depth = depth - 1;
            result.push(tok.clone());
            pos = pos + 1;
            continue;
        }
        let mut at_root_depth: bool = depth == 0.clone();
        if kind == "LPAREN" && at_root_depth {
            let mut imp_r: ParseResult = scan_import(tok_raw.clone(), pos.clone());
            let mut imp_path: String = pr_code(imp_r.clone());
            let mut imp_end: i32 = pr_pos(imp_r.clone());
            if !is_empty(imp_path.clone()) {
                let mut is_new: bool = file_is_new(imp_path.clone());
                if is_new {
                    let mut imp_tokens: Vec<Token> = load_file(imp_path.clone());
                    let mut imp_len: i32 = (imp_tokens.len() as i32);
                    for imp_index in 0..imp_len {
                        let mut imp_tok: Token = imp_tokens[imp_index as usize].clone();
                        let kind = imp_tok.kind.clone();
                        let mut imp_is_eof: bool = kind == "EOF".clone();
                        if !imp_is_eof {
                            result.push(imp_tok.clone());
                        }
                    }
                }
                pos = imp_end;
                continue;
            }
        }
        result.push(tok.clone());
        pos = pos + 1;
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
        let mut is_raw: bool = kind == "KW_RAW".clone();
        let mut is_rust_blk: bool = kind == "KW_RUST".clone();
        let mut is_block_decl: bool = is_fn || is_struct || is_enum.clone();
        if is_block_decl {
            let mut decl_name: String = name_of_decl(tokens.clone(), pos.clone(), is_fn.clone());
            let mut already_seen: bool = list_has(seen.clone(), decl_name.clone());
            if !already_seen {
                seen.push(decl_name.clone());
            }
            let mut block_end: i32 = end_of_block(tokens.clone(), pos.clone());
            if !already_seen {
                let mut copy_len: i32 = block_end - pos.clone();
                for copy_idx in 0..copy_len {
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = block_end;
        } else if is_shape {
            let mut not_fn: bool = false;
            let mut decl_name: String = name_of_decl(tokens.clone(), pos.clone(), not_fn.clone());
            let mut already_seen: bool = list_has(seen.clone(), decl_name.clone());
            if !already_seen {
                seen.push(decl_name.clone());
            }
            let mut shape_end: i32 = end_of_shape(tokens.clone(), pos.clone());
            if !already_seen {
                let mut copy_len: i32 = shape_end - pos.clone();
                for copy_idx in 0..copy_len {
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = shape_end;
        } else if is_raw {
            let mut not_fn2: bool = false;
            let mut raw_name: String = name_of_decl(tokens.clone(), pos.clone(), not_fn2.clone());
            let mut raw_pfx: String = "_raw_".to_string();
            let mut raw_key_parts: Vec<String> = vec![raw_pfx.clone(), raw_name.clone()];
            let mut raw_key: String = s_join(raw_key_parts.clone());
            let mut already_seen: bool = list_has(seen.clone(), raw_key.clone());
            if !already_seen {
                seen.push(raw_key.clone());
            }
            let mut raw_end: i32 = end_of_shape(tokens.clone(), pos.clone());
            if !already_seen {
                let mut copy_len: i32 = raw_end - pos.clone();
                for copy_idx in 0..copy_len {
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = raw_end;
        } else if is_rust_blk {
            let mut rust_content_pos: i32 = pos + 2.clone();
            let mut rust_in_range: bool = rust_content_pos < token_count.clone();
            if rust_in_range {
                let mut rust_tok: Token = tokens[rust_content_pos as usize].clone();
                let value = rust_tok.value.clone();
                let mut newline: String = "\n".to_string();
                let mut rust_lines: Vec<String> = s_split(value.clone(), newline.clone());
                let mut rust_first: String = rust_lines[0 as usize].clone();
                let mut rk_pfx: String = "_rust_".to_string();
                let mut rk_parts: Vec<String> = vec![rk_pfx.clone(), rust_first.clone()];
                let mut rust_key: String = s_join(rk_parts.clone());
                let mut already_seen: bool = list_has(seen.clone(), rust_key.clone());
                if !already_seen {
                    seen.push(rust_key.clone());
                    for copy_idx in 0..3 {
                        let mut tok_pos: i32 = pos + copy_idx.clone();
                        let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                        result.push(copy_tok.clone());
                    }
                }
            }
            pos = pos + 3;
        } else {
            result.push(token.clone());
            pos = pos + 1;
        }
    }
    return result;
}

fn collect_all_tokens_with_all_imports(path: String) -> Vec<Token> {
    let mut merged: Vec<Token> = load_file(path.clone());
    return deduplicate_decls(merged.clone());
}

fn generate_rust_from_tokens(tokens: Vec<Token>) -> String {
    let mut t_reg_start: i32 = now_ms();
    let mut struct_reg: Vec<String> = build_struct_reg(tokens.clone());
    let mut shape_reg: Vec<String> = build_shape_reg(tokens.clone());
    let mut enum_reg: Vec<String> = build_enum_reg(tokens.clone());
    let mut variant_reg: Vec<String> = build_variant_reg(tokens.clone(), enum_reg.clone());
    let mut type_reg: Vec<String> = build_type_reg(tokens.clone());
    let mut mut_names: Vec<String> = Vec::new();
    let mut var_type_reg: Vec<String> = build_var_type_reg(tokens.clone());
    let mut t_reg: i32 = elapsed_ms(t_reg_start.clone());
    let mut trg_str: String = n_to_str(t_reg.clone());
    let mut trg_pfx: String = "[timer]   registries: ".to_string();
    let mut trg_sfx: String = "ms".to_string();
    let mut trg_parts: Vec<String> = vec![trg_pfx.clone(), trg_str.clone(), trg_sfx.clone()];
    println!("{}", s_join(trg_parts.clone()));
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
/* unhandled(IDENT) */
    let ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone() };
    let mut ctx: RcCtx = make_rctx(ctx_raw);
    let mut output: String = "".to_string();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pos: i32 = 0;
    let mut t_loop_start: i32 = now_ms();
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
            let mut decl_code: String = pr_code(result.clone());
            output = s_cat(output.clone(), decl_code.clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_SHAPE" {
            let mut result: ParseResult = gen_shape_decl(tokens.clone(), pos.clone());
            let mut decl_code: String = pr_code(result.clone());
            output = s_cat(output.clone(), decl_code.clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_ENUM" {
            let mut result: ParseResult = gen_enum_decl(tokens.clone(), pos.clone());
            let mut decl_code: String = pr_code(result.clone());
            output = s_cat(output.clone(), decl_code.clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_TYPE" {
            let mut result: ParseResult = gen_type_decl(tokens.clone(), pos.clone(), ctx.clone());
            let mut decl_code: String = pr_code(result.clone());
            output = s_cat(output.clone(), decl_code.clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_FN" {
            let mut result: ParseResult = gen_fn_decl(tokens.clone(), pos.clone(), ctx.clone());
            let mut decl_code: String = pr_code(result.clone());
            output = s_cat(output.clone(), decl_code.clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_RAW" {
            let mut result: ParseResult = gen_raw_decl(tokens.clone(), pos.clone());
            let mut decl_code: String = pr_code(result.clone());
            output = s_cat(output.clone(), decl_code.clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_RUST" {
            let mut block_pos: i32 = pos + 2.clone();
            let mut block_token: Token = tokens[block_pos as usize].clone();
            let value = block_token.value.clone();
            output = s_cat(output.clone(), value.clone());
            let mut newline: String = "\n".to_string();
            output = s_cat(output.clone(), newline.clone());
            pos = block_pos + 1;
            continue;
        }
        pos = pos + 1;
    }
    let mut t_loop: i32 = elapsed_ms(t_loop_start.clone());
    let mut tlp_str: String = n_to_str(t_loop.clone());
    let mut tlp_pfx: String = "[timer]   codegen-loop: ".to_string();
    let mut tlp_sfx: String = "ms".to_string();
    let mut tlp_parts: Vec<String> = vec![tlp_pfx.clone(), tlp_str.clone(), tlp_sfx.clone()];
    println!("{}", s_join(tlp_parts.clone()));
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
        let mut t_start: i32 = now_ms();
        let mut tokens: Vec<Token> = collect_all_tokens_with_all_imports(input_path.clone());
        let mut t_load: i32 = elapsed_ms(t_start.clone());
        let mut tld_str: String = n_to_str(t_load.clone());
        let mut tld_pfx: String = "[timer] load+dedup: ".to_string();
        let mut tld_sfx: String = "ms".to_string();
        let mut tld_parts: Vec<String> = vec![tld_pfx.clone(), tld_str.clone(), tld_sfx.clone()];
        println!("{}", s_join(tld_parts.clone()));
        validate_tokens(tokens.clone());
        let mut t_gen_start: i32 = now_ms();
        let mut rust_code: String = generate_rust_from_tokens(tokens.clone());
        let mut t_gen: i32 = elapsed_ms(t_gen_start.clone());
        let mut tgn_str: String = n_to_str(t_gen.clone());
        let mut tgn_pfx: String = "[timer] total-codegen: ".to_string();
        let mut tgn_sfx: String = "ms".to_string();
        let mut tgn_parts: Vec<String> = vec![tgn_pfx.clone(), tgn_str.clone(), tgn_sfx.clone()];
        println!("{}", s_join(tgn_parts.clone()));
        f_write(output_path.clone(), rust_code.clone());
    }
}

