#![allow(warnings)]
// transpiler-deor/types.deor
type TokenList = Vec<Token>;

type StrList = Vec<String>;

#[derive(Clone, PartialEq, Debug)]
struct Token {
    kind: String,
    value: String,
    line: i64,
    file: String,
}

#[derive(Clone, PartialEq, Debug)]
struct ParseResult {
    code: String,
    new_pos: i64,
}

#[derive(Clone, PartialEq, Debug)]
struct TokenMeta {
    line: i64,
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
    tokens: TokensRef,
    typed_enum_reg: StrList,
    typed_variant_reg: StrList,
}

#[derive(Clone, PartialEq, Debug)]
struct TokenCursor {
    token_count: i64,
    pos: i64,
    current: Token,
}

use std::rc::Rc;
type TokensRef = Rc<Vec<Token>>;
type RcCtx = Rc<GenCtx>;
fn tokens_wrap(t: Vec<Token>) -> TokensRef { Rc::new(t) }
fn make_rctx(ctx: GenCtx) -> RcCtx { Rc::new(ctx) }
fn now_ms() -> i64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64 }
fn elapsed_ms(start: i64) -> i64 { now_ms() - start }
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
fn s_upper_char(chr: String) -> bool {
    // transpiler-deor/lib/string.deor
    chr.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn s_lower_char(chr: String) -> bool {
    // transpiler-deor/lib/string.deor
    chr.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
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

fn s_from(source: String, start: i64) -> String {
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

fn s_repeat(source: String, count: i64) -> String {
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
fn n_parse(source: String) -> i64 {
    // transpiler-deor/lib/num.deor
    source.parse::<i64>().unwrap_or(0)
}

fn n_to_str(number: i64) -> String {
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
fn l_slice(tokens: Vec<Token>, start: i64, end_val: i64) -> Vec<Token> {
    // transpiler-deor/lib/list.deor
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
}

fn l_slice_ref(tokens: TokensRef, start: i64, end_val: i64) -> Vec<Token> {
    // transpiler-deor/lib/list.deor
    {
    	let end = (end_val as usize).min(tokens.len());
    	tokens[start as usize..end].to_vec()
    }
}

// transpiler-deor/utils.deor
fn is_empty(source: String) -> bool {
    // transpiler-deor/utils.deor
    let mut length: i64 = (source.len() as i64);
    return length == 0;
}

fn str_eq(left: String, right: String) -> bool {
    // transpiler-deor/utils.deor
    return left == right;
}

fn reg_get_stride(pairs: Vec<String>, key: String, stride: i64) -> String {
    // transpiler-deor/utils.deor
    let mut pairs_count: i64 = (pairs.len() as i64);
    let mut index: i64 = 0;
    while index < pairs_count {
        // transpiler-deor/utils.deor
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            // transpiler-deor/utils.deor
            let mut val_index: i64 = index + 1.clone();
            return pairs[val_index as usize].clone();
        }
        index = index + stride;
    }
    return "".to_string();
}

fn reg_has_stride(pairs: Vec<String>, key: String, stride: i64) -> bool {
    // transpiler-deor/utils.deor
    let mut pairs_count: i64 = (pairs.len() as i64);
    let mut index: i64 = 0;
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
    let mut two: i64 = 2;
    return reg_get_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg_has(pairs: Vec<String>, key: String) -> bool {
    // transpiler-deor/utils.deor
    let mut two: i64 = 2;
    return reg_has_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg3_get(pairs: Vec<String>, key: String) -> String {
    // transpiler-deor/utils.deor
    let mut thr: i64 = 3;
    return reg_get_stride(pairs.clone(), key.clone(), thr.clone());
}

fn reg3_has(pairs: Vec<String>, key: String) -> bool {
    // transpiler-deor/utils.deor
    let mut thr: i64 = 3;
    return reg_has_stride(pairs.clone(), key.clone(), thr.clone());
}

fn list_has(items: Vec<String>, val: String) -> bool {
    // transpiler-deor/utils.deor
    let mut item_count: i64 = (items.len() as i64);
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

fn pr_pos(result: ParseResult) -> i64 {
    // transpiler-deor/deor_helpers.deor
    result.new_pos
}

fn make_result(code: String, new_pos: i64) -> ParseResult {
    // transpiler-deor/deor_helpers.deor
    let result = ParseResult { code: code.clone(), new_pos: new_pos.clone() };
    return result;
}

fn adv_nl(pos: i64, tokens: Vec<Token>) -> i64 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i64 = (tokens.len() as i64);
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

fn adv_indent(pos: i64, tokens: Vec<Token>) -> i64 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i64 = (tokens.len() as i64);
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

fn skip_to_body(tokens: Vec<Token>, pos: i64) -> i64 {
    // transpiler-deor/deor_helpers.deor
    let mut cur: i64 = adv_nl(pos.clone(), tokens.clone());
    cur = adv_indent(cur.clone(), tokens.clone());
    return cur;
}

fn adv_nl_ref(pos: i64, tokens: TokensRef) -> i64 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i64 = (tokens.len() as i64);
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

fn adv_indent_ref(pos: i64, tokens: TokensRef) -> i64 {
    // transpiler-deor/deor_helpers.deor
    let mut token_count: i64 = (tokens.len() as i64);
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

fn skip_to_body_ref(tokens: TokensRef, pos: i64) -> i64 {
    // transpiler-deor/deor_helpers.deor
    let mut cur: i64 = adv_nl_ref(pos.clone(), tokens.clone());
    cur = adv_indent_ref(cur.clone(), tokens.clone());
    return cur;
}

fn make_nl_result(code: String, pos: i64, tokens: TokensRef) -> ParseResult {
    // transpiler-deor/deor_helpers.deor
    let mut next_pos: i64 = adv_nl_ref(pos.clone(), tokens.clone());
    return make_result(code, next_pos.clone());
}

// transpiler-deor/importer/lexer/token_factory.deor
fn make_meta(line: i64, file: String) -> TokenMeta {
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
fn count_tabs(line: String) -> i64 {
    // transpiler-deor/importer/lexer/indent.deor
    let mut space: String = " ".to_string();
    let mut chars: Vec<String> = c_chars(line.clone());
    let mut char_count: i64 = (chars.len() as i64);
    let mut count: i64 = 0;
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
fn scan_string_literal(chars: Vec<String>, char_index: i64, char_count: i64) -> ParseResult {
    // transpiler-deor/importer/lexer/string_literal.deor
    let mut val: String = "".to_string();
    let mut escape_next: bool = false;
    let mut str_start: i64 = char_index + 1.clone();
    let mut new_pos: i64 = char_index + 1.clone();
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
fn scan_number(chars: Vec<String>, char_index: i64, char_count: i64) -> ParseResult {
    // transpiler-deor/importer/lexer/number_literal.deor
    let mut first_char: String = chars[char_index as usize].clone();
    let mut empty_str: String = "".to_string();
    let mut num: String = s_cat(empty_str.clone(), first_char.clone());
    let mut num_start: i64 = char_index + 1.clone();
    let mut new_pos: i64 = char_index + 1.clone();
    for number_index in num_start..char_count {
        // transpiler-deor/importer/lexer/number_literal.deor
        let mut number_char: String = chars[number_index as usize].clone();
        if c_digit(number_char.clone()) {
            // transpiler-deor/importer/lexer/number_literal.deor
            num = s_cat(num.clone(), number_char.clone());
            new_pos = number_index + 1;
        } else if number_char == "_" {
            // transpiler-deor/importer/lexer/number_literal.deor
            let mut peek_idx: i64 = number_index + 1.clone();
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
        let mut frac_start: i64 = new_pos + 1.clone();
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
    if word == "with" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_WITH".to_string();
    }
    if word == "move" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_MOVE".to_string();
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
    if word == "end" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_END".to_string();
    }
    if word == "func" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_FUNC".to_string();
    }
    if word == "to" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_TO".to_string();
    }
    if word == "none" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_NONE".to_string();
    }
    return "IDENT".to_string();
}

fn scan_word(chars: Vec<String>, char_index: i64, char_count: i64) -> ParseResult {
    // transpiler-deor/importer/lexer/word_token.deor
    let mut first_char: String = chars[char_index as usize].clone();
    let mut empty_str: String = "".to_string();
    let mut word: String = s_cat(empty_str.clone(), first_char.clone());
    let mut word_start: i64 = char_index + 1.clone();
    let mut new_pos: i64 = char_index + 1.clone();
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
    let mut n_lines: i64 = (lines.len() as i64);
    let mut indent_stack: Vec<String> = Vec::new();
    let mut zero_str: String = "0".to_string();
    indent_stack.push(zero_str.clone());
    let mut cur_line: i64 = 0;
    let mut skip: i64 = 0;
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
        let mut indent: i64 = count_tabs(line.clone());
        // macro: emit_indent_or_dedent (transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor)
        let mut iod_kind_indent: String = "INDENT".to_string();
        let mut iod_kind_dedent: String = "DEDENT".to_string();
        let mut iod_empty: String = "".to_string();
        let mut slen: i64 = (indent_stack.len() as i64);
        let mut top_idx: i64 = slen - 1.clone();
        let mut top: i64 = n_parse(indent_stack[top_idx as usize].clone());
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
                let mut new_slen: i64 = (indent_stack.len() as i64);
                let mut new_top_idx: i64 = new_slen - 1.clone();
                let mut cur_top: i64 = n_parse(indent_stack[new_top_idx as usize].clone());
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
            let mut rust_base: i64 = indent + 1.clone();
            let mut rust_lines: Vec<String> = Vec::new();
            let mut rli_start: i64 = raw_li + 1.clone();
            for rli in rli_start..n_lines {
                // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                let mut rust_line: String = lines[rli as usize].clone();
                let mut rl_indent: i64 = count_tabs(rust_line.clone());
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
                let mut rl_len: i64 = (rust_lines.len() as i64);
                if rl_len > 0 {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    let mut last_rl: i64 = rl_len - 1.clone();
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
        let mut char_count: i64 = (chars.len() as i64);
        let mut char_index: i64 = 0;
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
                let mut is_float: bool = (num_parts.len() as i64) > 1;
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
            let mut op_peek_idx: i64 = char_index + 1.clone();
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
            } else if character == "&" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                let mut op_kind_inv: String = "INVALID".to_string();
                tokens.push(make_token(op_kind_inv.clone(), character.clone(), meta.clone()).clone());
            } else if character == "|" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                let mut op_kind_inv: String = "INVALID".to_string();
                tokens.push(make_token(op_kind_inv.clone(), character.clone(), meta.clone()).clone());
            } else if character == "^" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                let mut op_kind_inv: String = "INVALID".to_string();
                tokens.push(make_token(op_kind_inv.clone(), character.clone(), meta.clone()).clone());
            } else if character == "{" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                let mut op_kind_inv: String = "INVALID".to_string();
                tokens.push(make_token(op_kind_inv.clone(), character.clone(), meta.clone()).clone());
            } else if character == "}" {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                let mut op_kind_inv: String = "INVALID".to_string();
                tokens.push(make_token(op_kind_inv.clone(), character.clone(), meta.clone()).clone());
            }
            char_index = char_index + 1;
        }
        tokens.push(make_token(kind_newline.clone(), empty_str.clone(), meta.clone()).clone());
    }
    let mut final_stack_len: i64 = (indent_stack.len() as i64);
    let mut tail_meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
    for _ in 1..final_stack_len {
        // transpiler-deor/importer/lexer/tokenizer.deor
        tokens.push(make_token(kind_dedent.clone(), empty_str.clone(), tail_meta.clone()).clone());
    }
    tokens.push(make_token(kind_eof.clone(), empty_str.clone(), tail_meta.clone()).clone());
    return tokens;
}

// transpiler-deor/importer/scan.deor
fn scan_import_new(tokens: Vec<Token>, pos: i64) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    let mut path_pos: i64 = pos + 1.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    if path_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut path_tok: Token = tokens[path_pos as usize].clone();
        let kind = path_tok.kind.clone();
        let value = path_tok.value.clone();
        if kind == "STRING" {
            // transpiler-deor/importer/scan.deor
            let mut after_path: i64 = path_pos + 1.clone();
            return make_result(value.clone(), after_path.clone());
        }
    }
    let mut emp: String = "".to_string();
    return make_result(emp.clone(), pos.clone());
}

fn scan_import_where(tokens: Vec<Token>, pos: i64) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut where_pos: i64 = pos.clone();
    let mut replacement_pos: i64 = pos + 1.clone();
    let mut eq_pos: i64 = pos + 2.clone();
    let mut concrete_pos: i64 = pos + 3.clone();
    if concrete_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut where_tok: Token = tokens[where_pos as usize].clone();
        let kind = where_tok.kind.clone();
        let value = where_tok.value.clone();
        let mut is_where: bool = kind == "IDENT" && value == "where".clone();
        if is_where {
            // transpiler-deor/importer/scan.deor
            let mut eq_tok: Token = tokens[eq_pos as usize].clone();
            let mut concrete_tok: Token = tokens[concrete_pos as usize].clone();
            let mut replacement_tok: Token = tokens[replacement_pos as usize].clone();
            let kind = eq_tok.kind.clone();
            let mut is_eq: bool = kind == "EQUALS".clone();
            if is_eq {
                // transpiler-deor/importer/scan.deor
                let value = replacement_tok.value.clone();
                let replacement_value = value;
                let value = concrete_tok.value.clone();
                let mut after_where: i64 = concrete_pos + 1.clone();
                let replace_with = replacement_value + "|" + value.as_str();
                return make_result(replace_with.clone(), after_where.clone());
            }
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

fn s_camel(source: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    let mut chars = source.chars();
    match chars.next() {
    	None => String::new(),
    	Some(c) => c.to_lowercase().to_string() + chars.as_str(),
    }
}

fn s_to_snake(source: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    {
    	let mut result = String::new();
    	for (i, c) in source.chars().enumerate() {
    		if c.is_uppercase() && i > 0 {
    			result.push('_');
    			result.push(c.to_lowercase().next().unwrap());
    		} else {
    			result.push(c.to_lowercase().next().unwrap());
    		}
    	}
    	result
    }
}

fn s_contains(source: String, needle: String) -> bool {
    // transpiler-deor/importer/t_substitute.deor
    source.contains(needle.as_str())
}

fn s_replace(source: String, from: String, output: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    source.replace(from.as_str(), output.as_str())
}

fn apply_t_in_name(name: String, placeholder: String, concrete: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    if name == placeholder {
        // transpiler-deor/importer/t_substitute.deor
        return concrete;
    }
    let mut pascal_ph: String = s_pascal(placeholder.clone());
    let mut camel_ph: String = s_camel(placeholder.clone());
    let mut ph_len: i64 = (placeholder.len() as i64);
    let mut name_len: i64 = (name.len() as i64);
    if name_len > ph_len {
        // transpiler-deor/importer/t_substitute.deor
        let mut after_ph: String = s_from(name.clone(), ph_len.clone());
        let mut after_chars: Vec<String> = c_chars(after_ph.clone());
        if (after_chars.len() as i64) > 0 {
            // transpiler-deor/importer/t_substitute.deor
            let mut next_char: String = after_chars[0 as usize].clone();
            let mut next_is_upper: bool = s_upper_char(next_char.clone());
            let mut starts_pascal: bool = s_starts_with(name.clone(), pascal_ph.clone());
            if starts_pascal && next_is_upper {
                // transpiler-deor/importer/t_substitute.deor
                let mut pascal_concrete: String = s_pascal(concrete.clone());
                return s_cat(pascal_concrete.clone(), after_ph.clone());
            }
            let mut starts_camel: bool = s_starts_with(name.clone(), camel_ph.clone());
            if starts_camel && next_is_upper {
                // transpiler-deor/importer/t_substitute.deor
                let mut camel_concrete: String = s_camel(concrete.clone());
                return s_cat(camel_concrete.clone(), after_ph.clone());
            }
        }
    }
    let mut pascal_sep: String = ["_", pascal_ph.as_str(), "_"].concat();
    let mut camel_sep: String = ["_", camel_ph.as_str(), "_"].concat();
    let mut snake_concrete: String = s_to_snake(concrete.clone());
    let mut new_sep: String = ["_", snake_concrete.as_str(), "_"].concat();
    let mut has_pascal_sep: bool = s_contains(name.clone(), pascal_sep.clone());
    if has_pascal_sep {
        // transpiler-deor/importer/t_substitute.deor
        return s_replace(name.clone(), pascal_sep.clone(), new_sep.clone());
    }
    let mut has_camel_sep: bool = s_contains(name.clone(), camel_sep.clone());
    if has_camel_sep {
        // transpiler-deor/importer/t_substitute.deor
        return s_replace(name.clone(), camel_sep.clone(), new_sep.clone());
    }
    return name;
}

fn replace_t_in_rust_block(content: String, placeholder: String, concrete: String) -> String {
    // transpiler-deor/importer/t_substitute.deor
    {
    	fn pascal_str(s: &str) -> String {
    		let mut c = s.chars();
    		match c.next() {
    			None => String::new(),
    			Some(f) => f.to_uppercase().to_string() + c.as_str(),
    		}
    	}
    	fn camel_str(s: &str) -> String {
    		let mut c = s.chars();
    		match c.next() {
    			None => String::new(),
    			Some(f) => f.to_lowercase().to_string() + c.as_str(),
    		}
    	}
    	fn sub_word(word: &str, placeholder: &str, concrete: &str) -> String {
    		if word == placeholder {
    			let rust_type = match concrete {
    				"int" => "i64",
    				"float" => "f64",
    				"string" => "String",
    				_ => concrete,
    			};
    			return rust_type.to_string();
    		}
    		let pascal_ph = pascal_str(placeholder);
    		let camel_ph = camel_str(placeholder);
    		let pascal_c = pascal_str(concrete);
    		let camel_c = camel_str(concrete);
    		let ph_len = placeholder.len();
    		if word.len() > ph_len {
    			if word.starts_with(&pascal_ph) {
    				let rest = &word[ph_len..];
    				if rest.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
    					return format!("{}{}", pascal_c, rest);
    				}
    			}
    			if word.starts_with(&camel_ph) {
    				let rest = &word[ph_len..];
    				if rest.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
    					return format!("{}{}", camel_c, rest);
    				}
    			}
    		}
    		let pascal_sep = format!("_{}_", pascal_ph);
    		let camel_sep = format!("_{}_", camel_ph);
    		let snake_c: String = {
    			let mut s = String::new();
    			for (i, c) in concrete.chars().enumerate() {
    				if c.is_uppercase() && i > 0 { s.push('_'); }
    				s.push(c.to_lowercase().next().unwrap());
    			}
    			s
    		};
    		let new_sep = format!("_{}_", snake_c);
    		if word.contains(&pascal_sep) {
    			return word.replace(&pascal_sep, &new_sep);
    		}
    		if word.contains(&camel_sep) {
    			return word.replace(&camel_sep, &new_sep);
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
    			result.push_str(&sub_word(&word, placeholder.as_str(), concrete.as_str()));
    		} else {
    			result.push(chars[i]);
    			i += 1;
    		}
    	}
    	result
    }
}

fn apply_t_substitution(tokens: Vec<Token>, placeholder: String, concrete: String) -> Vec<Token> {
    // transpiler-deor/importer/t_substitute.deor
    let mut result: Vec<Token> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    for index in 0..token_count {
        // transpiler-deor/importer/t_substitute.deor
        let mut tok: Token = tokens[index as usize].clone();
        let kind = tok.kind.clone();
        let value = tok.value.clone();
        let line = tok.line.clone();
        let file = tok.file.clone();
        if kind == "IDENT" {
            // transpiler-deor/importer/t_substitute.deor
            let mut new_value: String = apply_t_in_name(value.clone(), placeholder.clone(), concrete.clone());
            let mut tok_meta: TokenMeta = TokenMeta { line, file };
            let mut new_tok: Token = make_token(kind.clone(), new_value.clone(), tok_meta.clone());
            result.push(new_tok.clone());
        } else if kind == "RUST_BLOCK" {
            // transpiler-deor/importer/t_substitute.deor
            let mut new_content: String = replace_t_in_rust_block(value.clone(), placeholder.clone(), concrete.clone());
            let mut tok_meta: TokenMeta = TokenMeta { line, file };
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
fn name_of_decl(tokens: Vec<Token>, pos: i64, is_fn: bool) -> String {
    // transpiler-deor/importer/decl_bounds.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut name_offset: i64 = 1;
    if is_fn {
        // transpiler-deor/importer/decl_bounds.deor
        name_offset = 2;
    }
    let mut name_pos: i64 = pos + name_offset.clone();
    if name_pos < token_count {
        // transpiler-deor/importer/decl_bounds.deor
        let mut name_tok: Token = tokens[name_pos as usize].clone();
        let value = name_tok.value.clone();
        return value;
    }
    return "".to_string();
}

fn end_of_block(tokens: Vec<Token>, pos: i64) -> i64 {
    // transpiler-deor/importer/decl_bounds.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut cur: i64 = pos.clone();
    let mut depth: i64 = 0;
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

fn end_of_shape(tokens: Vec<Token>, pos: i64) -> i64 {
    // transpiler-deor/importer/decl_bounds.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut cur: i64 = pos.clone();
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
    let mut token_count: i64 = (tok_raw.len() as i64);
    let mut pos: i64 = 0;
    let mut depth: i64 = 0;
    let mut seen_decl: bool = false;
    loop {
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
        if is_new_import {
            // transpiler-deor/importer/load.deor
            let mut imp_r: ParseResult = scan_import_new(tok_raw.clone(), pos.clone());
            let mut imp_path: String = pr_code(imp_r.clone());
            let mut imp_end: i64 = pr_pos(imp_r.clone());
            let mut imp_t_concrete: String = "".to_string();
            let mut imp_t_placeholder: String = "".to_string();
            let mut where_r: ParseResult = scan_import_where(tok_raw.clone(), imp_end.clone());
            let imp_t_code = pr_code(where_r.clone());
            if !is_empty(imp_t_code.clone()) {
                // transpiler-deor/importer/load.deor
                imp_end = pr_pos(where_r.clone());
                let PIPE: String = "|".to_string();
                let list_code = s_split(imp_t_code.clone(), PIPE.clone());
                imp_t_placeholder = list_code[0 as usize].clone();
                imp_t_concrete = list_code[1 as usize].clone();
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
                    dedup_key = [imp_path.as_str(), "|", imp_t_placeholder.as_str(), "=", imp_t_concrete.as_str()].concat();
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
                        imp_tokens = apply_t_substitution(imp_tokens.clone(), imp_t_placeholder.clone(), imp_t_concrete.clone());
                    }
                    let mut imp_len: i64 = (imp_tokens.len() as i64);
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
    let mut token_count: i64 = (tokens.len() as i64);
    let mut pos: i64 = 0;
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
            // macro: dd_handle_block_decl (transpiler-deor/importer/macros/dd_handle_block_decl.deor)
            let mut dn_offset: i64 = 1;
            if is_fn {
                // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                dn_offset = 2;
            }
            let mut dn_pos: i64 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            let mut already_seen: bool = false;
            let mut cs_len: i64 = (seen.len() as i64);
            for cs_i in 0..cs_len {
                // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                    already_seen = true;
                    break;
                }
            }
            if !already_seen {
                // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                seen.push(decl_name.clone());
            }
            let mut fbe_cur: i64 = pos.clone();
            let mut fbe_depth: i64 = 0;
            let mut fbe_entered: bool = false;
            while fbe_cur < token_count {
                // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                let mut fbe_tok: Token = tokens[fbe_cur as usize].clone();
                let kind = fbe_tok.kind.clone();
                fbe_cur = fbe_cur + 1;
                if kind == "INDENT" {
                    // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                    fbe_depth = fbe_depth + 1;
                    fbe_entered = true;
                } else if kind == "DEDENT" {
                    // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                    fbe_depth = fbe_depth - 1;
                    if fbe_depth == 0 && fbe_entered {
                        // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                        break;
                    }
                }
            }
            let mut end_pos: i64 = fbe_cur.clone();
            if !already_seen {
                // transpiler-deor/importer/macros/dd_handle_block_decl.deor
                for i in (pos as usize)..(end_pos as usize) {
                	result.push(tokens[i].clone());
                }
            }
            pos = end_pos;
        } else if is_shape {
            // macro: dd_handle_shape (transpiler-deor/importer/macros/dd_handle_shape.deor)
            let mut dn_offset: i64 = 1;
            if is_fn {
                // transpiler-deor/importer/macros/dd_handle_shape.deor
                dn_offset = 2;
            }
            let mut dn_pos: i64 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                // transpiler-deor/importer/macros/dd_handle_shape.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            let mut already_seen: bool = false;
            let mut cs_len: i64 = (seen.len() as i64);
            for cs_i in 0..cs_len {
                // transpiler-deor/importer/macros/dd_handle_shape.deor
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    // transpiler-deor/importer/macros/dd_handle_shape.deor
                    already_seen = true;
                    break;
                }
            }
            if !already_seen {
                // transpiler-deor/importer/macros/dd_handle_shape.deor
                seen.push(decl_name.clone());
            }
            let mut fse_cur: i64 = pos.clone();
            while fse_cur < token_count {
                // transpiler-deor/importer/macros/dd_handle_shape.deor
                let mut fse_tok: Token = tokens[fse_cur as usize].clone();
                let kind = fse_tok.kind.clone();
                fse_cur = fse_cur + 1;
                if kind == "NEWLINE" {
                    // transpiler-deor/importer/macros/dd_handle_shape.deor
                    break;
                }
            }
            let mut end_pos: i64 = fse_cur.clone();
            if !already_seen {
                // transpiler-deor/importer/macros/dd_handle_shape.deor
                let mut copy_len: i64 = end_pos - pos.clone();
                for copy_idx in 0..copy_len {
                    // transpiler-deor/importer/macros/dd_handle_shape.deor
                    let mut tok_pos: i64 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = end_pos;
        } else if is_raw {
            // macro: dd_handle_raw (transpiler-deor/importer/macros/dd_handle_raw.deor)
            let mut dn_offset: i64 = 1;
            if is_fn {
                // transpiler-deor/importer/macros/dd_handle_raw.deor
                dn_offset = 2;
            }
            let mut dn_pos: i64 = pos + dn_offset.clone();
            let mut decl_name: String = "".to_string();
            if dn_pos < token_count {
                // transpiler-deor/importer/macros/dd_handle_raw.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let value = dn_tok.value.clone();
                decl_name = value;
            }
            let mut raw_pfx: String = "_raw_".to_string();
            let mut raw_key_parts: Vec<String> = vec![raw_pfx.clone(), decl_name.clone()];
            decl_name = s_join(raw_key_parts.clone());
            let mut already_seen: bool = false;
            let mut cs_len: i64 = (seen.len() as i64);
            for cs_i in 0..cs_len {
                // transpiler-deor/importer/macros/dd_handle_raw.deor
                let mut cs_val: String = seen[cs_i as usize].clone();
                if cs_val == decl_name {
                    // transpiler-deor/importer/macros/dd_handle_raw.deor
                    already_seen = true;
                    break;
                }
            }
            if !already_seen {
                // transpiler-deor/importer/macros/dd_handle_raw.deor
                seen.push(decl_name.clone());
            }
            let mut fse_cur: i64 = pos.clone();
            while fse_cur < token_count {
                // transpiler-deor/importer/macros/dd_handle_raw.deor
                let mut fse_tok: Token = tokens[fse_cur as usize].clone();
                let kind = fse_tok.kind.clone();
                fse_cur = fse_cur + 1;
                if kind == "NEWLINE" {
                    // transpiler-deor/importer/macros/dd_handle_raw.deor
                    break;
                }
            }
            let mut end_pos: i64 = fse_cur.clone();
            if !already_seen {
                // transpiler-deor/importer/macros/dd_handle_raw.deor
                let mut copy_len: i64 = end_pos - pos.clone();
                for copy_idx in 0..copy_len {
                    // transpiler-deor/importer/macros/dd_handle_raw.deor
                    let mut tok_pos: i64 = pos + copy_idx.clone();
                    let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                    result.push(copy_tok.clone());
                }
            }
            pos = end_pos;
        } else if is_rust_blk {
            // macro: dd_handle_rust_block (transpiler-deor/importer/macros/dd_handle_rust_block.deor)
            let mut rust_nl_pos: i64 = pos + 1.clone();
            let mut rust_content_pos: i64 = pos + 2.clone();
            let mut rust_is_block: bool = false;
            let mut block_value: String = "".to_string();
            if rust_content_pos < token_count {
                // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                let mut rust_nl_tok: Token = tokens[rust_nl_pos as usize].clone();
                let mut rust_tok: Token = tokens[rust_content_pos as usize].clone();
                let kind = rust_nl_tok.kind.clone();
                let mut rust_nl_ok: bool = kind == "NEWLINE".clone();
                let kind = rust_tok.kind.clone();
                let value = rust_tok.value.clone();
                let mut rust_block_ok: bool = kind == "RUST_BLOCK".clone();
                rust_is_block = rust_nl_ok && rust_block_ok;
                block_value = value;
            }
            if rust_is_block {
                // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                let mut rk_pfx: String = "_rust_".to_string();
                let mut rk_parts: Vec<String> = vec![rk_pfx.clone(), block_value.clone()];
                let mut decl_name: String = s_join(rk_parts.clone());
                let mut already_seen: bool = false;
                let mut cs_len: i64 = (seen.len() as i64);
                for cs_i in 0..cs_len {
                    // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                    let mut cs_val: String = seen[cs_i as usize].clone();
                    if cs_val == decl_name {
                        // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                        already_seen = true;
                        break;
                    }
                }
                if !already_seen {
                    // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                    seen.push(decl_name.clone());
                    for copy_idx in 0..3 {
                        // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                        let mut tok_pos: i64 = pos + copy_idx.clone();
                        let mut copy_tok: Token = tokens[tok_pos as usize].clone();
                        result.push(copy_tok.clone());
                    }
                }
                pos = pos + 3;
            } else {
                // transpiler-deor/importer/macros/dd_handle_rust_block.deor
                let mut rb_fallback_tok: Token = tokens[pos as usize].clone();
                result.push(rb_fallback_tok.clone());
                pos = pos + 1;
            }
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

// transpiler-deor/tokens_validator/casing.deor
fn is_pascal(name: String) -> bool {
    // transpiler-deor/tokens_validator/casing.deor
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i64 = (chars.len() as i64);
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
    let mut name_len: i64 = (chars.len() as i64);
    if name_len == 0 {
        // transpiler-deor/tokens_validator/casing.deor
        return false;
    }
    let mut first: String = chars[0 as usize].clone();
    if !s_lower_char(first.clone()) {
        // transpiler-deor/tokens_validator/casing.deor
        return false;
    }
    let mut idx: i64 = 0;
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

fn is_screaming_snake(name: String) -> bool {
    // transpiler-deor/tokens_validator/casing.deor
    let mut chars: Vec<String> = c_chars(name.clone());
    let mut name_len: i64 = (chars.len() as i64);
    let mut idx: i64 = 0;
    while idx < name_len {
        // transpiler-deor/tokens_validator/casing.deor
        let mut chr: String = chars[idx as usize].clone();
        if s_lower_char(chr.clone()) {
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
    let mut name_len: i64 = (chars.len() as i64);
    let mut idx: i64 = 0;
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
fn find_struct_field_str(reg: Vec<String>, name: String) -> String {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    let mut reg_count: i64 = (reg.len() as i64);
    let mut rdx: i64 = 0;
    while rdx < reg_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        let mut entry: String = reg[rdx as usize].clone();
        if entry == name {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut val_pos: i64 = rdx + 1.clone();
            if val_pos < reg_count {
                // transpiler-deor/tokens_validator/arg_helpers.deor
                let mut fields: String = reg[val_pos as usize].clone();
                return fields;
            }
        }
        rdx = rdx + 2;
    }
    return "".to_string();
}

fn arg_is_named(tokens: TokensRef, scan_pos: i64, kind: String) -> bool {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut check_pos: i64 = scan_pos.clone();
    let mut chk_kind: String = kind.clone();
    if kind == "KW_MOVE" {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        check_pos = scan_pos + 1;
        if check_pos < token_count {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut move_tok: Token = tokens[check_pos as usize].clone();
            let kind = move_tok.kind.clone();
            chk_kind = kind;
        }
    }
    if chk_kind != "IDENT" {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        return false;
    }
    let mut peek_pos: i64 = check_pos + 1.clone();
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

fn count_call_args(tokens: TokensRef, lp_pos: i64) -> i64 {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut cur: i64 = lp_pos + 1.clone();
    let mut depth: i64 = 0;
    let mut comma_count: i64 = 0;
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
    let mut result: i64 = 0;
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
    let mut error_count: i64 = (errors.len() as i64);
    if error_count > 0 {
        // transpiler-deor/tokens_validator/error_handling.deor
        let mut err_idx: i64 = 0;
        while err_idx < error_count {
            // transpiler-deor/tokens_validator/error_handling.deor
            let mut err_msg: String = errors[err_idx as usize].clone();
            println!("{}", err_msg.clone());
            err_idx = err_idx + 1;
        }
        std::process::exit(1);
    }
}

// transpiler-deor/macro_builder/macro_expander.deor
fn expand_deor_macros(tokens: Vec<Token>) -> Vec<Token> {
    // transpiler-deor/macro_builder/macro_expander.deor
    let mut macros: std::collections::HashMap<String, (Vec<Token>, i32)> = std::collections::HashMap::new();
    let mut result: Vec<Token> = vec![];
    let mut queue: std::collections::VecDeque<Token> = tokens.into_iter().collect();
    let mut scope_depth: i32 = 0;
    while let Some(cur) = queue.pop_front() {
    	let kind = cur.kind.as_str();

    	// track scope depth for macro privacy
    	if kind == "INDENT" { scope_depth += 1; }
    	if kind == "DEDENT" {
    		scope_depth -= 1;
    		// remove any macros defined at the depth we are leaving
    		macros.retain(|_, (_, def_depth)| *def_depth <= scope_depth);
    	}

    	// collect macro definition
    	if kind == "KW_MACRO" {
    		let name = if let Some(t) = queue.pop_front() { t.value } else { String::new() };
    		// skip NEWLINE then INDENT
    		while queue.front().map(|t| t.kind == "NEWLINE").unwrap_or(false) { queue.pop_front(); }
    		while queue.front().map(|t| t.kind == "INDENT").unwrap_or(false) { queue.pop_front(); }
    		// collect body tokens, excluding the outer INDENT/DEDENT pair
    		let mut body: Vec<Token> = vec![];
    		let mut depth: i32 = 1;
    		loop {
    			match queue.pop_front() {
    				None => break,
    				Some(t) => {
    					if t.kind == "KW_MACRO" {
    						let name_tok = queue.pop_front().unwrap_or(t.clone());
    						handle_errors(vec![val_err(name_tok, "macro".to_string(), "cannot be defined inside another macro body — use macro_run to call an existing macro".to_string())]);
    					} else if t.kind == "INDENT" {
    						depth += 1;
    						body.push(t);
    					} else if t.kind == "DEDENT" {
    						depth -= 1;
    						if depth == 0 { break; }
    						body.push(t);
    					} else {
    						body.push(t);
    					}
    				}
    			}
    		}
    		if !name.is_empty() { macros.insert(name, (body, scope_depth)); }
    		// skip trailing NEWLINE after the definition block
    		while queue.front().map(|t| t.kind == "NEWLINE").unwrap_or(false) { queue.pop_front(); }
    		continue;
    	}

    	// expand macro_run call site — prepend body to queue for recursive expansion
    	if kind == "KW_MACRO_RUN" {
    		let name = if let Some(t) = queue.pop_front() { t.value } else { String::new() };
    		// skip trailing NEWLINE after the call
    		if queue.front().map(|t| t.kind == "NEWLINE").unwrap_or(false) { queue.pop_front(); }
    		// prepend body tokens to front of queue so they are processed next
    		if let Some((body, _)) = macros.get(&name) {
    			let marker_file = body.first().map(|t| t.file.clone()).unwrap_or_default();
    			for tok in body.iter().rev() { queue.push_front(tok.clone()); }
    			queue.push_front(Token { kind: "MACRO_MARKER".to_string(), value: name.clone(), line: 0, file: marker_file });
    		}
    		continue;
    	}

    	result.push(cur);
    }
    result
}

// transpiler-deor/macro_builder/macro_validation.deor
fn validate_macros(raw_tokens: Vec<Token>) -> Vec<Token> {
    // transpiler-deor/macro_builder/macro_validation.deor
    let mut token_count: i64 = (raw_tokens.len() as i64);
    let mut errors: Vec<String> = Vec::new();
    let mut macro_names: Vec<String> = Vec::new();
    let mut pdx: i64 = 0;
    while pdx < token_count {
        // transpiler-deor/macro_builder/macro_validation.deor
        let mut tok: Token = raw_tokens[pdx as usize].clone();
        let kind = tok.kind.clone();
        if kind == "KW_MACRO" {
            // transpiler-deor/macro_builder/macro_validation.deor
            let mut nm_pos: i64 = pdx + 1.clone();
            if nm_pos < token_count {
                // transpiler-deor/macro_builder/macro_validation.deor
                let mut nm_tok: Token = raw_tokens[nm_pos as usize].clone();
                let kind = nm_tok.kind.clone();
                let value = nm_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/macro_builder/macro_validation.deor
                    macro_names.push(value.clone());
                }
            }
        }
        pdx = pdx + 1;
    }
    let mut lbl_macro: String = "macro_run".to_string();
    let mut rule_macro_run: String = "macro is not defined — check the name or add a 'macro <name>' definition".to_string();
    let mut mdx: i64 = 0;
    while mdx < token_count {
        // transpiler-deor/macro_builder/macro_validation.deor
        let mut tok: Token = raw_tokens[mdx as usize].clone();
        let kind = tok.kind.clone();
        if kind == "KW_MACRO_RUN" {
            // transpiler-deor/macro_builder/macro_validation.deor
            let mut nm_pos: i64 = mdx + 1.clone();
            if nm_pos < token_count {
                // transpiler-deor/macro_builder/macro_validation.deor
                let mut nm_tok: Token = raw_tokens[nm_pos as usize].clone();
                let kind = nm_tok.kind.clone();
                let value = nm_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/macro_builder/macro_validation.deor
                    if !list_has(macro_names.clone(), value.clone()) {
                        // transpiler-deor/macro_builder/macro_validation.deor
                        errors.push(val_err(nm_tok.clone(), lbl_macro.clone(), rule_macro_run.clone()).clone());
                    }
                }
            }
        }
        mdx = mdx + 1;
    }
    handle_errors(errors.clone());
    return raw_tokens;
}

// transpiler-deor/macro_builder/macro_builder.deor
fn build_macros(raw_tokens: Vec<Token>) -> Vec<Token> {
    // transpiler-deor/macro_builder/macro_builder.deor
    let mut validated: Vec<Token> = validate_macros(raw_tokens.clone());
    return expand_deor_macros(validated.clone());
}

// transpiler-deor/tokens_validator/tokens_validation.deor
type FnTestRule = fn(String);

fn validate_tokens(tokens: TokensRef) {
    // transpiler-deor/tokens_validator/tokens_validation.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut errors: Vec<String> = Vec::new();
    let mut pos: i64 = 0;
    let mut paren_depth: i64 = 0;
    let mut block_depth: i64 = 0;
    let mut in_void_fn: bool = false;
    let mut lbl_struct: String = "struct".to_string();
    let mut lbl_enum: String = "enum".to_string();
    let mut lbl_shape: String = "shape".to_string();
    let mut lbl_type: String = "type".to_string();
    let mut lbl_fn: String = "fn".to_string();
    let mut lbl_var: String = "variable".to_string();
    let mut lbl_call: String = "call to".to_string();
    let mut lbl_rust: String = "identifier".to_string();
    let mut lbl_decl: String = "declaration".to_string();
    let mut lbl_field: String = "struct field".to_string();
    let mut lbl_variant: String = "enum variant".to_string();
    // macro: define_errors (transpiler-deor/tokens_validator/macros/define_errors.deor)
    let mut rule_min3: String = "name must be at least 3 characters".to_string();
    let mut rule_no_option: String = "Rust generic types (Option/Vec/Box/Rc/Arc/Result) are not valid in Deor — use shapes or validator types".to_string();
    let mut rule_pascal: String = "name must be PascalCase (start with uppercase letter)".to_string();
    let mut rule_camel: String = "name must be camelCase (start lowercase, no underscores)".to_string();
    let mut rule_snake: String = "name must be lower_snake_case (no uppercase letters)".to_string();
    let mut rule_screaming: String = "const name must be SCREAMING_SNAKE_CASE (all caps, underscores between words)".to_string();
    let mut rule_named_arg: String = "each arg must be a named variable when passing 2 or more args".to_string();
    let mut rule_dup: String = "duplicate declaration — this name is already used by another struct, enum, shape, fn, or type".to_string();
    let mut rule_enum_pascal: String = "enum variant must be PascalCase".to_string();
    let mut rule_enum_data: String = "enum variants cannot carry data — use a struct alongside the enum instead".to_string();
    let mut rule_typed_enum_eq: String = "typed enum variant must have a value — add '= value' after the variant name".to_string();
    let mut rule_untyped_enum_eq: String = "untyped enum variant cannot have a value — use 'enum string/int Name' to associate values with variants".to_string();
    let mut rule_list_validator: String = "list shapes cannot be validator base types — validators only wrap primitives".to_string();
    let mut rule_max_params: String = "functions may have at most 3 parameters".to_string();
    let mut rule_param_shadow: String = "parameter name cannot be the same as its type — choose a descriptive name".to_string();
    let mut rule_type_param_shadow: String = "validator parameter name cannot be the same as the type name — use a descriptive name like 'val' or 'num'".to_string();
    let mut rule_no_ret: String = "missing return type — use 'fn void name()' for functions that return nothing".to_string();
    let mut rule_nested_fn: String = "functions may only be declared at the top level of a file — nested fn declarations are not allowed".to_string();
    let mut rule_void_return: String = "void functions must not use return — remove the return statement and let the function fall through".to_string();
    let mut rule_return_empty: String = "cannot return 'empty' — declare a validator type variable without a value and return it to signal not-valid".to_string();
    let mut rule_return_none: String = "none is not a Deor keyword — declare a validator type variable without a value and return it to signal not-valid".to_string();
    let mut rule_void_var: String = "'void' is not a valid variable type — only functions can return void".to_string();
    let mut rule_crash: String = "crash takes exactly 1 string argument".to_string();
    let mut rule_print_args: String = "print takes 1 argument, or 2 arguments where the second replaces the trailing newline".to_string();
    let mut rule_avow: String = "avow can only be used on a validator type variable".to_string();
    let mut rule_invalid_char: String = "character is not valid in Deor — use Deor operators and keywords; raw Rust syntax belongs inside a 'rust' block".to_string();
    let mut rule_validator_empty: String = "empty is not valid for validator types — declare without a value to start as not valid: 'Roll best'".to_string();
    let mut rule_bad_stmt: String = "literal cannot follow 'name ident' — capture in a named variable first".to_string();
    let mut rule_typed_as: String = "typed `as` bindings are not supported — use `a as b` to transfer ownership, or `Type a = move b` for an explicit typed move".to_string();
    let mut rule_as_move: String = "`as` already transfers ownership — use `a as b` instead of `a as move b`".to_string();
    let mut rule_bracket_index: String = "bracket indexing is not valid in Deor — use 'name at index' instead".to_string();
    let mut rule_empty_bracket: String = "use 'empty' to initialize an empty list — [] is only valid with items inside".to_string();
    let mut rule_move: String = "'move' can only precede a variable name — 'move 5' or 'move \"hello\"' are not valid".to_string();
    let mut rule_not_is: String = "use 'x is not y' instead of 'not x is y' — 'not' binds before 'is' resolves".to_string();
    let mut rule_kw_in_parens: String = "reserved keyword cannot be used as a name — choose a different variable name".to_string();
    let mut rule_valid: String = "'valid' can only appear after 'is' or 'is not' — it cannot be assigned or returned".to_string();
    let mut rule_end: String = "'end' can only appear directly after 'at' (list at end / list at end = val) — it cannot be used as a variable name or expression".to_string();
    let mut rule_with_parens: String = "'with' must be followed by a parenthesized field list — 'with (area)', not 'with area' — parens are required even for a single field".to_string();
    let mut rule_unmatched_open_paren: String = "'(' is never closed — every open paren needs a matching ')'".to_string();
    let mut rule_unmatched_close_paren: String = "')' has no matching '(' before it — remove the extra ')' or add the missing '('".to_string();
    let mut rule_const_reassign: String = "cannot reassign a const variable — const bindings are immutable".to_string();
    let mut rule_validator_reassign: String = "cannot reassign a validator type variable with '=' or 'as' — both skip the predicate check; use 'TypeName name = expr' to re-validate".to_string();
    let mut rule_raw_in_expr: String = "raw variables cannot be used in Deor operators, builtins, or rebindings — pass them to a function or consume them inside a rust block".to_string();
    let mut rule_raw_reassign: String = "raw variables cannot be reassigned — declare a new 'raw name = expr' instead".to_string();
    let mut rule_raw_assignment: String = "raw variables can only be assigned from a function call — use 'raw name = some_function()', not a literal or an inline rust block".to_string();
    let mut rule_no_func_field: String = "func shapes cannot be struct fields — pass the func shape as a function parameter instead".to_string();
    let mut rule_no_raw_field: String = "raw cannot be a struct field — raw values are opaque and cannot be stored in structs".to_string();
    let mut rule_struct_field_count: String = "wrong number of fields in struct construction — all fields must be provided".to_string();
    let mut rule_struct_field_name: String = "unknown field name in struct construction — variable name does not match any field in this struct".to_string();
    // macro: check_paren_balance (transpiler-deor/tokens_validator/macros/check_paren_balance.deor)
    let mut pb_depth: i64 = 0;
    let mut pb_open_line: i64 = 0;
    let mut pb_open_file: String = "".to_string();
    let mut pb_i: i64 = 0;
    while pb_i < token_count {
        // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
        let mut pb_tok: Token = tokens[pb_i as usize].clone();
        let mut kind = pb_tok.kind.clone();
        let mut line = pb_tok.line.clone();
        let mut file = pb_tok.file.clone();
        if kind == "LPAREN" {
            // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
            if pb_depth == 0 {
                // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
                pb_open_line = line;
                pb_open_file = file;
            }
            pb_depth = pb_depth + 1;
        } else if kind == "RPAREN" {
            // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
            if pb_depth == 0 {
                // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
                errors.push(val_err(pb_tok.clone(), lbl_var.clone(), rule_unmatched_close_paren.clone()).clone());
            } else {
                // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
                pb_depth = pb_depth - 1;
            }
        }
        pb_i = pb_i + 1;
    }
    if pb_depth > 0 {
        // transpiler-deor/tokens_validator/macros/check_paren_balance.deor
        let mut kind: String = "LPAREN".to_string();
        let mut value: String = "(".to_string();
        let mut line: i64 = pb_open_line.clone();
        let mut file: String = pb_open_file.clone();
        let mut pb_open_tok = Token { kind: kind.clone(), value: value.clone(), line: line.clone(), file: file.clone() };
        errors.push(val_err(pb_open_tok.clone(), lbl_var.clone(), rule_unmatched_open_paren.clone()).clone());
    }
    // transpiler-deor/tokens_validator/tokens_validation.deor
    handle_errors(errors.clone());
    let mut forbidden_in_parens: Vec<String> = vec!["KW_LIST".to_string(), "KW_STRUCT".to_string(), "KW_SHAPE".to_string(), "KW_ENUM".to_string(), "KW_TYPE".to_string(), "KW_FN".to_string(), "KW_OF".to_string(), "KW_FOR".to_string(), "KW_IF".to_string(), "KW_ELSE".to_string(), "KW_RETURN".to_string(), "KW_BREAK".to_string(), "KW_CONTINUE".to_string(), "KW_REMOVE".to_string(), "KW_RUST".to_string(), "KW_IMPORT".to_string(), "KW_MACRO".to_string(), "KW_VOID".to_string(), "KW_RAW".to_string()];
    let mut reserved_keywords: Vec<String> = vec!["KW_AND".to_string(), "KW_AS".to_string(), "KW_AT".to_string(), "KW_AVOW".to_string(), "KW_BLOCK".to_string(), "KW_BREAK".to_string(), "KW_CONST".to_string(), "KW_CONTINUE".to_string(), "KW_ELSE".to_string(), "KW_EMPTY".to_string(), "KW_ENUM".to_string(), "KW_FALSE".to_string(), "KW_FN".to_string(), "KW_FOR".to_string(), "KW_FUNC".to_string(), "KW_IF".to_string(), "KW_IMPORT".to_string(), "KW_IN".to_string(), "KW_IS".to_string(), "KW_LIST".to_string(), "KW_MACRO".to_string(), "KW_MACRO_RUN".to_string(), "KW_MOVE".to_string(), "KW_NONE".to_string(), "KW_NOT".to_string(), "KW_OF".to_string(), "KW_OR".to_string(), "KW_RAW".to_string(), "KW_REMOVE".to_string(), "KW_RETURN".to_string(), "KW_RUST".to_string(), "KW_SHAPE".to_string(), "KW_STRUCT".to_string(), "KW_TO".to_string(), "KW_TRUE".to_string(), "KW_TYPE".to_string(), "KW_VALID".to_string(), "KW_VOID".to_string(), "KW_WITH".to_string()];
    let mut func_shape_names: Vec<String> = Vec::new();
    let mut validator_type_names: Vec<String> = Vec::new();
    let mut pre_i: i64 = 0;
    while pre_i < token_count {
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut pre_tok: Token = tokens[pre_i as usize].clone();
        let mut kind = pre_tok.kind.clone();
        // macro: prescan_collect_func_shapes (transpiler-deor/tokens_validator/macros/prescan_collect_func_shapes.deor)
        if kind == "KW_SHAPE" {
            // transpiler-deor/tokens_validator/macros/prescan_collect_func_shapes.deor
            let mut cfs_form_pos: i64 = pre_i + 3.clone();
            if cfs_form_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_collect_func_shapes.deor
                let mut cfs_form_tok: Token = tokens[cfs_form_pos as usize].clone();
                let mut kind = cfs_form_tok.kind.clone();
                if kind == "KW_FUNC" {
                    // transpiler-deor/tokens_validator/macros/prescan_collect_func_shapes.deor
                    let mut cfs_name_pos: i64 = pre_i + 1.clone();
                    if cfs_name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan_collect_func_shapes.deor
                        let mut cfs_name_tok: Token = tokens[cfs_name_pos as usize].clone();
                        let mut kind = cfs_name_tok.kind.clone();
                        let mut value = cfs_name_tok.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/prescan_collect_func_shapes.deor
                            func_shape_names.push(value.clone());
                        }
                    }
                }
            }
        }
        // macro: prescan_collect_validator_types (transpiler-deor/tokens_validator/macros/prescan_collect_validator_types.deor)
        if kind == "KW_TYPE" {
            // transpiler-deor/tokens_validator/macros/prescan_collect_validator_types.deor
            let mut pvt_name_p: i64 = pre_i + 1.clone();
            if pvt_name_p < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_collect_validator_types.deor
                let mut pvt_tok: Token = tokens[pvt_name_p as usize].clone();
                let mut kind = pvt_tok.kind.clone();
                let mut value = pvt_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_collect_validator_types.deor
                    validator_type_names.push(value.clone());
                }
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        pre_i = pre_i + 1;
    }
    let mut shape_names: Vec<String> = Vec::new();
    let mut decl_names: Vec<String> = Vec::new();
    let mut struct_field_reg: Vec<String> = Vec::new();
    let mut validator_vars: Vec<String> = Vec::new();
    let mut raw_var_names: Vec<String> = Vec::new();
    let mut const_var_names: Vec<String> = Vec::new();
    pre_i = 0;
    while pre_i < token_count {
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut pre_tok: Token = tokens[pre_i as usize].clone();
        let mut kind = pre_tok.kind.clone();
        // macro: prescan_collect_shapes (transpiler-deor/tokens_validator/macros/prescan_collect_shapes.deor)
        if kind == "KW_SHAPE" {
            // transpiler-deor/tokens_validator/macros/prescan_collect_shapes.deor
            let mut sn_pos: i64 = pre_i + 1.clone();
            if sn_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_collect_shapes.deor
                let mut sn_tok: Token = tokens[sn_pos as usize].clone();
                let mut value = sn_tok.value.clone();
                shape_names.push(value.clone());
            }
        }
        // macro: prescan_collect_const_names (transpiler-deor/tokens_validator/macros/prescan_collect_const_names.deor)
        if kind == "KW_CONST" {
            // transpiler-deor/tokens_validator/macros/prescan_collect_const_names.deor
            let mut cn_name_pos: i64 = pre_i + 2.clone();
            if cn_name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_collect_const_names.deor
                let mut cn_name_tok: Token = tokens[cn_name_pos as usize].clone();
                let mut kind = cn_name_tok.kind.clone();
                let mut value = cn_name_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_collect_const_names.deor
                    if !list_has(const_var_names.clone(), value.clone()) {
                        // transpiler-deor/tokens_validator/macros/prescan_collect_const_names.deor
                        const_var_names.push(value.clone());
                    }
                }
            }
        }
        // macro: prescan_check_duplicate_decls (transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor)
        let mut is_kw_struct: bool = kind == "KW_STRUCT".clone();
        let mut is_kw_enum: bool = kind == "KW_ENUM".clone();
        let mut is_kw_shape: bool = kind == "KW_SHAPE".clone();
        let mut is_kw_type: bool = kind == "KW_TYPE".clone();
        let mut is_named_decl: bool = is_kw_struct || is_kw_enum || is_kw_shape || is_kw_type.clone();
        if is_named_decl {
            // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
            let mut dn_pos: i64 = pre_i + 1.clone();
            if dn_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                let mut dn_tok: Token = tokens[dn_pos as usize].clone();
                let mut kind = dn_tok.kind.clone();
                let mut value = dn_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                    let mut dn_is_typed_kw: bool = false;
                    if value == "string" {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        dn_is_typed_kw = true;
                    } else if value == "int" {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        dn_is_typed_kw = true;
                    } else if value == "float" {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        dn_is_typed_kw = true;
                    } else if value == "bool" {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        dn_is_typed_kw = true;
                    }
                    if is_kw_enum {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        if dn_is_typed_kw {
                            // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                            let mut dn_name_pos: i64 = dn_pos + 1.clone();
                            if dn_name_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                                let mut dn_name_tok: Token = tokens[dn_name_pos as usize].clone();
                                let mut kind = dn_name_tok.kind.clone();
                                let mut value = dn_name_tok.value.clone();
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                                    if list_has(decl_names.clone(), value.clone()) {
                                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                                        errors.push(val_err(dn_name_tok.clone(), lbl_decl.clone(), rule_dup.clone()).clone());
                                    } else {
                                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                                        decl_names.push(value.clone());
                                    }
                                }
                            }
                        } else {
                            // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                            if list_has(decl_names.clone(), value.clone()) {
                                // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                                errors.push(val_err(dn_tok.clone(), lbl_decl.clone(), rule_dup.clone()).clone());
                            } else {
                                // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                                decl_names.push(value.clone());
                            }
                        }
                    } else {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        if list_has(decl_names.clone(), value.clone()) {
                            // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                            errors.push(val_err(dn_tok.clone(), lbl_decl.clone(), rule_dup.clone()).clone());
                        } else {
                            // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                            decl_names.push(value.clone());
                        }
                    }
                }
            }
        }
        if kind == "KW_FN" {
            // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
            let mut fn_name_pos: i64 = pre_i + 2.clone();
            if fn_name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                let mut fn_name_tok: Token = tokens[fn_name_pos as usize].clone();
                let mut kind = fn_name_tok.kind.clone();
                let mut value = fn_name_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                    if list_has(decl_names.clone(), value.clone()) {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        errors.push(val_err(fn_name_tok.clone(), lbl_decl.clone(), rule_dup.clone()).clone());
                    } else {
                        // transpiler-deor/tokens_validator/macros/prescan_check_duplicate_decls.deor
                        decl_names.push(value.clone());
                    }
                }
            }
        }
        // macro: prescan_check_struct_fields (transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor)
        if kind == "KW_STRUCT" {
            // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
            let mut sf_struct_name: String = "".to_string();
            let mut sf_name_p: i64 = pre_i + 1.clone();
            if sf_name_p < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                let mut sf_nm: Token = tokens[sf_name_p as usize].clone();
                let mut kind = sf_nm.kind.clone();
                let mut value = sf_nm.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                    sf_struct_name = value;
                }
            }
            let mut sf_pos: i64 = pre_i + 1.clone();
            while sf_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                let mut sf_tok: Token = tokens[sf_pos as usize].clone();
                let mut kind = sf_tok.kind.clone();
                if kind == "INDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                    break;
                }
                sf_pos = sf_pos + 1;
            }
            sf_pos = sf_pos + 1;
            let mut sf_fields: Vec<String> = Vec::new();
            while sf_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                let mut sf_tok: Token = tokens[sf_pos as usize].clone();
                let mut kind = sf_tok.kind.clone();
                if kind == "DEDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                    break;
                }
                if kind == "KW_RAW" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                    let mut sf_raw_name_pos: i64 = sf_pos + 1.clone();
                    if sf_raw_name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                        let mut sf_raw_name_tok: Token = tokens[sf_raw_name_pos as usize].clone();
                        let mut kind = sf_raw_name_tok.kind.clone();
                        let mut value = sf_raw_name_tok.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                            errors.push(val_err(sf_raw_name_tok.clone(), lbl_field.clone(), rule_no_raw_field.clone()).clone());
                        }
                    }
                }
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                    let mut value = sf_tok.value.clone();
                    let mut sf_field_type: String = value.clone();
                    let mut field_name_pos: i64 = sf_pos + 1.clone();
                    if field_name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                        let mut field_name_tok: Token = tokens[field_name_pos as usize].clone();
                        let mut kind = field_name_tok.kind.clone();
                        let mut value = field_name_tok.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                            if (value.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                                errors.push(val_err(field_name_tok.clone(), lbl_field.clone(), rule_min3.clone()).clone());
                            }
                            if !is_snake(value.clone()) {
                                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                                errors.push(val_err(field_name_tok.clone(), lbl_field.clone(), rule_snake.clone()).clone());
                            }
                            let mut sf_is_func: bool = list_has(func_shape_names.clone(), sf_field_type.clone());
                            if sf_is_func {
                                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                                errors.push(val_err(field_name_tok.clone(), lbl_field.clone(), rule_no_func_field.clone()).clone());
                            }
                            sf_fields.push(value.clone());
                        } else if list_has(reserved_keywords.clone(), kind.clone()) {
                            // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                            errors.push(val_err(field_name_tok.clone(), lbl_field.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                }
                sf_pos = sf_pos + 1;
            }
            let mut sf_has_name: bool = sf_struct_name != "".clone();
            if sf_has_name {
                // transpiler-deor/tokens_validator/macros/prescan_check_struct_fields.deor
                let mut sf_sep: String = ",".to_string();
                let mut sf_fields_str: String = s_join_with(sf_fields.clone(), sf_sep.clone());
                struct_field_reg.push(sf_struct_name.clone());
                struct_field_reg.push(sf_fields_str.clone());
            }
        }
        // macro: prescan_check_enum_variants (transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor)
        if kind == "KW_ENUM" {
            // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
            let mut ev_is_typed: bool = false;
            let mut ev_pos: i64 = pre_i + 1.clone();
            if ev_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                let mut ev_type_tok: Token = tokens[ev_pos as usize].clone();
                let mut value = ev_type_tok.value.clone();
                if value == "string" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    ev_is_typed = true;
                } else if value == "int" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    ev_is_typed = true;
                } else if value == "float" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    ev_is_typed = true;
                } else if value == "bool" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    ev_is_typed = true;
                }
            }
            while ev_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                let mut ev_tok: Token = tokens[ev_pos as usize].clone();
                let mut kind = ev_tok.kind.clone();
                if kind == "INDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    break;
                }
                ev_pos = ev_pos + 1;
            }
            ev_pos = ev_pos + 1;
            while ev_pos < token_count {
                // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                let mut ev_tok: Token = tokens[ev_pos as usize].clone();
                let mut kind = ev_tok.kind.clone();
                if kind == "DEDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    break;
                }
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    let mut value = ev_tok.value.clone();
                    if (value.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                        errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_min3.clone()).clone());
                    }
                    if !is_pascal(value.clone()) {
                        // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                        errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_enum_pascal.clone()).clone());
                    }
                    let mut after_variant: i64 = ev_pos + 1.clone();
                    if after_variant < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                        let mut after_tok: Token = tokens[after_variant as usize].clone();
                        let mut kind = after_tok.kind.clone();
                        if ev_is_typed {
                            // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                            if kind != "EQUALS" {
                                // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                                errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_typed_enum_eq.clone()).clone());
                            }
                        } else {
                            // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                            if kind == "LPAREN" {
                                // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                                errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_enum_data.clone()).clone());
                            }
                            if kind == "EQUALS" {
                                // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                                errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_untyped_enum_eq.clone()).clone());
                            }
                        }
                    }
                } else if list_has(reserved_keywords.clone(), kind.clone()) {
                    // transpiler-deor/tokens_validator/macros/prescan_check_enum_variants.deor
                    errors.push(val_err(ev_tok.clone(), lbl_variant.clone(), rule_kw_in_parens.clone()).clone());
                }
                ev_pos = ev_pos + 1;
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        pre_i = pre_i + 1;
    }
    while pos < token_count {
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut tok: Token = tokens[pos as usize].clone();
        let mut kind = tok.kind.clone();
        let mut value = tok.value.clone();
        let mut line = tok.line.clone();
        let mut file = tok.file.clone();
        let mut cur_kind: String = kind.clone();
        let mut cur_val: String = value.clone();
        let mut cur_line: i64 = line.clone();
        let mut cur_file: String = file.clone();
        // macro: track_paren_depth (transpiler-deor/tokens_validator/macros/track_paren_depth.deor)
        if cur_kind == "LPAREN" {
            // transpiler-deor/tokens_validator/macros/track_paren_depth.deor
            paren_depth = paren_depth + 1;
        }
        if cur_kind == "RPAREN" {
            // transpiler-deor/tokens_validator/macros/track_paren_depth.deor
            paren_depth = paren_depth - 1;
        }
        // macro: track_block_scope (transpiler-deor/tokens_validator/macros/track_block_scope.deor)
        if cur_kind == "INDENT" {
            // transpiler-deor/tokens_validator/macros/track_block_scope.deor
            block_depth = block_depth + 1;
        }
        if cur_kind == "DEDENT" {
            // transpiler-deor/tokens_validator/macros/track_block_scope.deor
            block_depth = block_depth - 1;
            if block_depth == 0 {
                // transpiler-deor/tokens_validator/macros/track_block_scope.deor
                in_void_fn = false;
            }
        }
        if cur_kind == "KW_FN" {
            // transpiler-deor/tokens_validator/macros/track_block_scope.deor
            if block_depth == 0 {
                // transpiler-deor/tokens_validator/macros/track_block_scope.deor
                validator_vars.clear();
            }
            let mut void_check: i64 = pos + 1.clone();
            if void_check < token_count {
                // transpiler-deor/tokens_validator/macros/track_block_scope.deor
                let mut void_tok: Token = tokens[void_check as usize].clone();
                let mut kind = void_tok.kind.clone();
                if kind == "KW_VOID" {
                    // transpiler-deor/tokens_validator/macros/track_block_scope.deor
                    in_void_fn = true;
                }
            }
        }
        // macro: check_void_return (transpiler-deor/tokens_validator/macros/check_void_return.deor)
        if cur_kind == "KW_RETURN" {
            // transpiler-deor/tokens_validator/macros/check_void_return.deor
            if in_void_fn {
                // transpiler-deor/tokens_validator/macros/check_void_return.deor
                errors.push(val_err(tok.clone(), lbl_fn.clone(), rule_void_return.clone()).clone());
            }
        }
        // macro: check_return_invalid (transpiler-deor/tokens_validator/macros/check_return_invalid.deor)
        if cur_kind == "KW_RETURN" {
            // transpiler-deor/tokens_validator/macros/check_return_invalid.deor
            let mut ri_next: i64 = pos + 1.clone();
            if ri_next < token_count {
                // transpiler-deor/tokens_validator/macros/check_return_invalid.deor
                let mut ri_tok: Token = tokens[ri_next as usize].clone();
                let mut kind = ri_tok.kind.clone();
                if kind == "KW_EMPTY" {
                    // transpiler-deor/tokens_validator/macros/check_return_invalid.deor
                    errors.push(val_err(ri_tok.clone(), lbl_fn.clone(), rule_return_empty.clone()).clone());
                }
                if kind == "KW_NONE" {
                    // transpiler-deor/tokens_validator/macros/check_return_invalid.deor
                    errors.push(val_err(ri_tok.clone(), lbl_fn.clone(), rule_return_none.clone()).clone());
                }
            }
        }
        // macro: check_move_target (transpiler-deor/tokens_validator/macros/check_move_target.deor)
        if cur_kind == "KW_MOVE" {
            // transpiler-deor/tokens_validator/macros/check_move_target.deor
            let mut mv_next: i64 = pos + 1.clone();
            if mv_next < token_count {
                // transpiler-deor/tokens_validator/macros/check_move_target.deor
                let mut mv_tok: Token = tokens[mv_next as usize].clone();
                let mut kind = mv_tok.kind.clone();
                let mut mv_ok: bool = kind == "IDENT".clone();
                let mut mv_destruct: bool = kind == "LPAREN".clone();
                let mut mv_valid: bool = mv_ok || mv_destruct.clone();
                if !mv_valid {
                    // transpiler-deor/tokens_validator/macros/check_move_target.deor
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_move.clone()).clone());
                }
            }
        }
        // macro: check_keyword_in_parens (transpiler-deor/tokens_validator/macros/check_keyword_in_parens.deor)
        if paren_depth > 0 {
            // transpiler-deor/tokens_validator/macros/check_keyword_in_parens.deor
            let mut is_forbidden: bool = list_has(forbidden_in_parens.clone(), cur_kind.clone());
            if is_forbidden {
                // transpiler-deor/tokens_validator/macros/check_keyword_in_parens.deor
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
            }
        }
        // macro: check_kw_as_name (transpiler-deor/tokens_validator/macros/check_kw_as_name.deor)
        let mut kan_is_reserved: bool = list_has(reserved_keywords.clone(), cur_kind.clone());
        if kan_is_reserved {
            // transpiler-deor/tokens_validator/macros/check_kw_as_name.deor
            let mut kan_next_pos: i64 = pos + 1.clone();
            if kan_next_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_kw_as_name.deor
                let mut kan_next_tok: Token = tokens[kan_next_pos as usize].clone();
                let mut kind = kan_next_tok.kind.clone();
                let mut kan_next_is_eq: bool = kind == "EQUALS".clone();
                let mut kan_next_is_as: bool = kind == "KW_AS".clone();
                if kan_next_is_eq || kan_next_is_as {
                    // transpiler-deor/tokens_validator/macros/check_kw_as_name.deor
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
                }
            }
        }
        // macro: check_destructure_binding_kw (transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor)
        if cur_kind == "LPAREN" {
            // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
            let mut dbk_is_call: bool = false;
            if pos > 0 {
                // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                let mut dbk_prev_pos: i64 = pos - 1.clone();
                let mut dbk_prev_tok: Token = tokens[dbk_prev_pos as usize].clone();
                let mut kind = dbk_prev_tok.kind.clone();
                dbk_is_call = kind == "IDENT";
            }
            if !dbk_is_call {
                // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                let mut dbk_scan: i64 = pos + 1.clone();
                let mut dbk_depth: i64 = 1;
                while dbk_scan < token_count {
                    // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                    let mut dbk_dtok: Token = tokens[dbk_scan as usize].clone();
                    let mut kind = dbk_dtok.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                        dbk_depth = dbk_depth + 1;
                    } else if kind == "RPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                        dbk_depth = dbk_depth - 1;
                        if dbk_depth == 0 {
                            // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                            break;
                        }
                    }
                    dbk_scan = dbk_scan + 1;
                }
                let mut dbk_after_pos: i64 = dbk_scan + 1.clone();
                let mut dbk_is_destructure: bool = false;
                if dbk_after_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                    let mut dbk_after_tok: Token = tokens[dbk_after_pos as usize].clone();
                    let mut kind = dbk_after_tok.kind.clone();
                    dbk_is_destructure = kind == "KW_IN";
                }
                if dbk_is_destructure {
                    // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                    let mut dbk_name_pos: i64 = pos + 1.clone();
                    let mut dbk_at_name_slot: bool = true;
                    while dbk_name_pos < dbk_scan {
                        // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                        let mut dbk_name_tok: Token = tokens[dbk_name_pos as usize].clone();
                        let mut kind = dbk_name_tok.kind.clone();
                        if kind == "COMMA" {
                            // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                            dbk_at_name_slot = true;
                        } else if dbk_at_name_slot {
                            // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                            if kind != "IDENT" {
                                // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                                if list_has(reserved_keywords.clone(), kind.clone()) {
                                    // transpiler-deor/tokens_validator/macros/check_destructure_binding_kw.deor
                                    errors.push(val_err(dbk_name_tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
                                }
                            }
                            dbk_at_name_slot = false;
                        }
                        dbk_name_pos = dbk_name_pos + 1;
                    }
                }
            }
        }
        // macro: check_with_parens (transpiler-deor/tokens_validator/macros/check_with_parens.deor)
        if cur_kind == "KW_WITH" {
            // transpiler-deor/tokens_validator/macros/check_with_parens.deor
            let mut with_ok: bool = false;
            let mut with_next_pos: i64 = pos + 1.clone();
            if with_next_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_with_parens.deor
                let mut with_next_tok: Token = tokens[with_next_pos as usize].clone();
                let mut kind = with_next_tok.kind.clone();
                with_ok = kind == "LPAREN";
            }
            if !with_ok {
                // transpiler-deor/tokens_validator/macros/check_with_parens.deor
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_with_parens.clone()).clone());
            }
        }
        // macro: skip_rust_block (transpiler-deor/tokens_validator/macros/skip_rust_block.deor)
        if cur_kind == "KW_RUST" {
            // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
            let mut srb_nl_pos: i64 = pos + 1.clone();
            let mut srb_block_pos: i64 = pos + 2.clone();
            let mut srb_is_block: bool = false;
            if srb_block_pos < token_count {
                // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
                let mut srb_nl_tok: Token = tokens[srb_nl_pos as usize].clone();
                let mut srb_block_tok: Token = tokens[srb_block_pos as usize].clone();
                let mut kind = srb_nl_tok.kind.clone();
                let mut srb_nl_ok: bool = kind == "NEWLINE".clone();
                let mut kind = srb_block_tok.kind.clone();
                let mut srb_block_ok: bool = kind == "RUST_BLOCK".clone();
                srb_is_block = srb_nl_ok && srb_block_ok;
            }
            if srb_is_block {
                // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
                pos = srb_block_pos + 1;
                continue;
            }
        }
        // macro: check_not_is_order (transpiler-deor/tokens_validator/macros/check_not_is_order.deor)
        if cur_kind == "KW_NOT" {
            // transpiler-deor/tokens_validator/macros/check_not_is_order.deor
            let mut next_not: i64 = pos + 1.clone();
            let mut after_not: i64 = pos + 2.clone();
            if after_not < token_count {
                // transpiler-deor/tokens_validator/macros/check_not_is_order.deor
                let mut next_not_tok: Token = tokens[next_not as usize].clone();
                let mut after_not_tok: Token = tokens[after_not as usize].clone();
                let mut kind = next_not_tok.kind.clone();
                let mut next_not_kind: String = kind.clone();
                let mut kind = after_not_tok.kind.clone();
                let mut after_not_kind: String = kind.clone();
                let mut next_is_ident: bool = next_not_kind == "IDENT".clone();
                let mut after_is_is: bool = after_not_kind == "KW_IS".clone();
                if next_is_ident && after_is_is {
                    // transpiler-deor/tokens_validator/macros/check_not_is_order.deor
                    let mut value = next_not_tok.value.clone();
                    errors.push(val_err(next_not_tok.clone(), lbl_var.clone(), rule_not_is.clone()).clone());
                }
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut validate_indent_offset = 1;
        let mut keyword: String = "KW_STRUCT".to_string();
        let mut lbl: String = lbl_struct.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/validate_ident.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/validate_ident.deor
            let mut name_pos: i64 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/validate_ident.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let mut kind = name_tok.kind.clone();
                let mut value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    if (name_val.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
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
        let mut ev_name_offset: i64 = 1;
        if cur_kind == "KW_ENUM" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut ev_type_pos: i64 = pos + 1.clone();
            if ev_type_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut ev_type_tok: Token = tokens[ev_type_pos as usize].clone();
                let mut value = ev_type_tok.value.clone();
                if value == "string" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    ev_name_offset = 2;
                } else if value == "int" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    ev_name_offset = 2;
                } else if value == "float" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    ev_name_offset = 2;
                } else if value == "bool" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    ev_name_offset = 2;
                }
            }
        }
        let mut validate_indent_offset = ev_name_offset;
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/validate_ident.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/validate_ident.deor
            let mut name_pos: i64 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/validate_ident.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let mut kind = name_tok.kind.clone();
                let mut value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    if (name_val.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                }
            }
            pos = pos + 1;
            continue;
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut validate_indent_offset = 1;
        let mut keyword: String = "KW_SHAPE".to_string();
        let mut lbl: String = lbl_shape.clone();
        let mut rule: String = rule_camel.clone();
        let mut test_rule: fn(String) -> bool = is_camel.clone();
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/validate_ident.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/validate_ident.deor
            let mut name_pos: i64 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/validate_ident.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let mut kind = name_tok.kind.clone();
                let mut value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    if (name_val.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                }
            }
            pos = pos + 1;
            continue;
        }
        // macro: check_type_base_not_shape (transpiler-deor/tokens_validator/macros/check_type_base_not_shape.deor)
        if cur_kind == "KW_TYPE" {
            // transpiler-deor/tokens_validator/macros/check_type_base_not_shape.deor
            let mut base_type_pos: i64 = pos + 3.clone();
            if base_type_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_type_base_not_shape.deor
                let mut base_type_tok: Token = tokens[base_type_pos as usize].clone();
                let mut value = base_type_tok.value.clone();
                let mut base_is_shape: bool = list_has(shape_names.clone(), value.clone());
                if base_is_shape {
                    // transpiler-deor/tokens_validator/macros/check_type_base_not_shape.deor
                    let mut type_name_pos: i64 = pos + 1.clone();
                    let mut type_name_tok: Token = tokens[type_name_pos as usize].clone();
                    errors.push(val_err(type_name_tok.clone(), lbl_type.clone(), rule_list_validator.clone()).clone());
                }
            }
        }
        // macro: check_validator_declaration (transpiler-deor/tokens_validator/macros/check_validator_declaration.deor)
        if cur_kind == "KW_TYPE" {
            // transpiler-deor/tokens_validator/macros/check_validator_declaration.deor
            let mut vd_name_pos: i64 = pos + 1.clone();
            let mut vd_lp_pos: i64 = pos + 2.clone();
            let mut vd_ptype_pos: i64 = pos + 3.clone();
            let mut vd_pname_pos: i64 = pos + 4.clone();
            if vd_pname_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_validator_declaration.deor
                let mut vd_name_tok: Token = tokens[vd_name_pos as usize].clone();
                let mut vd_lp_tok: Token = tokens[vd_lp_pos as usize].clone();
                let mut vd_ptype_tok: Token = tokens[vd_ptype_pos as usize].clone();
                let mut vd_pname_tok: Token = tokens[vd_pname_pos as usize].clone();
                let mut kind = vd_name_tok.kind.clone();
                let mut value = vd_name_tok.value.clone();
                let mut vd_type_name: String = value.clone();
                let mut kind = vd_lp_tok.kind.clone();
                if kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/macros/check_validator_declaration.deor
                    let mut value = vd_ptype_tok.value.clone();
                    let mut vd_param_type: String = value.clone();
                    let mut kind = vd_pname_tok.kind.clone();
                    let mut value = vd_pname_tok.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/check_validator_declaration.deor
                        if value == vd_type_name {
                            // transpiler-deor/tokens_validator/macros/check_validator_declaration.deor
                            errors.push(val_err(vd_pname_tok.clone(), lbl_type.clone(), rule_type_param_shadow.clone()).clone());
                        }
                        if value == vd_param_type {
                            // transpiler-deor/tokens_validator/macros/check_validator_declaration.deor
                            errors.push(val_err(vd_pname_tok.clone(), lbl_type.clone(), rule_param_shadow.clone()).clone());
                        }
                    }
                }
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut keyword: String = "KW_TYPE".to_string();
        let mut lbl: String = lbl_type.clone();
        let mut rule: String = rule_pascal.clone();
        let mut test_rule: fn(String) -> bool = is_pascal.clone();
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/validate_ident.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/validate_ident.deor
            let mut name_pos: i64 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/validate_ident.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let mut kind = name_tok.kind.clone();
                let mut value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    if (name_val.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                }
            }
            pos = pos + 1;
            continue;
        }
        // macro: check_fn_declaration (transpiler-deor/tokens_validator/macros/check_fn_declaration.deor)
        if cur_kind == "KW_FN" {
            // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
            if block_depth > 0 {
                // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                let mut nf_name_pos: i64 = pos + 2.clone();
                let mut nf_name_tok: Token = tokens[nf_name_pos as usize].clone();
                errors.push(val_err(nf_name_tok.clone(), lbl_fn.clone(), rule_nested_fn.clone()).clone());
            }
            let mut lp_pos: i64 = pos + 3.clone();
            if lp_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                let mut lp_tok: Token = tokens[lp_pos as usize].clone();
                let mut kind = lp_tok.kind.clone();
                if kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                    let mut param_count: i64 = count_call_args(tokens.clone(), lp_pos.clone());
                    if param_count > 3 {
                        // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                        let mut fn_name_pos: i64 = pos + 2.clone();
                        let mut fn_name_tok: Token = tokens[fn_name_pos as usize].clone();
                        errors.push(val_err(fn_name_tok.clone(), lbl_fn.clone(), rule_max_params.clone()).clone());
                    }
                    let mut ps_pos: i64 = lp_pos + 1.clone();
                    while ps_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                        let mut ps_tok: Token = tokens[ps_pos as usize].clone();
                        let mut kind = ps_tok.kind.clone();
                        let mut value = ps_tok.value.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                            break;
                        }
                        if kind == "COMMA" {
                            // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                            ps_pos = ps_pos + 1;
                            continue;
                        }
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                            let mut param_type_val: String = value.clone();
                            let mut pn_pos: i64 = ps_pos + 1.clone();
                            if pn_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                                let mut pn_tok: Token = tokens[pn_pos as usize].clone();
                                let mut kind = pn_tok.kind.clone();
                                let mut value = pn_tok.value.clone();
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                                    if value == param_type_val {
                                        // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                                        errors.push(val_err(pn_tok.clone(), lbl_fn.clone(), rule_param_shadow.clone()).clone());
                                    }
                                    if (value.len() as i64) < 3 {
                                        // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                                        errors.push(val_err(pn_tok.clone(), lbl_fn.clone(), rule_min3.clone()).clone());
                                    }
                                    if !is_snake(value.clone()) {
                                        // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                                        errors.push(val_err(pn_tok.clone(), lbl_fn.clone(), rule_snake.clone()).clone());
                                    }
                                    ps_pos = pn_pos;
                                } else if list_has(reserved_keywords.clone(), kind.clone()) {
                                    // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                                    errors.push(val_err(pn_tok.clone(), lbl_fn.clone(), rule_kw_in_parens.clone()).clone());
                                    ps_pos = pn_pos;
                                }
                            }
                        }
                        ps_pos = ps_pos + 1;
                    }
                }
            }
            let mut ret_pos: i64 = pos + 1.clone();
            let mut lp2_pos: i64 = pos + 2.clone();
            if lp2_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                let mut ret_tok: Token = tokens[ret_pos as usize].clone();
                let mut lp2_tok: Token = tokens[lp2_pos as usize].clone();
                let mut kind = ret_tok.kind.clone();
                let mut ret_kind: String = kind.clone();
                let mut kind = lp2_tok.kind.clone();
                if ret_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_fn_declaration.deor
                        errors.push(val_err(ret_tok.clone(), lbl_fn.clone(), rule_no_ret.clone()).clone());
                    }
                }
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        let mut keyword: String = "KW_FN".to_string();
        let mut lbl: String = lbl_fn.clone();
        let mut rule: String = rule_snake.clone();
        let mut test_rule: fn(String) -> bool = is_snake.clone();
        let mut validate_indent_offset: i64 = 2;
        // macro: validate_ident (transpiler-deor/tokens_validator/macros/validate_ident.deor)
        if cur_kind == keyword {
            // transpiler-deor/tokens_validator/macros/validate_ident.deor
            let mut name_pos: i64 = pos + validate_indent_offset.clone();
            if name_pos < token_count {
                // transpiler-deor/tokens_validator/macros/validate_ident.deor
                let mut name_tok: Token = tokens[name_pos as usize].clone();
                let mut kind = name_tok.kind.clone();
                let mut value = name_tok.value.clone();
                let mut name_kind: String = kind.clone();
                let mut name_val: String = value.clone();
                if name_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    if (name_val.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                    }
                    if !test_rule(name_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/validate_ident.deor
                        errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                    }
                } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                    // transpiler-deor/tokens_validator/macros/validate_ident.deor
                    errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                }
            }
            pos = pos + 1;
            continue;
        }
        // macro: check_invalid_char (transpiler-deor/tokens_validator/macros/check_invalid_char.deor)
        if cur_kind == "INVALID" {
            // transpiler-deor/tokens_validator/macros/check_invalid_char.deor
            errors.push(val_err(tok.clone(), lbl_rust.clone(), rule_invalid_char.clone()).clone());
        }
        // macro: check_raw_assignment (transpiler-deor/tokens_validator/macros/check_raw_assignment.deor)
        if cur_kind == "KW_RAW" {
            // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
            let mut ra_eq_pos: i64 = pos + 2.clone();
            if ra_eq_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                let mut ra_eq_tok: Token = tokens[ra_eq_pos as usize].clone();
                let mut kind = ra_eq_tok.kind.clone();
                if kind == "EQUALS" {
                    // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                    let mut ra_name_pos: i64 = pos + 1.clone();
                    let mut ra_name_tok: Token = tokens[ra_name_pos as usize].clone();
                    let mut kind = ra_name_tok.kind.clone();
                    let mut value = ra_name_tok.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                        raw_var_names.push(value.clone());
                        let mut ra_call_pos: i64 = ra_eq_pos + 1.clone();
                        let mut ra_is_call: bool = false;
                        if ra_call_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                            let mut ra_call_tok: Token = tokens[ra_call_pos as usize].clone();
                            let mut kind = ra_call_tok.kind.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                                let mut ra_lparen_pos: i64 = ra_call_pos + 1.clone();
                                if ra_lparen_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                                    let mut ra_lparen_tok: Token = tokens[ra_lparen_pos as usize].clone();
                                    let mut kind = ra_lparen_tok.kind.clone();
                                    ra_is_call = kind == "LPAREN";
                                }
                            }
                        }
                        if !ra_is_call {
                            // transpiler-deor/tokens_validator/macros/check_raw_assignment.deor
                            errors.push(val_err(ra_name_tok.clone(), lbl_var.clone(), rule_raw_assignment.clone()).clone());
                        }
                    }
                }
            }
        }
        // macro: track_validator_vars (transpiler-deor/tokens_validator/macros/track_validator_vars.deor)
        if cur_kind == "IDENT" {
            // transpiler-deor/tokens_validator/macros/track_validator_vars.deor
            let mut tvv_is_vtype: bool = list_has(validator_type_names.clone(), cur_val.clone());
            if tvv_is_vtype {
                // transpiler-deor/tokens_validator/macros/track_validator_vars.deor
                let mut tvv_next: i64 = pos + 1.clone();
                if tvv_next < token_count {
                    // transpiler-deor/tokens_validator/macros/track_validator_vars.deor
                    let mut tvv_tok: Token = tokens[tvv_next as usize].clone();
                    let mut kind = tvv_tok.kind.clone();
                    let mut value = tvv_tok.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/track_validator_vars.deor
                        validator_vars.push(value.clone());
                    }
                }
            }
        }
        // macro: check_crash_args (transpiler-deor/tokens_validator/macros/check_crash_args.deor)
        if cur_kind == "IDENT" {
            // transpiler-deor/tokens_validator/macros/check_crash_args.deor
            let mut is_crash: bool = cur_val == "crash".clone();
            if is_crash {
                // transpiler-deor/tokens_validator/macros/check_crash_args.deor
                let mut crash_lp: i64 = pos + 1.clone();
                if crash_lp < token_count {
                    // transpiler-deor/tokens_validator/macros/check_crash_args.deor
                    let mut crash_lp_tok: Token = tokens[crash_lp as usize].clone();
                    let mut kind = crash_lp_tok.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_crash_args.deor
                        let mut crash_arg_count: i64 = count_call_args(tokens.clone(), crash_lp.clone());
                        let mut wrong_count: bool = crash_arg_count != 1.clone();
                        if wrong_count {
                            // transpiler-deor/tokens_validator/macros/check_crash_args.deor
                            errors.push(val_err(tok.clone(), lbl_call.clone(), rule_crash.clone()).clone());
                        }
                    }
                }
            }
        }
        // macro: check_print_args (transpiler-deor/tokens_validator/macros/check_print_args.deor)
        if cur_kind == "IDENT" {
            // transpiler-deor/tokens_validator/macros/check_print_args.deor
            let mut is_print: bool = cur_val == "print".clone();
            if is_print {
                // transpiler-deor/tokens_validator/macros/check_print_args.deor
                let mut print_lp: i64 = pos + 1.clone();
                if print_lp < token_count {
                    // transpiler-deor/tokens_validator/macros/check_print_args.deor
                    let mut print_lp_tok: Token = tokens[print_lp as usize].clone();
                    let mut kind = print_lp_tok.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_print_args.deor
                        let mut print_arg_count: i64 = count_call_args(tokens.clone(), print_lp.clone());
                        let mut is_one: bool = print_arg_count == 1.clone();
                        let mut is_two: bool = print_arg_count == 2.clone();
                        if !is_one {
                            // transpiler-deor/tokens_validator/macros/check_print_args.deor
                            if !is_two {
                                // transpiler-deor/tokens_validator/macros/check_print_args.deor
                                errors.push(val_err(tok.clone(), lbl_call.clone(), rule_print_args.clone()).clone());
                            }
                        }
                    }
                }
            }
        }
        // macro: check_avow_target (transpiler-deor/tokens_validator/macros/check_avow_target.deor)
        if cur_kind == "KW_AVOW" {
            // transpiler-deor/tokens_validator/macros/check_avow_target.deor
            let mut avow_next: i64 = pos + 1.clone();
            if avow_next < token_count {
                // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                let mut avow_target: Token = tokens[avow_next as usize].clone();
                let mut kind = avow_target.kind.clone();
                let mut value = avow_target.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                    let mut avow_is_valid: bool = list_has(validator_vars.clone(), value.clone());
                    if !avow_is_valid {
                        // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                        errors.push(val_err(avow_target.clone(), lbl_var.clone(), rule_avow.clone()).clone());
                    }
                }
            }
        }
        // macro: check_validator_empty (transpiler-deor/tokens_validator/macros/check_validator_empty.deor)
        if cur_kind == "KW_EMPTY" {
            // transpiler-deor/tokens_validator/macros/check_validator_empty.deor
            if pos > 2 {
                // transpiler-deor/tokens_validator/macros/check_validator_empty.deor
                let mut ve_type_pos: i64 = pos - 3.clone();
                let mut ve_type_tok: Token = tokens[ve_type_pos as usize].clone();
                let mut kind = ve_type_tok.kind.clone();
                let mut value = ve_type_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/check_validator_empty.deor
                    let mut ve_is_validator: bool = list_has(validator_type_names.clone(), value.clone());
                    if ve_is_validator {
                        // transpiler-deor/tokens_validator/macros/check_validator_empty.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_validator_empty.clone()).clone());
                    }
                }
            }
        }
        // macro: check_empty_bracket (transpiler-deor/tokens_validator/macros/check_empty_bracket.deor)
        if cur_kind == "LBRACKET" {
            // transpiler-deor/tokens_validator/macros/check_empty_bracket.deor
            let mut next_pos: i64 = pos + 1.clone();
            if next_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_empty_bracket.deor
                let mut next_tok: Token = tokens[next_pos as usize].clone();
                let mut kind = next_tok.kind.clone();
                if kind == "RBRACKET" {
                    // transpiler-deor/tokens_validator/macros/check_empty_bracket.deor
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_empty_bracket.clone()).clone());
                }
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
        if cur_kind == "KW_FOR" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut fv_pos: i64 = pos + 1.clone();
            if fv_pos < token_count {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut fv_tok: Token = tokens[fv_pos as usize].clone();
                let mut kind = fv_tok.kind.clone();
                let mut value = fv_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/tokens_validation.deor
                    if (value.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        errors.push(val_err(fv_tok.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                    }
                    if !is_snake(value.clone()) {
                        // transpiler-deor/tokens_validator/tokens_validation.deor
                        errors.push(val_err(fv_tok.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                    }
                }
            }
        }
        if cur_kind == "IDENT" {
            // transpiler-deor/tokens_validator/tokens_validation.deor
            let mut is_fn_decl_name: bool = false;
            if pos > 1 {
                // transpiler-deor/tokens_validator/tokens_validation.deor
                let mut prev2_p: i64 = pos - 2.clone();
                let mut prev2_tok: Token = tokens[prev2_p as usize].clone();
                let mut kind = prev2_tok.kind.clone();
                is_fn_decl_name = kind == "KW_FN";
            }
            if !is_fn_decl_name {
                // macro: check_call_args (transpiler-deor/tokens_validator/macros/check_call_args.deor)
                let mut call_lp: i64 = pos + 1.clone();
                if call_lp < token_count {
                    // transpiler-deor/tokens_validator/macros/check_call_args.deor
                    let mut call_lp_tok: Token = tokens[call_lp as usize].clone();
                    let mut kind = call_lp_tok.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/check_call_args.deor
                        let mut arg_count: i64 = count_call_args(tokens.clone(), call_lp.clone());
                        if arg_count >= 2 {
                            // transpiler-deor/tokens_validator/macros/check_call_args.deor
                            let mut scan_pos: i64 = call_lp + 1.clone();
                            let mut scan_depth: i64 = 0;
                            let mut at_arg_start: bool = true;
                            while scan_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/check_call_args.deor
                                let mut scan_tok: Token = tokens[scan_pos as usize].clone();
                                let mut kind = scan_tok.kind.clone();
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
            // macro: check_raw_in_binding (transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor)
            if cur_kind == "IDENT" {
                // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                let mut rib_name_pos: i64 = pos + 1.clone();
                let mut rib_eq_pos: i64 = pos + 2.clone();
                let mut rib_val_pos: i64 = pos + 3.clone();
                if rib_val_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                    let mut rib_name_tok: Token = tokens[rib_name_pos as usize].clone();
                    let mut rib_eq_tok: Token = tokens[rib_eq_pos as usize].clone();
                    let mut rib_val_tok: Token = tokens[rib_val_pos as usize].clone();
                    let mut kind = rib_name_tok.kind.clone();
                    let mut rib_name_kind: String = kind.clone();
                    let mut kind = rib_eq_tok.kind.clone();
                    let mut rib_eq_kind: String = kind.clone();
                    let mut kind = rib_val_tok.kind.clone();
                    let mut value = rib_val_tok.value.clone();
                    let mut rib_is_binding: bool = rib_name_kind == "IDENT".clone();
                    rib_is_binding = rib_is_binding && rib_eq_kind == "EQUALS";
                    let mut rib_val_is_ident: bool = kind == "IDENT".clone();
                    if rib_is_binding && rib_val_is_ident {
                        // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                        let mut rib_is_raw: bool = list_has(raw_var_names.clone(), value.clone());
                        if rib_is_raw {
                            // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                            errors.push(val_err(rib_val_tok.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                        }
                    }
                }
                let mut rac_as_pos: i64 = pos + 1.clone();
                let mut rac_val_pos: i64 = pos + 2.clone();
                if rac_val_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                    let mut rac_as_tok: Token = tokens[rac_as_pos as usize].clone();
                    let mut rac_val_tok: Token = tokens[rac_val_pos as usize].clone();
                    let mut kind = rac_as_tok.kind.clone();
                    let mut rac_is_as: bool = kind == "KW_AS".clone();
                    let mut kind = rac_val_tok.kind.clone();
                    let mut value = rac_val_tok.value.clone();
                    let mut rac_val_is_ident: bool = kind == "IDENT".clone();
                    if rac_is_as && rac_val_is_ident {
                        // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                        let mut rac_is_raw: bool = list_has(raw_var_names.clone(), value.clone());
                        if rac_is_raw {
                            // transpiler-deor/tokens_validator/macros/check_raw_in_binding.deor
                            errors.push(val_err(rac_val_tok.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_raw_reassign (transpiler-deor/tokens_validator/macros/check_raw_reassign.deor)
            let mut rr_is_tracked: bool = list_has(raw_var_names.clone(), cur_val.clone());
            if rr_is_tracked {
                // transpiler-deor/tokens_validator/macros/check_raw_reassign.deor
                let mut rr_next_pos: i64 = pos + 1.clone();
                if rr_next_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_raw_reassign.deor
                    let mut rr_next_tok: Token = tokens[rr_next_pos as usize].clone();
                    let mut kind = rr_next_tok.kind.clone();
                    let mut rr_is_eq: bool = kind == "EQUALS".clone();
                    let mut rr_is_as: bool = kind == "KW_AS".clone();
                    if rr_is_eq || rr_is_as {
                        // transpiler-deor/tokens_validator/macros/check_raw_reassign.deor
                        let mut rr_is_decl: bool = false;
                        if pos > 0 {
                            // transpiler-deor/tokens_validator/macros/check_raw_reassign.deor
                            let mut rr_prev_pos: i64 = pos - 1.clone();
                            let mut rr_prev_tok: Token = tokens[rr_prev_pos as usize].clone();
                            let mut kind = rr_prev_tok.kind.clone();
                            rr_is_decl = kind == "KW_RAW";
                        }
                        if !rr_is_decl {
                            // transpiler-deor/tokens_validator/macros/check_raw_reassign.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_raw_reassign.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_raw_operator_use (transpiler-deor/tokens_validator/macros/check_raw_operator_use.deor)
            if cur_kind == "IDENT" {
                // transpiler-deor/tokens_validator/macros/check_raw_operator_use.deor
                let mut rop_is_raw: bool = list_has(raw_var_names.clone(), cur_val.clone());
                if rop_is_raw {
                    // transpiler-deor/tokens_validator/macros/check_raw_operator_use.deor
                    let mut rop_op_kinds: Vec<String> = vec!["PLUS".to_string(), "MINUS".to_string(), "STAR".to_string(), "SLASH".to_string(), "PERCENT".to_string(), "GT".to_string(), "LT".to_string(), "GTE".to_string(), "LTE".to_string(), "KW_IS".to_string(), "KW_AND".to_string(), "KW_OR".to_string()];
                    let mut rop_next_is_op: bool = false;
                    let mut rop_next_pos: i64 = pos + 1.clone();
                    if rop_next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/check_raw_operator_use.deor
                        let mut rop_next_tok: Token = tokens[rop_next_pos as usize].clone();
                        let mut kind = rop_next_tok.kind.clone();
                        rop_next_is_op = list_has(rop_op_kinds.clone(), kind.clone());
                    }
                    let mut rop_prev_is_op: bool = false;
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/check_raw_operator_use.deor
                        let mut rop_prev_pos: i64 = pos - 1.clone();
                        let mut rop_prev_tok: Token = tokens[rop_prev_pos as usize].clone();
                        let mut kind = rop_prev_tok.kind.clone();
                        rop_prev_is_op = list_has(rop_op_kinds.clone(), kind.clone());
                    }
                    if rop_next_is_op || rop_prev_is_op {
                        // transpiler-deor/tokens_validator/macros/check_raw_operator_use.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                    }
                }
            }
            // macro: check_raw_in_special_builtin (transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor)
            if cur_kind == "IDENT" {
                // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                let mut rsb_is_len: bool = cur_val == "len".clone();
                let mut rsb_is_crash: bool = cur_val == "crash".clone();
                let mut rsb_is_sjoin: bool = cur_val == "s_join".clone();
                let mut rsb_is_target: bool = rsb_is_len || rsb_is_crash || rsb_is_sjoin.clone();
                if rsb_is_target {
                    // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                    let mut rsb_lp: i64 = pos + 1.clone();
                    if rsb_lp < token_count {
                        // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                        let mut rsb_lp_tok: Token = tokens[rsb_lp as usize].clone();
                        let mut kind = rsb_lp_tok.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                            let mut rsb_scan: i64 = rsb_lp + 1.clone();
                            let mut rsb_depth: i64 = 0;
                            while rsb_scan < token_count {
                                // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                let mut rsb_tok: Token = tokens[rsb_scan as usize].clone();
                                let mut kind = rsb_tok.kind.clone();
                                let mut value = rsb_tok.value.clone();
                                if kind == "RPAREN" {
                                    // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                    let mut rsb_root: bool = rsb_depth == 0.clone();
                                    if rsb_root {
                                        // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                        break;
                                    }
                                    rsb_depth = rsb_depth - 1;
                                } else if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                    rsb_depth = rsb_depth + 1;
                                } else if kind == "LBRACKET" {
                                    // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                    rsb_depth = rsb_depth + 1;
                                } else if kind == "RBRACKET" {
                                    // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                    rsb_depth = rsb_depth - 1;
                                } else if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                    let mut rsb_is_raw: bool = list_has(raw_var_names.clone(), value.clone());
                                    if rsb_is_raw {
                                        // transpiler-deor/tokens_validator/macros/check_raw_in_special_builtin.deor
                                        errors.push(val_err(rsb_tok.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                                    }
                                }
                                rsb_scan = rsb_scan + 1;
                            }
                        }
                    }
                }
            }
            // macro: check_const_reassign (transpiler-deor/tokens_validator/macros/check_const_reassign.deor)
            let mut rar_names: Vec<String> = const_var_names.clone();
            let mut rar_allow_as: bool = false;
            let mut rar_rule: String = rule_const_reassign.clone();
            // macro: check_bare_reassign (transpiler-deor/tokens_validator/macros/check_bare_reassign.deor)
            let mut rar_is_tracked: bool = list_has(rar_names.clone(), cur_val.clone());
            if rar_is_tracked {
                // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                let mut rar_next_pos: i64 = pos + 1.clone();
                if rar_next_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                    let mut rar_next_tok: Token = tokens[rar_next_pos as usize].clone();
                    let mut kind = rar_next_tok.kind.clone();
                    let mut rar_is_eq: bool = kind == "EQUALS".clone();
                    let mut rar_is_as: bool = false;
                    if rar_allow_as {
                        // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                        rar_is_as = kind == "KW_AS";
                    }
                    if rar_is_eq || rar_is_as {
                        // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                        let mut rar_is_decl: bool = false;
                        if pos > 0 {
                            // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                            let mut rar_prev_pos: i64 = pos - 1.clone();
                            let mut rar_prev_tok: Token = tokens[rar_prev_pos as usize].clone();
                            let mut kind = rar_prev_tok.kind.clone();
                            rar_is_decl = kind == "IDENT";
                        }
                        if !rar_is_decl {
                            // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rar_rule.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_validator_reassign (transpiler-deor/tokens_validator/macros/check_validator_reassign.deor)
            let mut rar_names: Vec<String> = validator_vars.clone();
            let mut rar_allow_as: bool = true;
            let mut rar_rule: String = rule_validator_reassign.clone();
            // macro: check_bare_reassign (transpiler-deor/tokens_validator/macros/check_bare_reassign.deor)
            let mut rar_is_tracked: bool = list_has(rar_names.clone(), cur_val.clone());
            if rar_is_tracked {
                // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                let mut rar_next_pos: i64 = pos + 1.clone();
                if rar_next_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                    let mut rar_next_tok: Token = tokens[rar_next_pos as usize].clone();
                    let mut kind = rar_next_tok.kind.clone();
                    let mut rar_is_eq: bool = kind == "EQUALS".clone();
                    let mut rar_is_as: bool = false;
                    if rar_allow_as {
                        // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                        rar_is_as = kind == "KW_AS";
                    }
                    if rar_is_eq || rar_is_as {
                        // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                        let mut rar_is_decl: bool = false;
                        if pos > 0 {
                            // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                            let mut rar_prev_pos: i64 = pos - 1.clone();
                            let mut rar_prev_tok: Token = tokens[rar_prev_pos as usize].clone();
                            let mut kind = rar_prev_tok.kind.clone();
                            rar_is_decl = kind == "IDENT";
                        }
                        if !rar_is_decl {
                            // transpiler-deor/tokens_validator/macros/check_bare_reassign.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rar_rule.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_var_decl (transpiler-deor/tokens_validator/macros/check_var_decl.deor)
            let mut next1: i64 = pos + 1.clone();
            let mut next2: i64 = pos + 2.clone();
            if next2 < token_count {
                // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                let mut tok_one: Token = tokens[next1 as usize].clone();
                let mut tok_two: Token = tokens[next2 as usize].clone();
                let mut kind = tok_one.kind.clone();
                let mut one_kind: String = kind.clone();
                let mut kind = tok_two.kind.clone();
                let mut two_kind: String = kind.clone();
                if one_kind == "IDENT" && two_kind == "EQUALS" {
                    // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                    let mut value = tok_one.value.clone();
                    let mut line = tok_one.line.clone();
                    let mut file = tok_one.file.clone();
                    let mut var_name: String = value.clone();
                    let mut var_line: i64 = line.clone();
                    let mut var_file: String = file.clone();
                    if (var_name.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                    }
                    let mut cvd_is_const: bool = false;
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        let mut cvd_prev: i64 = pos - 1.clone();
                        let mut cvd_prev_tok: Token = tokens[cvd_prev as usize].clone();
                        let mut kind = cvd_prev_tok.kind.clone();
                        cvd_is_const = kind == "KW_CONST";
                    }
                    if cvd_is_const {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        if !is_screaming_snake(var_name.clone()) {
                            // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                            errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_screaming.clone()).clone());
                        }
                    }
                    if !cvd_is_const {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        if !is_snake(var_name.clone()) {
                            // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                            errors.push(val_err(tok_one.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                        }
                    }
                }
                if one_kind == "KW_AS" {
                    // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                    if (cur_val.len() as i64) < 3 {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                    }
                    if !is_snake(cur_val.clone()) {
                        // transpiler-deor/tokens_validator/macros/check_var_decl.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                    }
                }
            }
            // macro: check_bad_stmt (transpiler-deor/tokens_validator/macros/check_bad_stmt.deor)
            if paren_depth == 0 {
                // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                let mut next1: i64 = pos + 1.clone();
                let mut next2: i64 = pos + 2.clone();
                if next2 < token_count {
                    // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                    let mut tok_one: Token = tokens[next1 as usize].clone();
                    let mut tok_two: Token = tokens[next2 as usize].clone();
                    let mut kind = tok_one.kind.clone();
                    let mut one_kind: String = kind.clone();
                    let mut kind = tok_two.kind.clone();
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
                        if two_kind == "KW_AS" {
                            // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_typed_as.clone()).clone());
                        }
                    }
                    if one_kind == "KW_AS" {
                        // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                        if two_kind == "KW_MOVE" {
                            // transpiler-deor/tokens_validator/macros/check_bad_stmt.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_as_move.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_bracket_indexing (transpiler-deor/tokens_validator/macros/check_bracket_indexing.deor)
            let mut next_pos: i64 = pos + 1.clone();
            if next_pos < token_count {
                // transpiler-deor/tokens_validator/macros/check_bracket_indexing.deor
                let mut next_tok: Token = tokens[next_pos as usize].clone();
                let mut kind = next_tok.kind.clone();
                if kind == "LBRACKET" {
                    // transpiler-deor/tokens_validator/macros/check_bracket_indexing.deor
                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_bracket_index.clone()).clone());
                }
            }
            // macro: check_struct_construction (transpiler-deor/tokens_validator/macros/check_struct_construction.deor)
            if paren_depth == 0 {
                // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                let mut cc1: i64 = pos + 1.clone();
                let mut cc2: i64 = pos + 2.clone();
                let mut cc3: i64 = pos + 3.clone();
                let mut cc4: i64 = pos + 4.clone();
                if cc4 < token_count {
                    // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                    let mut cc_t1: Token = tokens[cc1 as usize].clone();
                    let mut cc_t2: Token = tokens[cc2 as usize].clone();
                    let mut cc_t3: Token = tokens[cc3 as usize].clone();
                    let mut cc_t4: Token = tokens[cc4 as usize].clone();
                    let mut kind = cc_t1.kind.clone();
                    let mut cc_k1: String = kind.clone();
                    let mut kind = cc_t2.kind.clone();
                    let mut cc_k2: String = kind.clone();
                    let mut kind = cc_t3.kind.clone();
                    let mut cc_k3: String = kind.clone();
                    let mut kind = cc_t4.kind.clone();
                    let mut cc_k4: String = kind.clone();
                    let mut cc_is_var: bool = cc_k1 == "IDENT".clone();
                    let mut cc_is_eq: bool = cc_k2 == "EQUALS".clone();
                    let mut cc_is_mv: bool = cc_k3 == "KW_MOVE".clone();
                    let mut cc_is_lp: bool = cc_k4 == "LPAREN".clone();
                    let mut cc_match: bool = cc_is_var && cc_is_eq.clone();
                    cc_match = cc_match && cc_is_mv;
                    cc_match = cc_match && cc_is_lp;
                    if cc_match {
                        // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                        let mut cc_sep: String = ",".to_string();
                        let mut cc_fields_str: String = find_struct_field_str(struct_field_reg.clone(), cur_val.clone());
                        let mut cc_is_struct: bool = cc_fields_str != "".clone();
                        if cc_is_struct {
                            // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                            let mut cc_expected: Vec<String> = s_split(cc_fields_str.clone(), cc_sep.clone());
                            let mut cc_exp_count: i64 = (cc_expected.len() as i64);
                            let mut cc_provided: Vec<String> = Vec::new();
                            let mut cc_scan: i64 = cc4 + 1.clone();
                            let mut cc_scanning: bool = true;
                            while cc_scanning {
                                // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                if cc_scan >= token_count {
                                    // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                    cc_scanning = false;
                                } else {
                                    // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                    let mut cc_stok: Token = tokens[cc_scan as usize].clone();
                                    let mut kind = cc_stok.kind.clone();
                                    let mut value = cc_stok.value.clone();
                                    if kind == "RPAREN" {
                                        // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                        cc_scanning = false;
                                    } else if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                        cc_provided.push(value.clone());
                                    }
                                    cc_scan = cc_scan + 1;
                                }
                            }
                            let mut cc_prov_count: i64 = (cc_provided.len() as i64);
                            let mut cc_wrong_count: bool = cc_prov_count != cc_exp_count.clone();
                            if cc_wrong_count {
                                // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                errors.push(val_err(tok.clone(), lbl_struct.clone(), rule_struct_field_count.clone()).clone());
                            } else {
                                // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                let mut cc_fi: i64 = 0;
                                while cc_fi < cc_prov_count {
                                    // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                    let mut cc_field: String = cc_provided[cc_fi as usize].clone();
                                    if !list_has(cc_expected.clone(), cc_field.clone()) {
                                        // transpiler-deor/tokens_validator/macros/check_struct_construction.deor
                                        errors.push(val_err(tok.clone(), lbl_struct.clone(), rule_struct_field_name.clone()).clone());
                                    }
                                    cc_fi = cc_fi + 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        // macro: check_void_var (transpiler-deor/tokens_validator/macros/check_void_var.deor)
        if cur_kind == "KW_VOID" {
            // transpiler-deor/tokens_validator/macros/check_void_var.deor
            let mut preceded_by_fn: bool = false;
            if pos > 0 {
                // transpiler-deor/tokens_validator/macros/check_void_var.deor
                let mut prev_void: i64 = pos - 1.clone();
                let mut prev_void_tok: Token = tokens[prev_void as usize].clone();
                let mut kind = prev_void_tok.kind.clone();
                preceded_by_fn = kind == "KW_FN";
            }
            if !preceded_by_fn {
                // transpiler-deor/tokens_validator/macros/check_void_var.deor
                let mut void_name_pos: i64 = pos + 1.clone();
                let mut void_eq_pos: i64 = pos + 2.clone();
                if void_eq_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/check_void_var.deor
                    let mut void_name_tok: Token = tokens[void_name_pos as usize].clone();
                    let mut void_eq_tok: Token = tokens[void_eq_pos as usize].clone();
                    let mut kind = void_name_tok.kind.clone();
                    let mut void_name_kind: String = kind.clone();
                    let mut kind = void_eq_tok.kind.clone();
                    if void_name_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/check_void_var.deor
                        if kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/check_void_var.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_void_var.clone()).clone());
                        }
                    }
                }
            }
        }
        // macro: check_valid_placement (transpiler-deor/tokens_validator/macros/check_valid_placement.deor)
        if cur_kind == "KW_VALID" {
            // transpiler-deor/tokens_validator/macros/check_valid_placement.deor
            let mut valid_ok: bool = false;
            if pos > 0 {
                // transpiler-deor/tokens_validator/macros/check_valid_placement.deor
                let mut valid_pos: i64 = pos - 1.clone();
                let mut prev_valid_tok: Token = tokens[valid_pos as usize].clone();
                let mut kind = prev_valid_tok.kind.clone();
                valid_ok = kind == "KW_IS";
                if !valid_ok {
                    // transpiler-deor/tokens_validator/macros/check_valid_placement.deor
                    valid_ok = kind == "KW_NOT";
                }
            }
            if !valid_ok {
                // transpiler-deor/tokens_validator/macros/check_valid_placement.deor
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_valid.clone()).clone());
            }
        }
        // macro: check_end_placement (transpiler-deor/tokens_validator/macros/check_end_placement.deor)
        if cur_kind == "KW_END" {
            // transpiler-deor/tokens_validator/macros/check_end_placement.deor
            let mut end_ok: bool = false;
            if pos > 0 {
                // transpiler-deor/tokens_validator/macros/check_end_placement.deor
                let mut end_prev_pos: i64 = pos - 1.clone();
                let mut prev_end_tok: Token = tokens[end_prev_pos as usize].clone();
                let mut kind = prev_end_tok.kind.clone();
                end_ok = kind == "KW_AT";
            }
            if !end_ok {
                // transpiler-deor/tokens_validator/macros/check_end_placement.deor
                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_end.clone()).clone());
            }
        }
        // transpiler-deor/tokens_validator/tokens_validation.deor
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
        return "i64".to_string();
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
fn skip_to_block_start(tokens: TokensRef, start: i64) -> ParseResult {
    // transpiler-deor/registry/struct.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut cur: i64 = start.clone();
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

fn collect_struct_fields(tokens: TokensRef, start: i64) -> ParseResult {
    // transpiler-deor/registry/struct.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i64 = start.clone();
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
            let mut fname_pos: i64 = field_index + 1.clone();
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
    let mut token_count: i64 = (tokens.len() as i64);
    let mut raw_i: i64 = 0;
    while raw_i < token_count {
        // transpiler-deor/registry/struct.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_STRUCT" {
            // transpiler-deor/registry/struct.deor
            let mut name_pos: i64 = raw_i + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/struct.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let kind = name_token.kind.clone();
                let value = name_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/registry/struct.deor
                    let mut blk_start: i64 = name_pos + 1.clone();
                    let mut block_r: ParseResult = skip_to_block_start(tokens.clone(), blk_start.clone());
                    let mut fld_start: i64 = pr_pos(block_r.clone());
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
    let mut token_count: i64 = (tokens.len() as i64);
    for index in 0..token_count {
        // transpiler-deor/registry/shape.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_SHAPE" {
            // transpiler-deor/registry/shape.deor
            let mut name_pos: i64 = index + 1.clone();
            let mut form_pos: i64 = index + 3.clone();
            let mut t4_pos: i64 = index + 4.clone();
            if t4_pos < token_count {
                // transpiler-deor/registry/shape.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let mut form_token: Token = tokens[form_pos as usize].clone();
                let value = name_token.value.clone();
                let mut shape_name: String = value.clone();
                let kind = form_token.kind.clone();
                if kind == "KW_LIST" {
                    // transpiler-deor/registry/shape.deor
                    let mut elem_pos: i64 = index + 5.clone();
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
                    let mut t4_is_to: bool = kind == "KW_TO".clone();
                    let mut in_type: String = "".to_string();
                    let mut out_type: String = "".to_string();
                    if t4_is_of {
                        // transpiler-deor/registry/shape.deor
                        let mut t5_pos: i64 = index + 5.clone();
                        if t5_pos < token_count {
                            // transpiler-deor/registry/shape.deor
                            let mut t5_token: Token = tokens[t5_pos as usize].clone();
                            let value = t5_token.value.clone();
                            in_type = value;
                        }
                        let mut t6_pos: i64 = index + 6.clone();
                        if t6_pos < token_count {
                            // transpiler-deor/registry/shape.deor
                            let mut t6_token: Token = tokens[t6_pos as usize].clone();
                            let kind = t6_token.kind.clone();
                            let value = t6_token.value.clone();
                            let mut t6_is_to: bool = kind == "KW_TO".clone();
                            if t6_is_to {
                                // transpiler-deor/registry/shape.deor
                                let mut t7_pos: i64 = index + 7.clone();
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
                        let mut t5_pos: i64 = index + 5.clone();
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
            let mut name_pos: i64 = index + 1.clone();
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
fn is_typed_enum_type(word: String) -> bool {
    // transpiler-deor/registry/enum.deor
    if word == "string" {
        // transpiler-deor/registry/enum.deor
        return true;
    }
    if word == "int" {
        // transpiler-deor/registry/enum.deor
        return true;
    }
    if word == "float" {
        // transpiler-deor/registry/enum.deor
        return true;
    }
    if word == "bool" {
        // transpiler-deor/registry/enum.deor
        return true;
    }
    return false;
}

fn build_enum_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    for index in 0..token_count {
        // transpiler-deor/registry/enum.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/enum.deor
            let mut name_pos: i64 = index + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/enum.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let value = name_token.value.clone();
                let mut be_is_typed: bool = is_typed_enum_type(value.clone());
                if !be_is_typed {
                    // transpiler-deor/registry/enum.deor
                    let mut enum_name: String = value.clone();
                    let mut rust_name: String = s_pascal(enum_name.clone());
                    result.push(enum_name.clone());
                    result.push(rust_name.clone());
                }
            }
        }
    }
    return result;
}

fn build_variant_reg(tokens: TokensRef, enum_reg: Vec<String>) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut raw_i: i64 = 0;
    while raw_i < token_count {
        // transpiler-deor/registry/enum.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/enum.deor
            let mut name_pos: i64 = raw_i + 1.clone();
            if name_pos < token_count {
                // transpiler-deor/registry/enum.deor
                let mut name_token: Token = tokens[name_pos as usize].clone();
                let value = name_token.value.clone();
                let mut bvr_is_typed: bool = is_typed_enum_type(value.clone());
                if !bvr_is_typed {
                    // transpiler-deor/registry/enum.deor
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
                } else {
                    // transpiler-deor/registry/enum.deor
                    raw_i = raw_i + 1;
                }
            }
            continue;
        }
        raw_i = raw_i + 1;
    }
    return result;
}

fn build_typed_enum_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    for index in 0..token_count {
        // transpiler-deor/registry/enum.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/enum.deor
            let mut type_pos: i64 = index + 1.clone();
            if type_pos < token_count {
                // transpiler-deor/registry/enum.deor
                let mut type_tok: Token = tokens[type_pos as usize].clone();
                let kind = type_tok.kind.clone();
                let value = type_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/registry/enum.deor
                    let mut bte_is_typed: bool = is_typed_enum_type(value.clone());
                    if bte_is_typed {
                        // transpiler-deor/registry/enum.deor
                        let mut val_type: String = value.clone();
                        let mut name_pos: i64 = type_pos + 1.clone();
                        if name_pos < token_count {
                            // transpiler-deor/registry/enum.deor
                            let mut name_tok: Token = tokens[name_pos as usize].clone();
                            let value = name_tok.value.clone();
                            result.push(value.clone());
                            result.push(val_type.clone());
                        }
                    }
                }
            }
        }
    }
    return result;
}

fn build_typed_variant_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut raw_i: i64 = 0;
    while raw_i < token_count {
        // transpiler-deor/registry/enum.deor
        let mut token: Token = tokens[raw_i as usize].clone();
        let kind = token.kind.clone();
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/enum.deor
            let mut btvr_type_pos: i64 = raw_i + 1.clone();
            let mut btvr_typed: bool = false;
            if btvr_type_pos < token_count {
                // transpiler-deor/registry/enum.deor
                let mut btvr_type_tok: Token = tokens[btvr_type_pos as usize].clone();
                let kind = btvr_type_tok.kind.clone();
                let value = btvr_type_tok.value.clone();
                btvr_typed = is_typed_enum_type(value.clone());
            }
            if btvr_typed {
                // transpiler-deor/registry/enum.deor
                let mut btvr_name_pos: i64 = raw_i + 2.clone();
                if btvr_name_pos < token_count {
                    // transpiler-deor/registry/enum.deor
                    let mut btvr_name_tok: Token = tokens[btvr_name_pos as usize].clone();
                    let value = btvr_name_tok.value.clone();
                    let mut btvr_enum: String = value.clone();
                    raw_i = btvr_name_pos + 1;
                    while raw_i < token_count {
                        // transpiler-deor/registry/enum.deor
                        let mut btvr_skip: Token = tokens[raw_i as usize].clone();
                        let kind = btvr_skip.kind.clone();
                        raw_i = raw_i + 1;
                        if kind == "INDENT" {
                            // transpiler-deor/registry/enum.deor
                            break;
                        }
                    }
                    while raw_i < token_count {
                        // transpiler-deor/registry/enum.deor
                        let mut btvr_var: Token = tokens[raw_i as usize].clone();
                        let kind = btvr_var.kind.clone();
                        let value = btvr_var.value.clone();
                        raw_i = raw_i + 1;
                        if kind == "DEDENT" {
                            // transpiler-deor/registry/enum.deor
                            break;
                        }
                        if kind == "IDENT" {
                            // transpiler-deor/registry/enum.deor
                            let mut btvr_vname: String = value.clone();
                            if raw_i < token_count {
                                // transpiler-deor/registry/enum.deor
                                let mut btvr_eq: Token = tokens[raw_i as usize].clone();
                                let kind = btvr_eq.kind.clone();
                                raw_i = raw_i + 1;
                                if kind == "EQUALS" {
                                    // transpiler-deor/registry/enum.deor
                                    if raw_i < token_count {
                                        // transpiler-deor/registry/enum.deor
                                        let mut btvr_lit: Token = tokens[raw_i as usize].clone();
                                        let value = btvr_lit.value.clone();
                                        raw_i = raw_i + 1;
                                        let mut dot: String = ".".to_string();
                                        let mut btvr_key: String = [btvr_enum.as_str(), dot.as_str(), btvr_vname.as_str()].concat();
                                        result.push(btvr_key.clone());
                                        result.push(value.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // transpiler-deor/registry/enum.deor
                raw_i = raw_i + 1;
            }
            continue;
        }
        raw_i = raw_i + 1;
    }
    return result;
}

// transpiler-deor/registry/validator_type.deor
fn build_type_reg(tokens: TokensRef) -> Vec<String> {
    // transpiler-deor/registry/validator_type.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    for index in 0..token_count {
        // transpiler-deor/registry/validator_type.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        let line = token.line.clone();
        if kind == "KW_TYPE" {
            // transpiler-deor/registry/validator_type.deor
            let mut name_pos: i64 = index + 1.clone();
            let mut param_type_pos: i64 = index + 3.clone();
            let mut param_name_pos: i64 = index + 4.clone();
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
fn find_block_end(tokens: Vec<Token>, indent_pos: i64) -> i64 {
    // transpiler-deor/registry/mut_scan.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut depth: i64 = 1;
    let mut result: i64 = indent_pos.clone();
    let mut start: i64 = indent_pos + 1.clone();
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

fn find_block_end_ref(tokens: TokensRef, indent_pos: i64) -> i64 {
    // transpiler-deor/registry/mut_scan.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut depth: i64 = 1;
    let mut result: i64 = indent_pos.clone();
    let mut start: i64 = indent_pos + 1.clone();
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

fn collect_mut_names(tokens: Vec<Token>, start: i64, end_pos: i64) -> Vec<String> {
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
            let mut const_name_pos: i64 = raw_i + 2.clone();
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
            let mut prev_pos: i64 = raw_i - 1.clone();
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
    let mut typed_enum_reg: Vec<String> = build_typed_enum_reg(tokens_ref.clone());
    let mut typed_variant_reg: Vec<String> = build_typed_variant_reg(tokens_ref.clone());
    let mut mut_names: Vec<String> = Vec::new();
    let mut placeholder: Vec<Token> = Vec::new();
    let mut tokens: TokensRef = tokens_wrap(placeholder);
    let mut ctx_raw: GenCtx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, tokens, typed_enum_reg, typed_variant_reg };
    let mut ctx: RcCtx = make_rctx(ctx_raw);
    return ctx;
}

// transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
fn find_struct_for_fields(struct_reg: Vec<String>, fields: Vec<String>) -> String {
    // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
    let mut comma: String = ",".to_string();
    let mut input_count: i64 = (fields.len() as i64);
    let mut reg_count: i64 = (struct_reg.len() as i64);
    let mut next_is_val: bool = false;
    let mut cur_name: String = "".to_string();
    for index in 0..reg_count {
        // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
        let mut item: String = struct_reg[index as usize].clone();
        if next_is_val {
            // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
            let mut reg_fields: Vec<String> = s_split(item.clone(), comma.clone());
            let mut reg_count_f: i64 = (reg_fields.len() as i64);
            if reg_count_f == input_count {
                // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                let mut all_match: bool = true;
                for fidx in 0..input_count {
                    // transpiler-deor/codegen/decl/stmt/expr/struct_lookup.deor
                    let mut field: String = fields[fidx as usize].clone();
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
    let mut reg_count: i64 = (struct_reg.len() as i64);
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
fn gen_call_args(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
    let mut arg_codes: Vec<String> = Vec::new();
    let mut cur: i64 = pos.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
    loop {
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
            arg_code = [arg_code.as_str(), RS_TOS.as_str()].concat();
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            let mut next_cur: i64 = cur + 1.clone();
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
                    arg_code = [arg_code.as_str(), RS_CLN.as_str()].concat();
                }
            }
        }
        arg_codes.push(arg_code.clone());
        cur = arg_pos;
    }
    let mut args_str: String = s_join_with(arg_codes.clone(), RS_CSEP.clone());
    return make_result(args_str, cur.clone());
}

// transpiler-deor/codegen/decl/stmt/expr/list_items.deor
fn gen_list_items(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
    let mut item_codes: Vec<String> = Vec::new();
    let mut cur: i64 = pos.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
    loop {
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
            item_code = [item_code.as_str(), RS_TOS.as_str()].concat();
        } else {
            // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
            item_code = [item_code.as_str(), RS_CLN.as_str()].concat();
        }
        item_codes.push(item_code.clone());
        cur = item_pos;
    }
    let mut items_str: String = s_join_with(item_codes.clone(), RS_CSEP.clone());
    return make_result(items_str, cur.clone());
}

fn gen_join_items(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
    let mut item_codes: Vec<String> = Vec::new();
    let mut cur: i64 = pos.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/expr/list_items.deor
    loop {
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
    let mut items_str: String = s_join_with(item_codes.clone(), RS_CSEP.clone());
    return make_result(items_str, cur.clone());
}

// transpiler-deor/codegen/decl/stmt/expr/primary.deor
fn gen_unary_method(args_pos: i64, suffix: String, ctx: RcCtx) -> ParseResult {
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

fn gen_primary(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/primary.deor
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut token_count: i64 = (tokens.len() as i64);
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
        let mut lit_next: i64 = pos + 1.clone();
        return make_result(lit_val, lit_next.clone());
    }
    if kind == "FLOAT" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_next: i64 = pos + 1.clone();
        return make_result(value, lit_next.clone());
    }
    if kind == "STRING" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_debug: String = s_debug(value.clone());
        let mut lit_next: i64 = pos + 1.clone();
        return make_result(lit_debug, lit_next.clone());
    }
    if kind == "KW_TRUE" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_true: String = "true".to_string();
        let mut lit_next: i64 = pos + 1.clone();
        return make_result(lit_true, lit_next.clone());
    }
    if kind == "KW_FALSE" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        let mut lit_false: String = "false".to_string();
        let mut lit_next: i64 = pos + 1.clone();
        return make_result(lit_false, lit_next.clone());
    }
    // macro: primary_list_literal (transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor)
    if kind == "LBRACKET" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor
        let mut ll_inner: i64 = pos + 1.clone();
        let mut ll_items_r: ParseResult = gen_list_items(tokens.clone(), ll_inner.clone(), ctx.clone());
        let code = ll_items_r.code;
        let new_pos = ll_items_r.new_pos;
        let ll_items_code = code;
        let ll_items_pos = new_pos;
        let mut ll_open: String = "vec![".to_string();
        let mut ll_close: String = "]".to_string();
        let mut ll_code: String = [ll_open.as_str(), ll_items_code.as_str(), ll_close.as_str()].concat();
        let mut ll_after: i64 = ll_items_pos + 1.clone();
        return make_result(ll_code, ll_after.clone());
    }
    // macro: primary_paren_expr (transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor)
    if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
        let mut pe_peek: i64 = pos + 1.clone();
        if pe_peek < token_count {
            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
            let mut pe_peek_tok: Token = tokens[pe_peek as usize].clone();
            let kind = pe_peek_tok.kind.clone();
            if kind == "KW_AVOW" {
                // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                let mut pe_expr_pos: i64 = pe_peek + 1.clone();
                let mut pe_expr_r: ParseResult = gen_expr(tokens.clone(), pe_expr_pos.clone(), ctx.clone());
                let code = pe_expr_r.code;
                let new_pos = pe_expr_r.new_pos;
                let pe_expr_code = code;
                let pe_after = new_pos + 1;
                let mut pe_unw: String = ".unwrap().0".to_string();
                let mut pe_unwrap_code: String = [pe_expr_code.as_str(), pe_unw.as_str()].concat();
                return make_result(pe_unwrap_code, pe_after.clone());
            }
            let mut pe_is_struct: bool = true;
            let mut pe_scan: i64 = pe_peek.clone();
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
                let mut pe_cur: i64 = pe_peek.clone();
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
    if kind == "KW_MOVE" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
        let mut po_inner: i64 = pos + 1.clone();
        let mut po_r: ParseResult = gen_primary(tokens.clone(), po_inner.clone(), ctx.clone());
        return po_r;
    }
    if kind == "KW_AVOW" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
        let mut po_avow_inner: i64 = pos + 1.clone();
        let mut po_avow_r: ParseResult = gen_primary(tokens.clone(), po_avow_inner.clone(), ctx.clone());
        let code = po_avow_r.code;
        let new_pos = po_avow_r.new_pos;
        let po_avow_code = code;
        let po_avow_end = new_pos;
        let mut po_avow_sfx: String = ".unwrap().0".to_string();
        let mut po_avow_unwrap_code: String = [po_avow_code.as_str(), po_avow_sfx.as_str()].concat();
        return make_result(po_avow_unwrap_code, po_avow_end.clone());
    }
    if kind == "KW_NOT" {
        // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
        let mut po_operand: i64 = pos + 1.clone();
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
        let mut ie_next: i64 = pos + 1.clone();
        if ie_next < token_count {
            // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
            let mut ie_next_tok: Token = tokens[ie_next as usize].clone();
            let kind = ie_next_tok.kind.clone();
            if kind == "LPAREN" {
                // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                let mut ie_func: String = value.clone();
                let mut ie_args_pos: i64 = ie_next + 1.clone();
                if ie_func == "len" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut ie_len_sfx: String = ".len() as i64".to_string();
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
                    let mut ie_after_crash: i64 = ie_crash_end + 1.clone();
                    return make_result(ie_panic_code, ie_after_crash.clone());
                }
                if ie_func == "s_join" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut sj_arg_tok: Token = tokens[ie_args_pos as usize].clone();
                    let kind = sj_arg_tok.kind.clone();
                    if kind == "LBRACKET" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                        let mut sj_inner: i64 = ie_args_pos + 1.clone();
                        let mut sj_r: ParseResult = gen_join_items(tokens.clone(), sj_inner.clone(), ctx.clone());
                        let code = sj_r.code;
                        let new_pos = sj_r.new_pos;
                        let sj_items = code;
                        let sj_end = new_pos;
                        let mut sj_after: i64 = sj_end + 2.clone();
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
                let mut ie_after: i64 = ie_args_end + 1.clone();
                let mut ie_lp: String = "(".to_string();
                let mut ie_rp: String = ")".to_string();
                let mut ie_call_code: String = [ie_func.as_str(), ie_lp.as_str(), ie_args_code.as_str(), ie_rp.as_str()].concat();
                return make_result(ie_call_code, ie_after.clone());
            }
            if kind == "KW_AT" {
                // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                let mut ie_idx_pos: i64 = ie_next + 1.clone();
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
    let mut next: i64 = pos + 1.clone();
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

fn gen_expr(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: expr_float_prescan (transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor)
    let mut pre_ctx_was: bool = float_ctx_get();
    let mut expr_has_float: bool = false;
    let mut pre_scan: i64 = pos.clone();
    let mut pre_depth: i64 = 0;
    while pre_scan < token_count {
        // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
        let mut pre_tok: Token = tokens[pre_scan as usize].clone();
        let kind = pre_tok.kind.clone();
        if kind == "FLOAT" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
            expr_has_float = true;
            break;
        }
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
            break;
        }
        if kind == "EOF" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
            break;
        }
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
            pre_depth = pre_depth + 1;
        }
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
            if pre_depth == 0 {
                // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
                break;
            }
            pre_depth = pre_depth - 1;
        }
        pre_scan = pre_scan + 1;
    }
    if expr_has_float {
        // transpiler-deor/codegen/decl/stmt/expr/macros/expr_float_prescan.deor
        float_ctx_enable();
    }
    // transpiler-deor/codegen/decl/stmt/expr/expr.deor
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
        let mut after_op: i64 = cur_pos + 1.clone();
        // macro: expr_is_special (transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor)
        if kind == "KW_IS" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
            if after_op < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                let mut maybe_not: Token = tokens[after_op as usize].clone();
                let kind = maybe_not.kind.clone();
                if kind == "KW_NOT" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                    operator_str = "is not".to_string();
                    after_op = after_op + 1;
                }
                if kind == "KW_EMPTY" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                    let mut ie_sfx: String = ".is_empty()".to_string();
                    left_code = s_cat(left_code, ie_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
                if kind == "KW_VALID" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                    let mut iv_sfx: String = ".is_some()".to_string();
                    left_code = s_cat(left_code, iv_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
            }
        }
        if operator_str == "is not" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
            if after_op < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                let mut maybe_empty: Token = tokens[after_op as usize].clone();
                let kind = maybe_empty.kind.clone();
                if kind == "KW_EMPTY" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                    let mut ine_pfx: String = "!".to_string();
                    let mut ine_sfx: String = ".is_empty()".to_string();
                    left_code = s_cat(ine_pfx.clone(), left_code);
                    left_code = s_cat(left_code, ine_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
                if kind == "KW_VALID" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                    let mut inv_pfx: String = "!(".to_string();
                    let mut inv_sfx: String = ".is_some())".to_string();
                    left_code = s_cat(inv_pfx.clone(), left_code);
                    left_code = s_cat(left_code, inv_sfx.clone());
                    cur_pos = after_op + 1;
                    continue;
                }
            }
        }
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        let mut rhs_r: ParseResult = gen_primary(tokens.clone(), after_op.clone(), ctx.clone());
        let code = rhs_r.code;
        let new_pos = rhs_r.new_pos;
        let mut rhs_code = code;
        let rhs_pos = new_pos;
        let mut rust_op: String = map_op(operator_str.clone());
        // macro: expr_string_concat (transpiler-deor/codegen/decl/stmt/expr/macros/expr_string_concat.deor)
        if operator_str == "+" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_string_concat.deor
            let mut left_token: Token = tokens[pos as usize].clone();
            let kind = left_token.kind.clone();
            let mut left_is_str: bool = kind == "STRING".clone();
            let mut rhs_token: Token = tokens[after_op as usize].clone();
            let kind = rhs_token.kind.clone();
            let mut rhs_is_str: bool = kind == "STRING".clone();
            let mut rhs_is_ident: bool = kind == "IDENT".clone();
            if left_is_str {
                // transpiler-deor/codegen/decl/stmt/expr/macros/expr_string_concat.deor
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
                // transpiler-deor/codegen/decl/stmt/expr/macros/expr_string_concat.deor
                if rhs_is_ident {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/expr_string_concat.deor
                    let mut astr_sfx: String = ".as_str()".to_string();
                    rhs_code = s_cat(rhs_code, astr_sfx.clone());
                }
            }
            if rhs_is_str {
                // transpiler-deor/codegen/decl/stmt/expr/macros/expr_string_concat.deor
                left_has_str = true;
            }
        }
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
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
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/helpers.deor
    if val_kind == "STRING" {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        return [val_code.as_str(), RS_TOS.as_str()].concat();
    }
    if val_kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        return [val_code.as_str(), RS_CLN.as_str()].concat();
    }
    return val_code;
}

// transpiler-deor/codegen/decl/stmt/destructure.deor
fn gen_destructure(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // macro: initialize_gen_destructure (transpiler-deor/codegen/decl/stmt/macros/initialize_gen_destructure.deor)
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i64 = pos + 1.clone();
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
        } else {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            cur = cur + 1;
        }
    }
    // macro: gen_input_check (transpiler-deor/codegen/decl/stmt/macros/gen_input_check.deor)
    let mut gic_matched: bool = false;
    let mut gic_is_args: bool = false;
    let mut gic_nm_ok: bool = false;
    let mut gic_lp: i64 = 0;
    // macro: gic_match_kw_and_name (transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor)
    let mut gic_in_tok: Token = tokens[cur as usize].clone();
    let kind = gic_in_tok.kind.clone();
    if kind == "KW_IN" {
        // transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor
        let mut gic_nm_pos: i64 = cur + 1.clone();
        if gic_nm_pos < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor
            let mut gic_nm_tok: Token = tokens[gic_nm_pos as usize].clone();
            let kind = gic_nm_tok.kind.clone();
            let value = gic_nm_tok.value.clone();
            if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor
                if value == "input" {
                    // transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor
                    gic_nm_ok = true;
                } else if value == "args" {
                    // transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor
                    gic_nm_ok = true;
                    gic_is_args = true;
                }
                if gic_nm_ok {
                    // transpiler-deor/codegen/decl/stmt/macros/gic_match_kw_and_name.deor
                    gic_lp = gic_nm_pos + 1;
                }
            }
        }
    }
    // transpiler-deor/codegen/decl/stmt/macros/gen_input_check.deor
    if gic_nm_ok {
        // macro: gic_match_parens (transpiler-deor/codegen/decl/stmt/macros/gic_match_parens.deor)
        if gic_lp < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/gic_match_parens.deor
            let mut gic_lp_tok: Token = tokens[gic_lp as usize].clone();
            let kind = gic_lp_tok.kind.clone();
            if kind == "LPAREN" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_match_parens.deor
                let mut gic_rp: i64 = gic_lp + 1.clone();
                if gic_rp < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/gic_match_parens.deor
                    let mut gic_rp_tok: Token = tokens[gic_rp as usize].clone();
                    let kind = gic_rp_tok.kind.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/macros/gic_match_parens.deor
                        gic_matched = true;
                    }
                }
            }
        }
    }
    if gic_matched {
        // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
        let RS_IND: String = "    ".to_string();
        let RS_NL: String = "\n".to_string();
        let RS_SC: String = ";\n".to_string();
        let RS_OB: String = " {\n".to_string();
        let RS_CB: String = "}\n".to_string();
        let RS_CB2: String = "\n}\n\n".to_string();
        let RS_FNC_CB: String = "}\n\n".to_string();
        let RS_EQ: String = " = ".to_string();
        let RS_LET: String = "let ".to_string();
        let RS_LETM: String = "let mut ".to_string();
        let RS_COL: String = ": ".to_string();
        let RS_COM: String = ",".to_string();
        let RS_CSEP: String = ", ".to_string();
        let RS_LP: String = "(".to_string();
        let RS_RP: String = ")".to_string();
        let RS_RP_SC: String = ");\n".to_string();
        let RS_ARR: String = " -> ".to_string();
        let RS_OB_SP: String = " { ".to_string();
        let RS_CB_SC: String = " };\n".to_string();
        let RS_CLN: String = ".clone()".to_string();
        let RS_TOS: String = ".to_string()".to_string();
        // transpiler-deor/codegen/decl/stmt/macros/gen_input_check.deor
        let mut gic_pad: String = s_repeat(RS_IND.clone(), depth.clone());
        let mut gic_out: Vec<String> = Vec::new();
        // macro: gic_emit_header (transpiler-deor/codegen/decl/stmt/macros/gic_emit_header.deor)
        if gic_is_args {
            // transpiler-deor/codegen/decl/stmt/macros/gic_emit_header.deor
            let mut gic_hdr: String = "let _deor_args: Vec<String> = std::env::args().skip(1).collect();".to_string();
            let mut gic_hdr_line: String = [gic_pad.as_str(), gic_hdr.as_str(), RS_NL.as_str()].concat();
            gic_out.push(gic_hdr_line.clone());
        } else {
            // transpiler-deor/codegen/decl/stmt/macros/gic_emit_header.deor
            let mut gic_raw_decl: String = "let mut _deor_raw: String = String::new();".to_string();
            let mut gic_raw_line: String = [gic_pad.as_str(), gic_raw_decl.as_str(), RS_NL.as_str()].concat();
            gic_out.push(gic_raw_line.clone());
            let mut gic_flush: String = "std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());".to_string();
            let mut gic_flush_line: String = [gic_pad.as_str(), gic_flush.as_str(), RS_NL.as_str()].concat();
            gic_out.push(gic_flush_line.clone());
            let mut gic_read: String = "std::io::stdin().read_line(&mut _deor_raw).unwrap_or_default();".to_string();
            let mut gic_read_line: String = [gic_pad.as_str(), gic_read.as_str(), RS_NL.as_str()].concat();
            gic_out.push(gic_read_line.clone());
            let mut gic_split: String = "let _deor_args: Vec<String> = _deor_raw.split_whitespace().map(|s| s.to_string()).collect();".to_string();
            let mut gic_split_line: String = [gic_pad.as_str(), gic_split.as_str(), RS_NL.as_str()].concat();
            gic_out.push(gic_split_line.clone());
        }
        // macro: gic_emit_bindings (transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor)
        let mut gic_fc: i64 = (fields.len() as i64);
        for gic_i in 0..gic_fc {
            // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
            let mut gic_fname: String = fields[gic_i as usize].clone();
            let mut gic_bind: String = "".to_string();
            if gic_fname == "first" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
                gic_bind = "let first: String = _deor_args.get(0).cloned().unwrap_or_default();".to_string();
            } else if gic_fname == "second" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
                gic_bind = "let second: String = _deor_args.get(1).cloned().unwrap_or_default();".to_string();
            } else if gic_fname == "third" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
                gic_bind = "let third: String = _deor_args.get(2).cloned().unwrap_or_default();".to_string();
            } else if gic_fname == "input_string" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
                gic_bind = "let input_string: String = _deor_args.join(\" \");".to_string();
            } else if gic_fname == "input_list" {
                // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
                gic_bind = "let input_list: Vec<String> = _deor_args.clone();".to_string();
            }
            if !is_empty(gic_bind.clone()) {
                // transpiler-deor/codegen/decl/stmt/macros/gic_emit_bindings.deor
                let mut gic_line: String = [gic_pad.as_str(), gic_bind.as_str(), RS_NL.as_str()].concat();
                gic_out.push(gic_line.clone());
            }
        }
        // transpiler-deor/codegen/decl/stmt/macros/gen_input_check.deor
        let mut gic_code: String = s_join(gic_out.clone());
        let mut gic_past: i64 = cur + 4.clone();
        return make_nl_result(gic_code, gic_past.clone(), tokens.clone());
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut field_count: i64 = (fields.len() as i64);
    // macro: gen_enum_extract_check (transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor)
    let typed_enum_reg = ctx.typed_enum_reg.clone();
    let typed_variant_reg = ctx.typed_variant_reg.clone();
    let mut geec_matched: bool = false;
    if cur < token_count {
        // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
        let mut geec_in_tok: Token = tokens[cur as usize].clone();
        let kind = geec_in_tok.kind.clone();
        if kind == "KW_IN" {
            // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
            let mut geec_name_pos: i64 = cur + 1.clone();
            if geec_name_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                let mut geec_name_tok: Token = tokens[geec_name_pos as usize].clone();
                let kind = geec_name_tok.kind.clone();
                let value = geec_name_tok.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                    let mut geec_val_type: String = reg_get(typed_enum_reg.clone(), value.clone());
                    if !is_empty(geec_val_type.clone()) {
                        // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                        geec_matched = true;
                        let mut geec_enum: String = value.clone();
                        // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
                        let RS_IND: String = "    ".to_string();
                        let RS_NL: String = "\n".to_string();
                        let RS_SC: String = ";\n".to_string();
                        let RS_OB: String = " {\n".to_string();
                        let RS_CB: String = "}\n".to_string();
                        let RS_CB2: String = "\n}\n\n".to_string();
                        let RS_FNC_CB: String = "}\n\n".to_string();
                        let RS_EQ: String = " = ".to_string();
                        let RS_LET: String = "let ".to_string();
                        let RS_LETM: String = "let mut ".to_string();
                        let RS_COL: String = ": ".to_string();
                        let RS_COM: String = ",".to_string();
                        let RS_CSEP: String = ", ".to_string();
                        let RS_LP: String = "(".to_string();
                        let RS_RP: String = ")".to_string();
                        let RS_RP_SC: String = ");\n".to_string();
                        let RS_ARR: String = " -> ".to_string();
                        let RS_OB_SP: String = " { ".to_string();
                        let RS_CB_SC: String = " };\n".to_string();
                        let RS_CLN: String = ".clone()".to_string();
                        let RS_TOS: String = ".to_string()".to_string();
                        // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                        let mut geec_pad: String = s_repeat(RS_IND.clone(), depth.clone());
                        let mut geec_out: Vec<String> = Vec::new();
                        for geec_fi in 0..field_count {
                            // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                            let mut geec_field: String = fields[geec_fi as usize].clone();
                            let mut geec_dot: String = ".".to_string();
                            let mut geec_key: String = [geec_enum.as_str(), geec_dot.as_str(), geec_field.as_str()].concat();
                            let mut geec_lit: String = reg_get(typed_variant_reg.clone(), geec_key.clone());
                            let mut geec_is_mut: bool = list_has(mut_names.clone(), geec_field.clone());
                            let mut geec_mut: String = "".to_string();
                            if geec_is_mut {
                                // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                geec_mut = "mut ".to_string();
                            }
                            let mut geec_line: String = "".to_string();
                            if geec_val_type == "string" {
                                // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                let mut geec_debug: String = s_debug(geec_lit.clone());
                                geec_line = [geec_pad.as_str(), "let ", geec_mut.as_str(), geec_field.as_str(), ": String = ", geec_debug.as_str(), ".to_string();"].concat();
                            } else {
                                // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                let mut geec_rust_t: String = render_rust_type(geec_val_type.clone());
                                geec_line = [geec_pad.as_str(), "let ", geec_mut.as_str(), geec_field.as_str(), ": ", geec_rust_t.as_str(), " = ", geec_lit.as_str(), ";"].concat();
                            }
                            geec_out.push(geec_line.clone());
                        }
                        let mut geec_code: String = s_join_nl(geec_out.clone());
                        let mut geec_past: i64 = cur + 2.clone();
                        return make_nl_result(geec_code, geec_past.clone(), tokens.clone());
                    }
                }
            }
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut val_pos: i64 = cur + 1.clone();
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = ge_r.code;
    let new_pos = ge_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut dest_lines: Vec<String> = Vec::new();
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
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
    let mut after: i64 = adv_nl_ref(val_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    dest_code = s_cat(dest_code.clone(), RS_NL.clone());
    return make_result(dest_code, after.clone());
}

fn gen_move_destructure(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // macro: initialize_gen_destructure (transpiler-deor/codegen/decl/stmt/macros/initialize_gen_destructure.deor)
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i64 = pos + 1.clone();
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
        } else {
            // transpiler-deor/codegen/decl/stmt/macros/for_collect_fields.deor
            cur = cur + 1;
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut val_pos: i64 = cur + 1.clone();
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = ge_r.code;
    let new_pos = ge_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut dest_lines: Vec<String> = Vec::new();
    let mut field_count: i64 = (fields.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
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
    let mut after: i64 = adv_nl_ref(val_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    dest_code = s_cat(dest_code.clone(), RS_NL.clone());
    return make_result(dest_code, after.clone());
}

// transpiler-deor/codegen/decl/stmt/block.deor
fn gen_block(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/block.deor
    let tokens = ctx.tokens.clone();
    let mut stmts: Vec<String> = Vec::new();
    let mut cur: i64 = pos.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut last_file: String = "".to_string();
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/block.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    loop {
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
            let mut mc1: String = s_cat(pad.clone(), mc_prefix.clone());
            let mut mc2: String = s_cat(mc1.clone(), value.clone());
            let mut mc3: String = s_cat(mc2.clone(), mc_open.clone());
            let mut mc4: String = s_cat(mc3.clone(), file.clone());
            let mut mc5: String = s_cat(mc4.clone(), mc_close.clone());
            let mut macro_comment: String = s_cat(mc5.clone(), RS_NL.clone());
            stmts.push(macro_comment.clone());
            last_file = file;
            cur = cur + 1;
            continue;
        }
        if file != last_file {
            // transpiler-deor/codegen/decl/stmt/block.deor
            let mut fc_slash: String = "// ".to_string();
            let mut fc1: String = s_cat(pad.clone(), fc_slash.clone());
            let mut fc2: String = s_cat(fc1.clone(), file.clone());
            let mut file_comment: String = s_cat(fc2.clone(), RS_NL.clone());
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
fn gen_if_branch(cond_pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/if.deor
    let tokens = ctx.tokens.clone();
    let mut cond_r: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let code = cond_r.code;
    let new_pos = cond_r.new_pos;
    let cond_code = code;
    let cond_end = new_pos;
    let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), cond_end.clone());
    // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
    let mut blk_depth: i64 = depth + 1.clone();
    let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
    let code = blk_r.code;
    let new_pos = blk_r.new_pos;
    let blk_code = code;
    let blk_end = new_pos;
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/if.deor
    let mut combined: String = [cond_code.as_str(), RS_OB.as_str(), blk_code.as_str()].concat();
    return make_result(combined, blk_end.clone());
}

fn gen_if(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/if.deor
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/if.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    let mut if_cond_pos: i64 = pos + 1.clone();
    let mut then_r: ParseResult = gen_if_branch(if_cond_pos.clone(), depth.clone(), ctx.clone());
    let code = then_r.code;
    let new_pos = then_r.new_pos;
    let then_code = code;
    let mut if_kw: String = "if ".to_string();
    let mut brc_cl: String = "}".to_string();
    let mut result_code: String = [pad.as_str(), if_kw.as_str(), then_code.as_str(), pad.as_str(), brc_cl.as_str()].concat();
    let mut cur = new_pos;
    loop {
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
        let mut after_else: i64 = cur + 1.clone();
        if after_else >= token_count {
            // transpiler-deor/codegen/decl/stmt/if.deor
            break;
        }
        let mut after_else_token: Token = tokens[after_else as usize].clone();
        let kind = after_else_token.kind.clone();
        if kind == "KW_IF" {
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut ei_cond: i64 = after_else + 1.clone();
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
            let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), after_else.clone());
            // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
            let mut blk_depth: i64 = depth + 1.clone();
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
    result_code = s_cat(result_code, RS_NL.clone());
    return make_result(result_code, cur.clone());
}

// transpiler-deor/codegen/decl/stmt/for.deor
fn gen_for(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/for.deor
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/for.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    let mut next_pos: i64 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_IF" {
        // macro: for_while (transpiler-deor/codegen/decl/stmt/macros/for_while.deor)
        let mut cond_pos: i64 = next_pos + 1.clone();
        let mut val_pos = cond_pos;
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), val_end.clone());
        let mut blk_depth: i64 = depth + 1.clone();
        let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
        let code = blk_r.code;
        let new_pos = blk_r.new_pos;
        let blk_code = code;
        let blk_end = new_pos;
        let mut whl_kw: String = "while ".to_string();
        let mut fw_head: String = [whl_kw.as_str(), val_code.as_str()].concat();
        if val_code == "true" {
            // transpiler-deor/codegen/decl/stmt/macros/for_while.deor
            fw_head = "loop".to_string();
        }
        let mut while_code: String = [pad.as_str(), fw_head.as_str(), RS_OB.as_str(), blk_code.as_str(), pad.as_str(), RS_CB.as_str()].concat();
        return make_result(while_code, blk_end.clone());
    }
    if kind == "KW_MOVE" {
        // macro: for_move (transpiler-deor/codegen/decl/stmt/macros/for_move.deor)
        let mut lparen_pos: i64 = next_pos + 1.clone();
        let mut var_pos: i64 = lparen_pos + 1.clone();
        let mut var_tok: Token = tokens[var_pos as usize].clone();
        let value = var_tok.value.clone();
        let mut move_var: String = value.clone();
        let mut in_pos: i64 = var_pos + 1.clone();
        let mut iter_pos: i64 = in_pos + 1.clone();
        let mut val_pos = iter_pos;
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut iter_next: i64 = val_end + 1.clone();
        let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), iter_next.clone());
        let mut blk_depth: i64 = depth + 1.clone();
        let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
        let code = blk_r.code;
        let new_pos = blk_r.new_pos;
        let blk_code = code;
        let blk_end = new_pos;
        let mut gfr_kw: String = "for ".to_string();
        let mut gfr_in: String = " in ".to_string();
        let mut for_code: String = [pad.as_str(), gfr_kw.as_str(), move_var.as_str(), gfr_in.as_str(), val_code.as_str(), RS_OB.as_str(), blk_code.as_str(), pad.as_str(), RS_CB.as_str()].concat();
        return make_result(for_code, blk_end.clone());
    }
    let mut var_name: String = "_".to_string();
    let mut iter_pos: i64 = 0;
    if kind == "KW_IN" {
        // transpiler-deor/codegen/decl/stmt/for.deor
        iter_pos = next_pos + 1;
    } else {
        // transpiler-deor/codegen/decl/stmt/for.deor
        let value = next_token.value.clone();
        var_name = value;
        let mut in_pos: i64 = next_pos + 1.clone();
        iter_pos = in_pos + 1;
    }
    let mut iter_token: Token = tokens[iter_pos as usize].clone();
    let kind = iter_token.kind.clone();
    let value = iter_token.value.clone();
    let mut range_expr: String = "".to_string();
    let mut body_tok_pos: i64 = 0;
    // macro: for_iter_expr (transpiler-deor/codegen/decl/stmt/macros/for_iter_expr.deor)
    if kind == "IDENT" && value == "range" {
        // transpiler-deor/codegen/decl/stmt/macros/for_iter_expr.deor
        let mut lparen: i64 = iter_pos + 1.clone();
        let mut first_pos: i64 = lparen + 1.clone();
        let mut val_pos = first_pos;
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut comma_token: Token = tokens[val_end as usize].clone();
        let kind = comma_token.kind.clone();
        let mut has_start: bool = kind == "COMMA".clone();
        if has_start {
            // transpiler-deor/codegen/decl/stmt/macros/for_iter_expr.deor
            let mut first_code: String = val_code;
            let mut val_pos: i64 = val_end + 1.clone();
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            let mut rng_dot: String = "..".to_string();
            range_expr = [first_code.as_str(), rng_dot.as_str(), val_code.as_str()].concat();
            body_tok_pos = val_end + 1;
        } else {
            // transpiler-deor/codegen/decl/stmt/macros/for_iter_expr.deor
            let mut rng0_pfx: String = "0..".to_string();
            range_expr = [rng0_pfx.as_str(), val_code.as_str()].concat();
            body_tok_pos = val_end + 1;
        }
    } else if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/macros/for_iter_expr.deor
        let mut start_pos: i64 = iter_pos + 1.clone();
        let mut val_pos = start_pos;
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut start_code: String = val_code;
        let mut val_pos: i64 = val_end + 1.clone();
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut rng2_dot: String = "..".to_string();
        range_expr = [start_code.as_str(), rng2_dot.as_str(), val_code.as_str()].concat();
        body_tok_pos = val_end + 1;
    } else {
        // transpiler-deor/codegen/decl/stmt/macros/for_iter_expr.deor
        let mut val_pos = iter_pos;
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut amp: String = "&".to_string();
        let mut collection_ref: String = s_cat(amp.clone(), val_code.clone());
        range_expr = collection_ref;
        body_tok_pos = val_end;
    }
    // transpiler-deor/codegen/decl/stmt/for.deor
    let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), body_tok_pos.clone());
    // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
    let mut blk_depth: i64 = depth + 1.clone();
    let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
    let code = blk_r.code;
    let new_pos = blk_r.new_pos;
    let blk_code = code;
    let blk_end = new_pos;
    // transpiler-deor/codegen/decl/stmt/for.deor
    let mut frc_kw: String = "for ".to_string();
    let mut frc_in: String = " in ".to_string();
    let mut for_code: String = [pad.as_str(), frc_kw.as_str(), var_name.as_str(), frc_in.as_str(), range_expr.as_str(), RS_OB.as_str(), blk_code.as_str(), pad.as_str(), RS_CB.as_str()].concat();
    return make_result(for_code, blk_end.clone());
}

// transpiler-deor/codegen/decl/stmt/as_binding.deor
fn gen_as_binding(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/as_binding.deor
    let tokens = ctx.tokens.clone();
    let struct_reg = ctx.struct_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/as_binding.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    let mut name_token: Token = tokens[pos as usize].clone();
    let value = name_token.value.clone();
    let mut ident_name: String = value.clone();
    let mut next_pos: i64 = pos + 1.clone();
    let mut after_as: i64 = next_pos + 1.clone();
    let mut after_as_token: Token = tokens[after_as as usize].clone();
    let kind = after_as_token.kind.clone();
    let value = after_as_token.value.clone();
    let mut after_as_value: String = value.clone();
    if kind == "LPAREN" {
        // macro: aas_struct (transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor)
        let mut aas_is_struct: bool = true;
        let mut aas_peek: i64 = after_as + 1.clone();
        while aas_peek < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
            let mut aas_peek_tok: Token = tokens[aas_peek as usize].clone();
            let kind = aas_peek_tok.kind.clone();
            if kind == "RPAREN" {
                // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                break;
            }
            if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                aas_peek = aas_peek + 1;
                continue;
            }
            if kind == "COMMA" {
                // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                aas_peek = aas_peek + 1;
                continue;
            }
            aas_is_struct = false;
            break;
        }
        if aas_is_struct {
            // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
            let mut aas_fields: Vec<String> = Vec::new();
            let mut aas_fend: i64 = after_as + 1.clone();
            while aas_fend < token_count {
                // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                let mut aas_field_tok: Token = tokens[aas_fend as usize].clone();
                let kind = aas_field_tok.kind.clone();
                let value = aas_field_tok.value.clone();
                if kind == "RPAREN" {
                    // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                    aas_fend = aas_fend + 1;
                    break;
                } else if kind == "COMMA" {
                    // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                    aas_fend = aas_fend + 1;
                } else if kind == "IDENT" {
                    // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                    aas_fields.push(value.clone());
                    aas_fend = aas_fend + 1;
                }
            }
            let mut aas_struct: String = find_struct_for_fields(struct_reg.clone(), aas_fields.clone());
            let mut var_name: String = ident_name.clone();
            let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
            let mut mut_kw: String = "".to_string();
            if mg_is_mut {
                // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
                mut_kw = "mut ".to_string();
            }
            let mut aas_fcount: i64 = (aas_fields.len() as i64);
            let mut aas_pairs: Vec<String> = Vec::new();
            for aas_fi in 0..aas_fcount {
                // transpiler-deor/codegen/decl/stmt/macros/aas_struct.deor
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
        // macro: aas_empty (transpiler-deor/codegen/decl/stmt/macros/aas_empty.deor)
        let mut aas_emp_pfx: String = "let mut ".to_string();
        let mut aas_emp_sfx: String = " = Vec::new();\n".to_string();
        let mut aas_empty_code: String = [pad.as_str(), aas_emp_pfx.as_str(), ident_name.as_str(), aas_emp_sfx.as_str()].concat();
        let mut aas_after_empty: i64 = after_as + 1.clone();
        return make_nl_result(aas_empty_code, aas_after_empty.clone(), tokens.clone());
    }
    if kind == "IDENT" {
        // macro: aas_with (transpiler-deor/codegen/decl/stmt/macros/aas_with.deor)
        let mut aas_with_pos: i64 = after_as + 1.clone();
        if aas_with_pos < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
            let mut aas_with_tok: Token = tokens[aas_with_pos as usize].clone();
            let kind = aas_with_tok.kind.clone();
            if kind == "KW_WITH" {
                // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
                let mut aas_src: String = after_as_value.clone();
                let mut aas_lp: i64 = aas_with_pos + 1.clone();
                let mut aas_ovr: Vec<String> = Vec::new();
                let mut aas_wend: i64 = aas_lp + 1.clone();
                while aas_wend < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
                    let mut aas_wtok: Token = tokens[aas_wend as usize].clone();
                    let kind = aas_wtok.kind.clone();
                    let value = aas_wtok.value.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
                        aas_wend = aas_wend + 1;
                        break;
                    }
                    if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
                        aas_wend = aas_wend + 1;
                        continue;
                    }
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
                        aas_ovr.push(value.clone());
                        aas_wend = aas_wend + 1;
                    }
                }
                let mut aas_first: String = aas_ovr[0 as usize].clone();
                let mut aas_struct_w: String = find_struct_for_field(struct_reg.clone(), aas_first.clone());
                let mut aas_wsep: String = ", ".to_string();
                let mut aas_wfields: String = s_join_with(aas_ovr.clone(), aas_wsep.clone());
                let mut var_name: String = ident_name.clone();
                let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                let mut mut_kw: String = "".to_string();
                if mg_is_mut {
                    // transpiler-deor/codegen/decl/stmt/macros/aas_with.deor
                    mut_kw = "mut ".to_string();
                }
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
    // macro: aas_default (transpiler-deor/codegen/decl/stmt/macros/aas_default.deor)
    let kind = after_as_token.kind.clone();
    let val_pos = after_as;
    let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = ge_r.code;
    let new_pos = ge_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    let mut aas_is_str: bool = kind == "STRING".clone();
    let mut var_name: String = ident_name.clone();
    let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
    let mut mut_kw: String = "".to_string();
    if mg_is_mut {
        // transpiler-deor/codegen/decl/stmt/macros/aas_default.deor
        mut_kw = "mut ".to_string();
    }
    let mut aas_suffix: String = "".to_string();
    if aas_is_str {
        // transpiler-deor/codegen/decl/stmt/macros/aas_default.deor
        aas_suffix = ".to_string()".to_string();
    }
    let mut aas_let2: String = "let ".to_string();
    let mut aas_eq2: String = " = ".to_string();
    let mut aas_sc2: String = ";\n".to_string();
    let mut aas_code2: String = [pad.as_str(), aas_let2.as_str(), mut_kw.as_str(), ident_name.as_str(), aas_eq2.as_str(), val_code.as_str(), aas_suffix.as_str(), aas_sc2.as_str()].concat();
    return make_nl_result(aas_code2, val_end.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/call_stmt.deor
fn gen_call_stmt(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/call_stmt.deor
    let tokens = ctx.tokens.clone();
    let mut_names = ctx.mut_names.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/call_stmt.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    let mut name_token: Token = tokens[pos as usize].clone();
    let value = name_token.value.clone();
    let mut ident_name: String = value.clone();
    let mut next_pos: i64 = pos + 1.clone();
    let mut args_pos: i64 = next_pos + 1.clone();
    let mut args_r: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
    let code = args_r.code;
    let new_pos = args_r.new_pos;
    let args_code = code;
    let args_end = new_pos;
    let mut after_paren: i64 = args_end + 1.clone();
    let mut call_code: String = "".to_string();
    if ident_name == "print" {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut print_arg_count: i64 = count_call_args(tokens.clone(), next_pos.clone());
        if print_arg_count == 2 {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut prt_pfx: String = "print!(\"{}{}\", ".to_string();
            call_code = [pad.as_str(), prt_pfx.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
        } else {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut prt_pfx: String = "println!(\"{}\", ".to_string();
            call_code = [pad.as_str(), prt_pfx.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
        }
    } else if ident_name == "crash" {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut crsh_pfx: String = "panic!(\"{}\", ".to_string();
        call_code = [pad.as_str(), crsh_pfx.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
    } else {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        call_code = [pad.as_str(), ident_name.as_str(), RS_LP.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
    }
    return make_nl_result(call_code, after_paren.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/list_mutation.deor
fn gen_list_mutation_stmt(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/list_mutation.deor
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/list_mutation.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    let mut name_token: Token = tokens[pos as usize].clone();
    let value = name_token.value.clone();
    let mut ident_name: String = value.clone();
    let mut next_pos: i64 = pos + 1.clone();
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_AT" {
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut after_at: i64 = next_pos + 1.clone();
        if after_at < token_count {
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
            let mut at_next_token: Token = tokens[after_at as usize].clone();
            let kind = at_next_token.kind.clone();
            let value = at_next_token.value.clone();
            if kind == "KW_END" {
                // transpiler-deor/codegen/decl/stmt/list_mutation.deor
                let mut val_pos: i64 = after_at + 2.clone();
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
                let mut app_code: String = [pad.as_str(), ident_name.as_str(), app_pfx.as_str(), app_val.as_str(), RS_RP_SC.as_str()].concat();
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
            let mut idx_code: String = val_code;
            let mut val_pos: i64 = val_end + 1.clone();
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
        let mut idx_pos: i64 = next_pos + 2.clone();
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
    let mut lm_next: i64 = pos + 1.clone();
    return make_result(lm_unhand, lm_next.clone());
}

// transpiler-deor/codegen/decl/stmt/typed_binding.deor
fn gen_typed_binding(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
    let tokens = ctx.tokens.clone();
    let struct_reg = ctx.struct_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let type_reg = ctx.type_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let variant_reg = ctx.variant_reg.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    let mut type_token: Token = tokens[pos as usize].clone();
    let value = type_token.value.clone();
    let mut var_type: String = value.clone();
    let mut name_pos: i64 = pos + 1.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let value = name_token.value.clone();
    let mut var_name: String = value.clone();
    let mut eq_pos: i64 = name_pos + 1.clone();
    let mut val_pos: i64 = eq_pos + 1.clone();
    let mut rust_type: String = resolve_type(var_type.clone(), shape_reg.clone(), variant_reg.clone());
    let mut val_token: Token = tokens[val_pos as usize].clone();
    let kind = val_token.kind.clone();
    if kind == "KW_EMPTY" {
        // macro: tb_empty (transpiler-deor/codegen/decl/stmt/macros/tb_empty.deor)
        let mut is_shape: bool = reg_has(shape_reg.clone(), var_type.clone());
        let mut val_next_pos: i64 = val_pos + 1.clone();
        let mut after_empty: i64 = adv_nl_ref(val_next_pos.clone(), tokens.clone());
        if is_shape {
            // transpiler-deor/codegen/decl/stmt/macros/tb_empty.deor
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
        // macro: tb_paren (transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor)
        let mut peek_pos: i64 = val_pos + 1.clone();
        let mut peek_token: Token = tokens[peek_pos as usize].clone();
        let kind = peek_token.kind.clone();
        let mut is_avow_expr: bool = kind == "KW_AVOW".clone();
        let mut is_struct_type: bool = reg_has(struct_reg.clone(), var_type.clone());
        if !is_avow_expr {
            // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
            if is_struct_type {
                // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                let mut tb_fields: Vec<String> = Vec::new();
                let mut tb_fend: i64 = val_pos + 1.clone();
                while tb_fend < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                    let mut tb_field_tok: Token = tokens[tb_fend as usize].clone();
                    let kind = tb_field_tok.kind.clone();
                    let value = tb_field_tok.value.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                        tb_fend = tb_fend + 1;
                        break;
                    } else if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                        tb_fend = tb_fend + 1;
                    } else if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                        tb_fields.push(value.clone());
                        tb_fend = tb_fend + 1;
                    } else {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                        tb_fend = tb_fend + 1;
                    }
                }
                let mut field_pairs: Vec<String> = Vec::new();
                let mut tb_fcount: i64 = (tb_fields.len() as i64);
                for tb_fi in 0..tb_fcount {
                    // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                    let mut tb_fname: String = tb_fields[tb_fi as usize].clone();
                    let mut sfp_sep: String = ": ".to_string();
                    let mut sfp_cln: String = ".clone()".to_string();
                    field_pairs.push([tb_fname.as_str(), sfp_sep.as_str(), tb_fname.as_str(), sfp_cln.as_str()].concat().clone());
                }
                let mut sep: String = ", ".to_string();
                let mut fields_code: String = s_join_with(field_pairs.clone(), sep.clone());
                let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                let mut mut_kw: String = "".to_string();
                if mg_is_mut {
                    // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                    mut_kw = "mut ".to_string();
                }
                let mut scc_let: String = "let ".to_string();
                let mut scc_eq: String = " = ".to_string();
                let mut scc_ob: String = " { ".to_string();
                let mut scc_cb: String = " };\n".to_string();
                let mut sc_code: String = [pad.as_str(), scc_let.as_str(), mut_kw.as_str(), var_name.as_str(), scc_eq.as_str(), var_type.as_str(), scc_ob.as_str(), fields_code.as_str(), scc_cb.as_str()].concat();
                return make_nl_result(sc_code, tb_fend.clone(), tokens.clone());
            }
        }
        if is_avow_expr {
            // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
            let mut inner_pos: i64 = peek_pos + 1.clone();
            let mut val_pos = inner_pos;
            let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = ge_r.code;
            let new_pos = ge_r.new_pos;
            let val_code = code;
            let val_end = new_pos;
            let mut after_rparen: i64 = val_end + 1.clone();
            let mut suf_unwrap: String = ".unwrap()".to_string();
            let mut suf_unwrap0: String = ".unwrap().0".to_string();
            let mut unwrap_expr: String = s_cat(val_code.clone(), suf_unwrap.clone());
            if var_type == "int" {
                // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            if var_type == "string" {
                // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            if var_type == "bool" {
                // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
                unwrap_expr = s_cat(val_code.clone(), suf_unwrap0.clone());
            }
            if var_type == "float" {
                // transpiler-deor/codegen/decl/stmt/macros/tb_paren.deor
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
        // macro: tb_list_literal (transpiler-deor/codegen/decl/stmt/macros/tb_list_literal.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut lst_pfx: String = "let mut ".to_string();
        let mut lst_col: String = ": ".to_string();
        let mut lst_eq: String = " = ".to_string();
        let mut lst_sc: String = ";\n".to_string();
        let mut lst_code: String = [pad.as_str(), lst_pfx.as_str(), var_name.as_str(), lst_col.as_str(), rust_type.as_str(), lst_eq.as_str(), val_code.as_str(), lst_sc.as_str()].concat();
        return make_nl_result(lst_code, val_end.clone(), tokens.clone());
    }
    if kind == "IDENT" {
        // macro: tb_with (transpiler-deor/codegen/decl/stmt/macros/tb_with.deor)
        let mut tb_with_pos: i64 = val_pos + 1.clone();
        if tb_with_pos < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
            let mut tb_with_tok: Token = tokens[tb_with_pos as usize].clone();
            let kind = tb_with_tok.kind.clone();
            if kind == "KW_WITH" {
                // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
                let value = val_token.value.clone();
                let mut tb_src: String = value.clone();
                let mut tb_lp: i64 = tb_with_pos + 1.clone();
                let mut tb_ovr: Vec<String> = Vec::new();
                let mut tb_wend: i64 = tb_lp + 1.clone();
                while tb_wend < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
                    let mut tb_wtok: Token = tokens[tb_wend as usize].clone();
                    let kind = tb_wtok.kind.clone();
                    let value = tb_wtok.value.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
                        tb_wend = tb_wend + 1;
                        break;
                    }
                    if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
                        tb_wend = tb_wend + 1;
                        continue;
                    }
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
                        tb_ovr.push(value.clone());
                        tb_wend = tb_wend + 1;
                    }
                }
                let mut tb_wsep: String = ", ".to_string();
                let mut tb_wfields: String = s_join_with(tb_ovr.clone(), tb_wsep.clone());
                let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                let mut mut_kw: String = "".to_string();
                if mg_is_mut {
                    // transpiler-deor/codegen/decl/stmt/macros/tb_with.deor
                    mut_kw = "mut ".to_string();
                }
                let mut tb_wlet: String = "let ".to_string();
                let mut tb_weq: String = " = ".to_string();
                let mut tb_wob: String = " { ".to_string();
                let mut tb_wsp: String = ", ..".to_string();
                let mut tb_wcb: String = " };\n".to_string();
                let mut tb_with_code: String = [pad.as_str(), tb_wlet.as_str(), mut_kw.as_str(), var_name.as_str(), tb_weq.as_str(), rust_type.as_str(), tb_wob.as_str(), tb_wfields.as_str(), tb_wsp.as_str(), tb_src.as_str(), tb_wcb.as_str()].concat();
                return make_nl_result(tb_with_code, tb_wend.clone(), tokens.clone());
            }
        }
    }
    let mut is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
    if is_validator {
        // macro: tb_validator (transpiler-deor/codegen/decl/stmt/macros/tb_validator.deor)
        let mut ge_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = ge_r.code;
        let new_pos = ge_r.new_pos;
        let val_code = code;
        let val_end = new_pos;
        let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
        let mut mut_kw: String = "".to_string();
        if mg_is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/tb_validator.deor
            mut_kw = "mut ".to_string();
        }
        let mut vld_let: String = "let ".to_string();
        let mut vld_opt: String = ": Option<".to_string();
        let mut vld_new: String = "> = ".to_string();
        let mut vld_nop: String = "::new(".to_string();
        let mut vld_sc: String = ");\n".to_string();
        let mut vld_code: String = [pad.as_str(), vld_let.as_str(), mut_kw.as_str(), var_name.as_str(), vld_opt.as_str(), var_type.as_str(), vld_new.as_str(), var_type.as_str(), vld_nop.as_str(), val_code.as_str(), vld_sc.as_str()].concat();
        return make_nl_result(vld_code, val_end.clone(), tokens.clone());
    }
    // macro: tb_default (transpiler-deor/codegen/decl/stmt/macros/tb_default.deor)
    let mut tb_is_float: bool = var_type == "float".clone();
    if tb_is_float {
        // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
        float_ctx_enable();
    }
    let mut val_r: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    if tb_is_float {
        // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
        float_ctx_disable();
    }
    let code = val_r.code;
    let new_pos = val_r.new_pos;
    let val_code = code;
    let val_end = new_pos;
    let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
    let mut mut_kw: String = "".to_string();
    if mg_is_mut {
        // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
        mut_kw = "mut ".to_string();
    }
    let mut suffix: String = "".to_string();
    if kind == "STRING" {
        // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
        suffix = ".to_string()".to_string();
    } else if kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
        let mut val_next_idx: i64 = val_pos + 1.clone();
        if val_next_idx < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
            let mut next_val_tok: Token = tokens[val_next_idx as usize].clone();
            let kind = next_val_tok.kind.clone();
            let mut val_is_call: bool = kind == "LPAREN".clone();
            let mut val_is_idx: bool = kind == "KW_AT".clone();
            if !val_is_call {
                // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
                if !val_is_idx {
                    // transpiler-deor/codegen/decl/stmt/macros/tb_default.deor
                    suffix = ".clone()".to_string();
                }
            }
        }
    }
    let mut bind_code: String = [pad.as_str(), RS_LET.as_str(), mut_kw.as_str(), var_name.as_str(), RS_COL.as_str(), rust_type.as_str(), RS_EQ.as_str(), val_code.as_str(), suffix.as_str(), RS_SC.as_str()].concat();
    return make_nl_result(bind_code, val_end.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/stmt.deor
fn gen_stmt(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut token: Token = tokens[pos as usize].clone();
    let kind = token.kind.clone();
    let value = token.value.clone();
    let line = token.line.clone();
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
    // macro: stmt_flow (transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor)
    if kind == "KW_RETURN" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
        let mut sf_val_pos: i64 = pos + 1.clone();
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
        let mut sf_brk_n: i64 = pos + 1.clone();
        return make_nl_result(sf_brk_code, sf_brk_n.clone(), tokens.clone());
    }
    if kind == "KW_CONTINUE" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
        let mut sf_cnt_kw: String = "continue;\n".to_string();
        let mut sf_cnt_code: String = [pad.as_str(), sf_cnt_kw.as_str()].concat();
        let mut sf_cnt_n: i64 = pos + 1.clone();
        return make_nl_result(sf_cnt_code, sf_cnt_n.clone(), tokens.clone());
    }
    // macro: stmt_blocks (transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor)
    if kind == "KW_BLOCK" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
        let mut sb_nl: i64 = pos + 1.clone();
        let mut sb_body_start: i64 = skip_to_body_ref(tokens.clone(), sb_nl.clone());
        let mut sb_body_depth: i64 = depth + 1.clone();
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
        let mut sb_block_pos: i64 = pos + 2.clone();
        let mut sb_block_tok: Token = tokens[sb_block_pos as usize].clone();
        let value = sb_block_tok.value.clone();
        let mut sb_content: String = value.clone();
        let mut sb_rust_lines: Vec<String> = s_split(sb_content.clone(), RS_NL.clone());
        let mut sb_padded: Vec<String> = Vec::new();
        let mut sb_line_count: i64 = (sb_rust_lines.len() as i64);
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
        sb_block_code = s_cat(sb_block_code.clone(), RS_NL.clone());
        let mut sb_block_next: i64 = sb_block_pos + 1.clone();
        return make_result(sb_block_code, sb_block_next.clone());
    }
    // macro: stmt_structural (transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor)
    if kind == "KW_MOVE" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
        let mut smd_next: i64 = pos + 1.clone();
        let mut smd_next_tok: Token = tokens[smd_next as usize].clone();
        let kind = smd_next_tok.kind.clone();
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
            return gen_move_destructure(smd_next.clone(), depth.clone(), ctx.clone());
        }
    }
    if kind == "LPAREN" {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
        let mut su_peek: i64 = pos + 1.clone();
        if su_peek < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
            let mut su_peek_tok: Token = tokens[su_peek as usize].clone();
            let kind = su_peek_tok.kind.clone();
            if kind == "KW_AVOW" {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
                let mut su_expr_pos: i64 = su_peek + 1.clone();
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
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    if kind == "KW_RAW" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut raw_name_pos: i64 = pos + 1.clone();
        let mut raw_name_tok: Token = tokens[raw_name_pos as usize].clone();
        let value = raw_name_tok.value.clone();
        let mut raw_var_name: String = value.clone();
        let mut val_pos: i64 = raw_name_pos + 2.clone();
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
        let mut raw_parts: Vec<String> = vec![pad.clone(), RS_LET.clone(), mut_kw.clone(), raw_var_name.clone(), RS_EQ.clone(), val_code.clone(), RS_SC.clone()];
        let mut raw_code: String = s_join(raw_parts.clone());
        return make_nl_result(raw_code, val_end.clone(), tokens.clone());
    }
    if kind == "KW_CONST" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut const_type_pos: i64 = pos + 1.clone();
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
        let mut next_pos: i64 = pos + 1.clone();
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
            let mut val_pos: i64 = next_pos + 1.clone();
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
            let mut asg_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), RS_EQ.clone(), val_code.clone(), assign_suffix.clone(), RS_SC.clone()];
            let mut asgn_code: String = s_join(asg_parts.clone());
            return make_nl_result(asgn_code, val_end.clone(), tokens.clone());
        }
        if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut eq_pos: i64 = next_pos + 1.clone();
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
                let mut bd_sfx: String = "> = None;\n".to_string();
                let mut bd_code: String = [pad.as_str(), RS_LETM.as_str(), bare_var_name.as_str(), RS_COL.as_str(), "Option<", bare_rust_type.as_str(), bd_sfx.as_str()].concat();
                let mut bd_after: i64 = next_pos + 1.clone();
                return make_nl_result(bd_code, bd_after.clone(), tokens.clone());
            }
        }
    }
    let mut unh_pfx: String = "/* unhandled(".to_string();
    let mut unh_sfx: String = ") */\n".to_string();
    let mut unh_parts: Vec<String> = vec![unh_pfx.clone(), kind.clone(), unh_sfx.clone()];
    let mut unhand: String = s_join(unh_parts.clone());
    let mut unhand_next: i64 = pos + 1.clone();
    return make_result(unhand, unhand_next.clone());
}

// transpiler-deor/codegen/decl/cursor.deor
fn cur_at(tokens: Vec<Token>, pos: i64) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut current: Token = tokens[pos as usize].clone();
    let cur = TokenCursor { token_count: token_count.clone(), pos: pos.clone(), current: current.clone() };
    return cur;
}

fn cur_next(cur: TokenCursor, tokens: Vec<Token>) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = cur.token_count.clone();
    let mut pos = cur.pos.clone();
    let mut current = cur.current.clone();
    let mut pos: i64 = pos + 1.clone();
    if pos < token_count {
        // transpiler-deor/codegen/decl/cursor.deor
        let mut current: Token = tokens[pos as usize].clone();
        return TokenCursor { token_count, pos, current };
    }
    return TokenCursor { token_count, pos, current };
}

fn c_at_end(cur: TokenCursor) -> bool {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    return pos >= token_count;
}

fn cur_skip_to_body(cur: TokenCursor, tokens: Vec<Token>) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = cur.pos.clone();
    let mut body_pos: i64 = adv_nl(pos.clone(), tokens.clone());
    body_pos = adv_indent(body_pos.clone(), tokens.clone());
    return cur_at(tokens.clone(), body_pos.clone());
}

fn cur_peek(cur: TokenCursor, tokens: Vec<Token>, offset: i64) -> Token {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = cur.pos.clone();
    let mut peek_pos: i64 = pos + offset.clone();
    return tokens[peek_pos as usize].clone();
}

fn cur_at_ref(tokens: TokensRef, pos: i64) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut current: Token = tokens[pos as usize].clone();
    let cur = TokenCursor { token_count: token_count.clone(), pos: pos.clone(), current: current.clone() };
    return cur;
}

fn cur_next_ref(cur: TokenCursor, tokens: TokensRef) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = cur.token_count.clone();
    let mut pos = cur.pos.clone();
    let mut current = cur.current.clone();
    let mut pos: i64 = pos + 1.clone();
    if pos < token_count {
        // transpiler-deor/codegen/decl/cursor.deor
        let mut current: Token = tokens[pos as usize].clone();
        return TokenCursor { token_count, pos, current };
    }
    return TokenCursor { token_count, pos, current };
}

fn cur_skip_to_body_ref(cur: TokenCursor, tokens: TokensRef) -> TokenCursor {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = cur.pos.clone();
    let mut body_pos: i64 = adv_nl_ref(pos.clone(), tokens.clone());
    body_pos = adv_indent_ref(body_pos.clone(), tokens.clone());
    return cur_at_ref(tokens.clone(), body_pos.clone());
}

fn cur_peek_ref(cur: TokenCursor, tokens: TokensRef, offset: i64) -> Token {
    // transpiler-deor/codegen/decl/cursor.deor
    let pos = cur.pos.clone();
    let mut peek_pos: i64 = pos + offset.clone();
    return tokens[peek_pos as usize].clone();
}

// transpiler-deor/codegen/decl/struct.deor
fn gen_struct_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/struct.deor
    let mut start_pos: i64 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), start_pos.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut struct_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/struct.deor
    let mut field_lines: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        // transpiler-deor/codegen/decl/struct.deor
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "DEDENT" {
            // transpiler-deor/codegen/decl/struct.deor
            cur = cur_next_ref(cur.clone(), tokens.clone());
            break;
        } else if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/struct.deor
            cur = cur_next_ref(cur.clone(), tokens.clone());
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/struct.deor
            let mut field_type: String = value.clone();
            cur = cur_next_ref(cur.clone(), tokens.clone());
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut field_name: String = value.clone();
            let mut rust_type: String = render_rust_type(field_type.clone());
            field_lines.push([RS_IND.as_str(), field_name.as_str(), RS_COL.as_str(), rust_type.as_str(), RS_COM.as_str()].concat().clone());
            cur = cur_next_ref(cur.clone(), tokens.clone());
        }
    }
    let mut fields_code: String = s_join_nl(field_lines.clone());
    let mut sdcl_pfx: String = "#[derive(Clone, PartialEq, Debug)]\nstruct ".to_string();
    let mut decl: String = [sdcl_pfx.as_str(), struct_name.as_str(), RS_OB.as_str(), fields_code.as_str(), RS_CB2.as_str()].concat();
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
}

// transpiler-deor/codegen/decl/enum.deor
fn gen_enum_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/enum.deor
    let mut start_pos: i64 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), start_pos.clone());
    let current = cur.current.clone();
    let kind = current.kind.clone();
    let value = current.value.clone();
    let mut ged_is_typed: bool = is_typed_enum_type(value.clone());
    if ged_is_typed {
        // transpiler-deor/codegen/decl/enum.deor
        cur = cur_next_ref(cur.clone(), tokens.clone());
        cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
        while !c_at_end(cur.clone()) {
            // transpiler-deor/codegen/decl/enum.deor
            let current = cur.current.clone();
            let kind = current.kind.clone();
            cur = cur_next_ref(cur.clone(), tokens.clone());
            if kind == "DEDENT" {
                // transpiler-deor/codegen/decl/enum.deor
                break;
            }
        }
        let pos = cur.pos.clone();
        let mut empty_str: String = "".to_string();
        return make_result(empty_str.clone(), pos.clone());
    }
    let mut enum_name: String = value.clone();
    let mut rust_name: String = s_pascal(enum_name.clone());
    cur = cur_next_ref(cur.clone(), tokens.clone());
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // transpiler-deor/codegen/decl/enum.deor
    let mut variant_lines: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        // transpiler-deor/codegen/decl/enum.deor
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        cur = cur_next_ref(cur.clone(), tokens.clone());
        if kind == "DEDENT" {
            // transpiler-deor/codegen/decl/enum.deor
            break;
        }
        if kind == "IDENT" {
            // transpiler-deor/codegen/decl/enum.deor
            variant_lines.push([RS_IND.as_str(), value.as_str(), RS_COM.as_str()].concat().clone());
        }
    }
    let mut variants_code: String = s_join_nl(variant_lines.clone());
    let mut enm_pfx: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nenum ".to_string();
    let mut decl: String = [enm_pfx.as_str(), rust_name.as_str(), RS_OB.as_str(), variants_code.as_str(), RS_CB2.as_str()].concat();
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

fn gen_shape_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/shape.deor
    let mut name_pos: i64 = pos + 1.clone();
    let mut form_pos: i64 = pos + 3.clone();
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut form_token: Token = tokens[form_pos as usize].clone();
    let value = name_token.value.clone();
    let mut shape_name: String = value.clone();
    let kind = form_token.kind.clone();
    let mut rust_name: String = s_pascal(shape_name.clone());
    // macro: shape_list (transpiler-deor/codegen/decl/macros/shape_list.deor)
    if kind == "KW_LIST" {
        // transpiler-deor/codegen/decl/macros/shape_list.deor
        let mut elem_pos: i64 = pos + 5.clone();
        let mut elem_token: Token = tokens[elem_pos as usize].clone();
        let value = elem_token.value.clone();
        let mut elem_type: String = value.clone();
        let mut rust_elem: String = render_rust_type(elem_type.clone());
        let mut decl: String = gen_list_shape_code(rust_name.clone(), rust_elem.clone());
        let mut after: i64 = elem_pos + 1.clone();
        return make_nl_result(decl, after.clone(), tokens.clone());
    }
    // macro: shape_func (transpiler-deor/codegen/decl/macros/shape_func.deor)
    let mut t4_pos: i64 = pos + 4.clone();
    let mut t4_token: Token = tokens[t4_pos as usize].clone();
    let kind = t4_token.kind.clone();
    let value = t4_token.value.clone();
    let mut t4_is_of: bool = kind == "KW_OF".clone();
    let mut t4_is_to: bool = kind == "KW_TO".clone();
    let mut in_type: String = "".to_string();
    let mut out_type: String = "".to_string();
    let mut func_end: i64 = t4_pos.clone();
    if t4_is_of {
        // transpiler-deor/codegen/decl/macros/shape_func.deor
        let mut t5_pos: i64 = pos + 5.clone();
        let mut t5_token: Token = tokens[t5_pos as usize].clone();
        let value = t5_token.value.clone();
        in_type = value;
        let mut t6_pos: i64 = pos + 6.clone();
        let mut t6_token: Token = tokens[t6_pos as usize].clone();
        let value = t6_token.value.clone();
        let mut t6_is_to: bool = kind == "KW_TO".clone();
        func_end = t6_pos;
        if t6_is_to {
            // transpiler-deor/codegen/decl/macros/shape_func.deor
            let mut t7_pos: i64 = pos + 7.clone();
            let mut t7_token: Token = tokens[t7_pos as usize].clone();
            let value = t7_token.value.clone();
            out_type = value;
            func_end = t7_pos;
        }
    } else if t4_is_to {
        // transpiler-deor/codegen/decl/macros/shape_func.deor
        let mut t5_pos: i64 = pos + 5.clone();
        let mut t5_token: Token = tokens[t5_pos as usize].clone();
        let value = t5_token.value.clone();
        out_type = value;
        func_end = t5_pos;
    }
    let mut rust_in: String = render_rust_type(in_type.clone());
    let mut rust_out: String = render_rust_type(out_type.clone());
    let mut decl: String = gen_func_shape_code(rust_name.clone(), rust_in.clone(), rust_out.clone());
    let mut after: i64 = func_end + 1.clone();
    return make_nl_result(decl, after.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/validator_type.deor
fn gen_type_decl(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/validator_type.deor
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let type_reg = ctx.type_reg.clone();
    let typed_enum_reg = ctx.typed_enum_reg.clone();
    let typed_variant_reg = ctx.typed_variant_reg.clone();
    let mut start_pos: i64 = pos + 1.clone();
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), start_pos.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut type_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut param_type: String = value.clone();
    let mut rust_param_type: String = render_rust_type(param_type.clone());
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut param_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), tokens.clone());
    cur = cur_next_ref(cur.clone(), tokens.clone());
    let pos = cur.pos.clone();
    let mut td_indent_pos: i64 = pos + 1.clone();
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    let pos = cur.pos.clone();
    let mut body_start: i64 = pos.clone();
    let mut pred_r: ParseResult = gen_expr(tokens.clone(), body_start.clone(), ctx.clone());
    let code = pred_r.code;
    let new_pos = pred_r.new_pos;
    let pred_code = code;
    let pred_end = new_pos;
    let mut td_peek: TokenCursor = cur_at_ref(tokens.clone(), pred_end.clone());
    while !c_at_end(td_peek.clone()) {
        // transpiler-deor/codegen/decl/validator_type.deor
        let current = td_peek.current.clone();
        let kind = current.kind.clone();
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/validator_type.deor
            td_peek = cur_next_ref(td_peek.clone(), tokens.clone());
            continue;
        }
        break;
    }
    let mut td_is_single: bool = true;
    if !c_at_end(td_peek.clone()) {
        // transpiler-deor/codegen/decl/validator_type.deor
        let current = td_peek.current.clone();
        let kind = current.kind.clone();
        td_is_single = kind == "DEDENT";
    }
    let mut final_pred_code: String = pred_code.clone();
    let mut final_pos: i64 = 0;
    if td_is_single {
        // transpiler-deor/codegen/decl/validator_type.deor
        let mut td_scan: TokenCursor = cur_at_ref(tokens.clone(), pred_end.clone());
        while !c_at_end(td_scan.clone()) {
            // transpiler-deor/codegen/decl/validator_type.deor
            let current = td_scan.current.clone();
            let kind = current.kind.clone();
            td_scan = cur_next_ref(td_scan.clone(), tokens.clone());
            if kind == "DEDENT" {
                // transpiler-deor/codegen/decl/validator_type.deor
                break;
            }
        }
        let pos = td_scan.pos.clone();
        final_pos = pos;
    } else {
        // transpiler-deor/codegen/decl/validator_type.deor
        let mut body_end_pos: i64 = find_block_end_ref(tokens.clone(), td_indent_pos.clone());
        let mut body_slice_end: i64 = body_end_pos + 1.clone();
        let mut td_body_tokens: Vec<Token> = l_slice_ref(tokens.clone(), body_start.clone(), body_slice_end.clone());
        let mut td_zero: i64 = 0;
        let mut td_last: i64 = (td_body_tokens.len() as i64) - 1;
        let mut mut_names: Vec<String> = collect_mut_names(td_body_tokens.clone(), td_zero.clone(), td_last.clone());
        let mut tokens: TokensRef = tokens_wrap(td_body_tokens);
        let mut td_ctx_raw: GenCtx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, tokens, typed_enum_reg, typed_variant_reg };
        let mut td_pred_ctx: RcCtx = make_rctx(td_ctx_raw);
        let mut td_depth: i64 = 2;
        let mut td_block_r: ParseResult = gen_block(td_zero.clone(), td_depth.clone(), td_pred_ctx.clone());
        let code = td_block_r.code;
        let new_pos = td_block_r.new_pos;
        let td_block_code = code;
        let td_block_new_pos = new_pos;
        let mut td_clo_open: String = "(|| -> bool {\n".to_string();
        let mut td_clo_close: String = "        })()".to_string();
        final_pred_code = [td_clo_open.as_str(), td_block_code.as_str(), td_clo_close.as_str()].concat();
        final_pos = body_start + td_block_new_pos;
    }
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
    let mut impl_code: String = [imp_pfx.as_str(), type_name.as_str(), imp_fn.as_str(), param_name.as_str(), imp_col.as_str(), rust_param_type.as_str(), imp_ret.as_str(), final_pred_code.as_str(), imp_som.as_str(), type_name.as_str(), imp_inn.as_str(), param_name.as_str(), imp_sfx.as_str()].concat();
    let mut type_code: String = s_cat(struct_code, impl_code);
    return make_result(type_code, final_pos.clone());
}

// transpiler-deor/codegen/decl/function.deor
fn gen_fn_decl(fn_tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/function.deor
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut start_pos: i64 = pos + 1.clone();
    // macro: rust_strings (transpiler-deor/codegen/rust_strings.deor)
    let RS_IND: String = "    ".to_string();
    let RS_NL: String = "\n".to_string();
    let RS_SC: String = ";\n".to_string();
    let RS_OB: String = " {\n".to_string();
    let RS_CB: String = "}\n".to_string();
    let RS_CB2: String = "\n}\n\n".to_string();
    let RS_FNC_CB: String = "}\n\n".to_string();
    let RS_EQ: String = " = ".to_string();
    let RS_LET: String = "let ".to_string();
    let RS_LETM: String = "let mut ".to_string();
    let RS_COL: String = ": ".to_string();
    let RS_COM: String = ",".to_string();
    let RS_CSEP: String = ", ".to_string();
    let RS_LP: String = "(".to_string();
    let RS_RP: String = ")".to_string();
    let RS_RP_SC: String = ");\n".to_string();
    let RS_ARR: String = " -> ".to_string();
    let RS_OB_SP: String = " { ".to_string();
    let RS_CB_SC: String = " };\n".to_string();
    let RS_CLN: String = ".clone()".to_string();
    let RS_TOS: String = ".to_string()".to_string();
    // macro: fn_parse_signature (transpiler-deor/codegen/decl/macros/fn_parse_signature.deor)
    let mut cur: TokenCursor = cur_at_ref(fn_tokens.clone(), start_pos.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut ret_type: String = resolve_type(value.clone(), shape_reg.clone(), enum_reg.clone());
    cur = cur_next_ref(cur.clone(), fn_tokens.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut fn_name: String = value.clone();
    cur = cur_next_ref(cur.clone(), fn_tokens.clone());
    cur = cur_next_ref(cur.clone(), fn_tokens.clone());
    let mut param_strs: Vec<String> = Vec::new();
    while !c_at_end(cur.clone()) {
        // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
        let current = cur.current.clone();
        let kind = current.kind.clone();
        let value = current.value.clone();
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            break;
        } else if kind == "COMMA" {
            // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
        } else if kind == "IDENT" {
            // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
            let mut param_type: String = value.clone();
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            let current = cur.current.clone();
            let value = current.value.clone();
            let mut param_name: String = value.clone();
            let mut rust_param_type: String = resolve_type(param_type.clone(), shape_reg.clone(), enum_reg.clone());
            let mut prm_sep: String = ": ".to_string();
            param_strs.push([param_name.as_str(), prm_sep.as_str(), rust_param_type.as_str()].concat().clone());
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
        } else {
            // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
        }
    }
    let pos = cur.pos.clone();
    let mut indent_pos: i64 = pos + 1.clone();
    cur = cur_skip_to_body_ref(cur.clone(), fn_tokens.clone());
    let pos = cur.pos.clone();
    let mut body_start: i64 = pos.clone();
    // macro: fn_build_body_ctx (transpiler-deor/codegen/decl/macros/fn_build_body_ctx.deor)
    let typed_enum_reg = ctx.typed_enum_reg.clone();
    let typed_variant_reg = ctx.typed_variant_reg.clone();
    let mut body_end_pos: i64 = find_block_end_ref(fn_tokens.clone(), indent_pos.clone());
    let mut body_slice_end: i64 = body_end_pos + 1.clone();
    let mut body_tokens_raw: Vec<Token> = l_slice_ref(fn_tokens.clone(), body_start.clone(), body_slice_end.clone());
    let mut body_len: i64 = (body_tokens_raw.len() as i64);
    let mut zero: i64 = 0;
    let mut body_last: i64 = body_len - 1.clone();
    let mut mut_names: Vec<String> = collect_mut_names(body_tokens_raw.clone(), zero.clone(), body_last.clone());
    let mut tokens: TokensRef = tokens_wrap(body_tokens_raw);
    let mut body_ctx_raw: GenCtx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, tokens, typed_enum_reg, typed_variant_reg };
    let mut body_ctx: RcCtx = make_rctx(body_ctx_raw);
    // macro: fn_emit (transpiler-deor/codegen/decl/macros/fn_emit.deor)
    let mut body_pos: i64 = 0;
    let mut body_depth: i64 = 1;
    let mut body_r: ParseResult = gen_block(body_pos.clone(), body_depth.clone(), body_ctx);
    let code = body_r.code;
    let new_pos = body_r.new_pos;
    let body_code = code;
    let body_end = body_start + new_pos;
    let mut params_code: String = s_join_with(param_strs.clone(), RS_CSEP.clone());
    let mut ret_suffix: String = "".to_string();
    if !is_empty(ret_type.clone()) {
        // transpiler-deor/codegen/decl/macros/fn_emit.deor
        ret_suffix = [RS_ARR.as_str(), ret_type.as_str()].concat();
    }
    let mut fnc_kw: String = "fn ".to_string();
    let mut fn_code: String = [fnc_kw.as_str(), fn_name.as_str(), RS_LP.as_str(), params_code.as_str(), RS_RP.as_str(), ret_suffix.as_str(), RS_OB.as_str(), body_code.as_str(), RS_FNC_CB.as_str()].concat();
    return make_result(fn_code, body_end.clone());
}

// transpiler-deor/codegen/decl/raw.deor
fn gen_raw_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/raw.deor
    let mut name_pos: i64 = pos + 1.clone();
    let mut after: i64 = name_pos + 1.clone();
    let mut emp: String = "".to_string();
    return make_nl_result(emp, after.clone(), tokens.clone());
}

// transpiler-deor/codegen/codegen.deor
fn generate_rust_from_tokens(all_ref: TokensRef, ctx: RcCtx) -> String {
    // transpiler-deor/codegen/codegen.deor
    let mut parts: Vec<String> = Vec::new();
    let mut token_count: i64 = (all_ref.len() as i64);
    println!("{}", ["[diag] token_count: ", n_to_str(token_count.clone()).as_str()].concat());
    let mut pos: i64 = 0;
    let mut last_file: String = "".to_string();
    let mut _timer_label: String = "[timer]   codegen-loop: ".to_string();
    // macro: timer_start (transpiler-deor/macros/timer.deor)
    let mut _timer_start: i64 = now_ms();
    // transpiler-deor/codegen/codegen.deor
    loop {
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
            let mut block_pos: i64 = pos + 2.clone();
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
    // macro: timer_end (transpiler-deor/macros/timer.deor)
    let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
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
    let mut arg_count: i64 = (args.len() as i64);
    if arg_count < 2 {
        // transpiler-deor/main.deor
        println!("{}", "usage: deor input.deor output.rs".to_string());
    } else {
        // transpiler-deor/main.deor
        let mut input_path: String = args[0 as usize].clone();
        let mut output_path: String = args[1 as usize].clone();
        let mut _timer_label: String = "[timer] load+dedup: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let mut raw_tokens: Vec<Token> = collect_all_tokens_with_all_imports(input_path.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        _timer_label = "[timer] macro-build: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let mut tokens: Vec<Token> = build_macros(raw_tokens.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut tokens_ref: TokensRef = tokens_wrap(tokens);
        _timer_label = "[timer] validate: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        validate_tokens(tokens_ref.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        _timer_label = "[timer] registry: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let ctx = build_registry(tokens_ref.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        _timer_label = "[timer] total-codegen: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let mut rust_code: String = generate_rust_from_tokens(tokens_ref.clone(), ctx.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
        let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
        let mut _timer_sfx: String = "ms".to_string();
        println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        // transpiler-deor/main.deor
        let mut allow_warnings: String = "#![allow(warnings)]\n".to_string();
        rust_code = s_cat(allow_warnings.clone(), rust_code.clone());
        f_write(output_path.clone(), rust_code.clone());
    }
}

