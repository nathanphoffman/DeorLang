#![allow(warnings)]
// transpiler-deor/types.deor
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
// transpiler-deor/lib/char.deor
fn c_chars(source: String) -> Vec<String> {
    // transpiler-deor/lib/char.deor
    source.chars().map(|c| c.to_string()).collect()
}

fn c_alpha(character: String) -> bool {
    // transpiler-deor/lib/char.deor
    character.chars().next().map(|ch| ch.is_alphabetic() || ch == '_').unwrap_or(false)
}

fn c_digit(character: String) -> bool {
    // transpiler-deor/lib/char.deor
    character.chars().next().map(|ch| ch.is_ascii_digit()).unwrap_or(false)
}

fn c_alnum(character: String) -> bool {
    // transpiler-deor/lib/char.deor
    character.chars().next().map(|ch| ch.is_alphanumeric() || ch == '_').unwrap_or(false)
}

// transpiler-deor/lib/string.deor
fn s_upper_char(ch: String) -> bool {
    // transpiler-deor/lib/string.deor
    ch.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn s_lower_char(ch: String) -> bool {
    // transpiler-deor/lib/string.deor
    ch.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
}

fn s_cat(left: String, right: String) -> String {
    // transpiler-deor/lib/string.deor
    left + right.as_str()
}

fn s_join(parts: Vec<String>) -> String {
    // transpiler-deor/lib/string.deor
    parts.join("")
}

fn s_join_nl(parts: Vec<String>) -> String {
    // transpiler-deor/lib/string.deor
    parts.join("\n")
}

fn s_join_with(parts: Vec<String>, sep: String) -> String {
    // transpiler-deor/lib/string.deor
    parts.join(sep.as_str())
}

fn s_from(source: String, start: i32) -> String {
    // transpiler-deor/lib/string.deor
    source.get(start as usize..).unwrap_or_default().to_string()
}

fn s_rtrim(source: String) -> String {
    // transpiler-deor/lib/string.deor
    source.trim_end().to_string()
}

fn s_trim(source: String) -> String {
    // transpiler-deor/lib/string.deor
    source.trim().to_string()
}

fn s_starts_with(source: String, prefix: String) -> bool {
    // transpiler-deor/lib/string.deor
    source.starts_with(prefix.as_str())
}

fn s_split(source: String, delimiter: String) -> Vec<String> {
    // transpiler-deor/lib/string.deor
    source.split(delimiter.as_str()).map(|s| s.to_string()).collect()
}

fn s_repeat(source: String, count: i32) -> String {
    // transpiler-deor/lib/string.deor
    source.repeat(count as usize)
}

fn s_pascal(source: String) -> String {
    // transpiler-deor/lib/string.deor
    {
    	let mut chars = source.chars();
    	match chars.next() {
    		None => String::new(),
    		Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    	}
    }
}

fn s_debug(source: String) -> String {
    // transpiler-deor/lib/string.deor
    format!("{:?}", source)
}

// transpiler-deor/lib/num.deor
fn n_parse(source: String) -> i32 {
    // transpiler-deor/lib/num.deor
    source.parse::<i32>().unwrap_or(0)
}

fn n_to_str(number: i32) -> String {
    // transpiler-deor/lib/num.deor
    number.to_string()
}

// transpiler-deor/lib/file.deor
fn f_exists(path: String) -> bool {
    // transpiler-deor/lib/file.deor
    std::path::Path::new(path.as_str()).exists()
}

fn f_read(path: String) -> String {
    // transpiler-deor/lib/file.deor
    std::fs::read_to_string(path.as_str())
    	.unwrap_or_else(|e| panic!("cannot read file '{}': {}", path, e))
}

fn f_write(path: String, content: String) {
    // transpiler-deor/lib/file.deor
    std::fs::write(path.as_str(), content.as_str()).expect("cannot write output file");
}

fn f_args() -> Vec<String> {
    // transpiler-deor/lib/file.deor
    std::env::args().skip(1).collect()
}

// transpiler-deor/lib/list.deor
fn l_slice(tokens: Vec<Token>, start: i32, end_val: i32) -> Vec<Token> {
    // transpiler-deor/lib/list.deor
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
}

fn l_slice_ref(tokens: TokensRef, start: i32, end_val: i32) -> Vec<Token> {
    // transpiler-deor/lib/list.deor
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
}

// transpiler-deor/utils.deor
fn is_empty(source: String) -> bool {
    // transpiler-deor/utils.deor
    let mut length: i32 = (source.len() as i32);
    return length == 0;
}

fn str_eq(left: String, right: String) -> bool {
    // transpiler-deor/utils.deor
    return left == right;
}

fn reg_get_stride(pairs: Vec<String>, key: String, stride: i32) -> String {
    // transpiler-deor/utils.deor
    let mut pairs_count: i32 = (pairs.len() as i32);
    let mut index: i32 = 0;
    while index < pairs_count {
        // transpiler-deor/utils.deor
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            // transpiler-deor/utils.deor
            let mut val_index: i32 = index + 1.clone();
            return pairs[val_index as usize].clone();
        }
        index = index + stride;
    }
    return "".to_string();
}

fn reg_has_stride(pairs: Vec<String>, key: String, stride: i32) -> bool {
    // transpiler-deor/utils.deor
    let mut pairs_count: i32 = (pairs.len() as i32);
    let mut index: i32 = 0;
    while index < pairs_count {
        // transpiler-deor/utils.deor
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            // transpiler-deor/utils.deor
            return true;
        }
        index = index + stride;
    }
    return false;
}

fn reg_get(pairs: Vec<String>, key: String) -> String {
    // transpiler-deor/utils.deor
    let mut two: i32 = 2;
    return reg_get_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg_has(pairs: Vec<String>, key: String) -> bool {
    // transpiler-deor/utils.deor
    let mut two: i32 = 2;
    return reg_has_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg3_get(pairs: Vec<String>, key: String) -> String {
    // transpiler-deor/utils.deor
    let mut thr: i32 = 3;
    return reg_get_stride(pairs.clone(), key.clone(), thr.clone());
}

fn reg3_has(pairs: Vec<String>, key: String) -> bool {
    // transpiler-deor/utils.deor
    let mut thr: i32 = 3;
    return reg_has_stride(pairs.clone(), key.clone(), thr.clone());
}

fn list_has(items: Vec<String>, val: String) -> bool {
    // transpiler-deor/utils.deor
    let mut item_count: i32 = (items.len() as i32);
    for index in 0..item_count {
        // transpiler-deor/utils.deor
        let mut item: String = items[index as usize].clone();
        if item == val {
            // transpiler-deor/utils.deor
            return true;
        }
    }
    return false;
}

// float literal context flag
thread_local! {
	static FLOAT_CTX: std::cell::Cell<bool> = std::cell::Cell::new(false);
}
fn _float_ctx_get() -> bool { FLOAT_CTX.with(|f| f.get()) }
fn _float_ctx_set(v: bool) { FLOAT_CTX.with(|f| f.set(v)); }
fn float_ctx_get() -> bool {
    // transpiler-deor/utils.deor
    _float_ctx_get()
}

fn float_ctx_enable() {
    // transpiler-deor/utils.deor
    _float_ctx_set(true)
}

fn float_ctx_disable() {
    // transpiler-deor/utils.deor
    _float_ctx_set(false)
}

// transpiler-deor/deor_helpers.deor
fn pr_code(result: ParseResult) -> String {
    // transpiler-deor/deor_helpers.deor
    result.code
}

fn pr_pos(result: ParseResult) -> i32 {
    // transpiler-deor/deor_helpers.deor
    result.new_pos
}

fn make_result(code: String, new_pos: i32) -> ParseResult {
    // transpiler-deor/deor_helpers.deor
    let result = ParseResult { code: code.clone(), new_pos: new_pos.clone() };
    return result;
}

fn adv_nl(pos: i32, tokens: Vec<Token>) -> i32 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i32 = (tokens.len() as i32);
    if pos < token_count {
        // transpiler-deor/deor_helpers.deor
        let mut cur_token: Token = tokens[pos as usize].clone();
        let kind = cur_token.kind.clone();
        if kind == "NEWLINE" {
            // transpiler-deor/deor_helpers.deor
            return pos + 1;
        }
    }
    return pos;
}

fn adv_indent(pos: i32, tokens: Vec<Token>) -> i32 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i32 = (tokens.len() as i32);
    if pos < token_count {
        // transpiler-deor/deor_helpers.deor
        let mut cur_token: Token = tokens[pos as usize].clone();
        let kind = cur_token.kind.clone();
        if kind == "INDENT" {
            // transpiler-deor/deor_helpers.deor
            return pos + 1;
        }
    }
    return pos;
}

fn skip_to_body(tokens: Vec<Token>, pos: i32) -> i32 {
    // transpiler-deor/deor_helpers.deor
    let mut cur: i32 = adv_nl(pos.clone(), tokens.clone());
    cur = adv_indent(cur.clone(), tokens.clone());
    return cur;
}

fn adv_nl_ref(pos: i32, tokens: TokensRef) -> i32 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i32 = (tokens.len() as i32);
    if pos < token_count {
        // transpiler-deor/deor_helpers.deor
        let mut cur_token: Token = tokens[pos as usize].clone();
        let kind = cur_token.kind.clone();
        if kind == "NEWLINE" {
            // transpiler-deor/deor_helpers.deor
            return pos + 1;
        }
    }
    return pos;
}

fn adv_indent_ref(pos: i32, tokens: TokensRef) -> i32 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i32 = (tokens.len() as i32);
    if pos < token_count {
        // transpiler-deor/deor_helpers.deor
        let mut cur_token: Token = tokens[pos as usize].clone();
        let kind = cur_token.kind.clone();
        if kind == "INDENT" {
            // transpiler-deor/deor_helpers.deor
            return pos + 1;
        }
    }
    return pos;
}

fn skip_to_body_ref(tokens: TokensRef, pos: i32) -> i32 {
    // transpiler-deor/deor_helpers.deor
    let mut cur: i32 = adv_nl_ref(pos.clone(), tokens.clone());
    cur = adv_indent_ref(cur.clone(), tokens.clone());
    return cur;
}

fn make_nl_result(code: String, pos: i32, tokens: TokensRef) -> ParseResult {
    // transpiler-deor/deor_helpers.deor
    let mut next_pos: i32 = adv_nl_ref(pos.clone(), tokens.clone());
    return make_result(code, next_pos.clone());
}

// transpiler-deor/importer/lexer/token_factory.deor
fn make_meta(line: i32, file: String) -> TokenMeta {
    // transpiler-deor/importer/lexer/token_factory.deor
    let meta = TokenMeta { line: line.clone(), file: file.clone() };
    return meta;
}

fn make_token(kind: String, value: String, meta: TokenMeta) -> Token {
    // transpiler-deor/importer/lexer/token_factory.deor
    let line = meta.line.clone();
    let file = meta.file.clone();
    let token = Token { kind: kind.clone(), value: value.clone(), line: line.clone(), file: file.clone() };
    return token;
}

// transpiler-deor/importer/lexer/indent.deor
fn count_tabs(line: String) -> i32 {
    // transpiler-deor/importer/lexer/indent.deor
    let mut space: String = " ".to_string();
    let mut chars: Vec<String> = c_chars(line.clone());
    let mut char_count: i32 = (chars.len() as i32);
    let mut count: i32 = 0;
    let mut space_run = 0;
    for index in 0..char_count {
        // transpiler-deor/importer/lexer/indent.deor
        let mut character: String = chars[index as usize].clone();
        if character == "\t" {
            // transpiler-deor/importer/lexer/indent.deor
            count = count + 1;
        } else if character == space {
            // transpiler-deor/importer/lexer/indent.deor
            space_run = space_run + 1;
            if space_run == 4 {
                // transpiler-deor/importer/lexer/indent.deor
                count = count + 1;
                space_run = 0;
            }
        } else {
            // transpiler-deor/importer/lexer/indent.deor
            break;
        }
    }
    return count;
}

// transpiler-deor/importer/lexer/string_literal.deor
fn scan_string_literal(chars: Vec<String>, char_index: i32, char_count: i32) -> ParseResult {
    // transpiler-deor/importer/lexer/string_literal.deor
    let mut val: String = "".to_string();
    let mut escape_next: bool = false;
    let mut str_start: i32 = char_index + 1.clone();
    let mut new_pos: i32 = char_index + 1.clone();
    let mut ch_nl: String = "\n".to_string();
    let mut ch_tab: String = "\t".to_string();
    let mut ch_bs: String = "\\".to_string();
    let mut ch_qt: String = "\"".to_string();
    for string_index in str_start..char_count {
        // transpiler-deor/importer/lexer/string_literal.deor
        let mut string_char: String = chars[string_index as usize].clone();
        if escape_next {
            // transpiler-deor/importer/lexer/string_literal.deor
            if string_char == "n" {
                // transpiler-deor/importer/lexer/string_literal.deor
                val = s_cat(val.clone(), ch_nl.clone());
            } else if string_char == "t" {
                // transpiler-deor/importer/lexer/string_literal.deor
                val = s_cat(val.clone(), ch_tab.clone());
            } else if string_char == "\\" {
                // transpiler-deor/importer/lexer/string_literal.deor
                val = s_cat(val.clone(), ch_bs.clone());
            } else if string_char == "\"" {
                // transpiler-deor/importer/lexer/string_literal.deor
                val = s_cat(val.clone(), ch_qt.clone());
            } else {
                // transpiler-deor/importer/lexer/string_literal.deor
                val = s_cat(val.clone(), ch_bs.clone());
                val = s_cat(val.clone(), string_char.clone());
            }
            escape_next = false;
            new_pos = string_index + 1;
        } else if string_char == ch_bs {
            // transpiler-deor/importer/lexer/string_literal.deor
            escape_next = true;
            new_pos = string_index + 1;
        } else if string_char == ch_qt {
            // transpiler-deor/importer/lexer/string_literal.deor
            new_pos = string_index + 1;
            break;
        } else {
            // transpiler-deor/importer/lexer/string_literal.deor
            val = s_cat(val.clone(), string_char.clone());
            new_pos = string_index + 1;
        }
    }
    return make_result(val.clone(), new_pos.clone());
}

// transpiler-deor/importer/lexer/number_literal.deor
fn scan_number(chars: Vec<String>, char_index: i32, char_count: i32) -> ParseResult {
    // transpiler-deor/importer/lexer/number_literal.deor
    let mut first_char: String = chars[char_index as usize].clone();
    let mut empty_str: String = "".to_string();
    let mut num: String = s_cat(empty_str.clone(), first_char.clone());
    let mut num_start: i32 = char_index + 1.clone();
    let mut new_pos: i32 = char_index + 1.clone();
    for number_index in num_start..char_count {
        // transpiler-deor/importer/lexer/number_literal.deor
        let mut number_char: String = chars[number_index as usize].clone();
        if c_digit(number_char.clone()) {
            // transpiler-deor/importer/lexer/number_literal.deor
            num = s_cat(num.clone(), number_char.clone());
            new_pos = number_index + 1;
        } else if number_char == "_" {
            // transpiler-deor/importer/lexer/number_literal.deor
            let mut peek_idx: i32 = number_index + 1.clone();
            if peek_idx < char_count {
                // transpiler-deor/importer/lexer/number_literal.deor
                let mut peek_char: String = chars[peek_idx as usize].clone();
                if c_digit(peek_char.clone()) {
                    // transpiler-deor/importer/lexer/number_literal.deor
                    new_pos = number_index + 1;
                } else {
                    // transpiler-deor/importer/lexer/number_literal.deor
                    break;
                }
            } else {
                // transpiler-deor/importer/lexer/number_literal.deor
                break;
            }
        } else {
            // transpiler-deor/importer/lexer/number_literal.deor
            break;
        }
    }
    if new_pos < char_count {
        // transpiler-deor/importer/lexer/number_literal.deor
        let mut dot_char: String = chars[new_pos as usize].clone();
        let mut frac_start: i32 = new_pos + 1.clone();
        if dot_char == "." && frac_start < char_count {
            // transpiler-deor/importer/lexer/number_literal.deor
            let mut frac_first: String = chars[frac_start as usize].clone();
            if c_digit(frac_first.clone()) {
                // transpiler-deor/importer/lexer/number_literal.deor
                let mut dot_str: String = ".".to_string();
                num = s_cat(num.clone(), dot_str.clone());
                new_pos = frac_start;
                for frac_index in frac_start..char_count {
                    // transpiler-deor/importer/lexer/number_literal.deor
                    let mut frac_char: String = chars[frac_index as usize].clone();
                    if c_digit(frac_char.clone()) {
                        // transpiler-deor/importer/lexer/number_literal.deor
                        num = s_cat(num.clone(), frac_char.clone());
                        new_pos = frac_index + 1;
                    } else {
                        // transpiler-deor/importer/lexer/number_literal.deor
                        break;
                    }
                }
            }
        }
    }
    return make_result(num.clone(), new_pos.clone());
}

// transpiler-deor/importer/lexer/word_token.deor
fn word_to_kind(word: String) -> String {
    // transpiler-deor/importer/lexer/word_token.deor
    if word == "fn" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_FN".to_string();
    }
    if word == "as" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_AS".to_string();
    }
    if word == "return" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_RETURN".to_string();
    }
    if word == "if" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_IF".to_string();
    }
    if word == "else" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_ELSE".to_string();
    }
    if word == "for" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_FOR".to_string();
    }
    if word == "in" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_IN".to_string();
    }
    if word == "break" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_BREAK".to_string();
    }
    if word == "continue" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_CONTINUE".to_string();
    }
    if word == "and" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_AND".to_string();
    }
    if word == "or" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_OR".to_string();
    }
    if word == "not" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_NOT".to_string();
    }
    if word == "is" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_IS".to_string();
    }
    if word == "true" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_TRUE".to_string();
    }
    if word == "false" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_FALSE".to_string();
    }
    if word == "valid" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_VALID".to_string();
    }
    if word == "avow" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_AVOW".to_string();
    }
    if word == "empty" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_EMPTY".to_string();
    }
    if word == "type" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_TYPE".to_string();
    }
    if word == "struct" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_STRUCT".to_string();
    }
    if word == "shape" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_SHAPE".to_string();
    }
    if word == "list" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_LIST".to_string();
    }
    if word == "of" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_OF".to_string();
    }
    if word == "enum" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_ENUM".to_string();
    }
    if word == "at" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_AT".to_string();
    }
    if word == "remove" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_REMOVE".to_string();
    }
    if word == "rust" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_RUST".to_string();
    }
    if word == "void" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_VOID".to_string();
    }
    if word == "using" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_USING".to_string();
    }
    if word == "with" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_WITH".to_string();
    }
    if word == "move" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_GIVEUP".to_string();
    }
    if word == "raw" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_RAW".to_string();
    }
    if word == "macro" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_MACRO".to_string();
    }
    if word == "macro_run" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_MACRO_RUN".to_string();
    }
    if word == "import" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_IMPORT".to_string();
    }
    if word == "block" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_BLOCK".to_string();
    }
    if word == "const" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_CONST".to_string();
    }
    return "IDENT".to_string();
}

fn scan_word(chars: Vec<String>, char_index: i32, char_count: i32) -> ParseResult {
    // transpiler-deor/importer/lexer/word_token.deor
    let mut first_char: String = chars[char_index as usize].clone();
    let mut empty_str: String = "".to_string();
    let mut word: String = s_cat(empty_str.clone(), first_char.clone());
    let mut word_start: i32 = char_index + 1.clone();
    let mut new_pos: i32 = char_index + 1.clone();
    for word_index in word_start..char_count {
        // transpiler-deor/importer/lexer/word_token.deor
        let mut word_char: String = chars[word_index as usize].clone();
        if c_alnum(word_char.clone()) {
            // transpiler-deor/importer/lexer/word_token.deor
            word = s_cat(word.clone(), word_char.clone());
            new_pos = word_index + 1;
        } else {
            // transpiler-deor/importer/lexer/word_token.deor
            break;
        }
    }
    return make_result(word.clone(), new_pos.clone());
}

// transpiler-deor/importer/lexer/tokenizer.deor
fn tokenize(source: String, path: String) -> Vec<Token> {
    // transpiler-deor/importer/lexer/tokenizer.deor
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
        // transpiler-deor/importer/lexer/tokenizer.deor
        cur_line = cur_line + 1;
        let mut meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
        if skip > 0 {
            // transpiler-deor/importer/lexer/tokenizer.deor
            skip = skip - 1;
            continue;
        }
        let mut raw_line: String = lines[raw_li as usize].clone();
        let mut line: String = s_rtrim(raw_line.clone());
        let mut content: String = s_trim(line.clone());
        if is_empty(content.clone()) {
            // transpiler-deor/importer/lexer/tokenizer.deor
            continue;
        }
        let mut indent: i32 = count_tabs(line.clone());
        // macro: emit_indent_or_dedent (transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor)
        let mut iod_kind_indent: String = "INDENT".to_string();
        let mut iod_kind_dedent: String = "DEDENT".to_string();
        let mut iod_empty: String = "".to_string();
        let mut slen: i32 = (indent_stack.len() as i32);
        let mut top_idx: i32 = slen - 1.clone();
        let mut top: i32 = n_parse(indent_stack[top_idx as usize].clone());
        if indent > top {
            // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
            tokens.push(make_token(iod_kind_indent.clone(), iod_empty.clone(), meta.clone()).clone());
            let mut indent_str: String = n_to_str(indent.clone());
            indent_stack.push(indent_str.clone());
        } else {
            // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
            let mut dedenting: bool = indent < top.clone();
            while dedenting {
                // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                let mut new_slen: i32 = (indent_stack.len() as i32);
                let mut new_top_idx: i32 = new_slen - 1.clone();
                let mut cur_top: i32 = n_parse(indent_stack[new_top_idx as usize].clone());
                if indent < cur_top {
                    // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                    tokens.push(make_token(iod_kind_dedent.clone(), iod_empty.clone(), meta.clone()).clone());
                    indent_stack.remove(new_top_idx as usize);
                } else {
                    // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                    dedenting = false;
                }
            }
        }
        // transpiler-deor/importer/lexer/tokenizer.deor
        if content == "rust" {
            // macro: collect_rust_block (transpiler-deor/importer/lexer/macros/collect_rust_block.deor)
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
                // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                let mut rust_line: String = lines[rli as usize].clone();
                let mut rl_indent: i32 = count_tabs(rust_line.clone());
                let mut rl_stripped: String = s_trim(rust_line.clone());
                if is_empty(rl_stripped.clone()) {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    rust_lines.push("".to_string());
                    skip = skip + 1;
                } else if rl_indent >= rust_base {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    let mut rl_content: String = s_from(rust_line.clone(), rust_base.clone());
                    rust_lines.push(s_rtrim(rl_content.clone()).clone());
                    skip = skip + 1;
                } else {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    break;
                }
            }
            let mut trimming: bool = true;
            while trimming {
                // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                let mut rl_len: i32 = (rust_lines.len() as i32);
                if rl_len > 0 {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    let mut last_rl: i32 = rl_len - 1.clone();
                    let mut last_line: String = rust_lines[last_rl as usize].clone();
                    if last_line == "" {
                        // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                        rust_lines.remove(last_rl as usize);
                    } else {
                        // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                        trimming = false;
                    }
                } else {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    trimming = false;
                }
            }
            let mut block_content: String = s_join_nl(rust_lines.clone());
            tokens.push(make_token(rb_kind_rust_block.clone(), block_content.clone(), meta.clone()).clone());
            // transpiler-deor/importer/lexer/tokenizer.deor
            continue;
        }
        let mut chars: Vec<String> = c_chars(content.clone());
        let mut char_count: i32 = (chars.len() as i32);
        let mut char_index: i32 = 0;
        while char_index < char_count {
            // transpiler-deor/importer/lexer/tokenizer.deor
            let mut character: String = chars[char_index as usize].clone();
            if character == " " {
                // transpiler-deor/importer/lexer/tokenizer.deor
                char_index = char_index + 1;
                continue;
            }
            if character == "#" {
                // transpiler-deor/importer/lexer/tokenizer.deor
                break;
            }
            if character == "\"" {
                // transpiler-deor/importer/lexer/tokenizer.deor
                let mut str_r: ParseResult = scan_string_literal(chars.clone(), char_index.clone(), char_count.clone());
                let mut kind_string: String = "STRING".to_string();
                let mut str_val: String = pr_code(str_r.clone());
                tokens.push(make_token(kind_string.clone(), str_val.clone(), meta.clone()).clone());
                char_index = pr_pos(str_r.clone());
                continue;
            }
            if c_digit(character.clone()) {
                // transpiler-deor/importer/lexer/tokenizer.deor
                let mut num_r: ParseResult = scan_number(chars.clone(), char_index.clone(), char_count.clone());
                let mut num_str: String = pr_code(num_r.clone());
                char_index = pr_pos(num_r.clone());
                let mut dot: String = ".".to_string();
                let mut num_parts: Vec<String> = s_split(num_str.clone(), dot.clone());
                let mut is_float: bool = (num_parts.len() as i32) > 1;
                if is_float {
                    // transpiler-deor/importer/lexer/tokenizer.deor
                    let mut kind_float: String = "FLOAT".to_string();
                    tokens.push(make_token(kind_float.clone(), num_str.clone(), meta.clone()).clone());
                } else {
                    // transpiler-deor/importer/lexer/tokenizer.deor
                    let mut kind_int: String = "INT".to_string();
                    tokens.push(make_token(kind_int.clone(), num_str.clone(), meta.clone()).clone());
                }
                continue;
            }
            if c_alpha(character.clone()) {
                // transpiler-deor/importer/lexer/tokenizer.deor
                let mut word_r: ParseResult = scan_word(chars.clone(), char_index.clone(), char_count.clone());
                let mut word: String = pr_code(word_r.clone());
                char_index = pr_pos(word_r.clone());
                let mut word_kind: String = word_to_kind(word.clone());
                tokens.push(make_token(word_kind.clone(), word.clone(), meta.clone()).clone());
                continue;
            }
            // macro: emit_operator_token (transpiler-deor/importer/lexer/macros/emit_operator_token.deor)
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
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                op_peek = chars[op_peek_idx as usize].clone();
            }
            if character == ">" && op_peek == "=" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_gte.clone(), op_val_gte.clone(), meta.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "<" && op_peek == "=" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_lte.clone(), op_val_lte.clone(), meta.clone()).clone());
                char_index = char_index + 2;
                continue;
            }
            if character == "+" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_plus.clone(), op_val_plus.clone(), meta.clone()).clone());
            } else if character == "-" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_minus.clone(), op_val_minus.clone(), meta.clone()).clone());
            } else if character == "*" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_star.clone(), op_val_star.clone(), meta.clone()).clone());
            } else if character == "/" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_slash.clone(), op_val_slash.clone(), meta.clone()).clone());
            } else if character == "%" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_pct.clone(), op_val_pct.clone(), meta.clone()).clone());
            } else if character == "=" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_eq.clone(), op_val_eq.clone(), meta.clone()).clone());
            } else if character == ">" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_gt.clone(), op_val_gt.clone(), meta.clone()).clone());
            } else if character == "<" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_lt.clone(), op_val_lt.clone(), meta.clone()).clone());
            } else if character == "(" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_lp.clone(), op_val_lp.clone(), meta.clone()).clone());
            } else if character == ")" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_rp.clone(), op_val_rp.clone(), meta.clone()).clone());
            } else if character == "[" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_lb.clone(), op_val_lb.clone(), meta.clone()).clone());
            } else if character == "]" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_rb.clone(), op_val_rb.clone(), meta.clone()).clone());
            } else if character == "," {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                tokens.push(make_token(op_kind_cm.clone(), op_val_cm.clone(), meta.clone()).clone());
            }
            char_index = char_index + 1;
        }
        tokens.push(make_token(kind_newline.clone(), empty_str.clone(), meta.clone()).clone());
    }
    let mut final_stack_len: i32 = (indent_stack.len() as i32);
    let mut tail_meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
    for _ in 1..final_stack_len {
        // transpiler-deor/importer/lexer/tokenizer.deor
        tokens.push(make_token(kind_dedent.clone(), empty_str.clone(), tail_meta.clone()).clone());
    }
    tokens.push(make_token(kind_eof.clone(), empty_str.clone(), tail_meta.clone()).clone());
    return tokens;
}

// transpiler-deor/importer/scan.deor
fn scan_import_new(tokens: Vec<Token>, pos: i32) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    let mut path_pos: i32 = pos + 1.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    if path_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut path_tok: Token = tokens[path_pos as usize].clone();
        let kind = path_tok.kind.clone();
        let value = path_tok.value.clone();
        if kind == "STRING" {
            // transpiler-deor/importer/scan.deor
            let mut after_path: i32 = path_pos + 1.clone();
            return make_result(value.clone(), after_path.clone());
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), pos.clone());
}

fn scan_import_where(tokens: Vec<Token>, pos: i32) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut where_pos: i32 = pos.clone();
    let mut eq_pos: i32 = pos + 2.clone();
    let mut concrete_pos: i32 = pos + 3.clone();
    if concrete_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut where_tok: Token = tokens[where_pos as usize].clone();
        let mut eq_tok: Token = tokens[eq_pos as usize].clone();
        let mut concrete_tok: Token = tokens[concrete_pos as usize].clone();
        let kind = where_tok.kind.clone();
        let value = where_tok.value.clone();
        let mut is_where: bool = kind == "IDENT" && value == "where".clone();
        if is_where {
            // transpiler-deor/importer/scan.deor
            let kind = eq_tok.kind.clone();
            let mut is_eq: bool = kind == "EQUALS".clone();
            if is_eq {
                // transpiler-deor/importer/scan.deor
                let value = concrete_tok.value.clone();
                let mut after_where: i32 = concrete_pos + 1.clone();
                return make_result(value.clone(), after_where.clone());
            }
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), pos.clone());
}

fn scan_import(tokens: Vec<Token>, pos: i32) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut scan: i32 = pos + 1.clone();
    while scan < token_count {
        // transpiler-deor/importer/scan.deor
        let mut scan_tok: Token = tokens[scan as usize].clone();
        let kind = scan_tok.kind.clone();
        scan = scan + 1;
        if kind == "RPAREN" {
            // transpiler-deor/importer/scan.deor
            break;
        }
    }
    let mut path_pos: i32 = scan + 1.clone();
    if path_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut in_tok: Token = tokens[scan as usize].clone();
        let kind = in_tok.kind.clone();
        let mut is_in: bool = kind == "KW_IN".clone();
        if is_in {
            // transpiler-deor/importer/scan.deor
            let mut path_tok: Token = tokens[path_pos as usize].clone();
            let value = path_tok.value.clone();
            let mut after_path: i32 = path_pos + 1.clone();
            return make_result(value.clone(), after_path.clone());
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), pos.clone());
}

// transpiler-deor/importer/t_substitute.deor
fn s_to_lower(source: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    source.to_lowercase()
}

fn apply_t_in_name(name: String, concrete: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    if name == "T" {
        // transpiler-deor/importer/t_substitute.deor
        return concrete;
    }
    let mut name_chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (name_chars.len() as i32);
    if name_len > 1 {
        // transpiler-deor/importer/t_substitute.deor
        let mut first: String = name_chars[0 as usize].clone();
        let mut second: String = name_chars[1 as usize].clone();
        let mut second_is_upper: bool = s_upper_char(second.clone());
        let mut first_is_upper_t: bool = first == "T".clone();
        let mut first_is_lower_t: bool = first == "t".clone();
        if first_is_upper_t && second_is_upper {
            // transpiler-deor/importer/t_substitute.deor
            let mut t_offset: i32 = 1;
            let mut rest: String = s_from(name.clone(), t_offset.clone());
            let mut pascal_concrete: String = s_pascal(concrete.clone());
            return s_cat(pascal_concrete.clone(), rest.clone());
        }
        if first_is_lower_t && second_is_upper {
            // transpiler-deor/importer/t_substitute.deor
            let mut t_offset: i32 = 1;
            let mut rest: String = s_from(name.clone(), t_offset.clone());
            let mut lower_concrete: String = s_to_lower(concrete.clone());
            return s_cat(lower_concrete.clone(), rest.clone());
        }
    }
    let mut t_sep: String = "_T_".to_string();
    let mut mid_parts: Vec<String> = s_split(name.clone(), t_sep.clone());
    let mut mid_count: i32 = (mid_parts.len() as i32);
    if mid_count > 1 {
        // transpiler-deor/importer/t_substitute.deor
        let mut lower: String = s_to_lower(concrete.clone());
        let mut new_sep: String = ["_", lower.as_str(), "_"].concat();
        return s_join_with(mid_parts.clone(), new_sep.clone());
    }
    return name;
}

fn replace_t_in_rust_block(content: String, concrete: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    {
    	fn sub_word(word: &str, concrete: &str) -> String {
    		if word == "T" {
    			return concrete.to_string();
    		}
    		let chars: Vec<char> = word.chars().collect();
    		let n = chars.len();
    		if n > 1 {
    			if chars[0] == 'T' && chars[1].is_uppercase() {
    				let rest: String = chars[1..].iter().collect();
    				let mut pascal = concrete.to_string();
    				if let Some(c) = pascal.chars().next() {
    					if c.is_lowercase() {
    						let upper: String = c.to_uppercase().to_string();
    						pascal = format!("{}{}", upper, &pascal[c.len_utf8()..]);
    					}
    				}
    				return format!("{}{}", pascal, rest);
    			}
    			if chars[0] == 't' && chars[1].is_uppercase() {
    				let rest: String = chars[1..].iter().collect();
    				let lower = concrete.to_lowercase();
    				return format!("{}{}", lower, rest);
    			}
    		}
    		if word.contains("_T_") {
    			let lower = concrete.to_lowercase();
    			let new_sep = format!("_{}_", lower);
    			return word.replace("_T_", &new_sep);
    		}
    		word.to_string()
    	}
    	let mut result = String::new();
    	let chars: Vec<char> = content.chars().collect();
    	let n = chars.len();
    	let mut i = 0;
    	while i < n {
    		if chars[i].is_alphanumeric() || chars[i] == '_' {
    			let start = i;
    			while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
    				i += 1;
    			}
    			let word: String = chars[start..i].iter().collect();
    			result.push_str(&sub_word(&word, concrete.as_str()));
    		} else {
    			result.push(chars[i]);
    			i += 1;
    		}
    	}
    	result
    }
}

fn apply_t_substitution(tokens: Vec<Token>, concrete: String) -> Vec<Token> {
    // transpiler-deor/importer/t_substitute.deor
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    for index in 0..token_count {
        // transpiler-deor/importer/t_substitute.deor
        let mut tok: Token = tokens[index as usize].clone();
        let kind = tok.kind.clone();
        let value = tok.value.clone();
        let line = tok.line.clone();
        let file = tok.file.clone();
        if kind == "IDENT" {
            // transpiler-deor/importer/t_substitute.deor
            let mut new_value: String = apply_t_in_name(value.clone(), concrete.clone());
/* unhandled(IDENT) */
            let tok_meta = TokenMeta { line: line.clone(), file: file.clone() };
            let mut new_tok: Token = make_token(kind.clone(), new_value.clone(), tok_meta.clone());
            result.push(new_tok.clone());
        } else if kind == "RUST_BLOCK" {
            // transpiler-deor/importer/t_substitute.deor
            let mut new_content: String = replace_t_in_rust_block(value.clone(), concrete.clone());
/* unhandled(IDENT) */
            let tok_meta = TokenMeta { line: line.clone(), file: file.clone() };
            let mut new_tok: Token = make_token(kind.clone(), new_content.clone(), tok_meta.clone());
            result.push(new_tok.clone());
        } else {
            // transpiler-deor/importer/t_substitute.deor
            result.push(tok.clone());
        }
    }
    return result;
}

// transpiler-deor/importer/decl_bounds.deor
fn name_of_decl(tokens: Vec<Token>, pos: i32, is_fn: bool) -> String {
    // transpiler-deor/importer/decl_bounds.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut name_offset: i32 = 1;
    if is_fn {
        // transpiler-deor/importer/decl_bounds.deor
        name_offset = 2;
    }
    let mut name_pos: i32 = pos + name_offset.clone();
    if name_pos < token_count {
        // transpiler-deor/importer/decl_bounds.deor
        let mut name_tok: Token = tokens[name_pos as usize].clone();
        let value = name_tok.value.clone();
        return value;
    }
    return "".to_string();
}

fn end_of_block(tokens: Vec<Token>, pos: i32) -> i32 {
    // transpiler-deor/importer/decl_bounds.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = pos.clone();
    let mut depth: i32 = 0;
    let mut entered: bool = false;
    while cur < token_count {
        // transpiler-deor/importer/decl_bounds.deor
        let mut tok: Token = tokens[cur as usize].clone();
        let kind = tok.kind.clone();
        cur = cur + 1;
        if kind == "INDENT" {
            // transpiler-deor/importer/decl_bounds.deor
            depth = depth + 1;
            entered = true;
        } else if kind == "DEDENT" {
            // transpiler-deor/importer/decl_bounds.deor
            depth = depth - 1;
            if depth == 0 && entered {
                // transpiler-deor/importer/decl_bounds.deor
                break;
            }
        }
    }
    return cur;
}

fn end_of_shape(tokens: Vec<Token>, pos: i32) -> i32 {
    // transpiler-deor/importer/decl_bounds.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = pos.clone();
    while cur < token_count {
        // transpiler-deor/importer/decl_bounds.deor
        let mut tok: Token = tokens[cur as usize].clone();
        let kind = tok.kind.clone();
        cur = cur + 1;
        if kind == "NEWLINE" {
            // transpiler-deor/importer/decl_bounds.deor
            break;
        }
    }
    return cur;
}

// transpiler-deor/importer/load.deor
thread_local! {
	static INCLUDED_FILES: std::cell::RefCell<std::collections::HashSet<String>> = std::cell::RefCell::new(std::collections::HashSet::new());
}
fn file_is_new(path: String) -> bool {
	INCLUDED_FILES.with(|set| {
		let mut s = set.borrow_mut();
		if s.contains(&path) { false } else { s.insert(path); true }
	})
}
fn file_is_new_keyed(key: String) -> bool {
	INCLUDED_FILES.with(|set| {
		let mut s = set.borrow_mut();
		if s.contains(&key) { false } else { s.insert(key); true }
	})
}
fn resolve_lib_path(path: String) -> String {
	if path.starts_with("lib/") {
		if let Ok(lib) = std::env::var("DEOR_LIB") {
			return format!("{}/{}", lib.trim_end_matches('/'), &path[4..]);
		}
	}
	path
}
fn load_file(path: String) -> Vec<Token> {
    // transpiler-deor/importer/load.deor
    let mut source: String = f_read(path.clone());
    let mut tok_raw: Vec<Token> = tokenize(source.clone(), path.clone());
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i32 = (tok_raw.len() as i32);
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut seen_decl: bool = false;
    while true {
        // transpiler-deor/importer/load.deor
        let is_at_end_of_file = pos >= token_count;
        if is_at_end_of_file {
            // transpiler-deor/importer/load.deor
            break;
        }
        let mut tok: Token = tok_raw[pos as usize].clone();
        let kind = tok.kind.clone();
        if kind == "EOF" {
            // transpiler-deor/importer/load.deor
            break;
        }
        if kind == "INDENT" {
            // transpiler-deor/importer/load.deor
            depth = depth + 1;
            result.push(tok.clone());
            pos = pos + 1;
            continue;
        }
        if kind == "DEDENT" {
            // transpiler-deor/importer/load.deor
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
            // transpiler-deor/importer/load.deor
            let mut imp_r_new: ParseResult = scan_import_new(tok_raw.clone(), pos.clone());
            let mut imp_r_old: ParseResult = scan_import(tok_raw.clone(), pos.clone());
            let mut imp_path: String = "".to_string();
            let mut imp_end: i32 = pos.clone();
            let mut imp_t_concrete: String = "".to_string();
            if is_new_import {
                // transpiler-deor/importer/load.deor
                imp_path = pr_code(imp_r_new.clone());
                imp_end = pr_pos(imp_r_new.clone());
                let mut where_r: ParseResult = scan_import_where(tok_raw.clone(), imp_end.clone());
                imp_t_concrete = pr_code(where_r.clone());
                if !is_empty(imp_t_concrete.clone()) {
                    // transpiler-deor/importer/load.deor
                    imp_end = pr_pos(where_r.clone());
                }
            } else {
                // transpiler-deor/importer/load.deor
                imp_path = pr_code(imp_r_old.clone());
                imp_end = pr_pos(imp_r_old.clone());
            }
            imp_path = resolve_lib_path(imp_path.clone());
            if !is_empty(imp_path.clone()) {
                // transpiler-deor/importer/load.deor
                if seen_decl {
                    // transpiler-deor/importer/load.deor
                    let mut err_pre: String = "[error] ".to_string();
                    let mut err_mid: String = ": imports must appear at the top of the file before any declarations".to_string();
                    let mut err_parts: Vec<String> = vec![err_pre.clone(), path.clone(), err_mid.clone()];
                    let mut err_msg: String = s_join(err_parts.clone());
                    println!("{}", err_msg.clone());
                    std::process::exit(1);
                }
                let mut dedup_key: String = imp_path.clone();
                if !is_empty(imp_t_concrete.clone()) {
                    // transpiler-deor/importer/load.deor
                    dedup_key = [imp_path.as_str(), "|T=", imp_t_concrete.as_str()].concat();
                }
                let mut is_new: bool = file_is_new_keyed(dedup_key.clone());
                if is_new {
                    // transpiler-deor/importer/load.deor
                    let mut exists: bool = f_exists(imp_path.clone());
                    if !exists {
                        // transpiler-deor/importer/load.deor
                        let mut err_pre: String = "[error] cannot find import: ".to_string();
                        let mut err_msg: String = s_cat(err_pre.clone(), imp_path.clone());
                        println!("{}", err_msg.clone());
                        std::process::exit(1);
                    }
                    let mut imp_tokens: Vec<Token> = load_file(imp_path.clone());
                    if !is_empty(imp_t_concrete.clone()) {
                        // transpiler-deor/importer/load.deor
                        imp_tokens = apply_t_substitution(imp_tokens.clone(), imp_t_concrete.clone());
                    }
                    let mut imp_len: i32 = (imp_tokens.len() as i32);
                    for imp_index in 0..imp_len {
                        // transpiler-deor/importer/load.deor
                        let mut imp_tok: Token = imp_tokens[imp_index as usize].clone();
                        let kind = imp_tok.kind.clone();
                        let mut imp_is_eof: bool = kind == "EOF".clone();
                        if !imp_is_eof {
                            // transpiler-deor/importer/load.deor
                            result.push(imp_tok.clone());
                        }
                    }
                }
                pos = imp_end;
                continue;
            }
        }
        if at_root_depth && kind != "NEWLINE" {
            // transpiler-deor/importer/load.deor
            seen_decl = true;
        }
        result.push(tok.clone());
        pos = pos + 1;
    }
    return result;
}

// transpiler-deor/importer/dedup.deor
fn deduplicate_decls(tokens: Vec<Token>) -> Vec<Token> {
    // transpiler-deor/importer/dedup.deor
    let mut result: Vec<Token> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pos: i32 = 0;
    while pos < token_count {
        // transpiler-deor/importer/dedup.deor
        let mut token: Token = tokens[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "EOF" {
            // transpiler-deor/importer/dedup.deor
            result.push(token.clone());
            pos = pos + 1;
            continue;
        }
        if kind == "NEWLINE" {
            // transpiler-deor/importer/dedup.deor
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
            // macro: get_decl_name (transpiler-deor/importer/macros/get_decl_name.deor)
            let mut dn_offset: i32 = 1;
            if is_fn {
                // transpiler-deor/importer/macros/get_decl_name.deor
                dn_offset = 2;
            }
            let mut dn_pos: i32 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                // transpiler-deor/importer/macros/get_decl_name.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            // macro: check_seen (transpiler-deor/importer/macros/check_seen.deor)
            let mut already_seen: bool = false;
            let mut cs_len: i32 = (seen.len() as i32);
            for cs_i in 0..cs_len {
                // transpiler-deor/importer/macros/check_seen.deor
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    // transpiler-deor/importer/macros/check_seen.deor
                    already_seen = true;
                    break;
                }
            }
            // transpiler-deor/importer/dedup.deor
            if !already_seen {
                // transpiler-deor/importer/dedup.deor
                seen.push(decl_name.clone());
            }
            // macro: dd_find_block_end (transpiler-deor/importer/macros/dd_find_block_end.deor)
            let mut fbe_cur: i32 = pos.clone();
            let mut fbe_depth: i32 = 0;
            let mut fbe_entered: bool = false;
            while fbe_cur < token_count {
                // transpiler-deor/importer/macros/dd_find_block_end.deor
                let mut fbe_tok: Token = tokens[fbe_cur as usize].clone();
                let kind = fbe_tok.kind.clone();
                fbe_cur = fbe_cur + 1;
                if kind == "INDENT" {
                    // transpiler-deor/importer/macros/dd_find_block_end.deor
                    fbe_depth = fbe_depth + 1;
                    fbe_entered = true;
                } else if kind == "DEDENT" {
                    // transpiler-deor/importer/macros/dd_find_block_end.deor
                    fbe_depth = fbe_depth - 1;
                    if fbe_depth == 0 && fbe_entered {
                        // transpiler-deor/importer/macros/dd_find_block_end.deor
                        break;
                    }
                }
            }
            let mut end_pos: i32 = fbe_cur.clone();
            // transpiler-deor/importer/dedup.deor
            if !already_seen {
                // transpiler-deor/importer/dedup.deor
                for i in (pos as usize)..(end_pos as usize) {
                	result.push(tokens[i].clone());
                }
            }
            pos = end_pos;
        } else if is_shape {
            // macro: get_decl_name (transpiler-deor/importer/macros/get_decl_name.deor)
            let mut dn_offset: i32 = 1;
            if is_fn {
                // transpiler-deor/importer/macros/get_decl_name.deor
                dn_offset = 2;
            }
            let mut dn_pos: i32 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                // transpiler-deor/importer/macros/get_decl_name.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            // macro: check_seen (transpiler-deor/importer/macros/check_seen.deor)
            let mut already_seen: bool = false;
            let mut cs_len: i32 = (seen.len() as i32);
            for cs_i in 0..cs_len {
                // transpiler-deor/importer/macros/check_seen.deor
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    // transpiler-deor/importer/macros/check_seen.deor
                    already_seen = true;
                    break;
                }
            }
            // transpiler-deor/importer/dedup.deor
            if !already_seen {
                // transpiler-deor/importer/dedup.deor
                seen.push(decl_name.clone());
            }
            // macro: find_shape_end (transpiler-deor/importer/macros/find_shape_end.deor)
            let mut fse_cur: i32 = pos.clone();
            while fse_cur < token_count {
                // transpiler-deor/importer/macros/find_shape_end.deor
                let mut fse_tok: Token = tokens[fse_cur as usize].clone();
                let kind = fse_tok.kind.clone();
                fse_cur = fse_cur + 1;
                if kind == "NEWLINE" {
                    // transpiler-deor/importer/macros/find_shape_end.deor
                    break;
                }
            }
            let mut end_pos: i32 = fse_cur.clone();
            // transpiler-deor/importer/dedup.deor
            if !already_seen {
                // transpiler-deor/importer/dedup.deor
                let mut copy_len: i32 = end_pos - pos.clone();
                for copy_idx in 0..copy_len {
                    // transpiler-deor/importer/dedup.deor
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = end_pos;
        } else if is_raw {
            // macro: get_decl_name (transpiler-deor/importer/macros/get_decl_name.deor)
            let mut dn_offset: i32 = 1;
            if is_fn {
                // transpiler-deor/importer/macros/get_decl_name.deor
                dn_offset = 2;
            }
            let mut dn_pos: i32 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                // transpiler-deor/importer/macros/get_decl_name.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            // transpiler-deor/importer/dedup.deor
            let mut raw_pfx: String = "_raw_".to_string();
            let mut raw_key_parts: Vec<String> = vec![raw_pfx.clone(), decl_name.clone()];
            decl_name = s_join(raw_key_parts.clone());
            // macro: check_seen (transpiler-deor/importer/macros/check_seen.deor)
            let mut already_seen: bool = false;
            let mut cs_len: i32 = (seen.len() as i32);
            for cs_i in 0..cs_len {
                // transpiler-deor/importer/macros/check_seen.deor
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    // transpiler-deor/importer/macros/check_seen.deor
                    already_seen = true;
                    break;
                }
            }
            // transpiler-deor/importer/dedup.deor
            if !already_seen {
                // transpiler-deor/importer/dedup.deor
                seen.push(decl_name.clone());
            }
            // macro: find_shape_end (transpiler-deor/importer/macros/find_shape_end.deor)
            let mut fse_cur: i32 = pos.clone();
            while fse_cur < token_count {
                // transpiler-deor/importer/macros/find_shape_end.deor
                let mut fse_tok: Token = tokens[fse_cur as usize].clone();
                let kind = fse_tok.kind.clone();
                fse_cur = fse_cur + 1;
                if kind == "NEWLINE" {
                    // transpiler-deor/importer/macros/find_shape_end.deor
                    break;
                }
            }
            let mut end_pos: i32 = fse_cur.clone();
            // transpiler-deor/importer/dedup.deor
            if !already_seen {
                // transpiler-deor/importer/dedup.deor
                let mut copy_len: i32 = end_pos - pos.clone();
                for copy_idx in 0..copy_len {
                    // transpiler-deor/importer/dedup.deor
                    let mut tok_pos: i32 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = end_pos;
        } else if is_rust_blk {
            // transpiler-deor/importer/dedup.deor
            let mut rust_content_pos: i32 = pos + 2.clone();
            let mut rust_in_range: bool = rust_content_pos < token_count.clone();
            if rust_in_range {
                // transpiler-deor/importer/dedup.deor
                let mut rust_tok: Token = tokens[rust_content_pos as usize].clone();
                let value = rust_tok.value.clone();
                let mut newline: String = "\n".to_string();
                let mut rust_lines: Vec<String> = s_split(value.clone(), newline.clone());
                let mut rust_first: String = rust_lines[0 as usize].clone();
                let mut rk_pfx: String = "_rust_".to_string();
                let mut rk_parts: Vec<String> = vec![rk_pfx.clone(), rust_first.clone()];
                let mut decl_name: String = s_join(rk_parts.clone());
                // macro: check_seen (transpiler-deor/importer/macros/check_seen.deor)
                let mut already_seen: bool = false;
                let mut cs_len: i32 = (seen.len() as i32);
                for cs_i in 0..cs_len {
                    // transpiler-deor/importer/macros/check_seen.deor
                    let mut cs_val: String = seen[cs_i as usize].clone();
                    if cs_val == decl_name {
                        // transpiler-deor/importer/macros/check_seen.deor
                        already_seen = true;
                        break;
                    }
                }
                // transpiler-deor/importer/dedup.deor
                if !already_seen {
                    // transpiler-deor/importer/dedup.deor
                    seen.push(decl_name.clone());
                    for copy_idx in 0..3 {
                        // transpiler-deor/importer/dedup.deor
                        let mut tok_pos: i32 = pos + copy_idx.clone();
                        let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                        result.push(copy_tok.clone());
                    }
                }
            }
            pos = pos + 3;
        } else {
            // transpiler-deor/importer/dedup.deor
            result.push(token.clone());
            pos = pos + 1;
        }
    }
    return result;
}

// transpiler-deor/importer/importer.deor
fn collect_all_tokens_with_all_imports(path: String) -> Vec<Token> {
    // transpiler-deor/importer/importer.deor
    let mut merged: Vec<Token> = load_file(path.clone());
    return deduplicate_decls(merged.clone());
}

// transpiler-deor/macro_expander.deor
fn expand_deor_macros(tokens: Vec<Token>) -> Vec<Token> {
    // transpiler-deor/macro_expander.deor
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
    		// splice body tokens inline, preceded by a marker carrying the macro name
    		if let Some((body, _)) = macros.get(&name) {
    			let marker_file = body.first().map(|t| t.file.clone()).unwrap_or_default();
    			result.push(Token { kind: "MACRO_MARKER".to_string(), value: name.clone(), line: 0, file: marker_file });
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

// transpiler-deor/tokens_validator/casing.deor
fn is_pascal(name: String) -> bool {
    // transpiler-deor/tokens_validator/casing.deor
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    if name_len == 0 {
        // transpiler-deor/tokens_validator/casing.deor
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    return s_upper_char(first.clone());
}

fn is_camel(name: String) -> bool {
    // transpiler-deor/tokens_validator/casing.deor
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    if name_len == 0 {
        // transpiler-deor/tokens_validator/casing.deor
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    if !s_lower_char(first.clone()) {
        // transpiler-deor/tokens_validator/casing.deor
        return false;
    }
    let mut idx: i32 = 0;
    while idx < name_len {
        // transpiler-deor/tokens_validator/casing.deor
        let mut chr: String = chars[idx as usize].clone();
        if chr == "_" {
            // transpiler-deor/tokens_validator/casing.deor
            return false;
        }
        idx = idx + 1;
    }
    return true;
}

fn is_snake(name: String) -> bool {
    // transpiler-deor/tokens_validator/casing.deor
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i32 = (chars.len() as i32);
    let mut idx: i32 = 0;
    while idx < name_len {
        // transpiler-deor/tokens_validator/casing.deor
        let mut chr: String = chars[idx as usize].clone();
        if s_upper_char(chr.clone()) {
            // transpiler-deor/tokens_validator/casing.deor
            return false;
        }
        idx = idx + 1;
    }
    return true;
}

// transpiler-deor/tokens_validator/arg_helpers.deor
fn arg_is_named(tokens: TokensRef, scan_pos: i32, kind: String) -> bool {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut check_pos: i32 = scan_pos.clone();
    let mut chk_kind: String = kind.clone();
    if kind == "KW_GIVEUP" {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        check_pos = scan_pos + 1;
        if check_pos < token_count {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut giveup_tok: Token = tokens[check_pos as usize].clone();
            let kind = giveup_tok.kind.clone();
            chk_kind = kind;
        }
    }
    if chk_kind != "IDENT" {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        return false;
    }
    let mut peek_pos: i32 = check_pos + 1.clone();
    if peek_pos < token_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
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
            // transpiler-deor/tokens_validator/arg_helpers.deor
            return false;
        }
    }
    return true;
}

fn count_call_args(tokens: TokensRef, lp_pos: i32) -> i32 {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = lp_pos + 1.clone();
    let mut depth: i32 = 0;
    let mut comma_count: i32 = 0;
    let mut saw_token: bool = false;
    while cur < token_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        let mut tok: Token = tokens[cur as usize].clone();
        let kind = tok.kind.clone();
        if kind == "RPAREN" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut at_root: bool = depth == 0.clone();
            if at_root {
                // transpiler-deor/tokens_validator/arg_helpers.deor
                break;
            }
            depth = depth - 1;
        } else if kind == "LPAREN" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            depth = depth + 1;
            saw_token = true;
        } else if kind == "LBRACKET" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            depth = depth + 1;
            saw_token = true;
        } else if kind == "RBRACKET" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            depth = depth - 1;
            saw_token = true;
        } else if kind == "COMMA" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut at_root: bool = depth == 0.clone();
            if at_root {
                // transpiler-deor/tokens_validator/arg_helpers.deor
                comma_count = comma_count + 1;
            }
            saw_token = true;
        } else {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            saw_token = true;
        }
        cur = cur + 1;
    }
    let mut result: i32 = 0;
    if saw_token {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        result = comma_count + 1;
    }
    return result;
}

// transpiler-deor/tokens_validator/error_handling.deor
fn val_err(tok: Token, label: String, rule: String) -> String {
    // transpiler-deor/tokens_validator/error_handling.deor
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
    // transpiler-deor/tokens_validator/error_handling.deor
    let mut error_count: i32 = (errors.len() as i32);
    if error_count > 0 {
        // transpiler-deor/tokens_validator/error_handling.deor
        let mut err_idx: i32 = 0;
        while err_idx < error_count {
            // transpiler-deor/tokens_validator/error_handling.deor
            let mut err_msg: String = errors[err_idx as usize].clone();
            println!("{}", err_msg.clone());
            err_idx = err_idx + 1;
        }
        std::process::exit(1);
    }
}

// transpiler-deor/tokens_validator/tokens_validation.deor
type FnTestRule = fn(String) -> bool;

fn validate_tokens(tokens: TokensRef) {
    // transpiler-deor/tokens_validator/tokens_validation.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut errors: Vec<String> = Vec::new();
    let mut pos: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut block_depth: i32 = 0;
    let mut in_void_fn: bool = false;
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
    let mut decl_names: Vec<String> = Vec::new();
    let mut rule_dup: String = "duplicate declaration — this name is already used by another struct, enum, shape, fn, or type".to_string();
    let mut rule_enum_pascal: String = "enum variant must be PascalCase".to_string();
    let mut rule_enum_data: String = "enum variants cannot carry data — use a struct alongside the enum instead".to_string();
    let mut lbl_decl: String = "declaration".to_string();
    let mut lbl_field: String = "struct field".to_string();
    let mut lbl_variant: String = "enum variant".to_string();
    let mut pre_i: i32 = 0;
    while pre_i < token_count {
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut pre_tok: Token = tokens[pre_i as usize].clone();
        let kind = pre_tok.kind.clone();
        if kind == "KW_SHAPE" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut sn_pos: i32 = pre_i + 1.clone();
            if sn_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut sn_tok: Token = tokens[sn_pos as usize].clone();
                let value = sn_tok.value.clone();
                shape_names.push(value.clone());
            }
        }
        let mut is_kw_struct: bool = kind == "KW_STRUCT".clone();
        let mut is_kw_enum: bool = kind == "KW_ENUM".clone();
        let mut is_kw_shape: bool = kind == "KW_SHAPE".clone();
        let mut is_kw_type: bool = kind == "KW_TYPE".clone();
        let mut is_named_decl: bool = is_kw_struct || is_kw_enum || is_kw_shape || is_kw_type.clone();
        if is_named_decl {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut dn_pos: i32 = pre_i + 1.clone();
            if dn_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let kind = dn_tok.kind.clone();
                let value = dn_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    if list_has(decl_names.clone(), value.clone()) {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        errors.push(val_err(dn_tok.clone(), lbl_decl.clone(), rule_dup.clone()).clone());
                    } else {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        decl_names.push(value.clone());
                    }
                }
            }
        }
        if kind == "KW_FN" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut fn_name_pos: i32 = pre_i + 2.clone();
            if fn_name_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut fn_name_tok: Token = tokens[fn_name_pos as usize].clone();
                let kind = fn_name_tok.kind.clone();
                let value = fn_name_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    if list_has(decl_names.clone(), value.clone()) {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        errors.push(val_err(fn_name_tok.clone(), lbl_decl.clone(), rule_dup.clone()).clone());
                    } else {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        decl_names.push(value.clone());
                    }
                }
            }
        }
        if kind == "KW_STRUCT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut sf_pos: i32 = pre_i + 1.clone();
            while sf_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut sf_tok: Token = tokens[sf_pos as usize].clone();
                let kind = sf_tok.kind.clone();
                if kind == "INDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    break;
                }
                sf_pos = sf_pos + 1;
            }
            sf_pos = sf_pos + 1;
            while sf_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut sf_tok: Token = tokens[sf_pos as usize].clone();
                let kind = sf_tok.kind.clone();
                if kind == "DEDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    break;
                }
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let mut field_name_pos: i32 = sf_pos + 1.clone();
                    if field_name_pos < token_count {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        let mut field_name_tok: Token = tokens[field_name_pos as usize].clone();
                        let kind = field_name_tok.kind.clone();
                        let value = field_name_tok.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            if (value.len() as i32) < 3 {
                                // transpiler-deor/tokens_validator/tokens_validation.deor
                                errors.push(val_err(field_name_tok.clone(), lbl_field.clone(), rule_min3.clone()).clone());
                            }
                            if !is_snake(value.clone()) {
                                // transpiler-deor/tokens_validator/tokens_validation.deor
                                errors.push(val_err(field_name_tok.clone(), lbl_field.clone(), rule_snake.clone()).clone());
                            }
                        }
                    }
                }
                sf_pos = sf_pos + 1;
            }
        }
        if kind == "KW_ENUM" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut ev_pos: i32 = pre_i + 1.clone();
            while ev_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut ev_tok: Token = tokens[ev_pos as usize].clone();
                let kind = ev_tok.kind.clone();
                if kind == "INDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    break;
                }
                ev_pos = ev_pos + 1;
            }
            ev_pos = ev_pos + 1;
            while ev_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut ev_tok: Token = tokens[ev_pos as usize].clone();
                let kind = ev_tok.kind.clone();
                if kind == "DEDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    break;
                }
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let value = ev_tok.value.clone();
                    if (value.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_min3.clone()).clone());
                    }
                    if !is_pascal(value.clone()) {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_enum_pascal.clone()).clone());
                    }
                    let mut after_variant: i32 = ev_pos + 1.clone();
                    if after_variant < token_count {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        let mut after_tok: Token = tokens[after_variant as usize].clone();
                        let kind = after_tok.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_enum_data.clone()).clone());
                        }
                    }
                }
                ev_pos = ev_pos + 1;
            }
        }
        pre_i = pre_i + 1;
    }
    while pos < token_count {
        // transpiler-deor/tokens_validator/tokens_validation.deor
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
            // transpiler-deor/tokens_validator/tokens_validation.deor
            paren_depth = paren_depth + 1;
        }
        if cur_kind == "RPAREN" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            paren_depth = paren_depth - 1;
        }
        if cur_kind == "INDENT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            block_depth = block_depth + 1;
        }
        if cur_kind == "DEDENT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            block_depth = block_depth - 1;
            if block_depth == 0 {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                in_void_fn = false;
            }
        }
        if cur_kind == "KW_FN" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut void_check: i32 = pos + 1.clone();
            if void_check < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut void_tok: Token = tokens[void_check as usize].clone();
                let kind = void_tok.kind.clone();
                if kind == "KW_VOID" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    in_void_fn = true;
                }
            }
        }
        if cur_kind == "KW_RETURN" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            if in_void_fn {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut rule_void_return: String = "void functions must not use return — remove the return statement and let the function fall through".to_string();
                errors.push(val_err(tok.clone(), lbl_fn.clone(), rule_void_return.clone()).clone());
            }
        }
        if cur_kind == "KW_GIVEUP" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut mv_next: i32 = pos + 1.clone();
            if mv_next < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut mv_tok: Token = tokens[mv_next as usize].clone();
                let kind = mv_tok.kind.clone();
                let mut mv_ok: bool = kind == "IDENT".clone();
                let mut mv_destruct: bool = kind == "LPAREN".clone();
                let mut mv_valid: bool = mv_ok || mv_destruct.clone();
                if !mv_valid {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let mut rule_move: String = "'move' can only precede a variable name — 'move 5' or 'move \"hello\"' are not valid".to_string();
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_move.clone()).clone());
                }
            }
        }
        if paren_depth > 0 {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut is_forbidden: bool = list_has(forbidden_in_parens.clone(), cur_kind.clone());
            if is_forbidden {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
            }
        }
        let mut cur_indicator: String = "KW_RUST".to_string();
        let mut next_indicator: String = "RUST_BLOCK".to_string();
        // macro: skip_if_match (transpiler-deor/tokens_validator/macros/skip_if_match.deor)
        if cur_kind == cur_indicator {
            // transpiler-deor/tokens_validator/macros/skip_if_match.deor
            let mut skip_pos: i32 = pos + 1.clone();
            while skip_pos < token_count {
                // transpiler-deor/tokens_validator/macros/skip_if_match.deor
                let mut skip_tok: Token = tokens[skip_pos as usize].clone();
                let kind = skip_tok.kind.clone();
                skip_pos = skip_pos + 1;
                if kind == next_indicator {
                    // transpiler-deor/tokens_validator/macros/skip_if_match.deor
                    break;
                }
            }
            pos = skip_pos;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        if cur_kind == "KW_NOT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut next_not: i32 = pos + 1.clone();
            let mut after_not: i32 = pos + 2.clone();
            if after_not < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut next_not_tok: Token = tokens[next_not as usize].clone();
                let mut after_not_tok: Token = tokens[after_not as usize].clone();
                let kind = next_not_tok.kind.clone();
                let mut next_not_kind: String = kind.clone();
                let kind = after_not_tok.kind.clone();
                let mut after_not_kind: String = kind.clone();
                let mut next_is_ident: bool = next_not_kind == "IDENT".clone();
                let mut after_is_is: bool = after_not_kind == "KW_IS".clone();
                if next_is_ident && after_is_is {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
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
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/ident_validator.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/ident_validator.deor
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/ident_validator.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/ident_validator.deor
                    if (name_val.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut keyword: String = "KW_ENUM".to_string();
        let mut lbl: String = lbl_enum.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/ident_validator.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/ident_validator.deor
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/ident_validator.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/ident_validator.deor
                    if (name_val.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut keyword: String = "KW_SHAPE".to_string();
        let mut lbl: String = lbl_shape.clone();
        let mut rule: String = rule_camel.clone();
        let mut test_rule: fn(String) -> bool = is_camel.clone();
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/ident_validator.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/ident_validator.deor
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/ident_validator.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/ident_validator.deor
                    if (name_val.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut keyword: String = "KW_TYPE".to_string();
        let mut lbl: String = lbl_type.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/ident_validator.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/ident_validator.deor
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/ident_validator.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/ident_validator.deor
                    if (name_val.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        if cur_kind == "KW_TYPE" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut base_type_pos: i32 = pos + 3.clone();
            if base_type_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut base_type_tok: Token = tokens[base_type_pos as usize].clone();
                let value = base_type_tok.value.clone();
                let mut base_is_shape: bool = list_has(shape_names.clone(), value.clone());
                if base_is_shape {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let mut type_name_pos: i32 = pos + 1.clone();
                    let mut type_name_tok: Token = tokens[type_name_pos as usize].clone();
                    errors.push(val_err(type_name_tok.clone(), lbl_type.clone(), rule_list_validator.clone()).clone());
                }
            }
        }
        if cur_kind == "KW_FN" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut lp_pos: i32 = pos + 3.clone();
            if lp_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut lp_tok: Token = tokens[lp_pos as usize].clone();
                let kind = lp_tok.kind.clone();
                if kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let mut param_count: i32 = count_call_args(tokens.clone(), lp_pos.clone());
                    if param_count > 3 {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        let mut fn_name_pos: i32 = pos + 2.clone();
                        let mut fn_name_tok: Token = tokens[fn_name_pos as usize].clone();
                        errors.push(val_err(fn_name_tok.clone(), lbl_fn.clone(), rule_max_params.clone()).clone());
                    }
                    let mut rule_param_shadow: String = "parameter name cannot be the same as its type — choose a descriptive name".to_string();
                    let mut ps_pos: i32 = lp_pos + 1.clone();
                    while ps_pos < token_count {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        let mut ps_tok: Token = tokens[ps_pos as usize].clone();
                        let kind = ps_tok.kind.clone();
                        let value = ps_tok.value.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            break;
                        }
                        if kind == "COMMA" {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            ps_pos = ps_pos + 1;
                            continue;
                        }
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            let mut param_type_val: String = value.clone();
                            let mut pn_pos: i32 = ps_pos + 1.clone();
                            if pn_pos < token_count {
                                // transpiler-deor/tokens_validator/tokens_validation.deor
                                let mut pn_tok: Token = tokens[pn_pos as usize].clone();
                                let kind = pn_tok.kind.clone();
                                let value = pn_tok.value.clone();
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/tokens_validation.deor
                                    if value == param_type_val {
                                        // transpiler-deor/tokens_validator/tokens_validation.deor
                                        errors.push(val_err(pn_tok.clone(), lbl_fn.clone(), rule_param_shadow.clone()).clone());
                                    }
                                    ps_pos = pn_pos;
                                }
                            }
                        }
                        ps_pos = ps_pos + 1;
                    }
                }
            }
            let mut rule_no_ret: String = "missing return type — use 'fn void name()' for functions that return nothing".to_string();
            let mut ret_pos: i32 = pos + 1.clone();
            let mut lp2_pos: i32 = pos + 2.clone();
            if lp2_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut ret_tok: Token = tokens[ret_pos as usize].clone();
                let mut lp2_tok: Token = tokens[lp2_pos as usize].clone();
                let kind = ret_tok.kind.clone();
                let mut ret_kind: String = kind.clone();
                let kind = lp2_tok.kind.clone();
                if ret_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
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
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/ident_validator.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/ident_validator.deor
            let mut name_pos: i32 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/ident_validator.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let kind = name_tok.kind.clone();
                let value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/ident_validator.deor
                    if (name_val.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/ident_validator.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                }
            }
            pos = pos + 1;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        if cur_kind == "IDENT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut is_crash: bool = cur_val == "crash".clone();
            if is_crash {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut crash_lp: i32 = pos + 1.clone();
                if crash_lp < token_count {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let mut crash_lp_tok: Token = tokens[crash_lp as usize].clone();
                    let kind = crash_lp_tok.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        let mut crash_arg_count: i32 = count_call_args(tokens.clone(), crash_lp.clone());
                        let mut wrong_count: bool = crash_arg_count != 1.clone();
                        if wrong_count {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            let mut rule_crash: String = "crash takes exactly 1 string argument".to_string();
                            errors.push(val_err(tok.clone(), lbl_call.clone(), rule_crash.clone()).clone());
                        }
                    }
                }
            }
        }
        if cur_kind == "LBRACKET" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut next_pos: i32 = pos + 1.clone();
            if next_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut next_tok: Token = tokens[next_pos as usize].clone();
                let kind = next_tok.kind.clone();
                if kind == "RBRACKET" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_empty_bracket.clone()).clone());
                }
            }
        }
        if cur_kind == "IDENT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut is_fn_decl_name: bool = false;
            if pos > 1 {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut prev2_p: i32 = pos - 2.clone();
                let mut prev2_tok: Token = tokens[prev2_p as usize].clone();
                let kind = prev2_tok.kind.clone();
                is_fn_decl_name = kind == "KW_FN";
            }
            if !is_fn_decl_name {
                // macro: check_call_args (transpiler-deor/tokens_validator/macros/check_call_args.deor)
                let mut call_lp: i32 = pos + 1.clone();
                if call_lp < token_count {
                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                    let mut call_lp_tok: Token = tokens[call_lp as usize].clone();
                    let kind = call_lp_tok.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_call_args.deor
                        let mut arg_count: i32 = count_call_args(tokens.clone(), call_lp.clone());
                        if arg_count >= 2 {
                            // transpiler-deor/tokens_validator/macros/check_call_args.deor
                            let mut scan_pos: i32 = call_lp + 1.clone();
                            let mut scan_depth: i32 = 0;
                            let mut at_arg_start: bool = true;
                            while scan_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                let mut scan_tok: Token = tokens[scan_pos as usize].clone();
                                let kind = scan_tok.kind.clone();
                                if kind == "RPAREN" {
                                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                    let mut scan_root: bool = scan_depth == 0.clone();
                                    if scan_root {
                                        // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                        break;
                                    }
                                    scan_depth = scan_depth - 1;
                                    at_arg_start = false;
                                } else if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                    scan_depth = scan_depth + 1;
                                    at_arg_start = false;
                                } else if kind == "LBRACKET" {
                                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                    scan_depth = scan_depth + 1;
                                    at_arg_start = false;
                                } else if kind == "RBRACKET" {
                                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                    scan_depth = scan_depth - 1;
                                    at_arg_start = false;
                                } else if kind == "COMMA" {
                                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                    let mut scan_root: bool = scan_depth == 0.clone();
                                    if scan_root {
                                        // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                        at_arg_start = true;
                                    }
                                } else if at_arg_start {
                                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                    let mut named: bool = arg_is_named(tokens.clone(), scan_pos.clone(), kind.clone());
                                    if !named {
                                        // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                        errors.push(val_err(tok.clone(), lbl_call.clone(), rule_named_arg.clone()).clone());
                                    }
                                    at_arg_start = false;
                                }
                                scan_pos = scan_pos + 1;
                            }
                        }
                    }
                }
            }
            // macro: check_rust_generic (transpiler-deor/tokens_validator/macros/check_rust_generic.deor)
            let mut is_option: bool = cur_val == "Option".clone();
            let mut is_vec: bool = cur_val == "Vec".clone();
            let mut is_box: bool = cur_val == "Box".clone();
            let mut is_rc: bool = cur_val == "Rc".clone();
            let mut is_arc: bool = cur_val == "Arc".clone();
            let mut is_result: bool = cur_val == "Result".clone();
            let mut is_rust_generic: bool = is_option || is_vec || is_box || is_rc || is_arc || is_result.clone();
            if is_rust_generic {
                // transpiler-deor/tokens_validator/macros/check_rust_generic.deor
                errors.push(val_err(tok.clone(), lbl_rust.clone(), rule_no_option.clone()).clone());
            }
            // macro: check_var_decl (transpiler-deor/tokens_validator/macros/check_var_decl.deor)
            let mut next1: i32 = pos + 1.clone();
            let mut next2: i32 = pos + 2.clone();
            if next2 < token_count {
                // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                let mut tok_one: Token = tokens[next1 as usize].clone();
                let mut tok_two: Token = tokens[next2 as usize].clone();
                let kind = tok_one.kind.clone();
                let mut one_kind: String = kind.clone();
                let kind = tok_two.kind.clone();
                let mut two_kind: String = kind.clone();
                if one_kind == "IDENT" && two_kind == "EQUALS" {
                    // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                    let value = tok_one.value.clone();
                    let line = tok_one.line.clone();
                    let file = tok_one.file.clone();
                    let mut var_name: String = value.clone();
                    let mut var_line: i32 = line.clone();
                    let mut var_file: String = file.clone();
                    if (var_name.len() as i32) < 3 {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                    }
                    if !is_snake(var_name.clone()) {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                    }
                }
            }
            // macro: check_bad_stmt (transpiler-deor/tokens_validator/macros/check_bad_stmt.deor)
            if paren_depth == 0 {
                // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                let mut next1: i32 = pos + 1.clone();
                let mut next2: i32 = pos + 2.clone();
                if next2 < token_count {
                    // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                    let mut tok_one: Token = tokens[next1 as usize].clone();
                    let mut tok_two: Token = tokens[next2 as usize].clone();
                    let kind = tok_one.kind.clone();
                    let mut one_kind: String = kind.clone();
                    let kind = tok_two.kind.clone();
                    let mut two_kind: String = kind.clone();
                    if one_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                        let mut two_is_str: bool = two_kind == "STRING".clone();
                        let mut two_is_int: bool = two_kind == "INT".clone();
                        let mut two_is_flt: bool = two_kind == "FLOAT".clone();
                        let mut two_is_lit: bool = two_is_str || two_is_int || two_is_flt.clone();
                        if two_is_lit {
                            // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_bad_stmt.clone()).clone());
                        }
                    }
                }
            }
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut next_pos: i32 = pos + 1.clone();
            if next_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut next_tok: Token = tokens[next_pos as usize].clone();
                let kind = next_tok.kind.clone();
                let mut bracket_rule: String = "bracket indexing is not valid in Deor — use 'name at index' instead".to_string();
                if kind == "LBRACKET" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    errors.push(val_err(tok.clone(), lbl_var.clone(), bracket_rule.clone()).clone());
                }
            }
        }
        if cur_kind == "KW_VOID" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut preceded_by_fn: bool = false;
            if pos > 0 {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut prev_void: i32 = pos - 1.clone();
                let mut prev_void_tok: Token = tokens[prev_void as usize].clone();
                let kind = prev_void_tok.kind.clone();
                preceded_by_fn = kind == "KW_FN";
            }
            if !preceded_by_fn {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut void_name_pos: i32 = pos + 1.clone();
                let mut void_eq_pos: i32 = pos + 2.clone();
                if void_eq_pos < token_count {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    let mut void_name_tok: Token = tokens[void_name_pos as usize].clone();
                    let mut void_eq_tok: Token = tokens[void_eq_pos as usize].clone();
                    let kind = void_name_tok.kind.clone();
                    let mut void_name_kind: String = kind.clone();
                    let kind = void_eq_tok.kind.clone();
                    if void_name_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        if kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/tokens_validation.deor
                            let mut rule_void_var: String = "'void' is not a valid variable type — only functions can return void".to_string();
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_void_var.clone()).clone());
                        }
                    }
                }
            }
        }
        if cur_kind == "KW_VALID" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut valid_ok: bool = false;
            if pos > 0 {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut valid_pos: i32 = pos - 1.clone();
                let mut prev_valid_tok: Token = tokens[valid_pos as usize].clone();
                let kind = prev_valid_tok.kind.clone();
                valid_ok = kind == "KW_IS";
                if !valid_ok {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    valid_ok = kind == "KW_NOT";
                }
            }
            if !valid_ok {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut rule_valid: String = "'valid' can only appear after 'is' or 'is not' — it cannot be assigned or returned".to_string();
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_valid.clone()).clone());
            }
        }
        pos = pos + 1;
    }
    handle_errors(errors.clone());
}

// transpiler-deor/codegen/type_map.deor
fn render_rust_type(type_name: String) -> String {
    // transpiler-deor/codegen/type_map.deor
    if type_name == "void" {
        // transpiler-deor/codegen/type_map.deor
        return "".to_string();
    }
    if type_name == "int" {
        // transpiler-deor/codegen/type_map.deor
        return "i32".to_string();
    }
    if type_name == "string" {
        // transpiler-deor/codegen/type_map.deor
        return "String".to_string();
    }
    if type_name == "bool" {
        // transpiler-deor/codegen/type_map.deor
        return "bool".to_string();
    }
    if type_name == "float" {
        // transpiler-deor/codegen/type_map.deor
        return "f64".to_string();
    }
    return s_pascal(type_name.clone());
}

// transpiler-deor/registry/struct.deor
fn skip_to_block_start(tokens: TokensRef, start: i32) -> ParseResult {
    // transpiler-deor/registry/struct.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut cur: i32 = start.clone();
    for skip_index in start..token_count {
        // transpiler-deor/registry/struct.deor
        let mut skip_token: Token = tokens[skip_index as usize].clone();
        let kind = skip_token.kind.clone();
        cur = skip_index + 1;
        if kind == "INDENT" {
            // transpiler-deor/registry/struct.deor
            break;
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), cur.clone());
}

fn collect_struct_fields(tokens: TokensRef, start: i32) -> ParseResult {
    // transpiler-deor/registry/struct.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i32 = start.clone();
    for field_index in start..token_count {
        // transpiler-deor/registry/struct.deor
        let mut field_token: Token = tokens[field_index as usize].clone();
        let kind = field_token.kind.clone();
        if kind == "DEDENT" {
            // transpiler-deor/registry/struct.deor
            cur = field_index + 1;
            break;
        } else if kind == "IDENT" {
            // transpiler-deor/registry/struct.deor
            let mut fname_pos: i32 = field_index + 1.clone();
            if fname_pos < token_count {
                // transpiler-deor/registry/struct.deor
                let mut fname_token: Token = tokens[fname_pos as usize].clone();
                let kind = fname_token.kind.clone();
                let value = fname_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/registry/struct.deor
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

fn build_struct_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/struct.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut raw_i: i32 = 0;
    while raw_i < token_count {
        // transpiler-deor/registry/struct.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_STRUCT" {
            // transpiler-deor/registry/struct.deor
            let mut name_pos: i32 = raw_i + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/struct.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let kind = name_token.kind.clone();
                let value = name_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/registry/struct.deor
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

// transpiler-deor/registry/shape.deor
fn build_shape_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/shape.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    for index in 0..token_count {
        // transpiler-deor/registry/shape.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_SHAPE" {
            // transpiler-deor/registry/shape.deor
            let mut name_pos: i32 = index + 1.clone();
            let mut form_pos: i32 = index + 3.clone();
            let mut t4_pos: i32 = index + 4.clone();
            if t4_pos < token_count {
                // transpiler-deor/registry/shape.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut form_token: Token = tokens[form_pos as usize].clone();
                let value = name_token.value.clone();
                let mut shape_name: String = value.clone();
                let kind = form_token.kind.clone();
                if kind == "KW_LIST" {
                    // transpiler-deor/registry/shape.deor
                    let mut elem_pos: i32 = index + 5.clone();
                    if elem_pos < token_count {
                        // transpiler-deor/registry/shape.deor
                        let mut elem_token: Token = tokens[elem_pos as usize].clone();
                        let value = elem_token.value.clone();
                        result.push(shape_name.clone());
                        result.push(value.clone());
                    }
                } else {
                    // transpiler-deor/registry/shape.deor
                    let mut t4_token: Token = tokens[t4_pos as usize].clone();
                    let kind = t4_token.kind.clone();
                    let value = t4_token.value.clone();
                    let mut t4_is_of: bool = kind == "KW_OF".clone();
                    let mut t4_is_to: bool = value == "to".clone();
                    let mut in_type: String = "".to_string();
                    let mut out_type: String = "".to_string();
                    if t4_is_of {
                        // transpiler-deor/registry/shape.deor
                        let mut t5_pos: i32 = index + 5.clone();
                        if t5_pos < token_count {
                            // transpiler-deor/registry/shape.deor
                            let mut t5_token: Token = tokens[t5_pos as usize].clone();
                            let value = t5_token.value.clone();
                            in_type = value;
                        }
                        let mut t6_pos: i32 = index + 6.clone();
                        if t6_pos < token_count {
                            // transpiler-deor/registry/shape.deor
                            let mut t6_token: Token = tokens[t6_pos as usize].clone();
                            let value = t6_token.value.clone();
                            let mut t6_is_to: bool = value == "to".clone();
                            if t6_is_to {
                                // transpiler-deor/registry/shape.deor
                                let mut t7_pos: i32 = index + 7.clone();
                                if t7_pos < token_count {
                                    // transpiler-deor/registry/shape.deor
                                    let mut t7_token: Token = tokens[t7_pos as usize].clone();
                                    let value = t7_token.value.clone();
                                    out_type = value;
                                }
                            }
                        }
                    } else if t4_is_to {
                        // transpiler-deor/registry/shape.deor
                        let mut t5_pos: i32 = index + 5.clone();
                        if t5_pos < token_count {
                            // transpiler-deor/registry/shape.deor
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
            // transpiler-deor/registry/shape.deor
            let mut name_pos: i32 = index + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/shape.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let value = name_token.value.clone();
                result.push(value.clone());
                result.push("raw:".to_string());
            }
        }
    }
    return result;
}

// transpiler-deor/registry/enum.deor
fn build_enum_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    for index in 0..token_count {
        // transpiler-deor/registry/enum.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/enum.deor
            let mut name_pos: i32 = index + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/enum.deor
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

fn build_variant_reg(tokens: TokensRef, enum_reg: Vec<String>) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut raw_i: i32 = 0;
    while raw_i < token_count {
        // transpiler-deor/registry/enum.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/enum.deor
            let mut name_pos: i32 = raw_i + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/enum.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let value = name_token.value.clone();
                let mut enum_name: String = value.clone();
                let mut rust_name: String = reg_get(enum_reg.clone(), enum_name.clone());
                raw_i = name_pos + 1;
                while raw_i < token_count {
                    // transpiler-deor/registry/enum.deor
                    let mut skip_token: Token = tokens[raw_i as usize].clone();
                    let kind = skip_token.kind.clone();
                    raw_i = raw_i + 1;
                    if kind == "INDENT" {
                        // transpiler-deor/registry/enum.deor
                        break;
                    }
                }
                while raw_i < token_count {
                    // transpiler-deor/registry/enum.deor
                    let mut variant_token: Token = tokens[raw_i as usize].clone();
                    let kind = variant_token.kind.clone();
                    let value = variant_token.value.clone();
                    raw_i = raw_i + 1;
                    if kind == "DEDENT" {
                        // transpiler-deor/registry/enum.deor
                        break;
                    } else if kind == "IDENT" {
                        // transpiler-deor/registry/enum.deor
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

// transpiler-deor/registry/validator_type.deor
fn build_type_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/validator_type.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    for index in 0..token_count {
        // transpiler-deor/registry/validator_type.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_TYPE" {
            // transpiler-deor/registry/validator_type.deor
            let mut name_pos: i32 = index + 1.clone();
            let mut param_type_pos: i32 = index + 3.clone();
            let mut param_name_pos: i32 = index + 4.clone();
            if param_name_pos < token_count {
                // transpiler-deor/registry/validator_type.deor
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

// transpiler-deor/registry/var_type.deor
fn build_var_type_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/var_type.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut tok_idx: i32 = 0;
    while tok_idx < token_count {
        // transpiler-deor/registry/var_type.deor
        let mut tok: Token = tokens[tok_idx as usize].clone();
        let kind = tok.kind.clone();
        let value = tok.value.clone();
        if kind == "IDENT" {
            // transpiler-deor/registry/var_type.deor
            let mut next_i: i32 = tok_idx + 1.clone();
            let mut eq_i: i32 = tok_idx + 2.clone();
            if eq_i < token_count {
                // transpiler-deor/registry/var_type.deor
                let mut next_tok: Token = tokens[next_i as usize].clone();
                let mut eq_tok: Token = tokens[eq_i as usize].clone();
                let kind = next_tok.kind.clone();
                let mut next_is_ident: bool = kind == "IDENT".clone();
                let value = next_tok.value.clone();
                let mut var_name: String = value.clone();
                let kind = eq_tok.kind.clone();
                let mut eq_is_equals: bool = kind == "EQUALS".clone();
                if next_is_ident && eq_is_equals {
                    // transpiler-deor/registry/var_type.deor
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
                        // transpiler-deor/registry/var_type.deor
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

// transpiler-deor/registry/type_resolve.deor
fn resolve_type(type_name: String, shape_reg: Vec<String>, enum_reg: Vec<String>) -> String {
    // transpiler-deor/registry/type_resolve.deor
    let mut enum_rust: String = reg_get(enum_reg.clone(), type_name.clone());
    if !is_empty(enum_rust.clone()) {
        // transpiler-deor/registry/type_resolve.deor
        return enum_rust;
    }
    let mut elem_type: String = reg_get(shape_reg.clone(), type_name.clone());
    if !is_empty(elem_type.clone()) {
        // transpiler-deor/registry/type_resolve.deor
        let mut raw_prefix: String = "raw:".to_string();
        let mut fn_prefix: String = "fn:".to_string();
        let mut colon: String = ":".to_string();
        if s_starts_with(elem_type.clone(), raw_prefix.clone()) {
            // transpiler-deor/registry/type_resolve.deor
            return type_name;
        }
        if s_starts_with(elem_type.clone(), fn_prefix.clone()) {
            // transpiler-deor/registry/type_resolve.deor
            let mut parts: Vec<String> = s_split(elem_type.clone(), colon.clone());
            let mut in_type: String = parts[1 as usize].clone();
            let mut out_type: String = parts[2 as usize].clone();
            let mut rust_in: String = render_rust_type(in_type.clone());
            let mut rust_out: String = render_rust_type(out_type.clone());
            if is_empty(rust_out.clone()) {
                // transpiler-deor/registry/type_resolve.deor
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

// transpiler-deor/registry/mut_scan.deor
fn find_block_end(tokens: Vec<Token>, indent_pos: i32) -> i32 {
    // transpiler-deor/registry/mut_scan.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut depth: i32 = 1;
    let mut result: i32 = indent_pos.clone();
    let mut start: i32 = indent_pos + 1.clone();
    for raw_i in start..token_count {
        // transpiler-deor/registry/mut_scan.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "INDENT" {
            // transpiler-deor/registry/mut_scan.deor
            depth = depth + 1;
        } else if kind == "DEDENT" {
            // transpiler-deor/registry/mut_scan.deor
            depth = depth - 1;
            if depth == 0 {
                // transpiler-deor/registry/mut_scan.deor
                result = raw_i;
                break;
            }
        }
    }
    return result;
}

fn find_block_end_ref(tokens: TokensRef, indent_pos: i32) -> i32 {
    // transpiler-deor/registry/mut_scan.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut depth: i32 = 1;
    let mut result: i32 = indent_pos.clone();
    let mut start: i32 = indent_pos + 1.clone();
    for raw_i in start..token_count {
        // transpiler-deor/registry/mut_scan.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "INDENT" {
            // transpiler-deor/registry/mut_scan.deor
            depth = depth + 1;
        } else if kind == "DEDENT" {
            // transpiler-deor/registry/mut_scan.deor
            depth = depth - 1;
            if depth == 0 {
                // transpiler-deor/registry/mut_scan.deor
                result = raw_i;
                break;
            }
        }
    }
    return result;
}

fn collect_mut_names(tokens: Vec<Token>, start: i32, end_pos: i32) -> Vec<String> {
    // transpiler-deor/registry/mut_scan.deor
    let mut result: Vec<String> = Vec::new();
    let mut const_names: Vec<String> = Vec::new();
    for raw_i in start..end_pos {
        // transpiler-deor/registry/mut_scan.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_CONST" {
            // transpiler-deor/registry/mut_scan.deor
            let mut const_name_pos: i32 = raw_i + 2.clone();
            if const_name_pos < end_pos {
                // transpiler-deor/registry/mut_scan.deor
                let mut const_name_tok: Token = tokens[const_name_pos as usize].clone();
                let value = const_name_tok.value.clone();
                if !list_has(const_names.clone(), value.clone()) {
                    // transpiler-deor/registry/mut_scan.deor
                    const_names.push(value.clone());
                }
            }
        }
        if kind == "EQUALS" {
            // transpiler-deor/registry/mut_scan.deor
            let mut prev_pos: i32 = raw_i - 1.clone();
            if prev_pos >= start {
                // transpiler-deor/registry/mut_scan.deor
                let mut prev_token: Token = tokens[prev_pos as usize].clone();
                let kind = prev_token.kind.clone();
                let value = prev_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/registry/mut_scan.deor
                    if !list_has(result.clone(), value.clone()) {
                        // transpiler-deor/registry/mut_scan.deor
                        if !list_has(const_names.clone(), value.clone()) {
                            // transpiler-deor/registry/mut_scan.deor
                            result.push(value.clone());
                        }
                    }
                }
            }
        }
        if kind == "KW_USING" {
            // transpiler-deor/registry/mut_scan.deor
            let mut using_var_pos: i32 = raw_i + 1.clone();
            if using_var_pos < end_pos {
                // transpiler-deor/registry/mut_scan.deor
                let mut using_var_token: Token = tokens[using_var_pos as usize].clone();
                let value = using_var_token.value.clone();
                if !list_has(result.clone(), value.clone()) {
                    // transpiler-deor/registry/mut_scan.deor
                    result.push(value.clone());
                }
            }
        }
    }
    return result;
}

// transpiler-deor/registry/registry.deor
fn build_registry(tokens_ref: TokensRef) -> RcCtx {
    // transpiler-deor/registry/registry.deor
    let mut struct_reg: Vec<String> = build_struct_reg(tokens_ref.clone());
    let mut shape_reg: Vec<String> = build_shape_reg(tokens_ref.clone());
    let mut enum_reg: Vec<String> = build_enum_reg(tokens_ref.clone());
    let mut variant_reg: Vec<String> = build_variant_reg(tokens_ref.clone(), enum_reg.clone());
    let mut type_reg: Vec<String> = build_type_reg(tokens_ref.clone());
    let mut var_type_reg: Vec<String> = build_var_type_reg(tokens_ref.clone());
    let mut mut_names: Vec<String> = Vec::new();
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
    let mut placeholder: Vec<Token> = Vec::new();
    let mut tokens: TokensRef = tokens_wrap(placeholder);
/* unhandled(IDENT) */
    let ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone(), tokens: tokens.clone() };
    let mut ctx: RcCtx = make_rctx(ctx_raw);
    return ctx;
}

// transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
fn find_struct_for_fields(struct_reg: Vec<String>, fields: Vec<String>) -> String {
    // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
    let mut comma: String = ",".to_string();
    let mut input_count: i32 = (fields.len() as i32);
    let mut reg_count: i32 = (struct_reg.len() as i32);
    let mut next_is_val: bool = false;
    let mut cur_name: String = "".to_string();
    for index in 0..reg_count {
        // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
        let mut item: String = struct_reg[index as usize].clone();
        if next_is_val {
            // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
            let mut reg_fields: Vec<String> = s_split(item.clone(), comma.clone());
            let mut reg_count_f: i32 = (reg_fields.len() as i32);
            if reg_count_f == input_count {
                // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                let mut all_match: bool = true;
                for fi in 0..input_count {
                    // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                    let mut field: String = fields[fi as usize].clone();
                    let mut found: bool = list_has(reg_fields.clone(), field.clone());
                    if !found {
                        // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                        all_match = false;
                    }
                }
                if all_match {
                    // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                    return cur_name;
                }
            }
            next_is_val = false;
        } else {
            // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
            cur_name = item;
            next_is_val = true;
        }
    }
    return "Unknown".to_string();
}

fn find_struct_for_field(struct_reg: Vec<String>, field: String) -> String {
    // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
    let mut reg_count: i32 = (struct_reg.len() as i32);
    let mut next_is_val: bool = false;
    let mut cur_name: String = "".to_string();
    for index in 0..reg_count {
        // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
        let mut item: String = struct_reg[index as usize].clone();
        if next_is_val {
            // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
            let mut comma: String = ",".to_string();
            let mut fields: Vec<String> = s_split(item.clone(), comma.clone());
            let mut has_field: bool = list_has(fields.clone(), field.clone());
            if has_field {
                // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                return cur_name;
            }
            next_is_val = false;
        } else {
            // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
            cur_name = item;
            next_is_val = true;
        }
    }
    return "Unknown".to_string();
}

// transpiler-deor/codegen/decl/stmt/expr/call_args.deor
fn gen_call_args(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
    let mut arg_codes: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    while true {
        // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
        if cur >= token_count {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            break;
        }
        if kind == "COMMA" {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            cur = cur + 1;
            continue;
        }
        let mut arg_saved_ctx: bool = float_ctx_get();
        float_ctx_disable();
        let mut arg_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        if arg_saved_ctx {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            float_ctx_enable();
        }
        let code = arg_r.code;
        let new_pos = arg_r.new_pos;
        let mut arg_code = code;
        let arg_pos = new_pos;
        let mut start_token: Token = tokens[cur as usize].clone();
        let kind = start_token.kind.clone();
        if kind == "STRING" {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            let mut str_suf: String = ".to_string()".to_string();
            arg_code = [arg_code.as_str(), str_suf.as_str()].concat();
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            let mut next_cur: i32 = cur + 1.clone();
            let mut peek_is_call: bool = false;
            let mut peek_is_idx: bool = false;
            if next_cur < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
                let mut next_tok: Token = tokens[next_cur as usize].clone();
                let kind = next_tok.kind.clone();
                peek_is_call = kind == "LPAREN";
                peek_is_idx = kind == "KW_AT";
            }
            if !peek_is_call {
                // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
                if !peek_is_idx {
                    // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
                    let mut cln_suf: String = ".clone()".to_string();
                    arg_code = [arg_code.as_str(), cln_suf.as_str()].concat();
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

// transpiler-deor/codegen/decl/stmt/expr/list_items.deor
fn gen_list_items(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
    let mut item_codes: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    while true {
        // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
        if cur >= token_count {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "RBRACKET" {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            break;
        }
        if kind == "COMMA" {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            cur = cur + 1;
            continue;
        }
        let mut item_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        let code = item_r.code;
        let new_pos = item_r.new_pos;
        let mut item_code = code;
        let item_pos = new_pos;
        let mut start_token: Token = tokens[cur as usize].clone();
        let kind = start_token.kind.clone();
        if kind == "STRING" {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            let mut its_suf: String = ".to_string()".to_string();
            item_code = [item_code.as_str(), its_suf.as_str()].concat();
        } else {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            let mut itc_suf: String = ".clone()".to_string();
            item_code = [item_code.as_str(), itc_suf.as_str()].concat();
        }
        item_codes.push(item_code.clone());
        cur = item_pos;
    }
    let mut sep: String = ", ".to_string();
    let mut items_str: String = s_join_with(item_codes.clone(), sep.clone());
    return make_result(items_str, cur.clone());
}

fn gen_join_items(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
    let mut item_codes: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    while true {
        // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
        if cur >= token_count {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "RBRACKET" {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            break;
        }
        if kind == "COMMA" {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            cur = cur + 1;
            continue;
        }
        let mut item_r: ParseResult = gen_expr(tokens.clone(), cur.clone(), ctx.clone());
        let code = item_r.code;
        let new_pos = item_r.new_pos;
        let mut item_code = code;
        let item_pos = new_pos;
        let mut start_token: Token = tokens[cur as usize].clone();
        let kind = start_token.kind.clone();
        if kind != "STRING" {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            let mut ias_suf: String = ".as_str()".to_string();
            item_code = s_cat(item_code, ias_suf.clone());
        }
        item_codes.push(item_code.clone());
        cur = item_pos;
    }
    let mut sep: String = ", ".to_string();
    let mut items_str: String = s_join_with(item_codes.clone(), sep.clone());
    return make_result(items_str, cur.clone());
}

// transpiler-deor/codegen/decl/stmt/expr/primary.deor
fn gen_unary_method(args_pos: i32, suffix: String, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/primary.deor
    let tokens = ctx.tokens.clone();
    let mut inner_r: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
    let code = inner_r.code;
    let new_pos = inner_r.new_pos;
    let inner_code = code;
    let close = new_pos + 1;
    let mut result_code: String = [inner_code.as_str(), suffix.as_str()].concat();
    return make_result(result_code, close.clone());
}

fn gen_primary(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/primary.deor
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
    // macro: primary_literals (transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor)
    if kind == "INT" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_val: String = value.clone();
        let mut lit_in_float: bool = float_ctx_get();
        if lit_in_float {
            // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
            let mut lit_dot: String = ".0".to_string();
            lit_val = s_cat(lit_val.clone(), lit_dot.clone());
        }
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_val, lit_next.clone());
    }
    if kind == "FLOAT" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(value, lit_next.clone());
    }
    if kind == "STRING" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_debug: String = s_debug(value.clone());
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_debug, lit_next.clone());
    }
    if kind == "KW_TRUE" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_true: String = "true".to_string();
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_true, lit_next.clone());
    }
    if kind == "KW_FALSE" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_false: String = "false".to_string();
        let mut lit_next: i32 = pos + 1.clone();
        return make_result(lit_false, lit_next.clone());
    }
    // macro: primary_list_literal (transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor)
    if kind == "LBRACKET" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor
        let mut ll_inner: i32 = pos + 1.clone();
        let mut ll_items_r: ParseResult = gen_list_items(tokens.clone(), ll_inner.clone(), ctx.clone());
        let code = ll_items_r.code;
        let new_pos = ll_items_r.new_pos;
        let ll_items_code = code;
        let ll_items_pos = new_pos;
        let mut ll_open: String = "vec![".to_string();
        let mut ll_close: String = "]".to_string();
        let mut ll_code: String = [ll_open.as_str(), ll_items_code.as_str(), ll_close.as_str()].concat();
        let mut ll_after: i32 = ll_items_pos + 1.clone();
        return make_result(ll_code, ll_after.clone());
    }
    // macro: primary_paren_expr (transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor)
    if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
        let mut pe_peek: i32 = pos + 1.clone();
        if pe_peek < token_count {
            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
            let mut pe_peek_tok: Token = tokens[pe_peek as usize].clone();
            let kind = pe_peek_tok.kind.clone();
            if kind == "KW_AVOW" {
                // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                let mut pe_expr_pos: i32 = pe_peek + 1.clone();
                let mut pe_expr_r: ParseResult = gen_expr(tokens.clone(), pe_expr_pos.clone(), ctx.clone());
                let code = pe_expr_r.code;
                let new_pos = pe_expr_r.new_pos;
                let pe_expr_code = code;
                let pe_after = new_pos + 1;
                let mut pe_unw: String = ".unwrap()".to_string();
                let mut pe_unwrap_code: String = [pe_expr_code.as_str(), pe_unw.as_str()].concat();
                return make_result(pe_unwrap_code, pe_after.clone());
            }
            let mut pe_is_struct: bool = true;
            let mut pe_scan: i32 = pe_peek.clone();
            while pe_scan < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                let mut pe_scan_tok: Token = tokens[pe_scan as usize].clone();
                let kind = pe_scan_tok.kind.clone();
                if kind == "RPAREN" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    break;
                }
                if kind == "IDENT" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    pe_scan = pe_scan + 1;
                    continue;
                }
                if kind == "COMMA" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    pe_scan = pe_scan + 1;
                    continue;
                }
                pe_is_struct = false;
                break;
            }
            if pe_is_struct {
                // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                let mut pe_fields: Vec<String> = Vec::new();
                let mut pe_cur: i32 = pe_peek.clone();
                while pe_cur < token_count {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    let mut pe_field_tok: Token = tokens[pe_cur as usize].clone();
                    let kind = pe_field_tok.kind.clone();
                    let value = pe_field_tok.value.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        pe_cur = pe_cur + 1;
                        break;
                    } else if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        pe_cur = pe_cur + 1;
                    } else if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        pe_fields.push(value.clone());
                        pe_cur = pe_cur + 1;
                    } else {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        pe_cur = pe_cur + 1;
                    }
                }
                let mut pe_struct_name: String = find_struct_for_fields(struct_reg.clone(), pe_fields.clone());
                let mut pe_sep: String = ", ".to_string();
                let mut pe_fields_code: String = s_join_with(pe_fields.clone(), pe_sep.clone());
                let mut pe_sco: String = " { ".to_string();
                let mut pe_scc: String = " }".to_string();
                let mut pe_struct_code: String = [pe_struct_name.as_str(), pe_sco.as_str(), pe_fields_code.as_str(), pe_scc.as_str()].concat();
                return make_result(pe_struct_code, pe_cur.clone());
            }
            let mut pe_inner_r: ParseResult = gen_expr(tokens.clone(), pe_peek.clone(), ctx.clone());
            let code = pe_inner_r.code;
            let new_pos = pe_inner_r.new_pos;
            let pe_inner_code = code;
            let pe_after_inner = new_pos + 1;
            let mut pe_open: String = "(".to_string();
            let mut pe_close: String = ")".to_string();
            let mut pe_grouped: String = [pe_open.as_str(), pe_inner_code.as_str(), pe_close.as_str()].concat();
            return make_result(pe_grouped, pe_after_inner.clone());
        }
    }
    // macro: primary_prefix_ops (transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor)
    if kind == "KW_GIVEUP" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
        let mut po_inner: i32 = pos + 1.clone();
        let mut po_r: ParseResult = gen_primary(tokens.clone(), po_inner.clone(), ctx.clone());
        return po_r;
    }
    if kind == "KW_NOT" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
        let mut po_operand: i32 = pos + 1.clone();
        let mut po_operand_r: ParseResult = gen_primary(tokens.clone(), po_operand.clone(), ctx.clone());
        let code = po_operand_r.code;
        let new_pos = po_operand_r.new_pos;
        let po_code = code;
        let po_end = new_pos;
        let mut po_bang: String = "!".to_string();
        let mut po_not_code: String = [po_bang.as_str(), po_code.as_str()].concat();
        return make_result(po_not_code, po_end.clone());
    }
    // macro: primary_ident_expr (transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor)
    if kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
        let mut ie_next: i32 = pos + 1.clone();
        if ie_next < token_count {
            // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
            let mut ie_next_tok: Token = tokens[ie_next as usize].clone();
            let kind = ie_next_tok.kind.clone();
            if kind == "LPAREN" {
                // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                let mut ie_func: String = value.clone();
                let mut ie_args_pos: i32 = ie_next + 1.clone();
                if ie_func == "len" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut ie_len_sfx: String = ".len() as i32".to_string();
                    let mut ie_len_r: ParseResult = gen_unary_method(ie_args_pos.clone(), ie_len_sfx.clone(), ctx.clone());
                    let code = ie_len_r.code;
                    let new_pos = ie_len_r.new_pos;
                    let ie_len_code = code;
                    let ie_len_end = new_pos;
                    let mut ie_lp: String = "(".to_string();
                    let mut ie_rp: String = ")".to_string();
                    let mut ie_len_wrapped: String = [ie_lp.as_str(), ie_len_code.as_str(), ie_rp.as_str()].concat();
                    return make_result(ie_len_wrapped, ie_len_end.clone());
                } else if ie_func == "crash" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut ie_crash_r: ParseResult = gen_call_args(tokens.clone(), ie_args_pos.clone(), ctx.clone());
                    let code = ie_crash_r.code;
                    let new_pos = ie_crash_r.new_pos;
                    let ie_crash_code = code;
                    let ie_crash_end = new_pos;
                    let mut ie_pan_pfx: String = "panic!(\"{}\", ".to_string();
                    let mut ie_pan_sfx: String = ")".to_string();
                    let mut ie_panic_code: String = [ie_pan_pfx.as_str(), ie_crash_code.as_str(), ie_pan_sfx.as_str()].concat();
                    let mut ie_after_crash: i32 = ie_crash_end + 1.clone();
                    return make_result(ie_panic_code, ie_after_crash.clone());
                }
                if ie_func == "s_join" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut sj_arg_tok: Token = tokens[ie_args_pos as usize].clone();
                    let kind = sj_arg_tok.kind.clone();
                    if kind == "LBRACKET" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                        let mut sj_inner: i32 = ie_args_pos + 1.clone();
                        let mut sj_r: ParseResult = gen_join_items(tokens.clone(), sj_inner.clone(), ctx.clone());
                        let code = sj_r.code;
                        let new_pos = sj_r.new_pos;
                        let sj_items = code;
                        let sj_end = new_pos;
                        let mut sj_after: i32 = sj_end + 2.clone();
                        let mut sj_open: String = "[".to_string();
                        let mut sj_close: String = "].concat()".to_string();
                        let mut sj_code: String = [sj_open.as_str(), sj_items.as_str(), sj_close.as_str()].concat();
                        return make_result(sj_code, sj_after.clone());
                    }
                }
                let mut ie_args_r: ParseResult = gen_call_args(tokens.clone(), ie_args_pos.clone(), ctx.clone());
                let code = ie_args_r.code;
                let new_pos = ie_args_r.new_pos;
                let ie_args_code = code;
                let ie_args_end = new_pos;
                let mut ie_after: i32 = ie_args_end + 1.clone();
                let mut ie_lp: String = "(".to_string();
                let mut ie_rp: String = ")".to_string();
                let mut ie_call_code: String = [ie_func.as_str(), ie_lp.as_str(), ie_args_code.as_str(), ie_rp.as_str()].concat();
                return make_result(ie_call_code, ie_after.clone());
            }
            if kind == "KW_AT" {
                // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                let mut ie_idx_pos: i32 = ie_next + 1.clone();
                let mut ie_idx_r: ParseResult = gen_primary(tokens.clone(), ie_idx_pos.clone(), ctx.clone());
                let code = ie_idx_r.code;
                let new_pos = ie_idx_r.new_pos;
                let ie_idx_code = code;
                let ie_idx_end = new_pos;
                let mut ie_idx_mid: String = "[".to_string();
                let mut ie_idx_sfx: String = " as usize].clone()".to_string();
                let mut ie_idx_expr: String = [value.as_str(), ie_idx_mid.as_str(), ie_idx_code.as_str(), ie_idx_sfx.as_str()].concat();
                return make_result(ie_idx_expr, ie_idx_end.clone());
            }
        }
        let mut ie_variant_enum: String = reg_get(variant_reg.clone(), value.clone());
        if !is_empty(ie_variant_enum.clone()) {
            // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
            let mut ie_dbl: String = "::".to_string();
            let mut ie_variant_code: String = [ie_variant_enum.as_str(), ie_dbl.as_str(), value.as_str()].concat();
            return make_result(ie_variant_code, ie_next.clone());
        }
        return make_result(value, ie_next.clone());
    }
    // transpiler-deor/codegen/decl/stmt/expr/primary.deor
    let mut unknown: String = "/* unknown_primary */".to_string();
    let mut next: i32 = pos + 1.clone();
    return make_result(unknown, next.clone());
}

// transpiler-deor/codegen/decl/stmt/expr/expr.deor
fn is_binary_op(kind: String) -> bool {
    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
    if kind == "PLUS" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "MINUS" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "STAR" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "SLASH" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "PERCENT" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "GT" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "LT" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "GTE" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "LTE" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "KW_IS" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "KW_AND" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    if kind == "KW_OR" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return true;
    }
    return false;
}

fn map_op(operator: String) -> String {
    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
    if operator == "is" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return "==".to_string();
    }
    if operator == "is not" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return "!=".to_string();
    }
    if operator == "and" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return "&&".to_string();
    }
    if operator == "or" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return "||".to_string();
    }
    if operator == ">" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return ">".to_string();
    }
    if operator == "<" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return "<".to_string();
    }
    if operator == ">=" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return ">=".to_string();
    }
    if operator == "<=" {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        return "<=".to_string();
    }
    return operator;
}

fn gen_expr(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut pre_ctx_was: bool = float_ctx_get();
    let mut expr_has_float: bool = false;
    let mut pre_scan: i32 = pos.clone();
    let mut pre_depth: i32 = 0;
    while pre_scan < token_count {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        let mut pre_tok: Token = tokens[pre_scan as usize].clone();
        let kind = pre_tok.kind.clone();
        if kind == "FLOAT" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            expr_has_float = true;
            break;
        }
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            break;
        }
        if kind == "EOF" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            break;
        }
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            pre_depth = pre_depth + 1;
        }
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            if pre_depth == 0 {
                // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                break;
            }
            pre_depth = pre_depth - 1;
        }
        pre_scan = pre_scan + 1;
    }
    if expr_has_float {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        float_ctx_enable();
    }
    let mut primary_r: ParseResult = gen_primary(tokens.clone(), pos.clone(), ctx.clone());
    let code = primary_r.code;
    let new_pos = primary_r.new_pos;
    let mut left_code = code;
    let mut cur_pos = new_pos;
    let mut first_token: Token = tokens[pos as usize].clone();
    let kind = first_token.kind.clone();
    let mut left_has_str: bool = kind == "STRING".clone();
    while cur_pos < token_count {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        let mut operator_token: Token = tokens[cur_pos as usize].clone();
        let kind = operator_token.kind.clone();
        let value = operator_token.value.clone();
        if !is_binary_op(kind.clone()) {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            break;
        }
        let mut operator_str: String = value.clone();
        let mut after_op: i32 = cur_pos + 1.clone();
        if kind == "KW_IS" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            if after_op < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                let mut maybe_not: Token = tokens[after_op as usize].clone();
                let kind = maybe_not.kind.clone();
                if kind == "KW_NOT" {
                    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                    operator_str = "is not".to_string();
                    after_op = after_op + 1;
                }
                if kind == "KW_EMPTY" {
                    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                    let mut ie_sfx: String = ".is_empty()".to_string();
                    left_code = s_cat(left_code, ie_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
                if kind == "KW_VALID" {
                    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                    let mut iv_sfx: String = ".is_some()".to_string();
                    left_code = s_cat(left_code, iv_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
            }
        }
        if operator_str == "is not" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            if after_op < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                let mut maybe_empty: Token = tokens[after_op as usize].clone();
                let kind = maybe_empty.kind.clone();
                if kind == "KW_EMPTY" {
                    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                    let mut ine_pfx: String = "!".to_string();
                    let mut ine_sfx: String = ".is_empty()".to_string();
                    left_code = s_cat(ine_pfx.clone(), left_code);
                    left_code = s_cat(left_code, ine_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
                if kind == "KW_VALID" {
                    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                    let mut inv_pfx: String = "!(".to_string();
                    let mut inv_sfx: String = ".is_some())".to_string();
                    left_code = s_cat(inv_pfx.clone(), left_code);
                    left_code = s_cat(left_code, inv_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
            }
        }
        let mut rhs_r: ParseResult = gen_primary(tokens.clone(), after_op.clone(), ctx.clone());
        let code = rhs_r.code;
        let new_pos = rhs_r.new_pos;
        let mut rhs_code = code;
        let rhs_pos = new_pos;
        let mut rust_op: String = map_op(operator_str.clone());
        if operator_str == "+" {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            let mut left_token: Token = tokens[pos as usize].clone();
            let kind = left_token.kind.clone();
            let mut left_is_str: bool = kind == "STRING".clone();
            let mut rhs_token: Token = tokens[after_op as usize].clone();
            let kind = rhs_token.kind.clone();
            let mut rhs_is_str: bool = kind == "STRING".clone();
            let mut rhs_is_ident: bool = kind == "IDENT".clone();
            if left_is_str {
                // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                let mut fmt_pfx: String = "format!(\"{}{}\", ".to_string();
                let mut fmt_mid: String = ", ".to_string();
                let mut fmt_sfx: String = ")".to_string();
                left_code = s_cat(fmt_pfx.clone(), left_code);
                left_code = s_cat(left_code, fmt_mid.clone());
                left_code = s_cat(left_code, rhs_code.clone());
                left_code = s_cat(left_code, fmt_sfx.clone());
                left_has_str = true;
                cur_pos = rhs_pos;
                continue;
            }
            if left_has_str {
                // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                if rhs_is_ident {
                    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                    let mut astr_sfx: String = ".as_str()".to_string();
                    rhs_code = s_cat(rhs_code, astr_sfx.clone());
                }
            }
            if rhs_is_str {
                // transpiler-deor/codegen/decl/stmt/expr/expr.deor
                left_has_str = true;
            }
        }
        let mut op_sp: String = " ".to_string();
        left_code = s_cat(left_code, op_sp.clone());
        left_code = s_cat(left_code, rust_op.clone());
        left_code = s_cat(left_code, op_sp.clone());
        left_code = s_cat(left_code, rhs_code.clone());
        cur_pos = rhs_pos;
    }
    if expr_has_float {
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        if !pre_ctx_was {
            // transpiler-deor/codegen/decl/stmt/expr/expr.deor
            float_ctx_disable();
        }
    }
    return make_result(left_code, cur_pos.clone());
}

// transpiler-deor/codegen/decl/stmt/helpers.deor
fn emit_val(val_code: String, val_kind: String) -> String {
    // transpiler-deor/codegen/decl/stmt/helpers.deor
    if val_kind == "STRING" {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        let mut tos_sfx: String = ".to_string()".to_string();
        return [val_code.as_str(), tos_sfx.as_str()].concat();
    }
    if val_kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        let mut cln_sfx: String = ".clone()".to_string();
        return [val_code.as_str(), cln_sfx.as_str()].concat();
    }
    return val_code;
}

fn make_destruct_code(var_name: String, depth: i32, ctx: RcCtx) -> String {
    // transpiler-deor/codegen/decl/stmt/helpers.deor
    let struct_reg = ctx.struct_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let var_type_reg = ctx.var_type_reg.clone();
    let mut struct_type: String = reg_get(var_type_reg.clone(), var_name.clone());
    let mut fields_str: String = reg_get(struct_reg.clone(), struct_type.clone());
    if is_empty(fields_str.clone()) {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        return "".to_string();
    }
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut comma: String = ",".to_string();
    let mut fields: Vec<String> = s_split(fields_str.clone(), comma.clone());
    let mut field_count: i32 = (fields.len() as i32);
    let mut lines: Vec<String> = Vec::new();
    for i in 0..field_count {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        let mut field: String = fields[i as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            // transpiler-deor/codegen/decl/stmt/helpers.deor
            mut_kw = "mut ".to_string();
        }
        let mut fld_let: String = "let ".to_string();
        let mut fld_eq: String = " = ".to_string();
        let mut fld_dot: String = ".".to_string();
        let mut fld_cln: String = ".clone();".to_string();
        lines.push([pad.as_str(), fld_let.as_str(), mut_kw.as_str(), field.as_str(), fld_eq.as_str(), var_name.as_str(), fld_dot.as_str(), field.as_str(), fld_cln.as_str()].concat().clone());
    }
    let mut code: String = s_join_nl(lines.clone());
    let mut newline: String = "\n".to_string();
    return s_cat(code.clone(), newline.clone());
}

// transpiler-deor/codegen/decl/stmt/destructure.deor
fn gen_destructure(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // macro: initialize_gen_destructure (transpiler-deor/codegen/decl/stmt/macros/initialize_gen_destructure.deor)
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
    // macro: for_collect_fields_into_fields_list (transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor)
    while cur < token_count {
        // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
        let mut field_token: Token = tokens[cur as usize].clone();
        let kind = field_token.kind.clone();
        let value = field_token.value.clone();
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            cur = cur + 1;
            break;
        } else if kind == "COMMA" {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            cur = cur + 1;
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            fields.push(value.clone());
            cur = cur + 1;
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut val_pos: i32 = cur + 1.clone();
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = ge_r.code;
    let new_pos = ge_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = (fields.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut field_suffix: String = ".clone();".to_string();
    // macro: for_build_fields (transpiler-deor/codegen/decl/stmt/macros/for_build_fields.deor)
    for field_index in 0..field_count {
        // transpiler-deor/codegen/decl/stmt/macros/for_build_fields.deor
        let mut field: String = fields[field_index as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/for_build_fields.deor
            mut_kw = "mut ".to_string();
        }
        let mut dst_let: String = "let ".to_string();
        let mut dst_eq: String = " = ".to_string();
        let mut dst_dot: String = ".".to_string();
        dest_lines.push([pad.as_str(), dst_let.as_str(), mut_kw.as_str(), field.as_str(), dst_eq.as_str(), val_code.as_str(), dst_dot.as_str(), field.as_str(), field_suffix.as_str()].concat().clone());
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut after: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    let mut newline: String = "\n".to_string();
    dest_code = s_cat(dest_code.clone(), newline.clone());
    return make_result(dest_code, after.clone());
}

fn gen_move_destructure(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // macro: initialize_gen_destructure (transpiler-deor/codegen/decl/stmt/macros/initialize_gen_destructure.deor)
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
    // macro: for_collect_fields_into_fields_list (transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor)
    while cur < token_count {
        // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
        let mut field_token: Token = tokens[cur as usize].clone();
        let kind = field_token.kind.clone();
        let value = field_token.value.clone();
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            cur = cur + 1;
            break;
        } else if kind == "COMMA" {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            cur = cur + 1;
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            fields.push(value.clone());
            cur = cur + 1;
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut val_pos: i32 = cur + 1.clone();
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = ge_r.code;
    let new_pos = ge_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i32 = (fields.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut field_suffix: String = ";".to_string();
    // macro: for_build_fields (transpiler-deor/codegen/decl/stmt/macros/for_build_fields.deor)
    for field_index in 0..field_count {
        // transpiler-deor/codegen/decl/stmt/macros/for_build_fields.deor
        let mut field: String = fields[field_index as usize].clone();
        let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
        let mut mut_kw: String = "".to_string();
        if is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/for_build_fields.deor
            mut_kw = "mut ".to_string();
        }
        let mut dst_let: String = "let ".to_string();
        let mut dst_eq: String = " = ".to_string();
        let mut dst_dot: String = ".".to_string();
        dest_lines.push([pad.as_str(), dst_let.as_str(), mut_kw.as_str(), field.as_str(), dst_eq.as_str(), val_code.as_str(), dst_dot.as_str(), field.as_str(), field_suffix.as_str()].concat().clone());
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut after: i32 = adv_nl_ref(val_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    let mut newline: String = "\n".to_string();
    dest_code = s_cat(dest_code.clone(), newline.clone());
    return make_result(dest_code, after.clone());
}

// transpiler-deor/codegen/decl/stmt/block.deor
fn gen_block(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/block.deor
    let tokens = ctx.tokens.clone();
    let mut stmts: Vec<String> = Vec::new();
    let mut cur: i32 = pos.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut last_file: String = "".to_string();
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    while true {
        // transpiler-deor/codegen/decl/stmt/block.deor
        if cur >= token_count {
            // transpiler-deor/codegen/decl/stmt/block.deor
            break;
        }
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let file = token.file.clone();
        if kind == "DEDENT" || kind == "EOF" {
            // transpiler-deor/codegen/decl/stmt/block.deor
            if kind == "DEDENT" {
                // transpiler-deor/codegen/decl/stmt/block.deor
                cur = cur + 1;
            }
            break;
        }
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/stmt/block.deor
            cur = cur + 1;
            continue;
        }
        if kind == "MACRO_MARKER" {
            // transpiler-deor/codegen/decl/stmt/block.deor
            let mut mc_prefix: String = "// macro: ".to_string();
            let mut mc_open: String = " (".to_string();
            let mut mc_close: String = ")".to_string();
            let mut mc_nl: String = "\n".to_string();
            let mut mc1: String = s_cat(pad.clone(), mc_prefix.clone());
            let mut mc2: String = s_cat(mc1.clone(), value.clone());
            let mut mc3: String = s_cat(mc2.clone(), mc_open.clone());
            let mut mc4: String = s_cat(mc3.clone(), file.clone());
            let mut mc5: String = s_cat(mc4.clone(), mc_close.clone());
            let mut macro_comment: String = s_cat(mc5.clone(), mc_nl.clone());
            stmts.push(macro_comment.clone());
            last_file = file;
            cur = cur + 1;
            continue;
        }
        if file != last_file {
            // transpiler-deor/codegen/decl/stmt/block.deor
            let mut fc_slash: String = "// ".to_string();
            let mut fc_nl: String = "\n".to_string();
            let mut fc1: String = s_cat(pad.clone(), fc_slash.clone());
            let mut fc2: String = s_cat(fc1.clone(), file.clone());
            let mut file_comment: String = s_cat(fc2.clone(), fc_nl.clone());
            stmts.push(file_comment.clone());
            last_file = file;
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

// transpiler-deor/codegen/decl/stmt/if.deor
fn gen_if_branch(cond_pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/if.deor
    let tokens = ctx.tokens.clone();
    let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let code = cond_r.code;
    let new_pos = cond_r.new_pos;
    let cond_code = code;
    let cond_end = new_pos;
    let mut blk_start: i32 = skip_to_body_ref(tokens.clone(), cond_end.clone());
    // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
    let mut blk_depth: i32 = depth + 1.clone();
    let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
    let code = blk_r.code;
    let new_pos = blk_r.new_pos;
    let blk_code = code;
    let blk_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/if.deor
    let mut brc_open: String = " {\n".to_string();
    let mut combined: String = [cond_code.as_str(), brc_open.as_str(), blk_code.as_str()].concat();
    return make_result(combined, blk_end.clone());
}

fn gen_if(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/if.deor
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut if_cond_pos: i32 = pos + 1.clone();
    let mut then_r: ParseResult = gen_if_branch(if_cond_pos.clone(), depth.clone(), ctx.clone());
    let code = then_r.code;
    let new_pos = then_r.new_pos;
    let then_code = code;
    let mut if_kw: String = "if ".to_string();
    let mut brc_cl: String = "}".to_string();
    let mut result_code: String = [pad.as_str(), if_kw.as_str(), then_code.as_str(), pad.as_str(), brc_cl.as_str()].concat();
    let mut cur = new_pos;
    while true {
        // transpiler-deor/codegen/decl/stmt/if.deor
        if cur >= token_count {
            // transpiler-deor/codegen/decl/stmt/if.deor
            break;
        }
        let mut else_token: Token = tokens[cur as usize].clone();
        let kind = else_token.kind.clone();
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/stmt/if.deor
            cur = cur + 1;
            continue;
        }
        if kind != "KW_ELSE" {
            // transpiler-deor/codegen/decl/stmt/if.deor
            break;
        }
        let mut after_else: i32 = cur + 1.clone();
        if after_else >= token_count {
            // transpiler-deor/codegen/decl/stmt/if.deor
            break;
        }
        let mut after_else_token: Token = tokens[after_else as usize].clone();
        let kind = after_else_token.kind.clone();
        if kind == "KW_IF" {
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut ei_cond: i32 = after_else + 1.clone();
            let mut ei_r: ParseResult = gen_if_branch(ei_cond.clone(), depth.clone(), ctx.clone());
            let code = ei_r.code;
            let new_pos = ei_r.new_pos;
            let ei_code = code;
            let mut eli_kw: String = " else if ".to_string();
            let mut eli_cl: String = "}".to_string();
            result_code = s_cat(result_code, eli_kw.clone());
            result_code = s_cat(result_code, ei_code.clone());
            result_code = s_cat(result_code, pad.clone());
            result_code = s_cat(result_code, eli_cl.clone());
            cur = new_pos;
        } else {
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut blk_start: i32 = skip_to_body_ref(tokens.clone(), after_else.clone());
            // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
            let mut blk_depth: i32 = depth + 1.clone();
            let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
            let code = blk_r.code;
            let new_pos = blk_r.new_pos;
            let blk_code = code;
            let blk_end = new_pos;
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut els_kw: String = " else {\n".to_string();
            let mut els_cl: String = "}".to_string();
            result_code = s_cat(result_code, els_kw.clone());
            result_code = s_cat(result_code, blk_code.clone());
            result_code = s_cat(result_code, pad.clone());
            result_code = s_cat(result_code, els_cl.clone());
            cur = blk_end;
            break;
        }
    }
    let mut if_newline: String = "\n".to_string();
    result_code = s_cat(result_code, if_newline.clone());
    return make_result(result_code, cur.clone());
}

// transpiler-deor/codegen/decl/stmt/for.deor
fn gen_for(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/for.deor
    let tokens = ctx.tokens.clone();
    let mut token_count: i32 = (tokens.len() as i32);
    let mut indent: String = "    ".to_string();
    let mut pad: String = s_repeat(indent.clone(), depth.clone());
    let mut next_pos: i32 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_IF" {
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut cond_pos: i32 = next_pos + 1.clone();
        let mut val_pos = cond_pos;
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut blk_start: i32 = skip_to_body_ref(tokens.clone(), val_end.clone());
        // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
        let mut blk_depth: i32 = depth + 1.clone();
        let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
        let code = blk_r.code;
        let new_pos = blk_r.new_pos;
        let blk_code = code;
        let blk_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut whl_kw: String = "while ".to_string();
        let mut whl_ob: String = " {\n".to_string();
        let mut whl_cb: String = "}\n".to_string();
        let mut while_code: String = [pad.as_str(), whl_kw.as_str(), val_code.as_str(), whl_ob.as_str(), blk_code.as_str(), pad.as_str(), whl_cb.as_str()].concat();
        return make_result(while_code, blk_end.clone());
    }
    if kind == "KW_GIVEUP" {
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut lparen_pos: i32 = next_pos + 1.clone();
        let mut var_pos: i32 = lparen_pos + 1.clone();
        let mut var_tok: Token = tokens[var_pos as usize].clone();
        let value = var_tok.value.clone();
        let mut giveup_var: String = value.clone();
        let mut in_pos: i32 = var_pos + 1.clone();
        let mut iter_pos: i32 = in_pos + 1.clone();
        let mut val_pos = iter_pos;
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut iter_next: i32 = val_end + 1.clone();
        let mut blk_start: i32 = skip_to_body_ref(tokens.clone(), iter_next.clone());
        // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
        let mut blk_depth: i32 = depth + 1.clone();
        let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
        let code = blk_r.code;
        let new_pos = blk_r.new_pos;
        let blk_code = code;
        let blk_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut gfr_kw: String = "for ".to_string();
        let mut gfr_in: String = " in ".to_string();
        let mut gfr_ob: String = " {\n".to_string();
        let mut gfr_cb: String = "}\n".to_string();
        let mut for_code: String = [pad.as_str(), gfr_kw.as_str(), giveup_var.as_str(), gfr_in.as_str(), val_code.as_str(), gfr_ob.as_str(), blk_code.as_str(), pad.as_str(), gfr_cb.as_str()].concat();
        return make_result(for_code, blk_end.clone());
    }
    let mut var_name: String = "_".to_string();
    let mut iter_pos: i32 = 0;
    if kind == "KW_IN" {
        // transpiler-deor/codegen/decl/stmt/for.deor
        iter_pos = next_pos + 1;
    } else {
        // transpiler-deor/codegen/decl/stmt/for.deor
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
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut lparen: i32 = iter_pos + 1.clone();
        let mut first_pos: i32 = lparen + 1.clone();
        let mut val_pos = first_pos;
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut comma_token: Token = tokens[val_end as usize].clone();
        let kind = comma_token.kind.clone();
        let mut has_start: bool = kind == "COMMA".clone();
        if has_start {
            // transpiler-deor/codegen/decl/stmt/for.deor
/* unhandled(IDENT) */
            let first_code = val_code;
            let mut val_pos: i32 = val_end + 1.clone();
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            // transpiler-deor/codegen/decl/stmt/for.deor
            let mut rng_dot: String = "..".to_string();
            range_expr = [first_code.as_str(), rng_dot.as_str(), val_code.as_str()].concat();
            body_tok_pos = val_end + 1;
        } else {
            // transpiler-deor/codegen/decl/stmt/for.deor
            let mut rng0_pfx: String = "0..".to_string();
            range_expr = [rng0_pfx.as_str(), val_code.as_str()].concat();
            body_tok_pos = val_end + 1;
        }
    } else if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut start_pos: i32 = iter_pos + 1.clone();
        let mut val_pos = start_pos;
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
/* unhandled(IDENT) */
        let start_code = val_code;
        let mut val_pos: i32 = val_end + 1.clone();
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut rng2_dot: String = "..".to_string();
        range_expr = [start_code.as_str(), rng2_dot.as_str(), val_code.as_str()].concat();
        body_tok_pos = val_end + 1;
    } else {
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut val_pos = iter_pos;
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/for.deor
        let mut range_expr = val_code;
        body_tok_pos = val_end;
    }
    let mut blk_start: i32 = skip_to_body_ref(tokens.clone(), body_tok_pos.clone());
    // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
    let mut blk_depth: i32 = depth + 1.clone();
    let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
    let code = blk_r.code;
    let new_pos = blk_r.new_pos;
    let blk_code = code;
    let blk_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/for.deor
    let mut frc_kw: String = "for ".to_string();
    let mut frc_in: String = " in ".to_string();
    let mut frc_ob: String = " {\n".to_string();
    let mut frc_cb: String = "}\n".to_string();
    let mut for_code: String = [pad.as_str(), frc_kw.as_str(), var_name.as_str(), frc_in.as_str(), range_expr.as_str(), frc_ob.as_str(), blk_code.as_str(), pad.as_str(), frc_cb.as_str()].concat();
    return make_result(for_code, blk_end.clone());
}

// transpiler-deor/codegen/decl/stmt/as_binding.deor
fn gen_as_binding(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/as_binding.deor
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
        // transpiler-deor/codegen/decl/stmt/as_binding.deor
        let mut aas_is_struct: bool = true;
        let mut aas_peek: i32 = after_as + 1.clone();
        while aas_peek < token_count {
            // transpiler-deor/codegen/decl/stmt/as_binding.deor
            let mut aas_peek_tok: Token = tokens[aas_peek as usize].clone();
            let kind = aas_peek_tok.kind.clone();
            if kind == "RPAREN" {
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                break;
            }
            if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                aas_peek = aas_peek + 1;
                continue;
            }
            if kind == "COMMA" {
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                aas_peek = aas_peek + 1;
                continue;
            }
            aas_is_struct = false;
            break;
        }
        if aas_is_struct {
            // transpiler-deor/codegen/decl/stmt/as_binding.deor
            let mut aas_fields: Vec<String> = Vec::new();
            let mut aas_fend: i32 = after_as + 1.clone();
            while aas_fend < token_count {
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                let mut aas_field_tok: Token = tokens[aas_fend as usize].clone();
                let kind = aas_field_tok.kind.clone();
                let value = aas_field_tok.value.clone();
                if kind == "RPAREN" {
                    // transpiler-deor/codegen/decl/stmt/as_binding.deor
                    aas_fend = aas_fend + 1;
                    break;
                } else if kind == "COMMA" {
                    // transpiler-deor/codegen/decl/stmt/as_binding.deor
                    aas_fend = aas_fend + 1;
                } else if kind == "IDENT" {
                    // transpiler-deor/codegen/decl/stmt/as_binding.deor
                    aas_fields.push(value.clone());
                    aas_fend = aas_fend + 1;
                }
            }
            let mut aas_struct: String = find_struct_for_fields(struct_reg.clone(), aas_fields.clone());
            let mut var_name: String = ident_name.clone();
            // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
            let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
            let mut mut_kw: String = "".to_string();
            if mg_is_mut {
                // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                mut_kw = "mut ".to_string();
            }
            // transpiler-deor/codegen/decl/stmt/as_binding.deor
            let mut aas_fcount: i32 = (aas_fields.len() as i32);
            let mut aas_pairs: Vec<String> = Vec::new();
            for aas_fi in 0..aas_fcount {
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                let mut aas_fname: String = aas_fields[aas_fi as usize].clone();
                let mut aas_fp_sep: String = ": ".to_string();
                let mut aas_fp_cln: String = ".clone()".to_string();
                aas_pairs.push([aas_fname.as_str(), aas_fp_sep.as_str(), aas_fname.as_str(), aas_fp_cln.as_str()].concat().clone());
            }
            let mut aas_fld_sep: String = ", ".to_string();
            let mut aas_fields_code: String = s_join_with(aas_pairs.clone(), aas_fld_sep.clone());
            let mut aas_let: String = "let ".to_string();
            let mut aas_eq: String = " = ".to_string();
            let mut aas_ob: String = " { ".to_string();
            let mut aas_cb: String = " };\n".to_string();
            let mut aas_code: String = [pad.as_str(), aas_let.as_str(), mut_kw.as_str(), ident_name.as_str(), aas_eq.as_str(), aas_struct.as_str(), aas_ob.as_str(), aas_fields_code.as_str(), aas_cb.as_str()].concat();
            return make_nl_result(aas_code, aas_fend.clone(), tokens.clone());
        }
    }
    if kind == "KW_EMPTY" {
        // transpiler-deor/codegen/decl/stmt/as_binding.deor
        let mut aas_emp_pfx: String = "let mut ".to_string();
        let mut aas_emp_sfx: String = " = Vec::new();\n".to_string();
        let mut aas_empty_code: String = [pad.as_str(), aas_emp_pfx.as_str(), ident_name.as_str(), aas_emp_sfx.as_str()].concat();
        let mut aas_after_empty: i32 = after_as + 1.clone();
        return make_nl_result(aas_empty_code, aas_after_empty.clone(), tokens.clone());
    }
    if kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/as_binding.deor
        let mut aas_with_pos: i32 = after_as + 1.clone();
        if aas_with_pos < token_count {
            // transpiler-deor/codegen/decl/stmt/as_binding.deor
            let mut aas_with_tok: Token = tokens[aas_with_pos as usize].clone();
            let kind = aas_with_tok.kind.clone();
            if kind == "KW_WITH" {
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                let mut aas_src: String = after_as_value.clone();
                let mut aas_lp: i32 = aas_with_pos + 1.clone();
                let mut aas_ovr: Vec<String> = Vec::new();
                let mut aas_wend: i32 = aas_lp + 1.clone();
                while aas_wend < token_count {
                    // transpiler-deor/codegen/decl/stmt/as_binding.deor
                    let mut aas_wtok: Token = tokens[aas_wend as usize].clone();
                    let kind = aas_wtok.kind.clone();
                    let value = aas_wtok.value.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/as_binding.deor
                        aas_wend = aas_wend + 1;
                        break;
                    }
                    if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/as_binding.deor
                        aas_wend = aas_wend + 1;
                        continue;
                    }
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/as_binding.deor
                        aas_ovr.push(value.clone());
                        aas_wend = aas_wend + 1;
                    }
                }
                let mut aas_first: String = aas_ovr[0 as usize].clone();
                let mut aas_struct_w: String = find_struct_for_field(struct_reg.clone(), aas_first.clone());
                let mut aas_wsep: String = ", ".to_string();
                let mut aas_wfields: String = s_join_with(aas_ovr.clone(), aas_wsep.clone());
                let mut var_name: String = ident_name.clone();
                // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
                let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                let mut mut_kw: String = "".to_string();
                if mg_is_mut {
                    // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                    mut_kw = "mut ".to_string();
                }
                // transpiler-deor/codegen/decl/stmt/as_binding.deor
                let mut aas_wlet: String = "let ".to_string();
                let mut aas_weq: String = " = ".to_string();
                let mut aas_wob: String = " { ".to_string();
                let mut aas_wsp: String = ", ..".to_string();
                let mut aas_wcb: String = " };\n".to_string();
                let mut aas_with_code: String = [pad.as_str(), aas_wlet.as_str(), mut_kw.as_str(), ident_name.as_str(), aas_weq.as_str(), aas_struct_w.as_str(), aas_wob.as_str(), aas_wfields.as_str(), aas_wsp.as_str(), aas_src.as_str(), aas_wcb.as_str()].concat();
                return make_nl_result(aas_with_code, aas_wend.clone(), tokens.clone());
            }
        }
    }
    let kind = after_as_token.kind.clone();
    let val_pos = after_as;
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = ge_r.code;
    let new_pos = ge_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/as_binding.deor
    let mut aas_is_str: bool = kind == "STRING".clone();
    let mut var_name: String = ident_name.clone();
    // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
    let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
    let mut mut_kw: String = "".to_string();
    if mg_is_mut {
        // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
        mut_kw = "mut ".to_string();
    }
    // transpiler-deor/codegen/decl/stmt/as_binding.deor
    let mut aas_suffix: String = "".to_string();
    if aas_is_str {
        // transpiler-deor/codegen/decl/stmt/as_binding.deor
        aas_suffix = ".to_string()".to_string();
    }
    let mut aas_let2: String = "let ".to_string();
    let mut aas_eq2: String = " = ".to_string();
    let mut aas_sc2: String = ";\n".to_string();
    let mut aas_code2: String = [pad.as_str(), aas_let2.as_str(), mut_kw.as_str(), ident_name.as_str(), aas_eq2.as_str(), val_code.as_str(), aas_suffix.as_str(), aas_sc2.as_str()].concat();
    return make_nl_result(aas_code2, val_end.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/call_stmt.deor
fn gen_call_stmt(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/call_stmt.deor
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
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut after_rparen: i32 = args_pos + 1.clone();
        let mut has_with_arg: bool = false;
        let mut extra_arg: String = "".to_string();
        let mut after_with: i32 = after_rparen.clone();
        if after_rparen < token_count {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut with_token: Token = tokens[after_rparen as usize].clone();
            let kind = with_token.kind.clone();
            if kind == "KW_WITH" {
                // transpiler-deor/codegen/decl/stmt/call_stmt.deor
                let mut extra_pos: i32 = after_rparen + 1.clone();
                if extra_pos < token_count {
                    // transpiler-deor/codegen/decl/stmt/call_stmt.deor
                    let mut extra_token: Token = tokens[extra_pos as usize].clone();
                    let kind = extra_token.kind.clone();
                    let value = extra_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
                        has_with_arg = true;
                        extra_arg = value;
                        after_with = extra_pos + 1;
                    }
                }
            }
        }
        let mut shim_code: String = "".to_string();
        if has_with_arg {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut shm_eq: String = " = ".to_string();
            let mut shm_op: String = "(".to_string();
            let mut shm_cl: String = ".clone(), ".to_string();
            let mut shm_ea: String = ".clone());\n".to_string();
            shim_code = [pad.as_str(), using_var.as_str(), shm_eq.as_str(), ident_name.as_str(), shm_op.as_str(), using_var.as_str(), shm_cl.as_str(), extra_arg.as_str(), shm_ea.as_str()].concat();
        } else {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut shm2_eq: String = " = ".to_string();
            let mut shm2_op: String = "(".to_string();
            let mut shm2_cl: String = ".clone());\n".to_string();
            shim_code = [pad.as_str(), using_var.as_str(), shm2_eq.as_str(), ident_name.as_str(), shm2_op.as_str(), using_var.as_str(), shm2_cl.as_str()].concat();
        }
        let mut re_destruct: String = make_destruct_code(using_var.clone(), depth.clone(), ctx.clone());
        shim_code = s_cat(shim_code.clone(), re_destruct.clone());
        return make_nl_result(shim_code, after_with.clone(), tokens.clone());
    }
    let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
    let code = args_r.code;
    let new_pos = args_r.new_pos;
    let args_code = code;
    let args_end = new_pos;
    let mut after_paren: i32 = args_end + 1.clone();
    let mut call_code: String = "".to_string();
    if ident_name == "print" {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut prt_pfx: String = "println!(\"{}\", ".to_string();
        let mut prt_sfx: String = ");\n".to_string();
        call_code = [pad.as_str(), prt_pfx.as_str(), args_code.as_str(), prt_sfx.as_str()].concat();
    } else if ident_name == "crash" {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut crsh_pfx: String = "panic!(\"{}\", ".to_string();
        let mut crsh_sfx: String = ");\n".to_string();
        call_code = [pad.as_str(), crsh_pfx.as_str(), args_code.as_str(), crsh_sfx.as_str()].concat();
    } else {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut cal_op: String = "(".to_string();
        let mut cal_sfx: String = ");\n".to_string();
        call_code = [pad.as_str(), ident_name.as_str(), cal_op.as_str(), args_code.as_str(), cal_sfx.as_str()].concat();
    }
    return make_nl_result(call_code, after_paren.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/list_mutation.deor
fn gen_list_mutation_stmt(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/list_mutation.deor
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
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut after_at: i32 = next_pos + 1.clone();
        if after_at < token_count {
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
            let mut at_next_token: Token = tokens[after_at as usize].clone();
            let kind = at_next_token.kind.clone();
            let value = at_next_token.value.clone();
            if kind == "IDENT" && value == "end" {
                // transpiler-deor/codegen/decl/stmt/list_mutation.deor
                let mut val_pos: i32 = after_at + 2.clone();
                // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
                let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                let code = ge_r.code;
                let new_pos = ge_r.new_pos;
                let val_code = code;
                let val_end = new_pos;
                // transpiler-deor/codegen/decl/stmt/list_mutation.deor
                let mut val_tok: Token = tokens[val_pos as usize].clone();
                let kind = val_tok.kind.clone();
                let mut app_val: String = emit_val(val_code.clone(), kind.clone());
                let mut app_pfx: String = ".push(".to_string();
                let mut app_sfx: String = ");\n".to_string();
                let mut app_code: String = [pad.as_str(), ident_name.as_str(), app_pfx.as_str(), app_val.as_str(), app_sfx.as_str()].concat();
                return make_nl_result(app_code, val_end.clone(), tokens.clone());
            }
            let mut val_pos = after_at;
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
/* unhandled(IDENT) */
            let idx_code = val_code;
            let mut val_pos: i32 = val_end + 1.clone();
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
            let mut val_tok: Token = tokens[val_pos as usize].clone();
            let kind = val_tok.kind.clone();
            let mut idx_val: String = emit_val(val_code.clone(), kind.clone());
            let mut idw_op: String = "[".to_string();
            let mut idw_mid: String = " as usize] = ".to_string();
            let mut idw_sfx: String = ";\n".to_string();
            let mut idw_code: String = [pad.as_str(), ident_name.as_str(), idw_op.as_str(), idx_code.as_str(), idw_mid.as_str(), idx_val.as_str(), idw_sfx.as_str()].concat();
            return make_nl_result(idw_code, val_end.clone(), tokens.clone());
        }
    }
    if kind == "KW_REMOVE" {
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut idx_pos: i32 = next_pos + 2.clone();
        let mut val_pos = idx_pos;
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut rem_pfx: String = ".remove(".to_string();
        let mut rem_sfx: String = " as usize);\n".to_string();
        let mut rem_code: String = [pad.as_str(), ident_name.as_str(), rem_pfx.as_str(), val_code.as_str(), rem_sfx.as_str()].concat();
        return make_nl_result(rem_code, val_end.clone(), tokens.clone());
    }
    let mut lm_unh_pfx: String = "/* unhandled_list_mut(".to_string();
    let mut lm_unh_sfx: String = ") */\n".to_string();
    let mut lm_unhand: String = [lm_unh_pfx.as_str(), kind.as_str(), lm_unh_sfx.as_str()].concat();
    let mut lm_next: i32 = pos + 1.clone();
    return make_result(lm_unhand, lm_next.clone());
}

// transpiler-deor/codegen/decl/stmt/typed_binding.deor
fn gen_typed_binding(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
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
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        let mut is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
        let mut is_shape: bool = reg_has(shape_reg.clone(), var_type.clone());
        let mut val_next_pos: i32 = val_pos + 1.clone();
        let mut after_empty: i32 = adv_nl_ref(val_next_pos.clone(), tokens.clone());
        if is_validator {
            // transpiler-deor/codegen/decl/stmt/typed_binding.deor
            let mut err_msg: String = "/* error: empty is not valid for validator types — declare without a value instead */\n".to_string();
            let mut err_code: String = [pad.as_str(), err_msg.as_str()].concat();
            return make_result(err_code, after_empty.clone());
        }
        if is_shape {
            // transpiler-deor/codegen/decl/stmt/typed_binding.deor
            let mut sh_pfx: String = "let mut ".to_string();
            let mut sh_mid: String = ": ".to_string();
            let mut sh_sfx: String = " = Vec::new();\n".to_string();
            let mut sh_code: String = [pad.as_str(), sh_pfx.as_str(), var_name.as_str(), sh_mid.as_str(), rust_type.as_str(), sh_sfx.as_str()].concat();
            return make_result(sh_code, after_empty.clone());
        }
        let mut err_msg: String = "/* error: empty is only valid for list shapes */\n".to_string();
        let mut err_code: String = [pad.as_str(), err_msg.as_str()].concat();
        return make_result(err_code, after_empty.clone());
    }
    if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        let mut peek_pos: i32 = val_pos + 1.clone();
        let mut peek_token: Token = tokens[peek_pos as usize].clone();
        let kind = peek_token.kind.clone();
        let mut is_avow_expr: bool = kind == "KW_AVOW".clone();
        let mut is_struct_type: bool = reg_has(struct_reg.clone(), var_type.clone());
        if !is_avow_expr {
            // transpiler-deor/codegen/decl/stmt/typed_binding.deor
            if is_struct_type {
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                let mut struct_fields_str: String = reg_get(struct_reg.clone(), var_type.clone());
                let mut comma: String = ",".to_string();
                let mut field_names: Vec<String> = s_split(struct_fields_str.clone(), comma.clone());
                let mut field_pairs: Vec<String> = Vec::new();
                let mut fend: i32 = val_pos + 1.clone();
                let mut fni: i32 = 0;
                let mut fn_count: i32 = (field_names.len() as i32);
                while fend < token_count {
                    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                    let mut field_tok: Token = tokens[fend as usize].clone();
                    let kind = field_tok.kind.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                        fend = fend + 1;
                        break;
                    }
                    if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                        fend = fend + 1;
                        continue;
                    }
                    let mut val_pos = fend;
                    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
                    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                    let code = ge_r.code;
                    let new_pos = ge_r.new_pos;
                    let val_code = code;
                    let val_end = new_pos;
                    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                    if fni < fn_count {
                        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                        let mut fname: String = field_names[fni as usize].clone();
                        let mut sfp_sep: String = ": ".to_string();
                        let mut sfp_cln: String = ".clone()".to_string();
                        field_pairs.push([fname.as_str(), sfp_sep.as_str(), val_code.as_str(), sfp_cln.as_str()].concat().clone());
                        fni = fni + 1;
                    }
                    fend = val_end;
                }
                let mut sep: String = ", ".to_string();
                let mut fields_code: String = s_join_with(field_pairs.clone(), sep.clone());
                // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
                let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                let mut mut_kw: String = "".to_string();
                if mg_is_mut {
                    // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                    mut_kw = "mut ".to_string();
                }
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                let mut scc_let: String = "let ".to_string();
                let mut scc_eq: String = " = ".to_string();
                let mut scc_ob: String = " { ".to_string();
                let mut scc_cb: String = " };\n".to_string();
                let mut sc_code: String = [pad.as_str(), scc_let.as_str(), mut_kw.as_str(), var_name.as_str(), scc_eq.as_str(), var_type.as_str(), scc_ob.as_str(), fields_code.as_str(), scc_cb.as_str()].concat();
                return make_nl_result(sc_code, fend.clone(), tokens.clone());
            }
        }
        if is_avow_expr {
            // transpiler-deor/codegen/decl/stmt/typed_binding.deor
            let mut inner_pos: i32 = peek_pos + 1.clone();
            let mut val_pos = inner_pos;
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            // transpiler-deor/codegen/decl/stmt/typed_binding.deor
            let mut after_rparen: i32 = val_end + 1.clone();
            let mut suf_unwrap: String = ".unwrap()".to_string();
            let mut suf_unwrap0: String = ".unwrap().0".to_string();
            let mut unwrap_expr: String = s_cat(val_code.clone(), suf_unwrap.clone());
            if var_type == "int" {
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            if var_type == "string" {
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            if var_type == "bool" {
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            if var_type == "float" {
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            let mut awc_let: String = "let ".to_string();
            let mut awc_col: String = ": ".to_string();
            let mut awc_eq: String = " = ".to_string();
            let mut awc_sc: String = ";\n".to_string();
            let mut aw_code: String = [pad.as_str(), awc_let.as_str(), var_name.as_str(), awc_col.as_str(), rust_type.as_str(), awc_eq.as_str(), unwrap_expr.as_str(), awc_sc.as_str()].concat();
            return make_nl_result(aw_code, after_rparen.clone(), tokens.clone());
        }
    }
    if kind == "LBRACKET" {
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        let mut lst_pfx: String = "let mut ".to_string();
        let mut lst_col: String = ": ".to_string();
        let mut lst_eq: String = " = ".to_string();
        let mut lst_sc: String = ";\n".to_string();
        let mut lst_code: String = [pad.as_str(), lst_pfx.as_str(), var_name.as_str(), lst_col.as_str(), rust_type.as_str(), lst_eq.as_str(), val_code.as_str(), lst_sc.as_str()].concat();
        return make_nl_result(lst_code, val_end.clone(), tokens.clone());
    }
    let mut is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
    if is_validator {
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
        let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
        let mut mut_kw: String = "".to_string();
        if mg_is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
            mut_kw = "mut ".to_string();
        }
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        let mut vld_let: String = "let ".to_string();
        let mut vld_opt: String = ": Option<".to_string();
        let mut vld_new: String = "> = ".to_string();
        let mut vld_nop: String = "::new(".to_string();
        let mut vld_sc: String = ");\n".to_string();
        let mut vld_code: String = [pad.as_str(), vld_let.as_str(), mut_kw.as_str(), var_name.as_str(), vld_opt.as_str(), var_type.as_str(), vld_new.as_str(), var_type.as_str(), vld_nop.as_str(), val_code.as_str(), vld_sc.as_str()].concat();
        return make_nl_result(vld_code, val_end.clone(), tokens.clone());
    }
    let mut tb_is_float: bool = var_type == "float".clone();
    if tb_is_float {
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        float_ctx_enable();
    }
    let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    if tb_is_float {
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        float_ctx_disable();
    }
    let code = val_r.code;
    let new_pos = val_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
    let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
    let mut mut_kw: String = "".to_string();
    if mg_is_mut {
        // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
        mut_kw = "mut ".to_string();
    }
    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
    let mut suffix: String = "".to_string();
    if kind == "STRING" {
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        suffix = ".to_string()".to_string();
    } else if kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/typed_binding.deor
        let mut val_next_idx: i32 = val_pos + 1.clone();
        if val_next_idx < token_count {
            // transpiler-deor/codegen/decl/stmt/typed_binding.deor
            let mut next_val_tok: Token = tokens[val_next_idx as usize].clone();
            let kind = next_val_tok.kind.clone();
            let mut val_is_call: bool = kind == "LPAREN".clone();
            let mut val_is_idx: bool = kind == "KW_AT".clone();
            if !val_is_call {
                // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                if !val_is_idx {
                    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
                    suffix = ".clone()".to_string();
                }
            }
        }
    }
    let mut bnd_let: String = "let ".to_string();
    let mut bnd_col: String = ": ".to_string();
    let mut bnd_eq: String = " = ".to_string();
    let mut bnd_sc: String = ";\n".to_string();
    let mut bind_code: String = [pad.as_str(), bnd_let.as_str(), mut_kw.as_str(), var_name.as_str(), bnd_col.as_str(), rust_type.as_str(), bnd_eq.as_str(), val_code.as_str(), suffix.as_str(), bnd_sc.as_str()].concat();
    return make_nl_result(bind_code, val_end.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/stmt.deor
fn gen_stmt(pos: i32, depth: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let using_type = ctx.using_type.clone();
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
    // macro: stmt_flow (transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor)
    if kind == "KW_RETURN" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
        let mut sf_val_pos: i32 = pos + 1.clone();
        let mut sf_val_tok: Token = tokens[sf_val_pos as usize].clone();
        let kind = sf_val_tok.kind.clone();
        let mut sf_val_r: ParseResult = gen_expr(tokens.clone(), sf_val_pos.clone(), ctx.clone());
        let code = sf_val_r.code;
        let new_pos = sf_val_r.new_pos;
        let sf_val_code = code;
        let sf_val_end = new_pos;
        let mut sf_suffix: String = "".to_string();
        if kind == "STRING" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
            sf_suffix = ".to_string()".to_string();
        }
        let mut sf_ret_kw: String = "return ".to_string();
        let mut sf_ret_sc: String = ";\n".to_string();
        let mut sf_ret_code: String = [pad.as_str(), sf_ret_kw.as_str(), sf_val_code.as_str(), sf_suffix.as_str(), sf_ret_sc.as_str()].concat();
        return make_nl_result(sf_ret_code, sf_val_end.clone(), tokens.clone());
    }
    if kind == "KW_BREAK" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
        let mut sf_brk_kw: String = "break;\n".to_string();
        let mut sf_brk_code: String = [pad.as_str(), sf_brk_kw.as_str()].concat();
        let mut sf_brk_n: i32 = pos + 1.clone();
        return make_nl_result(sf_brk_code, sf_brk_n.clone(), tokens.clone());
    }
    if kind == "KW_CONTINUE" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
        let mut sf_cnt_kw: String = "continue;\n".to_string();
        let mut sf_cnt_code: String = [pad.as_str(), sf_cnt_kw.as_str()].concat();
        let mut sf_cnt_n: i32 = pos + 1.clone();
        return make_nl_result(sf_cnt_code, sf_cnt_n.clone(), tokens.clone());
    }
    // macro: stmt_blocks (transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor)
    if kind == "KW_BLOCK" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
        let mut sb_nl: i32 = pos + 1.clone();
        let mut sb_body_start: i32 = skip_to_body_ref(tokens.clone(), sb_nl.clone());
        let mut sb_body_depth: i32 = depth + 1.clone();
        let mut sb_body_r: ParseResult = gen_block(sb_body_start.clone(), sb_body_depth.clone(), ctx.clone());
        let code = sb_body_r.code;
        let new_pos = sb_body_r.new_pos;
        let sb_body_code = code;
        let sb_body_end = new_pos;
        let mut sb_open_brace: String = "{\n".to_string();
        let mut sb_close_brace: String = "}\n".to_string();
        let mut sb_blk_open: String = s_cat(pad.clone(), sb_open_brace.clone());
        let mut sb_blk_close: String = s_cat(pad.clone(), sb_close_brace.clone());
        let mut sb_blk_code: String = [sb_blk_open.as_str(), sb_body_code.as_str(), sb_blk_close.as_str()].concat();
        return make_result(sb_blk_code, sb_body_end.clone());
    }
    if kind == "KW_RUST" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
        let mut sb_block_pos: i32 = pos + 2.clone();
        let mut sb_block_tok: Token = tokens[sb_block_pos as usize].clone();
        let value = sb_block_tok.value.clone();
        let mut sb_content: String = value.clone();
        let mut sb_rust_lines: Vec<String> = s_split(sb_content.clone(), newline.clone());
        let mut sb_padded: Vec<String> = Vec::new();
        let mut sb_line_count: i32 = (sb_rust_lines.len() as i32);
        for sb_ri in 0..sb_line_count {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
            let mut sb_rust_line: String = sb_rust_lines[sb_ri as usize].clone();
            if is_empty(sb_rust_line.clone()) {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
                let mut sb_empty_line: String = "".to_string();
                sb_padded.push(sb_empty_line.clone());
            } else {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
                sb_padded.push([pad.as_str(), sb_rust_line.as_str()].concat().clone());
            }
        }
        let mut sb_block_code: String = s_join_nl(sb_padded.clone());
        sb_block_code = s_cat(sb_block_code.clone(), newline.clone());
        let mut sb_block_next: i32 = sb_block_pos + 1.clone();
        return make_result(sb_block_code, sb_block_next.clone());
    }
    // macro: stmt_structural (transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor)
    if kind == "KW_GIVEUP" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
        let mut smd_next: i32 = pos + 1.clone();
        let mut smd_next_tok: Token = tokens[smd_next as usize].clone();
        let kind = smd_next_tok.kind.clone();
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
            return gen_move_destructure(smd_next.clone(), depth.clone(), ctx.clone());
        }
    }
    if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
        let mut su_peek: i32 = pos + 1.clone();
        if su_peek < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
            let mut su_peek_tok: Token = tokens[su_peek as usize].clone();
            let kind = su_peek_tok.kind.clone();
            if kind == "KW_AVOW" {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
                let mut su_expr_pos: i32 = su_peek + 1.clone();
                let mut su_expr_r: ParseResult = gen_expr(tokens.clone(), su_expr_pos.clone(), ctx.clone());
                let code = su_expr_r.code;
                let new_pos = su_expr_r.new_pos;
                let su_expr_code = code;
                let su_after_rp = new_pos + 1;
                let mut su_avw_sfx: String = ".unwrap();\n".to_string();
                let mut su_avow_code: String = [pad.as_str(), su_expr_code.as_str(), su_avw_sfx.as_str()].concat();
                return make_nl_result(su_avow_code, su_after_rp.clone(), tokens.clone());
            }
        }
        return gen_destructure(pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_USING" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
        let mut su_var_pos: i32 = pos + 1.clone();
        let mut su_var_tok: Token = tokens[su_var_pos as usize].clone();
        let value = su_var_tok.value.clone();
        let mut using_var: String = value.clone();
        let mut su_struct_type: String = reg_get(var_type_reg.clone(), using_var.clone());
/* unhandled(IDENT) */
        let using_type = su_struct_type;
        let mut su_init: String = make_destruct_code(using_var.clone(), depth.clone(), ctx.clone());
        let mut su_var_next: i32 = su_var_pos + 1.clone();
        let mut su_body_start: i32 = skip_to_body_ref(tokens.clone(), su_var_next.clone());
/* unhandled(IDENT) */
        let su_uctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone(), tokens: tokens.clone() };
        let mut su_using_ctx: RcCtx = make_rctx(su_uctx_raw);
        let mut su_block_r: ParseResult = gen_block(su_body_start.clone(), depth.clone(), su_using_ctx);
        let code = su_block_r.code;
        let new_pos = su_block_r.new_pos;
        let su_blk_code = code;
        let su_blk_pos = new_pos;
        let mut su_full: String = s_cat(su_init.clone(), su_blk_code.clone());
        return make_result(su_full, su_blk_pos.clone());
    }
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    if kind == "KW_RAW" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut raw_name_pos: i32 = pos + 1.clone();
        let mut raw_name_tok: Token = tokens[raw_name_pos as usize].clone();
        let value = raw_name_tok.value.clone();
        let mut raw_var_name: String = value.clone();
        let mut val_pos: i32 = raw_name_pos + 2.clone();
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut var_name: String = raw_var_name.clone();
        // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
        let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
        let mut mut_kw: String = "".to_string();
        if mg_is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
            mut_kw = "mut ".to_string();
        }
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut raw_let: String = "let ".to_string();
        let mut raw_eq: String = " = ".to_string();
        let mut raw_sc: String = ";\n".to_string();
        let mut raw_parts: Vec<String> = vec![pad.clone(), raw_let.clone(), mut_kw.clone(), raw_var_name.clone(), raw_eq.clone(), val_code.clone(), raw_sc.clone()];
        let mut raw_code: String = s_join(raw_parts.clone());
        return make_nl_result(raw_code, val_end.clone(), tokens.clone());
    }
    if kind == "KW_CONST" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut const_type_pos: i32 = pos + 1.clone();
        return gen_typed_binding(const_type_pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_IF" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        return gen_if(pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "KW_FOR" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        return gen_for(pos.clone(), depth.clone(), ctx.clone());
    }
    if kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut ident_name: String = value.clone();
        let mut next_pos: i32 = pos + 1.clone();
        if next_pos >= token_count {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut eof_code: String = "/* eof */\n".to_string();
            return make_result(eof_code, next_pos.clone());
        }
        let mut next_token: Token = tokens[next_pos as usize].clone();
        let kind = next_token.kind.clone();
        if kind == "KW_AS" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            return gen_as_binding(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            return gen_call_stmt(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "KW_AT" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            return gen_list_mutation_stmt(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "KW_REMOVE" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            return gen_list_mutation_stmt(pos.clone(), depth.clone(), ctx.clone());
        }
        if kind == "EQUALS" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut val_pos: i32 = next_pos + 1.clone();
            let mut eq_val_token: Token = tokens[val_pos as usize].clone();
            let kind = eq_val_token.kind.clone();
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut assign_suffix: String = "".to_string();
            if kind == "STRING" {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                assign_suffix = ".to_string()".to_string();
            }
            let mut asg_eq: String = " = ".to_string();
            let mut asg_sc: String = ";\n".to_string();
            let mut asg_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), asg_eq.clone(), val_code.clone(), assign_suffix.clone(), asg_sc.clone()];
            let mut asgn_code: String = s_join(asg_parts.clone());
            return make_nl_result(asgn_code, val_end.clone(), tokens.clone());
        }
        if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut eq_pos: i32 = next_pos + 1.clone();
            if eq_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                let mut eq_token: Token = tokens[eq_pos as usize].clone();
                let kind = eq_token.kind.clone();
                if kind == "EQUALS" {
                    // transpiler-deor/codegen/decl/stmt/stmt.deor
                    return gen_typed_binding(pos.clone(), depth.clone(), ctx.clone());
                }
            }
            let value = next_token.value.clone();
            let mut bare_var_name: String = value.clone();
            let mut bare_rust_type: String = resolve_type(ident_name.clone(), shape_reg.clone(), enum_reg.clone());
            let mut bare_is_validator: bool = reg3_has(type_reg.clone(), ident_name.clone());
            if bare_is_validator {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                let mut bd_let: String = "let mut ".to_string();
                let mut bd_opt: String = ": Option<".to_string();
                let mut bd_sfx: String = "> = None;\n".to_string();
                let mut bd_code: String = [pad.as_str(), bd_let.as_str(), bare_var_name.as_str(), bd_opt.as_str(), bare_rust_type.as_str(), bd_sfx.as_str()].concat();
                let mut bd_after: i32 = next_pos + 1.clone();
                return make_nl_result(bd_code, bd_after.clone(), tokens.clone());
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

// transpiler-deor/codegen/decl/cursor.deor
fn cur_at(tokens: Vec<Token>, pos: i32) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut current: Token = tokens[pos as usize].clone();
    let c = TokenCursor { token_count: token_count.clone(), pos: pos.clone(), current: current.clone() };
    return c;
}

fn cur_next(c: TokenCursor, tokens: Vec<Token>) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = c.token_count.clone();
    let mut pos = c.pos.clone();
    let mut current = c.current.clone();
    let mut pos: i32 = pos + 1.clone();
    if pos < token_count {
        // transpiler-deor/codegen/decl/cursor.deor
        let mut current: Token = tokens[pos as usize].clone();
        return TokenCursor { token_count, pos, current };
    }
    return TokenCursor { token_count, pos, current };
}

fn c_at_end(c: TokenCursor) -> bool {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = c.token_count.clone();
    let pos = c.pos.clone();
    return pos >= token_count;
}

fn cur_skip_to_body(c: TokenCursor, tokens: Vec<Token>) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = c.pos.clone();
    let mut body_pos: i32 = adv_nl(pos.clone(), tokens.clone());
    body_pos = adv_indent(body_pos.clone(), tokens.clone());
    return cur_at(tokens.clone(), body_pos.clone());
}

fn cur_peek(c: TokenCursor, tokens: Vec<Token>, offset: i32) -> Token {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = c.pos.clone();
    let mut peek_pos: i32 = pos + offset.clone();
    return tokens[peek_pos as usize].clone();
}

fn cur_at_ref(tokens: TokensRef, pos: i32) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let mut token_count: i32 = (tokens.len() as i32);
    let mut current: Token = tokens[pos as usize].clone();
    let c = TokenCursor { token_count: token_count.clone(), pos: pos.clone(), current: current.clone() };
    return c;
}

fn cur_next_ref(c: TokenCursor, tokens: TokensRef) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = c.token_count.clone();
    let mut pos = c.pos.clone();
    let mut current = c.current.clone();
    let mut pos: i32 = pos + 1.clone();
    if pos < token_count {
        // transpiler-deor/codegen/decl/cursor.deor
        let mut current: Token = tokens[pos as usize].clone();
        return TokenCursor { token_count, pos, current };
    }
    return TokenCursor { token_count, pos, current };
}

fn cur_skip_to_body_ref(c: TokenCursor, tokens: TokensRef) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = c.pos.clone();
    let mut body_pos: i32 = adv_nl_ref(pos.clone(), tokens.clone());
    body_pos = adv_indent_ref(body_pos.clone(), tokens.clone());
    return cur_at_ref(tokens.clone(), body_pos.clone());
}

fn cur_peek_ref(c: TokenCursor, tokens: TokensRef, offset: i32) -> Token {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = c.pos.clone();
    let mut peek_pos: i32 = pos + offset.clone();
    return tokens[peek_pos as usize].clone();
}

// transpiler-deor/codegen/decl/struct.deor
fn gen_struct_decl(tokens: TokensRef, pos: i32) -> ParseResult {
    // transpiler-deor/codegen/decl/struct.deor
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), start_pos.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    // transpiler-deor/codegen/decl/struct.deor
    let value = current.value.clone();
    let mut struct_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut field_lines: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        // transpiler-deor/codegen/decl/struct.deor
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "DEDENT" {
            // transpiler-deor/codegen/decl/struct.deor
            cur = cur_next_ref(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            break;
        } else if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/struct.deor
            cur = cur_next_ref(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/struct.deor
            let mut field_type: String = value.clone();
            cur = cur_next_ref(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut field_name: String = value.clone();
            let mut rust_type: String = render_rust_type(field_type.clone());
            let mut fln_ind: String = "    ".to_string();
            let mut fln_sep: String = ": ".to_string();
            let mut fln_com: String = ",".to_string();
            field_lines.push([fln_ind.as_str(), field_name.as_str(), fln_sep.as_str(), rust_type.as_str(), fln_com.as_str()].concat().clone());
            cur = cur_next_ref(cur.clone(), tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        }
    }
    let mut fields_code: String = s_join_nl(field_lines.clone());
    let mut sdcl_pfx: String = "#[derive(Clone, PartialEq, Debug)]\nstruct ".to_string();
    let mut sdcl_ob: String = " {\n".to_string();
    let mut sdcl_cb: String = "\n}\n\n".to_string();
    let mut decl: String = [sdcl_pfx.as_str(), struct_name.as_str(), sdcl_ob.as_str(), fields_code.as_str(), sdcl_cb.as_str()].concat();
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
}

// transpiler-deor/codegen/decl/enum.deor
fn gen_enum_decl(tokens: TokensRef, pos: i32) -> ParseResult {
    // transpiler-deor/codegen/decl/enum.deor
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), start_pos.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    // transpiler-deor/codegen/decl/enum.deor
    let value = current.value.clone();
    let mut enum_name: String = value.clone();
    let mut rust_name: String = s_pascal(enum_name.clone());
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut variant_lines: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        // transpiler-deor/codegen/decl/enum.deor
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        cur = cur_next_ref(cur.clone(), tokens.clone());
        let token_count = cur.token_count.clone();
        let pos = cur.pos.clone();
        let current = cur.current.clone();
        if kind == "DEDENT" {
            // transpiler-deor/codegen/decl/enum.deor
            break;
        }
        if kind == "IDENT" {
            // transpiler-deor/codegen/decl/enum.deor
            let mut vln_ind: String = "    ".to_string();
            let mut vln_com: String = ",".to_string();
            variant_lines.push([vln_ind.as_str(), value.as_str(), vln_com.as_str()].concat().clone());
        }
    }
    let mut variants_code: String = s_join_nl(variant_lines.clone());
    let mut enm_pfx: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nenum ".to_string();
    let mut enm_ob: String = " {\n".to_string();
    let mut enm_cb: String = "\n}\n\n".to_string();
    let mut decl: String = [enm_pfx.as_str(), rust_name.as_str(), enm_ob.as_str(), variants_code.as_str(), enm_cb.as_str()].concat();
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
}

// transpiler-deor/codegen/decl/shape.deor
fn gen_list_shape_code(rust_name: String, rust_elem: String) -> String {
    // transpiler-deor/codegen/decl/shape.deor
    let mut shp_pfx: String = "type ".to_string();
    let mut shp_mid: String = " = Vec<".to_string();
    let mut shp_sfx: String = ">;\n\n".to_string();
    return [shp_pfx.as_str(), rust_name.as_str(), shp_mid.as_str(), rust_elem.as_str(), shp_sfx.as_str()].concat();
}

fn gen_func_shape_code(rust_name: String, rust_in: String, rust_out: String) -> String {
    // transpiler-deor/codegen/decl/shape.deor
    let mut out_suffix: String = "".to_string();
    if !is_empty(rust_out.clone()) {
        // transpiler-deor/codegen/decl/shape.deor
        let mut ost_pfx: String = " -> ".to_string();
        out_suffix = [ost_pfx.as_str(), rust_out.as_str()].concat();
    }
    let mut fns_pfx: String = "type ".to_string();
    let mut fns_mid: String = " = fn(".to_string();
    let mut fns_rp: String = ")".to_string();
    let mut fns_sfx: String = ";\n\n".to_string();
    return [fns_pfx.as_str(), rust_name.as_str(), fns_mid.as_str(), rust_in.as_str(), fns_rp.as_str(), out_suffix.as_str(), fns_sfx.as_str()].concat();
}

fn gen_shape_decl(tokens: TokensRef, pos: i32) -> ParseResult {
    // transpiler-deor/codegen/decl/shape.deor
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
        // transpiler-deor/codegen/decl/shape.deor
        let mut elem_pos: i32 = pos + 5.clone();
        let mut elem_token: Token = tokens[elem_pos as usize].clone();
        let value = elem_token.value.clone();
        let mut elem_type: String = value.clone();
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        let mut decl: String = gen_list_shape_code(rust_name.clone(), rust_elem.clone());
        let mut after: i32 = elem_pos + 1.clone();
        return make_nl_result(decl, after.clone(), tokens.clone());
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
        // transpiler-deor/codegen/decl/shape.deor
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
            // transpiler-deor/codegen/decl/shape.deor
            let mut t7_pos: i32 = pos + 7.clone();
            let mut t7_token: Token = tokens[t7_pos as usize].clone();
            let value = t7_token.value.clone();
            out_type = value;
            func_end = t7_pos;
        }
    } else if t4_is_to {
        // transpiler-deor/codegen/decl/shape.deor
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
    return make_nl_result(decl, after.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/validator_type.deor
fn gen_type_decl(tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/validator_type.deor
    let mut start_pos: i32 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), start_pos.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    // transpiler-deor/codegen/decl/validator_type.deor
    let value = current.value.clone();
    let mut type_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut param_type: String = value.clone();
    let mut rust_param_type: String = render_rust_type(param_type.clone());
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut param_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let pos = cur.pos.clone();
    let mut pred_r: ParseResult = gen_expr(tokens.clone(), pos.clone(), ctx.clone());
    let code = pred_r.code;
    let new_pos = pred_r.new_pos;
    let pred_code = code;
    let pred_end = new_pos;
    let mut cur2: TokenCursor = cur_at_ref(tokens.clone(), pred_end.clone());
    let token_count = cur2.token_count.clone();
    let pos = cur2.pos.clone();
    let current = cur2.current.clone();
    // transpiler-deor/codegen/decl/validator_type.deor
    while !c_at_end(cur2.clone()) {
        // transpiler-deor/codegen/decl/validator_type.deor
        let current = cur2.current.clone();
        let kind = current.kind.clone();
        cur2 = cur_next_ref(cur2.clone(), tokens.clone());
        let token_count = cur2.token_count.clone();
        let pos = cur2.pos.clone();
        let current = cur2.current.clone();
        if kind == "DEDENT" {
            // transpiler-deor/codegen/decl/validator_type.deor
            break;
        }
    }
    let pos = cur2.pos.clone();
    let mut tsc_pfx: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nstruct ".to_string();
    let mut tsc_op: String = "(".to_string();
    let mut tsc_sfx: String = ");\n\n".to_string();
    let mut struct_code: String = [tsc_pfx.as_str(), type_name.as_str(), tsc_op.as_str(), rust_param_type.as_str(), tsc_sfx.as_str()].concat();
    let mut imp_pfx: String = "impl ".to_string();
    let mut imp_fn: String = " {\n    fn new(".to_string();
    let mut imp_col: String = ": ".to_string();
    let mut imp_ret: String = ") -> Option<Self> {\n        if ".to_string();
    let mut imp_som: String = " {\n            Some(".to_string();
    let mut imp_inn: String = "(".to_string();
    let mut imp_sfx: String = "))\n        } else {\n            None\n        }\n    }\n}\n\n".to_string();
    let mut impl_code: String = [imp_pfx.as_str(), type_name.as_str(), imp_fn.as_str(), param_name.as_str(), imp_col.as_str(), rust_param_type.as_str(), imp_ret.as_str(), pred_code.as_str(), imp_som.as_str(), type_name.as_str(), imp_inn.as_str(), param_name.as_str(), imp_sfx.as_str()].concat();
    let mut type_code: String = s_cat(struct_code, impl_code);
    return make_result(type_code, pos.clone());
}

// transpiler-deor/codegen/decl/function.deor
fn gen_fn_decl(fn_tokens: TokensRef, pos: i32, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/function.deor
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
    let mut cur: TokenCursor = cur_at_ref(fn_tokens.clone(), start_pos.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    // transpiler-deor/codegen/decl/function.deor
    let value = current.value.clone();
    let mut ret_type: String = resolve_type(value.clone(), shape_reg.clone(), enum_reg.clone());
    cur = cur_next_ref(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut fn_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    cur = cur_next_ref(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut param_strs: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        // transpiler-deor/codegen/decl/function.deor
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/function.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            break;
        } else if kind == "COMMA" {
            // transpiler-deor/codegen/decl/function.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/function.deor
            let mut param_type: String = value.clone();
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut param_name: String = value.clone();
            let mut rust_param_type: String = resolve_type(param_type.clone(), shape_reg.clone(), enum_reg.clone());
            let mut prm_sep: String = ": ".to_string();
            param_strs.push([param_name.as_str(), prm_sep.as_str(), rust_param_type.as_str()].concat().clone());
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        } else {
            // transpiler-deor/codegen/decl/function.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            let token_count = cur.token_count.clone();
            let pos = cur.pos.clone();
            let current = cur.current.clone();
        }
    }
    let pos = cur.pos.clone();
    let mut indent_pos: i32 = pos + 1.clone();
    cur = cur_skip_to_body_ref(cur.clone(), fn_tokens.clone());
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    let current = cur.current.clone();
    let mut body_end_pos: i32 = find_block_end_ref(fn_tokens.clone(), indent_pos.clone());
    let mut body_start: i32 = pos.clone();
    let mut body_slice_end: i32 = body_end_pos + 1.clone();
    let mut body_tokens_raw: Vec<Token> = l_slice_ref(fn_tokens.clone(), body_start.clone(), body_slice_end.clone());
    let mut body_len: i32 = (body_tokens_raw.len() as i32);
    let mut zero: i32 = 0;
    let mut body_last: i32 = body_len - 1.clone();
    let mut mut_names: Vec<String> = collect_mut_names(body_tokens_raw.clone(), zero.clone(), body_last.clone());
    let mut tokens: TokensRef = tokens_wrap(body_tokens_raw);
    let mut var_type_reg: Vec<String> = build_var_type_reg(tokens.clone());
    let mut using_type: String = "".to_string();
    let mut using_var: String = "".to_string();
/* unhandled(IDENT) */
    let body_ctx_raw = GenCtx { variant_reg: variant_reg.clone(), shape_reg: shape_reg.clone(), struct_reg: struct_reg.clone(), enum_reg: enum_reg.clone(), mut_names: mut_names.clone(), type_reg: type_reg.clone(), using_type: using_type.clone(), using_var: using_var.clone(), var_type_reg: var_type_reg.clone(), tokens: tokens.clone() };
    let mut body_ctx: RcCtx = make_rctx(body_ctx_raw);
    let mut body_pos: i32 = 0;
    let mut body_depth: i32 = 1;
    let mut body_r: ParseResult = gen_block(body_pos.clone(), body_depth.clone(), body_ctx);
    let code = body_r.code;
    let new_pos = body_r.new_pos;
    let body_code = code;
    let body_end = body_start + new_pos;
    let mut sep: String = ", ".to_string();
    let mut params_code: String = s_join_with(param_strs.clone(), sep.clone());
    let mut ret_suffix: String = "".to_string();
    if !is_empty(ret_type.clone()) {
        // transpiler-deor/codegen/decl/function.deor
        let mut rts_pfx: String = " -> ".to_string();
        ret_suffix = [rts_pfx.as_str(), ret_type.as_str()].concat();
    }
    let mut fnc_kw: String = "fn ".to_string();
    let mut fnc_op: String = "(".to_string();
    let mut fnc_rp: String = ")".to_string();
    let mut fnc_ob: String = " {\n".to_string();
    let mut fnc_cb: String = "}\n\n".to_string();
    let mut fn_code: String = [fnc_kw.as_str(), fn_name.as_str(), fnc_op.as_str(), params_code.as_str(), fnc_rp.as_str(), ret_suffix.as_str(), fnc_ob.as_str(), body_code.as_str(), fnc_cb.as_str()].concat();
    return make_result(fn_code, body_end.clone());
}

// transpiler-deor/codegen/decl/raw.deor
fn gen_raw_decl(tokens: TokensRef, pos: i32) -> ParseResult {
    // transpiler-deor/codegen/decl/raw.deor
    let mut name_pos: i32 = pos + 1.clone();
    let mut after: i32 = name_pos + 1.clone();
    let mut emp: String = "".to_string();
    return make_nl_result(emp, after.clone(), tokens.clone());
}

// transpiler-deor/codegen/codegen.deor
fn generate_rust_from_tokens(all_ref: TokensRef, ctx: RcCtx) -> String {
    // transpiler-deor/codegen/codegen.deor
    let mut parts: Vec<String> = Vec::new();
    let mut token_count: i32 = (all_ref.len() as i32);
    println!("{}", ["[diag] token_count: ", n_to_str(token_count.clone()).as_str()].concat());
    let mut pos: i32 = 0;
    let mut last_file: String = "".to_string();
    let mut _timer_label: String = "[timer]   codegen-loop: ".to_string();
    // macro: start_timer (transpiler-deor/utility_macros.deor)
    let mut _timer_start: i32 = now_ms();
    // transpiler-deor/codegen/codegen.deor
    while true {
        // transpiler-deor/codegen/codegen.deor
        if pos >= token_count {
            // transpiler-deor/codegen/codegen.deor
            break;
        }
        let mut token: Token = all_ref[pos as usize].clone();
        let kind = token.kind.clone();
        let file = token.file.clone();
        if kind == "EOF" {
            // transpiler-deor/codegen/codegen.deor
            break;
        }
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/codegen.deor
            pos = pos + 1;
            continue;
        }
        if file != last_file {
            // transpiler-deor/codegen/codegen.deor
            let mut fc_slash: String = "// ".to_string();
            let mut fc_nl: String = "\n".to_string();
            let mut fc1: String = s_cat(fc_slash.clone(), file.clone());
            let mut file_comment: String = s_cat(fc1.clone(), fc_nl.clone());
            parts.push(file_comment.clone());
            last_file = file;
        }
        if kind == "KW_STRUCT" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_struct_decl(all_ref.clone(), pos.clone());
            let code = result.code;
            let new_pos = result.new_pos;
            parts.push(code.clone());
            pos = new_pos;
            continue;
        }
        if kind == "KW_SHAPE" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_shape_decl(all_ref.clone(), pos.clone());
            let code = result.code;
            let new_pos = result.new_pos;
            parts.push(code.clone());
            pos = new_pos;
            continue;
        }
        if kind == "KW_ENUM" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_enum_decl(all_ref.clone(), pos.clone());
            let code = result.code;
            let new_pos = result.new_pos;
            parts.push(code.clone());
            pos = new_pos;
            continue;
        }
        if kind == "KW_TYPE" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_type_decl(all_ref.clone(), pos.clone(), ctx.clone());
            let code = result.code;
            let new_pos = result.new_pos;
            parts.push(code.clone());
            pos = new_pos;
            continue;
        }
        if kind == "KW_FN" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_fn_decl(all_ref.clone(), pos.clone(), ctx.clone());
            let code = result.code;
            let new_pos = result.new_pos;
            parts.push(code.clone());
            pos = new_pos;
            continue;
        }
        if kind == "KW_RAW" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_raw_decl(all_ref.clone(), pos.clone());
            let code = result.code;
            let new_pos = result.new_pos;
            parts.push(code.clone());
            pos = new_pos;
            continue;
        }
        if kind == "KW_RUST" {
            // transpiler-deor/codegen/codegen.deor
            let mut block_pos: i32 = pos + 2.clone();
            let mut block_token: Token = all_ref[block_pos as usize].clone();
            let value = block_token.value.clone();
            let mut newline: String = "\n".to_string();
            let mut rust_chunk: String = s_cat(value.clone(), newline.clone());
            parts.push(rust_chunk.clone());
            pos = block_pos + 1;
            continue;
        }
        pos = pos + 1;
    }
    // macro: end_timer (transpiler-deor/utility_macros.deor)
    let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
    let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
    let mut _timer_sfx: String = "ms".to_string();
    println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
    // transpiler-deor/codegen/codegen.deor
    return s_join(parts.clone());
}

// transpiler-deor/main.deor
fn main() {
    // transpiler-deor/main.deor
    let mut args: Vec<String> = f_args();
    let mut arg_count: i32 = (args.len() as i32);
    if arg_count < 2 {
        // transpiler-deor/main.deor
        println!("{}", "usage: deor input.deor output.rs".to_string());
    } else {
        // transpiler-deor/main.deor
        let mut input_path: String = args[0 as usize].clone();
        let mut output_path: String = args[1 as usize].clone();
        let mut _timer_label: String = "[timer] load+dedup: ".to_string();
        // macro: start_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_start: i32 = now_ms();
        // transpiler-deor/main.deor
        let mut raw_tokens: Vec<Token> = collect_all_tokens_with_all_imports(input_path.clone());
        // macro: end_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut _timer_label: String = "[timer] macro-expand: ".to_string();
        // macro: start_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_start: i32 = now_ms();
        // transpiler-deor/main.deor
        let mut tokens: Vec<Token> = expand_deor_macros(raw_tokens.clone());
        // macro: end_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut tokens_ref: TokensRef = tokens_wrap(tokens);
        let mut _timer_label: String = "[timer] validate: ".to_string();
        // macro: start_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_start: i32 = now_ms();
        // transpiler-deor/main.deor
        validate_tokens(tokens_ref.clone());
        // macro: end_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut _timer_label: String = "[timer] registry: ".to_string();
        // macro: start_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_start: i32 = now_ms();
        // transpiler-deor/main.deor
        let ctx = build_registry(tokens_ref.clone());
        // macro: end_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut _timer_label: String = "[timer] total-codegen: ".to_string();
        // macro: start_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_start: i32 = now_ms();
        // transpiler-deor/main.deor
        let mut rust_code: String = generate_rust_from_tokens(tokens_ref.clone(), ctx.clone());
        // macro: end_timer (transpiler-deor/utility_macros.deor)
        let mut _timer_elapsed: i32 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut allow_warnings: String = "#![allow(warnings)]\n".to_string();
        rust_code = s_cat(allow_warnings.clone(), rust_code.clone());
        f_write(output_path.clone(), rust_code.clone());
    }
}

