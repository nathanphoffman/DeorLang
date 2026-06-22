type TokenList = Vec<Token>;

type StrList = Vec<String>;

#[derive(Clone, PartialEq, Debug)]
struct Token {
    kind: String,
    value: String,
    line: i32,
    file: String,
}

#[derive(Clone, PartialEq, Debug)]
struct ParseResult {
    code: String,
    new_pos: i32,
}

#[derive(Clone, PartialEq, Debug)]
struct TokenMeta {
    line: i32,
    file: String,
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
    tokens: TokensRef,
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

fn s_upper_char(ch: String) -> bool {
    ch.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn s_lower_char(ch: String) -> bool {
    ch.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
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

fn n_parse(source: String) -> i32 {
    source.parse::<i32>().unwrap_or(0)
}

fn n_to_str(number: i32) -> String {
    number.to_string()
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

fn l_slice(tokens: Vec<Token>, start: i32, end_val: i32) -> Vec<Token> {
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
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

fn pr_code(result: ParseResult) -> String {
    result.code
}

fn pr_pos(result: ParseResult) -> i32 {
    result.new_pos
}

fn make_result(code: String, new_pos: i32) -> ParseResult {
    let result = ParseResult { code: code.clone(), new_pos: new_pos.clone() };
    return result;
}

fn make_meta(line: i32, file: String) -> TokenMeta {
    let meta = TokenMeta { line: line.clone(), file: file.clone() };
    return meta;
}

fn make_token(kind: String, value: String, meta: TokenMeta) -> Token {
    let line = meta.line.clone();
    let file = meta.file.clone();
    let token = Token { kind: kind.clone(), value: value.clone(), line: line.clone(), file: file.clone() };
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
    if word == "bad" {
        return "KW_BAD".to_string();
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
    if word == "move" {
        return "KW_GIVEUP".to_string();
    }
    if word == "raw" {
        return "KW_RAW".to_string();
    }
    if word == "macro" {
        return "KW_MACRO".to_string();
    }
    if word == "macro_run" {
        return "KW_MACRO_RUN".to_string();
    }
    if word == "import" {
        return "KW_IMPORT".to_string();
    }
    if word == "block" {
        return "KW_BLOCK".to_string();
    }
    if word == "const" {
        return "KW_CONST".to_string();
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

fn count_tabs(line: String) -> i32 {
    let mut space: String = " ".to_string();
    let mut chars: Vec<String> = c_chars(line.clone());
    let mut char_count: i32 = (chars.len() as i32);
    let mut count: i32 = 0;
    let mut space_run = 0;
    for index in 0..char_count {
        let mut character: String = chars[index as usize].clone();
        if character == "\t" {
            count = count + 1;
        } else if character == space {
            space_run = space_run + 1;
            if space_run == 4 {
                count = count + 1;
                space_run = 0;
            }
        } else {
            break;
        }
    }
    return count;
}

fn scan_string_literal(chars: Vec<String>, char_index: i32, char_count: i32) -> ParseResult {
    let mut val: String = "".to_string();
    let mut escape_next: bool = false;
    let mut str_start: i32 = char_index + 1.clone();
    let mut new_pos: i32 = char_index + 1.clone();
    let mut ch_nl: String = "\n".to_string();
    let mut ch_tab: String = "\t".to_string();
    let mut ch_bs: String = "\\".to_string();
    let mut ch_qt: String = "\"".to_string();
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
            new_pos = string_index + 1;
        } else if string_char == ch_bs {
            escape_next = true;
            new_pos = string_index + 1;
        } else if string_char == ch_qt {
            new_pos = string_index + 1;
            break;
        } else {
            val = s_cat(val.clone(), string_char.clone());
            new_pos = string_index + 1;
        }
    }
    return make_result(val.clone(), new_pos.clone());
}

fn scan_number(chars: Vec<String>, char_index: i32, char_count: i32) -> ParseResult {
    let mut first_char: String = chars[char_index as usize].clone();
    let mut empty_str: String = "".to_string();
    let mut num: String = s_cat(empty_str.clone(), first_char.clone());
    let mut num_start: i32 = char_index + 1.clone();
    let mut new_pos: i32 = char_index + 1.clone();
    for number_index in num_start..char_count {
        let mut number_char: String = chars[number_index as usize].clone();
        if c_digit(number_char.clone()) {
            num = s_cat(num.clone(), number_char.clone());
            new_pos = number_index + 1;
        } else if number_char == "_" {
            let mut peek_idx: i32 = number_index + 1.clone();
            if peek_idx < char_count {
                let mut peek_char: String = chars[peek_idx as usize].clone();
                if c_digit(peek_char.clone()) {
                    new_pos = number_index + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if new_pos < char_count {
        let mut dot_char: String = chars[new_pos as usize].clone();
        let mut frac_start: i32 = new_pos + 1.clone();
        if dot_char == "." && frac_start < char_count {
            let mut frac_first: String = chars[frac_start as usize].clone();
            if c_digit(frac_first.clone()) {
                let mut dot_str: String = ".".to_string();
                num = s_cat(num.clone(), dot_str.clone());
                new_pos = frac_start;
                for frac_index in frac_start..char_count {
                    let mut frac_char: String = chars[frac_index as usize].clone();
                    if c_digit(frac_char.clone()) {
                        num = s_cat(num.clone(), frac_char.clone());
                        new_pos = frac_index + 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return make_result(num.clone(), new_pos.clone());
}

fn scan_word(chars: Vec<String>, char_index: i32, char_count: i32) -> ParseResult {
    let mut first_char: String = chars[char_index as usize].clone();
    let mut empty_str: String = "".to_string();
    let mut word: String = s_cat(empty_str.clone(), first_char.clone());
    let mut word_start: i32 = char_index + 1.clone();
    let mut new_pos: i32 = char_index + 1.clone();
    for word_index in word_start..char_count {
        let mut word_char: String = chars[word_index as usize].clone();
        if c_alnum(word_char.clone()) {
            word = s_cat(word.clone(), word_char.clone());
            new_pos = word_index + 1;
        } else {
            break;
        }
    }
    return make_result(word.clone(), new_pos.clone());
}

fn tokenize(source: String, path: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut empty_str: String = "".to_string();
    let mut kind_newline: String = "NEWLINE".to_string();
    let mut kind_dedent: String = "DEDENT".to_string();
    let mut kind_eof: String = "EOF".to_string();
    let mut newline: String = "\n".to_string();
    let mut lines: Vec<String> = s_split(source.clone(), newline.clone());
    let mut n_lines: i32 = (lines.len() as i32);
    let mut indent_stack: Vec<String> = Vec::new();
    let mut zero_str: String = "0".to_string();
    indent_stack.push(zero_str.clone());
    let mut cur_line: i32 = 0;
    let mut skip: i32 = 0;
    for raw_li in 0..n_lines {
        cur_line = cur_line + 1;
        let mut meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
        if skip > 0 {
            skip = skip - 1;
            continue;
        }
        let mut raw_line: String = lines[raw_li as usize].clone();
        let mut line: String = s_rtrim(raw_line.clone());
        let mut content: String = s_trim(line.clone());
        if is_empty(content.clone()) {
            continue;
        }
        let mut indent: i32 = count_tabs(line.clone());
        let mut iod_kind_indent: String = "INDENT".to_string();
        let mut iod_kind_dedent: String = "DEDENT".to_string();
        let mut iod_empty: String = "".to_string();
        let mut slen: i32 = (indent_stack.len() as i32);
        let mut top_idx: i32 = slen - 1.clone();
        let mut top: i32 = n_parse(indent_stack[top_idx as usize].clone());
        if indent > top {
            tokens.push(make_token(iod_kind_indent.clone(), iod_empty.clone(), meta.clone()).clone());
            let mut indent_str: String = n_to_str(indent.clone());
            indent_stack.push(indent_str.clone());
        } else {
            let mut dedenting: bool = indent < top.clone();
            while dedenting {
                let mut new_slen: i32 = (indent_stack.len() as i32);
                let mut new_top_idx: i32 = new_slen - 1.clone();
                let mut cur_top: i32 = n_parse(indent_stack[new_top_idx as usize].clone());
                if indent < cur_top {
                    tokens.push(make_token(iod_kind_dedent.clone(), iod_empty.clone(), meta.clone()).clone());
                    indent_stack.remove(new_top_idx as usize);
                } else {
                    dedenting = false;
                }
            }
        }
        if content == "rust" {
            let mut rb_kind_kw_rust: String = "KW_RUST".to_string();
            let mut rb_kw_rust_val: String = "rust".to_string();
            let mut rb_kind_newline: String = "NEWLINE".to_string();
            let mut rb_kind_rust_block: String = "RUST_BLOCK".to_string();
            let mut rb_empty: String = "".to_string();
            tokens.push(make_token(rb_kind_kw_rust.clone(), rb_kw_rust_val.clone(), meta.clone()).clone());
            tokens.push(make_token(rb_kind_newline.clone(), rb_empty.clone(), meta.clone()).clone());
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
            tokens.push(make_token(rb_kind_rust_block.clone(), block_content.clone(), meta.clone()).clone());
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
                let mut str_r: ParseResult = scan_string_literal(chars.clone(), char_index.clone(), char_count.clone());
                let mut kind_string: String = "STRING".to_string();
                let mut str_val: String = pr_code(str_r.clone());
                tokens.push(make_token(kind_string.clone(), str_val.clone(), meta.clone()).clone());
                char_index = pr_pos(str_r.clone());
                continue;
            }
            if c_digit(character.clone()) {
                let mut num_r: ParseResult = scan_number(chars.clone(), char_index.clone(), char_count.clone());
                let mut num_str: String = pr_code(num_r.clone());
                char_index = pr_pos(num_r.clone());
                let mut dot: String = ".".to_string();
                let mut num_parts: Vec<String> = s_split(num_str.clone(), dot.clone());
                let mut is_float: bool = (num_parts.len() as i32) > 1;
                if is_float {
                    let mut kind_float: String = "FLOAT".to_string();
                    tokens.push(make_token(kind_float.clone(), num_str.clone(), meta.clone()).clone());
                } else {
                    let mut kind_int: String = "INT".to_string();
                    tokens.push(make_token(kind_int.clone(), num_str.clone(), meta.clone()).clone());
                }
                continue;
            }
            if c_alpha(character.clone()) {
                let mut word_r: ParseResult = scan_word(chars.clone(), char_index.clone(), char_count.clone());
                let mut word: String = pr_code(word_r.clone());
                char_index = pr_pos(word_r.clone());
                let mut word_kind: String = word_to_kind(word.clone());
                tokens.push(make_token(word_kind.clone(), word.clone(), meta.clone()).clone());
                continue;
            }
            let mut op_kind_gte: String = "GTE".to_string();
            let mut op_val_gte: String = ">=".to_string();
            let mut op_kind_lte: String = "LTE".to_string();
            let mut op_val_lte: String = "<=".to_string();
            let mut op_kind_plus: String = "PLUS".to_string();
            let mut op_val_plus: String = "+".to_string();
            let mut op_kind_minus: String = "MINUS".to_string();
            let mut op_val_minus: String = "-".to_string();
            let mut op_kind_star: String = "STAR".to_string();
            let mut op_val_star: String = "*".to_string();
            let mut op_kind_slash: String = "SLASH".to_string();
            let mut op_val_slash: String = "/".to_string();
            let mut op_kind_pct: String = "PERCENT".to_string();
            let mut op_val_pct: String = "%".to_string();
            let mut op_kind_eq: String = "EQUALS".to_string();
            let mut op_val_eq: String = "=".to_string();
            let mut op_kind_gt: String = "GT".to_string();
            let mut op_val_gt: String = ">".to_string();
            let mut op_kind_lt: String = "LT".to_string();
            let mut op_val_lt: String = "<".to_string();
            let mut op_kind_lp: String = "LPAREN".to_string();
            let mut op_val_lp: String = "(".to_string();
            let mut op_kind_rp: String = "RPAREN".to_string();
            let mut op_val_rp: String = ")".to_string();
            let mut op_kind_lb: String = "LBRACKET".to_string();
            let mut op_val_lb: String = "[".to_string();
            let mut op_kind_rb: String = "RBRACKET".to_string();
            let mut op_val_rb: String = "]".to_string();
            let mut op_kind_cm: String = "COMMA".to_string();
            let mut op_val_cm: String = ",".to_string();
            let mut op_peek_idx: i32 = char_index + 1.clone();
            let mut op_peek: String = "".to_string();
            if op_peek_idx < char_count {
                op_peek = chars[op_peek_idx as usize].clone();
            }
            if character == ">" && op_peek == "=" {
                tokens.push(make_token(op_kind_gte.clone(), op_val_gte.clone(), meta.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "<" && op_peek == "=" {
                tokens.push(make_token(op_kind_lte.clone(), op_val_lte.clone(), meta.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "+" {
                tokens.push(make_token(op_kind_plus.clone(), op_val_plus.clone(), meta.clone()).clone());
            } else if character == "-" {
                tokens.push(make_token(op_kind_minus.clone(), op_val_minus.clone(), meta.clone()).clone());
            } else if character == "*" {
                tokens.push(make_token(op_kind_star.clone(), op_val_star.clone(), meta.clone()).clone());
            } else if character == "/" {
                tokens.push(make_token(op_kind_slash.clone(), op_val_slash.clone(), meta.clone()).clone());
            } else if character == "%" {
                tokens.push(make_token(op_kind_pct.clone(), op_val_pct.clone(), meta.clone()).clone());
            } else if character == "=" {
                tokens.push(make_token(op_kind_eq.clone(), op_val_eq.clone(), meta.clone()).clone());
            } else if character == ">" {
                tokens.push(make_token(op_kind_gt.clone(), op_val_gt.clone(), meta.clone()).clone());
            } else if character == "<" {
                tokens.push(make_token(op_kind_lt.clone(), op_val_lt.clone(), meta.clone()).clone());
            } else if character == "(" {
                tokens.push(make_token(op_kind_lp.clone(), op_val_lp.clone(), meta.clone()).clone());
            } else if character == ")" {
                tokens.push(make_token(op_kind_rp.clone(), op_val_rp.clone(), meta.clone()).clone());
            } else if character == "[" {
                tokens.push(make_token(op_kind_lb.clone(), op_val_lb.clone(), meta.clone()).clone());
            } else if character == "]" {
                tokens.push(make_token(op_kind_rb.clone(), op_val_rb.clone(), meta.clone()).clone());
            } else if character == "," {
                tokens.push(make_token(op_kind_cm.clone(), op_val_cm.clone(), meta.clone()).clone());
            }
            char_index = char_index + 1;
        }
        tokens.push(make_token(kind_newline.clone(), empty_str.clone(), meta.clone()).clone());
    }
    let mut final_stack_len: i32 = (indent_stack.len() as i32);
    let mut tail_meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
    for _ in 1..final_stack_len {
        tokens.push(make_token(kind_dedent.clone(), empty_str.clone(), tail_meta.clone()).clone());
    }
    tokens.push(make_token(kind_eof.clone(), empty_str.clone(), tail_meta.clone()).clone());
    return tokens;
}

fn scan_import_new(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut path_pos: i32 = pos + 1.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    if path_pos < token_count {
        let mut path_tok: Token = tokens[path_pos as usize].clone();
        let kind = path_tok.kind.clone();
        let value = path_tok.value.clone();
        if kind == "STRING" {
            let mut after_path: i32 = path_pos + 1.clone();
            return make_result(value.clone(), after_path.clone());
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), pos.clone());
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

thread_local! {
	static INCLUDED_FILES: std::cell::RefCell<std::collections::HashSet<String>> = std::cell::RefCell::new(std::collections::HashSet::new());
}
fn file_is_new(path: String) -> bool {
	INCLUDED_FILES.with(|set| {
		let mut s = set.borrow_mut();
		if s.contains(&path) { false } else { s.insert(path); true }
	})
}
fn load_file(path: String) -> Vec<Token> {
    let mut source: String = f_read(path.clone());
    let mut tok_raw: Vec<Token> = tokenize(source.clone(), path.clone());
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = (tok_raw.len() as i32);
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut seen_decl: bool = false;
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
        let mut is_new_import: bool = kind == "KW_IMPORT" && at_root_depth.clone();
        let mut is_old_import: bool = kind == "LPAREN" && at_root_depth.clone();
        let mut is_any_import: bool = is_new_import || is_old_import.clone();
        if is_any_import {
            let mut imp_r_new: ParseResult = scan_import_new(tok_raw.clone(), pos.clone());
            let mut imp_r_old: ParseResult = scan_import(tok_raw.clone(), pos.clone());
            let mut imp_path: String = "".to_string();
            let mut imp_end: i32 = pos.clone();
            if is_new_import {
                imp_path = pr_code(imp_r_new.clone());
                imp_end = pr_pos(imp_r_new.clone());
            } else {
                imp_path = pr_code(imp_r_old.clone());
                imp_end = pr_pos(imp_r_old.clone());
            }
            if !is_empty(imp_path.clone()) {
                if seen_decl {
                    let mut err_pre: String = "[error] ".to_string();
                    let mut err_mid: String = ": imports must appear at the top of the file before any declarations".to_string();
                    let mut err_parts: Vec<String> = vec![err_pre.clone(), path.clone(), err_mid.clone()];
                    let mut err_msg: String = s_join(err_parts.clone());
                    println!("{}", err_msg.clone());
                    std::process::exit(1);
                }
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
        if at_root_depth && kind != "NEWLINE" {
            seen_decl = true;
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
        let mut is_type: bool = kind == "KW_TYPE".clone();
        let mut is_macro: bool = kind == "KW_MACRO".clone();
        let mut is_raw: bool = kind == "KW_RAW".clone();
        let mut is_rust_blk: bool = kind == "KW_RUST".clone();
        let mut is_block_decl: bool = is_fn || is_struct || is_enum || is_type || is_macro.clone();
        if is_block_decl {
            let mut dn_offset: i32 = 1;
            if is_fn {
                dn_offset = 2;
            }
            let mut dn_pos: i32 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            let mut already_seen: bool = false;
            let mut cs_len: i32 = (seen.len() as i32);
            for cs_i in 0..cs_len {
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    already_seen = true;
                    break;
                }
            }
            if !already_seen {
                seen.push(decl_name.clone());
            }
            let mut fbe_cur: i32 = pos.clone();
            let mut fbe_depth: i32 = 0;
            let mut fbe_entered: bool = false;
            while fbe_cur < token_count {
                let mut fbe_tok: Token = tokens[fbe_cur as usize].clone();
                let kind = fbe_tok.kind.clone();
                fbe_cur = fbe_cur + 1;
                if kind == "INDENT" {
                    fbe_depth = fbe_depth + 1;
                    fbe_entered = true;
                } else if kind == "DEDENT" {
                    fbe_depth = fbe_depth - 1;
                    if fbe_depth == 0 && fbe_entered {
                        break;
                    }
                }
            }
            let mut end_pos: i32 = fbe_cur.clone();
            if !already_seen {
                for i in (pos as usize)..(end_pos as usize) {
                	result.push(tokens[i].clone());
                }
            }
            pos = end_pos;
        } else if is_shape {
            let mut dn_offset: i32 = 1;
            if is_fn {
                dn_offset = 2;
            }
            let mut dn_pos: i32 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            let mut already_seen: bool = false;
            let mut cs_len: i32 = (seen.len() as i32);
            for cs_i in 0..cs_len {
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    already_seen = true;
                    break;
                }
            }
            if !already_seen {
                seen.push(decl_name.clone());
            }
            let mut fse_cur: i32 = pos.clone();
            while fse_cur < token_count {
                let mut fse_tok: Token = tokens[fse_cur as usize].clone();
                let kind = fse_tok.kind.clone();
                fse_cur = fse_cur + 1;
                if kind == "NEWLINE" {
                    break;
                }
            }
            let mut end_pos: i32 = fse_cur.clone();
            if !already_seen {
                let mut copy_len: i32 = end_pos - pos.clone();
                for copy_idx in 0..copy_len {
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = end_pos;
        } else if is_raw {
            let mut dn_offset: i32 = 1;
            if is_fn {
                dn_offset = 2;
            }
            let mut dn_pos: i32 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            let mut raw_pfx: String = "_raw_".to_string();
            let mut raw_key_parts: Vec<String> = vec![raw_pfx.clone(), decl_name.clone()];
            decl_name = s_join(raw_key_parts.clone());
            let mut already_seen: bool = false;
            let mut cs_len: i32 = (seen.len() as i32);
            for cs_i in 0..cs_len {
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    already_seen = true;
                    break;
                }
            }
            if !already_seen {
                seen.push(decl_name.clone());
            }
            let mut fse_cur: i32 = pos.clone();
            while fse_cur < token_count {
                let mut fse_tok: Token = tokens[fse_cur as usize].clone();
                let kind = fse_tok.kind.clone();
                fse_cur = fse_cur + 1;
                if kind == "NEWLINE" {
                    break;
                }
            }
            let mut end_pos: i32 = fse_cur.clone();
            if !already_seen {
                let mut copy_len: i32 = end_pos - pos.clone();
                for copy_idx in 0..copy_len {
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = end_pos;
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
                let mut decl_name: String = s_join(rk_parts.clone());
                let mut already_seen: bool = false;
                let mut cs_len: i32 = (seen.len() as i32);
                for cs_i in 0..cs_len {
                    let mut cs_val: String = seen[cs_i as usize].clone();
                    if cs_val == decl_name {
                        already_seen = true;
                        break;
                    }
                }
                if !already_seen {
                    seen.push(decl_name.clone());
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

fn expand_deor_macros(tokens: Vec<Token>) -> Vec<Token> {
    let mut macros: std::collections::HashMap<String, (Vec<Token>, i32)> = std::collections::HashMap::new();
    let mut result: Vec<Token> = vec![];
    let mut i: usize = 0;
    let mut scope_depth: i32 = 0;
    while i < tokens.len() {
    	let kind = tokens[i].kind.as_str();

    	// track scope depth for macro privacy
    	if kind == "INDENT" { scope_depth += 1; }
    	if kind == "DEDENT" {
    		scope_depth -= 1;
    		// remove any macros defined at the depth we are leaving
    		macros.retain(|_, (_, def_depth)| *def_depth <= scope_depth);
    	}

    	// collect macro definition
    	if kind == "KW_MACRO" {
    		let mut j = i + 1;
    		let name = if j < tokens.len() { tokens[j].value.clone() } else { String::new() };
    		j += 1;
    		// skip NEWLINE then INDENT
    		while j < tokens.len() && tokens[j].kind == "NEWLINE" { j += 1; }
    		while j < tokens.len() && tokens[j].kind == "INDENT" { j += 1; }
    		// collect body tokens, excluding the outer INDENT/DEDENT pair
    		let mut body: Vec<Token> = vec![];
    		let mut depth: i32 = 1;
    		while j < tokens.len() {
    			if tokens[j].kind == "INDENT" {
    				depth += 1;
    				body.push(tokens[j].clone());
    			} else if tokens[j].kind == "DEDENT" {
    				depth -= 1;
    				if depth == 0 { j += 1; break; }
    				body.push(tokens[j].clone());
    			} else {
    				body.push(tokens[j].clone());
    			}
    			j += 1;
    		}
    		if !name.is_empty() { macros.insert(name, (body, scope_depth)); }
    		// skip trailing NEWLINE after the definition block
    		while j < tokens.len() && tokens[j].kind == "NEWLINE" { j += 1; }
    		i = j;
    		continue;
    	}

    	// expand macro_run call site
    	if kind == "KW_MACRO_RUN" {
    		let mut j = i + 1;
    		let name = if j < tokens.len() { tokens[j].value.clone() } else { String::new() };
    		j += 1;
    		// skip trailing NEWLINE after the call
    		if j < tokens.len() && tokens[j].kind == "NEWLINE" { j += 1; }
    		// splice body tokens inline
    		if let Some((body, _)) = macros.get(&name) {
    			for tok in body { result.push(tok.clone()); }
    		}
    		i = j;
    		continue;
    	}

    	result.push(tokens[i].clone());
    	i += 1;
    }
    result
}

fn is_pascal(name: String) -> bool {
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    if name_len == 0 {
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    return s_upper_char(first.clone());
}

fn is_camel(name: String) -> bool {
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    if name_len == 0 {
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    if !s_lower_char(first.clone()) {
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
        if s_upper_char(chr.clone()) {
            return false;
        }
        idx = idx + 1;
    }
    return true;
}

fn arg_is_named(tokens: Vec<Token>, scan_pos: i32, kind: String) -> bool {
    let mut token_count: i32 = (tokens.len() as i32);
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
    if chk_kind != "IDENT" {
        return false;
    }
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
            return false;
        }
    }
    return true;
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
        } else if kind == "LBRACKET" {
            depth = depth + 1;
            saw_token = true;
        } else if kind == "RBRACKET" {
            depth = depth - 1;
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

fn val_err(tok: Token, label: String, rule: String) -> String {
    let value = tok.value.clone();
    let line = tok.line.clone();
    let file = tok.file.clone();
    let mut name: String = value.clone();
    let mut line_str: String = n_to_str(line.clone());
    let mut val_pfx: String = "[validation] ".to_string();
    let mut val_sep: String = " line ".to_string();
    let mut val_col: String = ": ".to_string();
    let mut val_qt1: String = " '".to_string();
    let mut val_qt2: String = "' - ".to_string();
    let mut val_parts: Vec<String> = vec![val_pfx.clone(), file.clone(), val_sep.clone(), line_str.clone(), val_col.clone(), label.clone(), val_qt1.clone(), name.clone(), val_qt2.clone(), rule.clone()];
    return s_join(val_parts.clone());
}

fn handle_errors(errors: Vec<String>) {
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

type FnTestRule = fn(String) -> bool;

fn validate_tokens(tokens: Vec<Token>) {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut errors: Vec<String> = Vec::new();
    let mut pos: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut lbl_struct: String = "struct".to_string();
    let mut lbl_enum: String = "enum".to_string();
    let mut lbl_shape: String = "shape".to_string();
    let mut lbl_type: String = "type".to_string();
    let mut lbl_fn: String = "fn".to_string();
    let mut lbl_var: String = "variable".to_string();
    let mut lbl_call: String = "call to".to_string();
    let mut lbl_rust: String = "identifier".to_string();
    let mut rule_min3: String = "name must be at least 3 characters".to_string();
    let mut rule_no_option: String = "Rust generic types (Option/Vec/Box/Rc/Arc/Result) are not valid in Deor — use shapes or validator types".to_string();
    let mut rule_pascal: String = "name must be PascalCase (start with uppercase letter)".to_string();
    let mut rule_camel: String = "name must be camelCase (start lowercase, no underscores)".to_string();
    let mut rule_snake: String = "name must be lower_snake_case (no uppercase letters)".to_string();
    let mut rule_named_arg: String = "each arg must be a named variable when passing 2 or more args".to_string();
    let mut rule_bad_stmt: String = "literal cannot follow 'name ident' — capture in a named variable first".to_string();
    let mut rule_not_is: String = "use 'x is not y' instead of 'not x is y' — 'not' binds before 'is' resolves".to_string();
    let mut rule_max_params: String = "functions may have at most 3 parameters".to_string();
    let mut rule_kw_in_parens: String = "reserved keyword cannot be used as a name — choose a different variable name".to_string();
    let mut rule_empty_bracket: String = "use 'empty' to initialize an empty list — [] is only valid with items inside".to_string();
    let mut rule_list_validator: String = "list shapes cannot be validator base types — validators only wrap primitives".to_string();
    let mut forbidden_in_parens: Vec<String> = vec!["KW_LIST".to_string(), "KW_STRUCT".to_string(), "KW_SHAPE".to_string(), "KW_ENUM".to_string(), "KW_TYPE".to_string(), "KW_FN".to_string(), "KW_OF".to_string(), "KW_FOR".to_string(), "KW_IF".to_string(), "KW_ELSE".to_string(), "KW_RETURN".to_string(), "KW_BREAK".to_string(), "KW_CONTINUE".to_string(), "KW_REMOVE".to_string(), "KW_RUST".to_string(), "KW_USING".to_string(), "KW_IMPORT".to_string(), "KW_MACRO".to_string(), "KW_VOID".to_string(), "KW_RAW".to_string()];
    let mut shape_names: Vec<String> = Vec::new();
    let mut pre_i: i32 = 0;
    while pre_i < token_count {
        let mut pre_tok: Token = tokens[pre_i as usize].clone();
        let kind = pre_tok.kind.clone();
        if kind == "KW_SHAPE" {
            let mut sn_pos: i32 = pre_i + 1.clone();
            if sn_pos < token_count {
                let mut sn_tok: Token = tokens[sn_pos as usize].clone();
                let value = sn_tok.value.clone();
                shape_names.push(value.clone());
            }
        }
        pre_i = pre_i + 1;
    }
    while pos < token_count {
        let mut tok: Token = tokens[pos as usize].clone();
        let kind = tok.kind.clone();
        let value = tok.value.clone();
        let line = tok.line.clone();
        let file = tok.file.clone();
        let mut cur_kind: String = kind.clone();
        let mut cur_val: String = value.clone();
        let mut cur_line: i32 = line.clone();
        let mut cur_file: String = file.clone();
        if cur_kind == "LPAREN" {
            paren_depth = paren_depth + 1;
        }
        if cur_kind == "RPAREN" {
            paren_depth = paren_depth - 1;
        }
        if paren_depth > 0 {
            let mut is_forbidden: bool = list_has(forbidden_in_parens.clone(), cur_kind.clone());
            if is_forbidden {
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
            }
        }
        let mut cur_indicator: String = "KW_RUST".to_string();
        let mut next_indicator: String = "RUST_BLOCK".to_string();
        if cur_kind == cur_indicator {
            let mut skip_pos: i32 = pos + 1.clone();
            while skip_pos < token_count {
                let mut skip_tok: Token = tokens[skip_pos as usize].clone();
                let kind = skip_tok.kind.clone();
                skip_pos = skip_pos + 1;
                if kind == next_indicator {
                    break;
                }
            }
            pos = skip_pos;
            continue;
        }
        if cur_kind == "KW_NOT" {
            let mut next_not: i32 = pos + 1.clone();
            let mut after_not: i32 = pos + 2.clone();
            if after_not < token_count {
                let mut next_not_tok: Token = tokens[next_not as usize].clone();
                let mut after_not_tok: Token = tokens[after_not as usize].clone();
                let kind = next_not_tok.kind.clone();
                let mut next_not_kind: String = kind.clone();
                let kind = after_not_tok.kind.clone();
                let mut after_not_kind: String = kind.clone();
                let mut next_is_ident: bool = next_not_kind == "IDENT".clone();
                let mut after_is_is: bool = after_not_kind == "KW_IS".clone();
                if next_is_ident && after_is_is {
                    let value = next_not_tok.value.clone();
                    errors.push(val_err(next_not_tok.clone(), lbl_var.clone(), rule_not_is.clone()).clone());
                }
            }
        }
        let mut validate_indent_offset = 1;
        let mut keyword: String = "KW_STRUCT".to_string();
        let mut lbl: String = lbl_struct.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        if cur_kind == keyword {
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        let mut keyword: String = "KW_ENUM".to_string();
        let mut lbl: String = lbl_enum.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        if cur_kind == keyword {
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        let mut keyword: String = "KW_SHAPE".to_string();
        let mut lbl: String = lbl_shape.clone();
        let mut rule: String = rule_camel.clone();
        let mut test_rule: fn(String) -> bool = is_camel.clone();
        if cur_kind == keyword {
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        let mut keyword: String = "KW_TYPE".to_string();
        let mut lbl: String = lbl_type.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        if cur_kind == keyword {
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "KW_TYPE" {
            let mut base_type_pos: i32 = pos + 3.clone();
            if base_type_pos < token_count {
                let mut base_type_tok: Token = tokens[base_type_pos as usize].clone();
                let value = base_type_tok.value.clone();
                let mut base_is_shape: bool = list_has(shape_names.clone(), value.clone());
                if base_is_shape {
                    let mut type_name_pos: i32 = pos + 1.clone();
                    let mut type_name_tok: Token = tokens[type_name_pos as usize].clone();
                    errors.push(val_err(type_name_tok.clone(), lbl_type.clone(), rule_list_validator.clone()).clone());
                }
            }
        }
        if cur_kind == "KW_FN" {
            let mut lp_pos: i32 = pos + 3.clone();
            if lp_pos < token_count {
                let mut lp_tok: Token = tokens[lp_pos as usize].clone();
                let kind = lp_tok.kind.clone();
                if kind == "LPAREN" {
                    let mut param_count: i32 = count_call_args(tokens.clone(), lp_pos.clone());
                    if param_count > 3 {
                        let mut fn_name_pos: i32 = pos + 2.clone();
                        let mut fn_name_tok: Token = tokens[fn_name_pos as usize].clone();
                        errors.push(val_err(fn_name_tok.clone(), lbl_fn.clone(), rule_max_params.clone()).clone());
                    }
                }
            }
            let mut rule_no_ret: String = "missing return type — use 'fn void name()' for functions that return nothing".to_string();
            let mut ret_pos: i32 = pos + 1.clone();
            let mut lp2_pos: i32 = pos + 2.clone();
            if lp2_pos < token_count {
                let mut ret_tok: Token = tokens[ret_pos as usize].clone();
                let mut lp2_tok: Token = tokens[lp2_pos as usize].clone();
                let kind = ret_tok.kind.clone();
                let mut ret_kind: String = kind.clone();
                let kind = lp2_tok.kind.clone();
                if ret_kind == "IDENT" {
                    if kind == "LPAREN" {
                        errors.push(val_err(ret_tok.clone(), lbl_fn.clone(), rule_no_ret.clone()).clone());
                    }
                }
            }
        }
        let mut keyword: String = "KW_FN".to_string();
        let mut lbl: String = lbl_fn.clone();
        let mut rule: String = rule_snake.clone();
        let mut test_rule: fn(String) -> bool = is_snake.clone();
        let mut validate_indent_offset: i32 = 2;
        if cur_kind == keyword {
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    if (name_val.len() as i32) < 3 {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        if cur_kind == "IDENT" {
            let mut is_crash: bool = cur_val == "crash".clone();
            if is_crash {
                let mut crash_lp: i32 = pos + 1.clone();
                if crash_lp < token_count {
                    let mut crash_lp_tok: Token = tokens[crash_lp as usize].clone();
                    let kind = crash_lp_tok.kind.clone();
                    if kind == "LPAREN" {
                        let mut crash_arg_count: i32 = count_call_args(tokens.clone(), crash_lp.clone());
                        let mut wrong_count: bool = crash_arg_count != 1.clone();
                        if wrong_count {
                            let mut rule_crash: String = "crash takes exactly 1 string argument".to_string();
                            errors.push(val_err(tok.clone(), lbl_call.clone(), rule_crash.clone()).clone());
                        }
                    }
                }
            }
        }
        if cur_kind == "LBRACKET" {
            let mut next_pos: i32 = pos + 1.clone();
            if next_pos < token_count {
                let mut next_tok: Token = tokens[next_pos as usize].clone();
                let kind = next_tok.kind.clone();
                if kind == "RBRACKET" {
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_empty_bracket.clone()).clone());
                }
            }
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
                            } else if kind == "LBRACKET" {
                                scan_depth = scan_depth + 1;
                                at_arg_start = false;
                            } else if kind == "RBRACKET" {
                                scan_depth = scan_depth - 1;
                                at_arg_start = false;
                            } else if kind == "COMMA" {
                                let mut scan_root: bool = scan_depth == 0.clone();
                                if scan_root {
                                    at_arg_start = true;
                                }
                            } else if at_arg_start {
                                let mut named: bool = arg_is_named(tokens.clone(), scan_pos.clone(), kind.clone());
                                if !named {
                                    errors.push(val_err(tok.clone(), lbl_call.clone(), rule_named_arg.clone()).clone());
                                }
                                at_arg_start = false;
                            }
                            scan_pos = scan_pos + 1;
                        }
                    }
                }
            }
            let mut is_option: bool = cur_val == "Option".clone();
            let mut is_vec: bool = cur_val == "Vec".clone();
            let mut is_box: bool = cur_val == "Box".clone();
            let mut is_rc: bool = cur_val == "Rc".clone();
            let mut is_arc: bool = cur_val == "Arc".clone();
            let mut is_result: bool = cur_val == "Result".clone();
            let mut is_rust_generic: bool = is_option || is_vec || is_box || is_rc || is_arc || is_result.clone();
            if is_rust_generic {
                errors.push(val_err(tok.clone(), lbl_rust.clone(), rule_no_option.clone()).clone());
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
                    let file = tok_one.file.clone();
                    let mut var_name: String = value.clone();
                    let mut var_line: i32 = line.clone();
                    let mut var_file: String = file.clone();
                    if (var_name.len() as i32) < 3 {
                        errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                    }
                    if !is_snake(var_name.clone()) {
                        errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                    }
                }
            }
            if paren_depth == 0 {
                let mut next1: i32 = pos + 1.clone();
                let mut next2: i32 = pos + 2.clone();
                if next2 < token_count {
                    let mut tok_one: Token = tokens[next1 as usize].clone();
                    let mut tok_two: Token = tokens[next2 as usize].clone();
                    let kind = tok_one.kind.clone();
                    let mut one_kind: String = kind.clone();
                    let kind = tok_two.kind.clone();
                    let mut two_kind: String = kind.clone();
                    if one_kind == "IDENT" {
                        let mut two_is_str: bool = two_kind == "STRING".clone();
                        let mut two_is_int: bool = two_kind == "INT".clone();
                        let mut two_is_flt: bool = two_kind == "FLOAT".clone();
                        let mut two_is_lit: bool = two_is_str || two_is_int || two_is_flt.clone();
                        if two_is_lit {
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_bad_stmt.clone()).clone());
                        }
                    }
                }
            }
            let mut next_pos: i32 = pos + 1.clone();
            if next_pos < token_count {
                let mut next_tok: Token = tokens[next_pos as usize].clone();
                let kind = next_tok.kind.clone();
                let mut bracket_rule: String = "bracket indexing is not valid in Deor — use 'name at index' instead".to_string();
                if kind == "LBRACKET" {
                    errors.push(val_err(tok.clone(), lbl_var.clone(), bracket_rule.clone()).clone());
                }
            }
        }
        pos = pos + 1;
    }
    handle_errors(errors.clone());
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
            let mut t4_pos: i32 = index + 4.clone();
            if t4_pos < token_count {
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut form_token: Token = tokens[form_pos as usize].clone();
                let value = name_token.value.clone();
                let mut shape_name: String = value.clone();
                let kind = form_token.kind.clone();
                if kind == "KW_LIST" {
                    let mut elem_pos: i32 = index + 5.clone();
                    if elem_pos < token_count {
                        let mut elem_token: Token = tokens[elem_pos as usize].clone();
                        let value = elem_token.value.clone();
                        result.push(shape_name.clone());
                        result.push(value.clone());
                    }
                } else {
                    let mut t4_token: Token = tokens[t4_pos as usize].clone();
                    let kind = t4_token.kind.clone();
                    let value = t4_token.value.clone();
                    let mut t4_is_of: bool = kind == "KW_OF".clone();
                    let mut t4_is_to: bool = value == "to".clone();
                    let mut in_type: String = "".to_string();
                    let mut out_type: String = "".to_string();
                    if t4_is_of {
                        let mut t5_pos: i32 = index + 5.clone();
                        if t5_pos < token_count {
                            let mut t5_token: Token = tokens[t5_pos as usize].clone();
                            let value = t5_token.value.clone();
                            in_type = value;
                        }
                        let mut t6_pos: i32 = index + 6.clone();
                        if t6_pos < token_count {
                            let mut t6_token: Token = tokens[t6_pos as usize].clone();
                            let value = t6_token.value.clone();
                            let mut t6_is_to: bool = value == "to".clone();
                            if t6_is_to {
                                let mut t7_pos: i32 = index + 7.clone();
                                if t7_pos < token_count {
                                    let mut t7_token: Token = tokens[t7_pos as usize].clone();
                                    let value = t7_token.value.clone();
                                    out_type = value;
                                }
                            }
                        }
                    } else if t4_is_to {
                        let mut t5_pos: i32 = index + 5.clone();
                        if t5_pos < token_count {
                            let mut t5_token: Token = tokens[t5_pos as usize].clone();
                            let value = t5_token.value.clone();
                            out_type = value;
                        }
                    }
                    result.push(shape_name.clone());
                    let mut fn_parts: Vec<String> = vec!["fn:".to_string(), in_type.clone(), ":".to_string(), out_type.clone()];
                    result.push(s_join(fn_parts.clone()).clone());
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
    let mut const_names: Vec<String> = Vec::new();
    for raw_i in start..end_pos {
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_CONST" {
            let mut const_name_pos: i32 = raw_i + 2.clone();
            if const_name_pos < end_pos {
                let mut const_name_tok: Token = tokens[const_name_pos as usize].clone();
                let value = const_name_tok.value.clone();
                if !list_has(const_names.clone(), value.clone()) {
                    const_names.push(value.clone());
                }
            }
        }
        if kind == "EQUALS" {
            let mut prev_pos: i32 = raw_i - 1.clone();
            if prev_pos >= start {
                let mut prev_token: Token = tokens[prev_pos as usize].clone();
                let kind = prev_token.kind.clone();
                let value = prev_token.value.clone();
                if kind == "IDENT" {
                    if !list_has(result.clone(), value.clone()) {
                        if !list_has(const_names.clone(), value.clone()) {
                            result.push(value.clone());
                        }
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

fn find_struct_for_fields(struct_reg: Vec<String>, fields: Vec<String>) -> String {
    let mut comma: String = ",".to_string();
    let mut input_count: i32 = (fields.len() as i32);
    let mut reg_count: i32 = (struct_reg.len() as i32);
    let mut next_is_val: bool = false;
    let mut cur_name: String = "".to_string();
    for index in 0..reg_count {
        let mut item: String = struct_reg[index as usize].clone();
        if next_is_val {
            let mut reg_fields: Vec<String> = s_split(item.clone(), comma.clone());
            let mut reg_count_f: i32 = (reg_fields.len() as i32);
            if reg_count_f == input_count {
                let mut all_match: bool = true;
                for fi in 0..input_count {
                    let mut field: String = fields[fi as usize].clone();
                    let mut found: bool = list_has(reg_fields.clone(), field.clone());
                    if !found {
                        all_match = false;
                    }
                }
                if all_match {
                    return cur_name;
                }
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
    return make_result(args_str, cur.clone());
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
    return make_result(items_str, cur.clone());
}

fn gen_unary_method(args_pos: i32, suffix: String, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let mut inner_r: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
    let mut inner_code: String = pr_code(inner_r.clone());
    let mut close: i32 = pr_pos(inner_r.clone()) + 1;
    let mut res_parts: Vec<String> = vec![inner_code.clone(), suffix.clone()];
    let mut result_code: String = s_join(res_parts.clone());
    return make_result(result_code, close.clone());
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
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(value, lit_next.clone());
    }
    if kind == "FLOAT" {
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(value, lit_next.clone());
    }
    if kind == "STRING" {
        let mut lit_debug: String = s_debug(value.clone());
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_debug, lit_next.clone());
    }
    if kind == "KW_TRUE" {
        let mut lit_true: String = "true".to_string();
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_true, lit_next.clone());
    }
    if kind == "KW_FALSE" {
        let mut lit_false: String = "false".to_string();
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_false, lit_next.clone());
    }
    if kind == "KW_BAD" {
        let mut lit_none: String = "None".to_string();
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_none, lit_next.clone());
    }
    if kind == "LBRACKET" {
        let mut ll_inner: i32 = pos + 1.clone();
        let mut ll_items_r: ParseResult = gen_list_items(tokens.clone(), ll_inner.clone(), ctx.clone());
        let mut ll_items_code: String = pr_code(ll_items_r.clone());
        let mut ll_items_pos: i32 = pr_pos(ll_items_r.clone());
        let mut ll_open: String = "vec![".to_string();
        let mut ll_close: String = "]".to_string();
        let mut ll_parts: Vec<String> = vec![ll_open.clone(), ll_items_code.clone(), ll_close.clone()];
        let mut ll_code: String = s_join(ll_parts.clone());
        let mut ll_after: i32 = ll_items_pos + 1.clone();
        return make_result(ll_code, ll_after.clone());
    }
    if kind == "LPAREN" {
        let mut pe_peek: i32 = pos + 1.clone();
        if pe_peek < token_count {
            let mut pe_peek_tok: Token = tokens[pe_peek as usize].clone();
            let kind = pe_peek_tok.kind.clone();
            if kind == "KW_AVOW" {
                let mut pe_expr_pos: i32 = pe_peek + 1.clone();
                let mut pe_expr_r: ParseResult = gen_expr(tokens.clone(), pe_expr_pos.clone(), ctx.clone());
                let mut pe_expr_code: String = pr_code(pe_expr_r.clone());
                let mut pe_after: i32 = pr_pos(pe_expr_r.clone()) + 1;
                let mut pe_unw: String = ".unwrap()".to_string();
                let mut pe_unw_parts: Vec<String> = vec![pe_expr_code.clone(), pe_unw.clone()];
                let mut pe_unwrap_code: String = s_join(pe_unw_parts.clone());
                return make_result(pe_unwrap_code, pe_after.clone());
            }
            let mut pe_fields: Vec<String> = Vec::new();
            let mut pe_cur: i32 = pe_peek.clone();
            while pe_cur < token_count {
                let mut pe_field_tok: Token = tokens[pe_cur as usize].clone();
                let kind = pe_field_tok.kind.clone();
                let value = pe_field_tok.value.clone();
                if kind == "RPAREN" {
                    pe_cur = pe_cur + 1;
                    break;
                } else if kind == "COMMA" {
                    pe_cur = pe_cur + 1;
                } else if kind == "IDENT" {
                    pe_fields.push(value.clone());
                    pe_cur = pe_cur + 1;
                }
            }
            let mut pe_struct_name: String = find_struct_for_fields(struct_reg.clone(), pe_fields.clone());
            let mut pe_sep: String = ", ".to_string();
            let mut pe_fields_code: String = s_join_with(pe_fields.clone(), pe_sep.clone());
            let mut pe_sco: String = " { ".to_string();
            let mut pe_scc: String = " }".to_string();
            let mut pe_sct_parts: Vec<String> = vec![pe_struct_name.clone(), pe_sco.clone(), pe_fields_code.clone(), pe_scc.clone()];
            let mut pe_struct_code: String = s_join(pe_sct_parts.clone());
            return make_result(pe_struct_code, pe_cur.clone());
        }
    }
    if kind == "KW_GIVEUP" {
        let mut po_inner: i32 = pos + 1.clone();
        let mut po_r: ParseResult = gen_primary(tokens.clone(), po_inner.clone(), ctx.clone());
        return po_r;
    }
    if kind == "KW_NOT" {
        let mut po_operand: i32 = pos + 1.clone();
        let mut po_operand_r: ParseResult = gen_primary(tokens.clone(), po_operand.clone(), ctx.clone());
        let mut po_code: String = pr_code(po_operand_r.clone());
        let mut po_end: i32 = pr_pos(po_operand_r.clone());
        let mut po_bang: String = "!".to_string();
        let mut po_parts: Vec<String> = vec![po_bang.clone(), po_code.clone()];
        let mut po_not_code: String = s_join(po_parts.clone());
        return make_result(po_not_code, po_end.clone());
    }
    if kind == "IDENT" {
        let mut ie_next: i32 = pos + 1.clone();
        if ie_next < token_count {
            let mut ie_next_tok: Token = tokens[ie_next as usize].clone();
            let kind = ie_next_tok.kind.clone();
            if kind == "LPAREN" {
                let mut ie_func: String = value.clone();
                let mut ie_args_pos: i32 = ie_next + 1.clone();
                if ie_func == "len" {
                    let mut ie_len_sfx: String = ".len() as i32".to_string();
                    let mut ie_len_r: ParseResult = gen_unary_method(ie_args_pos.clone(), ie_len_sfx.clone(), ctx.clone());
                    let mut ie_len_code: String = pr_code(ie_len_r.clone());
                    let mut ie_len_end: i32 = pr_pos(ie_len_r.clone());
                    let mut ie_lp: String = "(".to_string();
                    let mut ie_rp: String = ")".to_string();
                    let mut ie_len_parts: Vec<String> = vec![ie_lp.clone(), ie_len_code.clone(), ie_rp.clone()];
                    let mut ie_len_wrapped: String = s_join(ie_len_parts.clone());
                    return make_result(ie_len_wrapped, ie_len_end.clone());
                } else if ie_func == "crash" {
                    let mut ie_crash_r: ParseResult = gen_call_args(tokens.clone(), ie_args_pos.clone(), ctx.clone());
                    let mut ie_crash_code: String = pr_code(ie_crash_r.clone());
                    let mut ie_crash_end: i32 = pr_pos(ie_crash_r.clone());
                    let mut ie_pan_pfx: String = "panic!(\"{}\", ".to_string();
                    let mut ie_pan_sfx: String = ")".to_string();
                    let mut ie_pan_parts: Vec<String> = vec![ie_pan_pfx.clone(), ie_crash_code.clone(), ie_pan_sfx.clone()];
                    let mut ie_panic_code: String = s_join(ie_pan_parts.clone());
                    let mut ie_after_crash: i32 = ie_crash_end + 1.clone();
                    return make_result(ie_panic_code, ie_after_crash.clone());
                }
                let mut ie_args_r: ParseResult = gen_call_args(tokens.clone(), ie_args_pos.clone(), ctx.clone());
                let mut ie_args_code: String = pr_code(ie_args_r.clone());
                let mut ie_args_end: i32 = pr_pos(ie_args_r.clone());
                let mut ie_after: i32 = ie_args_end + 1.clone();
                let mut ie_lp: String = "(".to_string();
                let mut ie_rp: String = ")".to_string();
                let mut ie_call_parts: Vec<String> = vec![ie_func.clone(), ie_lp.clone(), ie_args_code.clone(), ie_rp.clone()];
                let mut ie_call_code: String = s_join(ie_call_parts.clone());
                return make_result(ie_call_code, ie_after.clone());
            }
            if kind == "KW_AT" {
                let mut ie_idx_pos: i32 = ie_next + 1.clone();
                let mut ie_idx_r: ParseResult = gen_primary(tokens.clone(), ie_idx_pos.clone(), ctx.clone());
                let mut ie_idx_code: String = pr_code(ie_idx_r.clone());
                let mut ie_idx_end: i32 = pr_pos(ie_idx_r.clone());
                let mut ie_idx_mid: String = "[".to_string();
                let mut ie_idx_sfx: String = " as usize].clone()".to_string();
                let mut ie_idx_parts: Vec<String> = vec![value.clone(), ie_idx_mid.clone(), ie_idx_code.clone(), ie_idx_sfx.clone()];
                let mut ie_idx_expr: String = s_join(ie_idx_parts.clone());
                return make_result(ie_idx_expr, ie_idx_end.clone());
            }
        }
        let mut ie_variant_enum: String = reg_get(variant_reg.clone(), value.clone());
        if !is_empty(ie_variant_enum.clone()) {
            let mut ie_dbl: String = "::".to_string();
            let mut ie_var_parts: Vec<String> = vec![ie_variant_enum.clone(), ie_dbl.clone(), value.clone()];
            let mut ie_variant_code: String = s_join(ie_var_parts.clone());
            return make_result(ie_variant_code, ie_next.clone());
        }
        return make_result(value, ie_next.clone());
    }
    let mut unknown: String = "/* unknown_primary */".to_string();
    let mut next: i32 = pos + 1.clone();
    return make_result(unknown, next.clone());
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
                if kind == "KW_EMPTY" {
                    let mut ie_sfx: String = ".is_empty()".to_string();
                    let mut ie_parts: Vec<String> = vec![left_code.clone(), ie_sfx.clone()];
                    left_code = s_join(ie_parts.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
            }
        }
        if operator_str == "is not" {
            if after_op < token_count {
                let mut maybe_empty: Token = tokens[after_op as usize].clone();
                let kind = maybe_empty.kind.clone();
                if kind == "KW_EMPTY" {
                    let mut ine_pfx: String = "!".to_string();
                    let mut ine_sfx: String = ".is_empty()".to_string();
                    let mut ine_parts: Vec<String> = vec![ine_pfx.clone(), left_code.clone(), ine_sfx.clone()];
                    left_code = s_join(ine_parts.clone());
                    cur_pos = after_op + 1;
                    continue;
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
    return make_result(left_code, cur_pos.clone());
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

fn make_destruct_code(var_name: String, depth: i32, ctx: RcCtx) -> String {
    let struct_reg = ctx.struct_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let mut struct_type: String = reg_get(var_type_reg.clone(), var_name.clone());
    let mut fields_str: String = reg_get(struct_reg.clone(), struct_type.clone());
    if is_empty(fields_str.clone()) {
        return "".to_string();
    }
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
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

fn gen_destructure(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let using_type = ctx.using_type.clone();
    let using_var = ctx.using_var.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
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
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = (fields.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
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
    let mut after: i32 = adv_nl_ref(src_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    let mut newline: String = "\n".to_string();
    dest_code = s_cat(dest_code.clone(), newline.clone());
    return make_result(dest_code, after.clone());
}

fn gen_move_destructure(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let using_type = ctx.using_type.clone();
    let using_var = ctx.using_var.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
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
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = (fields.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
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
        let mut dst_sc: String = ";".to_string();
        let mut dst_parts: Vec<String> = vec![pad.clone(), dst_let.clone(), mut_kw.clone(), field.clone(), dst_eq.clone(), src_code.clone(), dst_dot.clone(), field.clone(), dst_sc.clone()];
        dest_lines.push(s_join(dst_parts.clone()).clone());
    }
    let mut after: i32 = adv_nl_ref(src_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    let mut newline: String = "\n".to_string();
    dest_code = s_cat(dest_code.clone(), newline.clone());
    return make_result(dest_code, after.clone());
}

fn gen_block(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
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
        let mut stmt_r: ParseResult = gen_stmt(cur.clone(), depth.clone(), ctx.clone());
        let code = stmt_r.code;
        let new_pos = stmt_r.new_pos;
        stmts.push(code.clone());
        cur = new_pos;
    }
    let mut block_joined: String = s_join(stmts.clone());
    return make_result(block_joined, cur.clone());
}

fn gen_if_branch(cond_pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let mut cond_end: i32 = pr_pos(cond_r.clone());
    let mut body_start: i32 = skip_to_body_ref(tokens.clone(), cond_end.clone());
    let mut body_depth: i32 = depth + 1.clone();
    let mut body_r: ParseResult = gen_block(body_start.clone(), body_depth.clone(), ctx.clone());
    let mut cond_code: String = pr_code(cond_r.clone());
    let mut body_code: String = pr_code(body_r.clone());
    let mut brc_open: String = " {\n".to_string();
    let mut cmb_parts: Vec<String> = vec![cond_code.clone(), brc_open.clone(), body_code.clone()];
    let mut combined: String = s_join(cmb_parts.clone());
    let mut branch_end: i32 = pr_pos(body_r.clone());
    return make_result(combined, branch_end.clone());
}

fn gen_if(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut if_cond_pos: i32 = pos + 1.clone();
    let mut then_r: ParseResult = gen_if_branch(if_cond_pos.clone(), depth.clone(), ctx.clone());
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
        if kind == "NEWLINE" {
            cur = cur + 1;
            continue;
        }
        if kind != "KW_ELSE" {
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
            let mut ei_r: ParseResult = gen_if_branch(ei_cond.clone(), depth.clone(), ctx.clone());
            let mut ei_code: String = pr_code(ei_r.clone());
            let mut eli_kw: String = " else if ".to_string();
            let mut eli_cl: String = "}".to_string();
            let mut eli_parts: Vec<String> = vec![result_code.clone(), eli_kw.clone(), ei_code.clone(), pad.clone(), eli_cl.clone()];
            result_code = s_join(eli_parts.clone());
            cur = pr_pos(ei_r.clone());
        } else {
            let mut else_body_start: i32 = skip_to_body_ref(tokens.clone(), after_else.clone());
            let mut else_depth: i32 = depth + 1.clone();
            let mut else_r: ParseResult = gen_block(else_body_start.clone(), else_depth.clone(), ctx.clone());
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

fn gen_for(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
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
        let mut while_body_r: ParseResult = gen_block(while_body_start.clone(), whl_depth.clone(), ctx.clone());
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
        let mut body_r: ParseResult = gen_block(body_start.clone(), body_depth.clone(), ctx.clone());
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
    let mut body_r: ParseResult = gen_block(body_tok_pos.clone(), for_body_depth.clone(), ctx.clone());
    let mut body_code: String = pr_code(body_r.clone());
    let mut body_end: i32 = pr_pos(body_r.clone());
    let mut frc_kw: String = "for ".to_string();
    let mut frc_in: String = " in ".to_string();
    let mut frc_ob: String = " {\n".to_string();
    let mut frc_cb: String = "}\n".to_string();
    let mut frc_parts: Vec<String> = vec![pad.clone(), frc_kw.clone(), var_name.clone(), frc_in.clone(), range_expr.clone(), frc_ob.clone(), body_code.clone(), pad.clone(), frc_cb.clone()];
    let mut for_code: String = s_join(frc_parts.clone());
    return make_result(for_code, body_end.clone());
}

fn gen_as_binding(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let struct_reg = ctx.struct_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut name_token: Token = tokens[pos as usize].clone();
    let value = name_token.value.clone();
    let mut ident_name: String = value.clone();
    let mut next_pos: i32 = pos + 1.clone();
    let mut after_as: i32 = next_pos + 1.clone();
    let mut after_as_token: Token = tokens[after_as as usize].clone();
    let kind = after_as_token.kind.clone();
    let value = after_as_token.value.clone();
    let mut after_as_value: String = value.clone();
    if kind == "LPAREN" {
        let mut aas_fields: Vec<String> = Vec::new();
        let mut aas_fend: i32 = after_as + 1.clone();
        while aas_fend < token_count {
            let mut aas_field_tok: Token = tokens[aas_fend as usize].clone();
            let kind = aas_field_tok.kind.clone();
            let value = aas_field_tok.value.clone();
            if kind == "RPAREN" {
                aas_fend = aas_fend + 1;
                break;
            } else if kind == "COMMA" {
                aas_fend = aas_fend + 1;
            } else if kind == "IDENT" {
                aas_fields.push(value.clone());
                aas_fend = aas_fend + 1;
            }
        }
        let mut aas_struct: String = find_struct_for_fields(struct_reg.clone(), aas_fields.clone());
        let mut aas_is_mut: bool = list_has(mut_names.clone(), ident_name.clone());
        let mut aas_mut: String = "".to_string();
        if aas_is_mut {
            aas_mut = "mut ".to_string();
        }
        let mut aas_fcount: i32 = (aas_fields.len() as i32);
        let mut aas_pairs: Vec<String> = Vec::new();
        for aas_fi in 0..aas_fcount {
            let mut aas_fname: String = aas_fields[aas_fi as usize].clone();
            let mut aas_fp_sep: String = ": ".to_string();
            let mut aas_fp_cln: String = ".clone()".to_string();
            let mut aas_fp: Vec<String> = vec![aas_fname.clone(), aas_fp_sep.clone(), aas_fname.clone(), aas_fp_cln.clone()];
            aas_pairs.push(s_join(aas_fp.clone()).clone());
        }
        let mut aas_fld_sep: String = ", ".to_string();
        let mut aas_fields_code: String = s_join_with(aas_pairs.clone(), aas_fld_sep.clone());
        let mut aas_let: String = "let ".to_string();
        let mut aas_eq: String = " = ".to_string();
        let mut aas_ob: String = " { ".to_string();
        let mut aas_cb: String = " };\n".to_string();
        let mut aas_parts: Vec<String> = vec![pad.clone(), aas_let.clone(), aas_mut.clone(), ident_name.clone(), aas_eq.clone(), aas_struct.clone(), aas_ob.clone(), aas_fields_code.clone(), aas_cb.clone()];
        let mut aas_code: String = s_join(aas_parts.clone());
        let mut aas_next: i32 = adv_nl_ref(aas_fend.clone(), tokens.clone());
        return make_result(aas_code, aas_next.clone());
    }
    if kind == "KW_EMPTY" {
        let mut aas_emp_pfx: String = "let mut ".to_string();
        let mut aas_emp_sfx: String = " = Vec::new();\n".to_string();
        let mut aas_emp_parts: Vec<String> = vec![pad.clone(), aas_emp_pfx.clone(), ident_name.clone(), aas_emp_sfx.clone()];
        let mut aas_empty_code: String = s_join(aas_emp_parts.clone());
        let mut aas_after_empty: i32 = after_as + 1.clone();
        let mut aas_empty_next: i32 = adv_nl_ref(aas_after_empty.clone(), tokens.clone());
        return make_result(aas_empty_code, aas_empty_next.clone());
    }
    if kind == "IDENT" {
        let mut aas_with_pos: i32 = after_as + 1.clone();
        if aas_with_pos < token_count {
            let mut aas_with_tok: Token = tokens[aas_with_pos as usize].clone();
            let kind = aas_with_tok.kind.clone();
            if kind == "KW_WITH" {
                let mut aas_src: String = after_as_value.clone();
                let mut aas_lp: i32 = aas_with_pos + 1.clone();
                let mut aas_ovr: Vec<String> = Vec::new();
                let mut aas_wend: i32 = aas_lp + 1.clone();
                while aas_wend < token_count {
                    let mut aas_wtok: Token = tokens[aas_wend as usize].clone();
                    let kind = aas_wtok.kind.clone();
                    let value = aas_wtok.value.clone();
                    if kind == "RPAREN" {
                        aas_wend = aas_wend + 1;
                        break;
                    }
                    if kind == "COMMA" {
                        aas_wend = aas_wend + 1;
                        continue;
                    }
                    if kind == "IDENT" {
                        aas_ovr.push(value.clone());
                        aas_wend = aas_wend + 1;
                    }
                }
                let mut aas_first: String = aas_ovr[0 as usize].clone();
                let mut aas_struct_w: String = find_struct_for_field(struct_reg.clone(), aas_first.clone());
                let mut aas_wsep: String = ", ".to_string();
                let mut aas_wfields: String = s_join_with(aas_ovr.clone(), aas_wsep.clone());
                let mut aas_wmut: bool = list_has(mut_names.clone(), ident_name.clone());
                let mut aas_wmutkw: String = "".to_string();
                if aas_wmut {
                    aas_wmutkw = "mut ".to_string();
                }
                let mut aas_wlet: String = "let ".to_string();
                let mut aas_weq: String = " = ".to_string();
                let mut aas_wob: String = " { ".to_string();
                let mut aas_wsp: String = ", ..".to_string();
                let mut aas_wcb: String = " };\n".to_string();
                let mut aas_wparts: Vec<String> = vec![pad.clone(), aas_wlet.clone(), aas_wmutkw.clone(), ident_name.clone(), aas_weq.clone(), aas_struct_w.clone(), aas_wob.clone(), aas_wfields.clone(), aas_wsp.clone(), aas_src.clone(), aas_wcb.clone()];
                let mut aas_with_code: String = s_join(aas_wparts.clone());
                let mut aas_wnext: i32 = adv_nl_ref(aas_wend.clone(), tokens.clone());
                return make_result(aas_with_code, aas_wnext.clone());
            }
        }
    }
    let kind = after_as_token.kind.clone();
    let mut aas_val_r: ParseResult = gen_expr(tokens.clone(), after_as.clone(), ctx.clone());
    let mut aas_val_code: String = pr_code(aas_val_r.clone());
    let mut aas_val_end: i32 = pr_pos(aas_val_r.clone());
    let mut aas_is_str: bool = kind == "STRING".clone();
    let mut aas_is_mut2: bool = list_has(mut_names.clone(), ident_name.clone());
    let mut aas_mut2: String = "".to_string();
    if aas_is_mut2 {
        aas_mut2 = "mut ".to_string();
    }
    let mut aas_suffix: String = "".to_string();
    if aas_is_str {
        aas_suffix = ".to_string()".to_string();
    }
    let mut aas_let2: String = "let ".to_string();
    let mut aas_eq2: String = " = ".to_string();
    let mut aas_sc2: String = ";\n".to_string();
    let mut aas_parts2: Vec<String> = vec![pad.clone(), aas_let2.clone(), aas_mut2.clone(), ident_name.clone(), aas_eq2.clone(), aas_val_code.clone(), aas_suffix.clone(), aas_sc2.clone()];
    let mut aas_code2: String = s_join(aas_parts2.clone());
    let mut aas_next2: i32 = adv_nl_ref(aas_val_end.clone(), tokens.clone());
    return make_result(aas_code2, aas_next2.clone());
}

fn gen_call_stmt(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let using_var = ctx.using_var.clone();
    let using_type = ctx.using_type.clone();
    let mut_names = ctx.mut_names.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut name_token: Token = tokens[pos as usize].clone();
    let value = name_token.value.clone();
    let mut ident_name: String = value.clone();
    let mut next_pos: i32 = pos + 1.clone();
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
        let mut re_destruct: String = make_destruct_code(using_var.clone(), depth.clone(), ctx.clone());
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

fn gen_list_mutation_stmt(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut name_token: Token = tokens[pos as usize].clone();
    let value = name_token.value.clone();
    let mut ident_name: String = value.clone();
    let mut next_pos: i32 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
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
                return make_result(app_code, app_next.clone());
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
            let mut idw_code: String = s_join(idw_parts.clone());
            let mut idw_next: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
            return make_result(idw_code, idw_next.clone());
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
        return make_result(rem_code, rem_next.clone());
    }
    let mut lm_unh_pfx: String = "/* unhandled_list_mut(".to_string();
    let mut lm_unh_sfx: String = ") */\n".to_string();
    let mut lm_unh_parts: Vec<String> = vec![lm_unh_pfx.clone(), kind.clone(), lm_unh_sfx.clone()];
    let mut lm_unhand: String = s_join(lm_unh_parts.clone());
    let mut lm_next: i32 = pos + 1.clone();
    return make_result(lm_unhand, lm_next.clone());
}

fn gen_typed_binding(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let tokens = ctx.tokens.clone();
    let struct_reg = ctx.struct_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let type_reg = ctx.type_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let variant_reg = ctx.variant_reg.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut type_token: Token = tokens[pos as usize].clone();
    let value = type_token.value.clone();
    let mut var_type: String = value.clone();
    let mut name_pos: i32 = pos + 1.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let value = name_token.value.clone();
    let mut var_name: String = value.clone();
    let mut eq_pos: i32 = name_pos + 1.clone();
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
            let mut err_msg: String = "/* error: use 'bad' not 'empty' for validator types */\n".to_string();
            let mut err_parts: Vec<String> = vec![pad.clone(), err_msg.clone()];
            let mut err_code: String = s_join(err_parts.clone());
            return make_result(err_code, after_empty.clone());
        }
        if is_shape {
            let mut sh_pfx: String = "let mut ".to_string();
            let mut sh_mid: String = ": ".to_string();
            let mut sh_sfx: String = " = Vec::new();\n".to_string();
            let mut sh_parts: Vec<String> = vec![pad.clone(), sh_pfx.clone(), var_name.clone(), sh_mid.clone(), rust_type.clone(), sh_sfx.clone()];
            let mut sh_code: String = s_join(sh_parts.clone());
            return make_result(sh_code, after_empty.clone());
        }
        let mut err_msg: String = "/* error: empty is only valid for list shapes */\n".to_string();
        let mut err_parts: Vec<String> = vec![pad.clone(), err_msg.clone()];
        let mut err_code: String = s_join(err_parts.clone());
        return make_result(err_code, after_empty.clone());
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
            return make_result(sc_code, sc_next.clone());
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
        return make_result(aw_code, aw_next.clone());
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
        return make_result(lst_code, lst_next.clone());
    }
    if kind == "KW_BAD" {
        let mut bad_is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
        let mut bad_pos_next: i32 = val_pos + 1.clone();
        let mut bad_next: i32 = adv_nl_ref(bad_pos_next.clone(), tokens.clone());
        if bad_is_validator {
            let mut non_pfx: String = "let mut ".to_string();
            let mut non_mid: String = ": Option<".to_string();
            let mut non_sfx: String = "> = None;\n".to_string();
            let mut non_parts: Vec<String> = vec![pad.clone(), non_pfx.clone(), var_name.clone(), non_mid.clone(), rust_type.clone(), non_sfx.clone()];
            let mut none_code: String = s_join(non_parts.clone());
            return make_result(none_code, bad_next.clone());
        }
        let mut bad_err: String = "/* error: bad is only valid for validator types */\n".to_string();
        let mut bad_err_parts: Vec<String> = vec![pad.clone(), bad_err.clone()];
        let mut bad_err_code: String = s_join(bad_err_parts.clone());
        return make_result(bad_err_code, bad_next.clone());
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
        return make_result(vld_code, vld_next.clone());
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
        let mut val_next_idx: i32 = val_pos + 1.clone();
        if val_next_idx < token_count {
            let mut next_val_tok: Token = tokens[val_next_idx as usize].clone();
            let kind = next_val_tok.kind.clone();
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
    return make_result(bind_code, bind_next.clone());
}

fn gen_stmt(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut using_type = ctx.using_type.clone();
    let mut using_var = ctx.using_var.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut token: Token = tokens[pos as usize].clone();
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut newline: String = "\n".to_string();
    if kind == "KW_RETURN" {
        let mut sf_val_pos: i32 = pos + 1.clone();
        let mut sf_val_tok: Token = tokens[sf_val_pos as usize].clone();
        let kind = sf_val_tok.kind.clone();
        let mut sf_val_r: ParseResult = gen_expr(tokens.clone(), sf_val_pos.clone(), ctx.clone());
        let mut sf_val_code: String = pr_code(sf_val_r.clone());
        let mut sf_val_end: i32 = pr_pos(sf_val_r.clone());
        let mut sf_suffix: String = "".to_string();
        if kind == "STRING" {
            sf_suffix = ".to_string()".to_string();
        }
        let mut sf_ret_kw: String = "return ".to_string();
        let mut sf_ret_sc: String = ";\n".to_string();
        let mut sf_ret_parts: Vec<String> = vec![pad.clone(), sf_ret_kw.clone(), sf_val_code.clone(), sf_suffix.clone(), sf_ret_sc.clone()];
        let mut sf_ret_code: String = s_join(sf_ret_parts.clone());
        let mut sf_ret_next: i32 = adv_nl_ref(sf_val_end.clone(), tokens.clone());
        return make_result(sf_ret_code, sf_ret_next.clone());
    }
    if kind == "KW_BREAK" {
        let mut sf_brk_kw: String = "break;\n".to_string();
        let mut sf_brk_parts: Vec<String> = vec![pad.clone(), sf_brk_kw.clone()];
        let mut sf_brk_code: String = s_join(sf_brk_parts.clone());
        let mut sf_brk_n: i32 = pos + 1.clone();
        let mut sf_brk_next: i32 = adv_nl_ref(sf_brk_n.clone(), tokens.clone());
        return make_result(sf_brk_code, sf_brk_next.clone());
    }
    if kind == "KW_CONTINUE" {
        let mut sf_cnt_kw: String = "continue;\n".to_string();
        let mut sf_cnt_parts: Vec<String> = vec![pad.clone(), sf_cnt_kw.clone()];
        let mut sf_cnt_code: String = s_join(sf_cnt_parts.clone());
        let mut sf_cnt_n: i32 = pos + 1.clone();
        let mut sf_cnt_next: i32 = adv_nl_ref(sf_cnt_n.clone(), tokens.clone());
        return make_result(sf_cnt_code, sf_cnt_next.clone());
    }
    if kind == "KW_BLOCK" {
        let mut sb_nl: i32 = pos + 1.clone();
        let mut sb_body_start: i32 = skip_to_body_ref(tokens.clone(), sb_nl.clone());
        let mut sb_body_depth: i32 = depth + 1.clone();
        let mut sb_body_r: ParseResult = gen_block(sb_body_start.clone(), sb_body_depth.clone(), ctx.clone());
        let mut sb_body_code: String = pr_code(sb_body_r.clone());
        let mut sb_body_end: i32 = pr_pos(sb_body_r.clone());
        let mut sb_open_brace: String = "{\n".to_string();
        let mut sb_close_brace: String = "}\n".to_string();
        let mut sb_blk_open: String = s_cat(pad.clone(), sb_open_brace.clone());
        let mut sb_blk_close: String = s_cat(pad.clone(), sb_close_brace.clone());
        let mut sb_blk_parts: Vec<String> = vec![sb_blk_open.clone(), sb_body_code.clone(), sb_blk_close.clone()];
        let mut sb_blk_code: String = s_join(sb_blk_parts.clone());
        return make_result(sb_blk_code, sb_body_end.clone());
    }
    if kind == "KW_RUST" {
        let mut sb_block_pos: i32 = pos + 2.clone();
        let mut sb_block_tok: Token = tokens[sb_block_pos as usize].clone();
        let value = sb_block_tok.value.clone();
        let mut sb_content: String = value.clone();
        let mut sb_rust_lines: Vec<String> = s_split(sb_content.clone(), newline.clone());
        let mut sb_padded: Vec<String> = Vec::new();
        let mut sb_line_count: i32 = (sb_rust_lines.len() as i32);
        for sb_ri in 0..sb_line_count {
            let mut sb_rust_line: String = sb_rust_lines[sb_ri as usize].clone();
            if is_empty(sb_rust_line.clone()) {
                let mut sb_empty_line: String = "".to_string();
                sb_padded.push(sb_empty_line.clone());
            } else {
                let mut sb_rsl_parts: Vec<String> = vec![pad.clone(), sb_rust_line.clone()];
                sb_padded.push(s_join(sb_rsl_parts.clone()).clone());
            }
        }
        let mut sb_block_code: String = s_join_nl(sb_padded.clone());
        sb_block_code = s_cat(sb_block_code.clone(), newline.clone());
        let mut sb_block_next: i32 = sb_block_pos + 1.clone();
        return make_result(sb_block_code, sb_block_next.clone());
    }
    if kind == "KW_GIVEUP" {
        let mut smd_next: i32 = pos + 1.clone();
        let mut smd_next_tok: Token = tokens[smd_next as usize].clone();
        let kind = smd_next_tok.kind.clone();
        if kind == "LPAREN" {
            return gen_move_destructure(smd_next.clone(), depth.clone(), ctx.clone());
        }
    }
    if kind == "LPAREN" {
        let mut su_peek: i32 = pos + 1.clone();
        if su_peek < token_count {
            let mut su_peek_tok: Token = tokens[su_peek as usize].clone();
            let kind = su_peek_tok.kind.clone();
            if kind == "KW_AVOW" {
                let mut su_expr_pos: i32 = su_peek + 1.clone();
                let mut su_expr_r: ParseResult = gen_expr(tokens.clone(), su_expr_pos.clone(), ctx.clone());
                let mut su_expr_code: String = pr_code(su_expr_r.clone());
                let mut su_after_rp: i32 = pr_pos(su_expr_r.clone()) + 1;
                let mut su_avw_sfx: String = ".unwrap();\n".to_string();
                let mut su_avw_parts: Vec<String> = vec![pad.clone(), su_expr_code.clone(), su_avw_sfx.clone()];
                let mut su_avow_code: String = s_join(su_avw_parts.clone());
                let mut su_avow_next: i32 = adv_nl_ref(su_after_rp.clone(), tokens.clone());
                return make_result(su_avow_code, su_avow_next.clone());
            }
        }
        return gen_destructure(pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_USING" {
        let mut su_var_pos: i32 = pos + 1.clone();
        let mut su_var_tok: Token = tokens[su_var_pos as usize].clone();
        let value = su_var_tok.value.clone();
        let mut using_var: String = value.clone();
        let mut su_struct_type: String = reg_get(var_type_reg.clone(), using_var.clone());
        let mut using_type: String = su_struct_type.clone();
        let mut su_init: String = make_destruct_code(using_var.clone(), depth.clone(), ctx.clone());
        let mut su_var_next: i32 = su_var_pos + 1.clone();
        let mut su_body_start: i32 = skip_to_body_ref(tokens.clone(), su_var_next.clone());
/* unhandled(IDENT) */
        let su_uctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone(), tokens: tokens.clone() };
        let mut su_using_ctx: RcCtx = make_rctx(su_uctx_raw);
        let mut su_block_r: ParseResult = gen_block(su_body_start.clone(), depth.clone(), su_using_ctx);
        let mut su_blk_code: String = pr_code(su_block_r.clone());
        let mut su_blk_pos: i32 = pr_pos(su_block_r.clone());
        let mut su_full: String = s_cat(su_init.clone(), su_blk_code.clone());
        return make_result(su_full, su_blk_pos.clone());
    }
    if kind == "KW_RAW" {
        let mut raw_name_pos: i32 = pos + 1.clone();
        let mut raw_name_tok: Token = tokens[raw_name_pos as usize].clone();
        let value = raw_name_tok.value.clone();
        let mut raw_var_name: String = value.clone();
        let mut raw_val_pos: i32 = raw_name_pos + 2.clone();
        let mut raw_val_r: ParseResult = gen_expr(tokens.clone(), raw_val_pos.clone(), ctx.clone());
        let mut raw_val_code: String = pr_code(raw_val_r.clone());
        let mut raw_val_end: i32 = pr_pos(raw_val_r.clone());
        let mut raw_is_mut: bool = list_has(mut_names.clone(), raw_var_name.clone());
        let mut raw_mut_kw: String = "".to_string();
        if raw_is_mut {
            raw_mut_kw = "mut ".to_string();
        }
        let mut raw_let: String = "let ".to_string();
        let mut raw_eq: String = " = ".to_string();
        let mut raw_sc: String = ";\n".to_string();
        let mut raw_parts: Vec<String> = vec![pad.clone(), raw_let.clone(), raw_mut_kw.clone(), raw_var_name.clone(), raw_eq.clone(), raw_val_code.clone(), raw_sc.clone()];
        let mut raw_code: String = s_join(raw_parts.clone());
        let mut raw_next: i32 = adv_nl_ref(raw_val_end.clone(), tokens.clone());
        return make_result(raw_code, raw_next.clone());
    }
    if kind == "KW_CONST" {
        let mut const_type_pos: i32 = pos + 1.clone();
        return gen_typed_binding(const_type_pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_IF" {
        return gen_if(pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_FOR" {
        return gen_for(pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "IDENT" {
        let mut ident_name: String = value.clone();
        let mut next_pos: i32 = pos + 1.clone();
        if next_pos >= token_count {
            let mut eof_code: String = "/* eof */\n".to_string();
            return make_result(eof_code, next_pos.clone());
        }
        let mut next_token: Token = tokens[next_pos as usize].clone();
        let kind = next_token.kind.clone();
        if kind == "KW_AS" {
            return gen_as_binding(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "LPAREN" {
            return gen_call_stmt(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "KW_AT" {
            return gen_list_mutation_stmt(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "KW_REMOVE" {
            return gen_list_mutation_stmt(pos.clone(), depth.clone(), ctx.clone());
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
            return make_result(asgn_code, asgn_next.clone());
        }
        if kind == "IDENT" {
            let mut eq_pos: i32 = next_pos + 1.clone();
            if eq_pos < token_count {
                let mut eq_token: Token = tokens[eq_pos as usize].clone();
                let kind = eq_token.kind.clone();
                if kind == "EQUALS" {
                    return gen_typed_binding(pos.clone(), depth.clone(), ctx.clone());
                }
            }
        }
    }
    let mut unh_pfx: String = "/* unhandled(".to_string();
    let mut unh_sfx: String = ") */\n".to_string();
    let mut unh_parts: Vec<String> = vec![unh_pfx.clone(), kind.clone(), unh_sfx.clone()];
    let mut unhand: String = s_join(unh_parts.clone());
    let mut unhand_next: i32 = pos + 1.clone();
    return make_result(unhand, unhand_next.clone());
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

fn gen_list_shape_code(rust_name: String, rust_elem: String) -> String {
    let mut shp_pfx: String = "type ".to_string();
    let mut shp_mid: String = " = Vec<".to_string();
    let mut shp_sfx: String = ">;\n\n".to_string();
    let mut shp_parts: Vec<String> = vec![shp_pfx.clone(), rust_name.clone(), shp_mid.clone(), rust_elem.clone(), shp_sfx.clone()];
    return s_join(shp_parts.clone());
}

fn gen_func_shape_code(rust_name: String, rust_in: String, rust_out: String) -> String {
    let mut out_suffix: String = "".to_string();
    if !is_empty(rust_out.clone()) {
        let mut ost_pfx: String = " -> ".to_string();
        let mut ost_parts: Vec<String> = vec![ost_pfx.clone(), rust_out.clone()];
        out_suffix = s_join(ost_parts.clone());
    }
    let mut fns_pfx: String = "type ".to_string();
    let mut fns_mid: String = " = fn(".to_string();
    let mut fns_rp: String = ")".to_string();
    let mut fns_sfx: String = ";\n\n".to_string();
    let mut fns_parts: Vec<String> = vec![fns_pfx.clone(), rust_name.clone(), fns_mid.clone(), rust_in.clone(), fns_rp.clone(), out_suffix.clone(), fns_sfx.clone()];
    return s_join(fns_parts.clone());
}

fn gen_shape_decl(tokens: Vec<Token>, pos: i32) -> ParseResult {
    let mut token_count: i32 = (tokens.len() as i32);
    let mut name_pos: i32 = pos + 1.clone();
    let mut form_pos: i32 = pos + 3.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut form_token: Token = tokens[form_pos as usize].clone();
    let value = name_token.value.clone();
    let mut shape_name: String = value.clone();
    let kind = form_token.kind.clone();
    let mut rust_name: String = s_pascal(shape_name.clone());
    if kind == "KW_LIST" {
        let mut elem_pos: i32 = pos + 5.clone();
        let mut elem_token: Token = tokens[elem_pos as usize].clone();
        let value = elem_token.value.clone();
        let mut elem_type: String = value.clone();
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        let mut decl: String = gen_list_shape_code(rust_name.clone(), rust_elem.clone());
        let mut after: i32 = elem_pos + 1.clone();
        let mut shape_next: i32 = adv_nl(after.clone(), tokens.clone());
        return make_result(decl, shape_next.clone());
    }
    let mut t4_pos: i32 = pos + 4.clone();
    let mut t4_token: Token = tokens[t4_pos as usize].clone();
    let kind = t4_token.kind.clone();
    let value = t4_token.value.clone();
    let mut t4_is_of: bool = kind == "KW_OF".clone();
    let mut t4_is_to: bool = value == "to".clone();
    let mut in_type: String = "".to_string();
    let mut out_type: String = "".to_string();
    let mut func_end: i32 = t4_pos.clone();
    if t4_is_of {
        let mut t5_pos: i32 = pos + 5.clone();
        let mut t5_token: Token = tokens[t5_pos as usize].clone();
        let value = t5_token.value.clone();
        in_type = value;
        let mut t6_pos: i32 = pos + 6.clone();
        let mut t6_token: Token = tokens[t6_pos as usize].clone();
        let value = t6_token.value.clone();
        let mut t6_is_to: bool = value == "to".clone();
        func_end = t6_pos;
        if t6_is_to {
            let mut t7_pos: i32 = pos + 7.clone();
            let mut t7_token: Token = tokens[t7_pos as usize].clone();
            let value = t7_token.value.clone();
            out_type = value;
            func_end = t7_pos;
        }
    } else if t4_is_to {
        let mut t5_pos: i32 = pos + 5.clone();
        let mut t5_token: Token = tokens[t5_pos as usize].clone();
        let value = t5_token.value.clone();
        out_type = value;
        func_end = t5_pos;
    }
    let mut rust_in: String = render_rust_type(in_type.clone());
    let mut rust_out: String = render_rust_type(out_type.clone());
    let mut decl: String = gen_func_shape_code(rust_name.clone(), rust_in.clone(), rust_out.clone());
    let mut after: i32 = func_end + 1.clone();
    let mut fn_shape_next: i32 = adv_nl(after.clone(), tokens.clone());
    return make_result(decl, fn_shape_next.clone());
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
    return make_result(type_code, pos.clone());
}

fn gen_fn_decl(fn_tokens: Vec<Token>, pos: i32, ctx: RcCtx) -> ParseResult {
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
    let mut cur: TokenCursor = cur_at(fn_tokens.clone(), start_pos.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut ret_type: String = resolve_type(value.clone(), shape_reg.clone(), enum_reg.clone());
    cur = cur_next(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut fn_name: String = value.clone();
    cur = cur_next(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut param_strs: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "RPAREN" {
            cur = cur_next(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            break;
        } else if kind == "COMMA" {
            cur = cur_next(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else if kind == "IDENT" {
            let mut param_type: String = value.clone();
            cur = cur_next(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut param_name: String = value.clone();
            let mut rust_param_type: String = resolve_type(param_type.clone(), shape_reg.clone(), enum_reg.clone());
            let mut prm_sep: String = ": ".to_string();
            let mut prm_parts: Vec<String> = vec![param_name.clone(), prm_sep.clone(), rust_param_type.clone()];
            param_strs.push(s_join(prm_parts.clone()).clone());
            cur = cur_next(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else {
            cur = cur_next(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        }
    }
    let pos = cur.pos.clone();
    let mut indent_pos: i32 = pos + 1.clone();
    cur = cur_skip_to_body(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut body_end_pos: i32 = find_block_end(fn_tokens.clone(), indent_pos.clone());
    let mut body_start: i32 = pos.clone();
    let mut body_slice_end: i32 = body_end_pos + 1.clone();
    let mut body_tokens_raw: Vec<Token> = l_slice(fn_tokens.clone(), body_start.clone(), body_slice_end.clone());
    let mut body_len: i32 = (body_tokens_raw.len() as i32);
    let mut zero: i32 = 0;
    let mut body_last: i32 = body_len - 1.clone();
    let mut mut_names: Vec<String> = collect_mut_names(body_tokens_raw.clone(), zero.clone(), body_last.clone());
    let mut var_type_reg: Vec<String> = build_var_type_reg(body_tokens_raw.clone());
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
    let mut tokens: TokensRef = tokens_wrap(body_tokens_raw);
/* unhandled(IDENT) */
    let body_ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone(), tokens: tokens.clone() };
    let mut body_ctx: RcCtx = make_rctx(body_ctx_raw);
    let mut body_pos: i32 = 0;
    let mut body_depth: i32 = 1;
    let mut body_r: ParseResult = gen_block(body_pos.clone(), body_depth.clone(), body_ctx);
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
    return make_result(emp, raw_next.clone());
}

fn generate_rust_from_tokens(all_tokens: Vec<Token>) -> String {
    let mut _timer_label: String = "[timer]   registries: ".to_string();
    let mut _timer_start: i32 = now_ms();
    let mut struct_reg: Vec<String> = build_struct_reg(all_tokens.clone());
    let mut shape_reg: Vec<String> = build_shape_reg(all_tokens.clone());
    let mut enum_reg: Vec<String> = build_enum_reg(all_tokens.clone());
    let mut variant_reg: Vec<String> = build_variant_reg(all_tokens.clone(), enum_reg.clone());
    let mut type_reg: Vec<String> = build_type_reg(all_tokens.clone());
    let mut mut_names: Vec<String> = Vec::new();
    let mut var_type_reg: Vec<String> = build_var_type_reg(all_tokens.clone());
    let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
    let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
    let mut _timer_sfx: String = "ms".to_string();
    let mut _timer_parts: Vec<String> = vec![_timer_label.clone(), _timer_str.clone(), _timer_sfx.clone()];
    println!("{}", s_join(_timer_parts.clone()));
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
    let mut tok_placeholder: Vec<Token> = Vec::new();
    let mut tokens: TokensRef = tokens_wrap(tok_placeholder);
/* unhandled(IDENT) */
    let ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone(), tokens: tokens.clone() };
    let mut ctx: RcCtx = make_rctx(ctx_raw);
    let mut parts: Vec<String> = Vec::new();
    let mut token_count: i32 = (all_tokens.len() as i32);
    let mut pos: i32 = 0;
    let mut _timer_label: String = "[timer]   codegen-loop: ".to_string();
    let mut _timer_start: i32 = now_ms();
    while true {
        if pos >= token_count {
            break;
        }
        let mut token: Token = all_tokens[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "EOF" {
            break;
        }
        if kind == "NEWLINE" {
            pos = pos + 1;
            continue;
        }
        if kind == "KW_STRUCT" {
            let mut result: ParseResult = gen_struct_decl(all_tokens.clone(), pos.clone());
            parts.push(pr_code(result.clone()).clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_SHAPE" {
            let mut result: ParseResult = gen_shape_decl(all_tokens.clone(), pos.clone());
            parts.push(pr_code(result.clone()).clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_ENUM" {
            let mut result: ParseResult = gen_enum_decl(all_tokens.clone(), pos.clone());
            parts.push(pr_code(result.clone()).clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_TYPE" {
            let mut result: ParseResult = gen_type_decl(all_tokens.clone(), pos.clone(), ctx.clone());
            parts.push(pr_code(result.clone()).clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_FN" {
            let mut result: ParseResult = gen_fn_decl(all_tokens.clone(), pos.clone(), ctx.clone());
            parts.push(pr_code(result.clone()).clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_RAW" {
            let mut result: ParseResult = gen_raw_decl(all_tokens.clone(), pos.clone());
            parts.push(pr_code(result.clone()).clone());
            pos = pr_pos(result.clone());
            continue;
        }
        if kind == "KW_RUST" {
            let mut block_pos: i32 = pos + 2.clone();
            let mut block_token: Token = all_tokens[block_pos as usize].clone();
            let value = block_token.value.clone();
            let mut newline: String = "\n".to_string();
            let mut rust_chunk: String = s_cat(value.clone(), newline.clone());
            parts.push(rust_chunk.clone());
            pos = block_pos + 1;
            continue;
        }
        pos = pos + 1;
    }
    let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
    let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
    let mut _timer_sfx: String = "ms".to_string();
    let mut _timer_parts: Vec<String> = vec![_timer_label.clone(), _timer_str.clone(), _timer_sfx.clone()];
    println!("{}", s_join(_timer_parts.clone()));
    return s_join(parts.clone());
}

fn main() {
    let mut args: Vec<String> = f_args();
    let mut arg_count: i32 = (args.len() as i32);
    if arg_count < 2 {
        println!("{}", "usage: deor input.deor output.rs".to_string());
    } else {
        let mut input_path: String = args[0 as usize].clone();
        let mut output_path: String = args[1 as usize].clone();
        let mut _timer_label: String = "[timer] load+dedup: ".to_string();
        let mut _timer_start: i32 = now_ms();
        let mut raw_tokens: Vec<Token> = collect_all_tokens_with_all_imports(input_path.clone());
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        let mut _timer_parts: Vec<String> = vec![_timer_label.clone(), _timer_str.clone(), _timer_sfx.clone()];
        println!("{}", s_join(_timer_parts.clone()));
        let mut _timer_label: String = "[timer] macro-expand: ".to_string();
        let mut _timer_start: i32 = now_ms();
        let mut tokens: Vec<Token> = expand_deor_macros(raw_tokens.clone());
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        let mut _timer_parts: Vec<String> = vec![_timer_label.clone(), _timer_str.clone(), _timer_sfx.clone()];
        println!("{}", s_join(_timer_parts.clone()));
        validate_tokens(tokens.clone());
        let mut _timer_label: String = "[timer] total-codegen: ".to_string();
        let mut _timer_start: i32 = now_ms();
        let mut rust_code: String = generate_rust_from_tokens(tokens.clone());
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        let mut _timer_parts: Vec<String> = vec![_timer_label.clone(), _timer_str.clone(), _timer_sfx.clone()];
        println!("{}", s_join(_timer_parts.clone()));
        f_write(output_path.clone(), rust_code.clone());
    }
}

