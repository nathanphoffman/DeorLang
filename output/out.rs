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
struct Reg2Scan {
    matched: bool,
    key: String,
    val: String,
    new_pos: i64,
}

#[derive(Clone, PartialEq, Debug)]
struct Reg3Scan {
    matched: bool,
    key: String,
    val: String,
    val2: String,
    new_pos: i64,
}

#[derive(Clone, PartialEq, Debug)]
struct TokenMeta {
    line: i64,
    file: String,
}

#[derive(Clone, PartialEq, Debug)]
struct GenCtx {
    variant_reg: Vec<String>,
    shape_reg: Vec<String>,
    struct_reg: Vec<String>,
    enum_reg: Vec<String>,
    mut_names: Vec<String>,
    type_reg: Vec<String>,
    tokens: TokensRef,
    typed_enum_reg: Vec<String>,
    typed_variant_reg: Vec<String>,
    validator_var_reg: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
struct TokenCursor {
    token_count: i64,
    pos: i64,
    current: Token,
}

#[derive(Clone, PartialEq, Debug)]
struct DedupResult {
    tokens: Vec<Token>,
    enforce_macro_file_depth: i64,
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

// transpiler-deor/registry_lookup.deor
fn reg_get_stride(pairs: Vec<String>, key: String, stride: i64) -> String {
    // transpiler-deor/registry_lookup.deor
    let mut pairs_count: i64 = (pairs.len() as i64);
    let mut index: i64 = 0;
    while index < pairs_count {
        // transpiler-deor/registry_lookup.deor
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            // transpiler-deor/registry_lookup.deor
            let mut val_index: i64 = index + 1;
            return pairs[val_index as usize].clone();
        }
        index = index + stride;
    }
    return "".to_string();
}

fn reg_has_stride(pairs: Vec<String>, key: String, stride: i64) -> bool {
    // transpiler-deor/registry_lookup.deor
    let mut pairs_count: i64 = (pairs.len() as i64);
    let mut index: i64 = 0;
    while index < pairs_count {
        // transpiler-deor/registry_lookup.deor
        let mut current_key: String = pairs[index as usize].clone();
        if current_key == key {
            // transpiler-deor/registry_lookup.deor
            return true;
        }
        index = index + stride;
    }
    return false;
}

fn reg_get(pairs: Vec<String>, key: String) -> String {
    // transpiler-deor/registry_lookup.deor
    let mut two: i64 = 2;
    return reg_get_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg_has(pairs: Vec<String>, key: String) -> bool {
    // transpiler-deor/registry_lookup.deor
    let mut two: i64 = 2;
    return reg_has_stride(pairs.clone(), key.clone(), two.clone());
}

fn reg3_has(pairs: Vec<String>, key: String) -> bool {
    // transpiler-deor/registry_lookup.deor
    let mut thr: i64 = 3;
    return reg_has_stride(pairs.clone(), key.clone(), thr.clone());
}

// transpiler-deor/global_flags.deor
// float literal context flag
thread_local! {
	static FLOAT_CTX: std::cell::Cell<bool> = std::cell::Cell::new(false);
}
fn _float_ctx_get() -> bool { FLOAT_CTX.with(|f| f.get()) }
fn _float_ctx_set(v: bool) { FLOAT_CTX.with(|f| f.set(v)); }

// verbose diagnostics flag — off by default, turned on by main() when
// --verbose/-v is passed on the command line
thread_local! {
	static VERBOSE: std::cell::Cell<bool> = std::cell::Cell::new(false);
}
fn _verbose_get() -> bool { VERBOSE.with(|f| f.get()) }
fn _verbose_set(v: bool) { VERBOSE.with(|f| f.set(v)); }
fn float_ctx_get() -> bool {
    // transpiler-deor/global_flags.deor
    _float_ctx_get()
}

fn float_ctx_enable() {
    // transpiler-deor/global_flags.deor
    _float_ctx_set(true)
}

fn float_ctx_disable() {
    // transpiler-deor/global_flags.deor
    _float_ctx_set(false)
}

fn verbose_get() -> bool {
    // transpiler-deor/global_flags.deor
    _verbose_get()
}

fn verbose_enable() {
    // transpiler-deor/global_flags.deor
    _verbose_set(true)
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
    let mut str_start: i64 = char_index + 1;
    let mut new_pos: i64 = char_index + 1;
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
    let mut num_start: i64 = char_index + 1;
    let mut new_pos: i64 = char_index + 1;
    for number_index in num_start..char_count {
        // transpiler-deor/importer/lexer/number_literal.deor
        let mut number_char: String = chars[number_index as usize].clone();
        if c_digit(number_char.clone()) {
            // transpiler-deor/importer/lexer/number_literal.deor
            num = s_cat(num.clone(), number_char.clone());
            new_pos = number_index + 1;
        } else if number_char == "_" {
            // transpiler-deor/importer/lexer/number_literal.deor
            let mut peek_idx: i64 = number_index + 1;
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
        let mut frac_start: i64 = new_pos + 1;
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
    if word == "unsafe_macro_run" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_UNSAFE_MACRO_RUN".to_string();
    }
    if word == "unsafe_macro" {
        // transpiler-deor/importer/lexer/word_token.deor
        return "KW_UNSAFE_MACRO".to_string();
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
    let mut word_start: i64 = char_index + 1;
    let mut new_pos: i64 = char_index + 1;
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
    let mut line_count: i64 = (lines.len() as i64);
    let mut indent_stack: Vec<String> = Vec::new();
    let mut zero_str: String = "0".to_string();
    indent_stack.push(zero_str.clone());
    let mut cur_line: i64 = 0;
    let mut skip: i64 = 0;
    for line_index in 0..line_count {
        // transpiler-deor/importer/lexer/tokenizer.deor
        cur_line = cur_line + 1;
        let mut meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
        if skip > 0 {
            // transpiler-deor/importer/lexer/tokenizer.deor
            skip = skip - 1;
            continue;
        }
        let mut raw_line: String = lines[line_index as usize].clone();
        let mut line: String = s_rtrim(raw_line.clone());
        let mut content: String = s_trim(line.clone());
        if is_empty(content.clone()) {
            // transpiler-deor/importer/lexer/tokenizer.deor
            continue;
        }
        let mut indent: i64 = count_tabs(line.clone());
        // macro: emit_indent_or_dedent (transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor)
        {
            // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
            let mut kind_indent: String = "INDENT".to_string();
            let mut kind_dedent: String = "DEDENT".to_string();
            let mut empty_str: String = "".to_string();
            let mut stack_len: i64 = (indent_stack.len() as i64);
            let mut top_idx: i64 = stack_len - 1;
            let mut top: i64 = n_parse(indent_stack[top_idx as usize].clone());
            if indent > top {
                // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                tokens.push(make_token(kind_indent.clone(), empty_str.clone(), meta.clone()).clone());
                let mut indent_str: String = n_to_str(indent.clone());
                indent_stack.push(indent_str.clone());
            } else {
                // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                let mut dedenting: bool = indent < top;
                while dedenting {
                    // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                    let mut new_stack_len: i64 = (indent_stack.len() as i64);
                    let mut new_top_idx: i64 = new_stack_len - 1;
                    let mut cur_top: i64 = n_parse(indent_stack[new_top_idx as usize].clone());
                    if indent < cur_top {
                        // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                        tokens.push(make_token(kind_dedent.clone(), empty_str.clone(), meta.clone()).clone());
                        indent_stack.remove(new_top_idx as usize);
                    } else {
                        // transpiler-deor/importer/lexer/macros/emit_indent_or_dedent.deor
                        dedenting = false;
                    }
                }
            }
        }
        // transpiler-deor/importer/lexer/tokenizer.deor
        if content == "rust" {
            // macro: collect_rust_block (transpiler-deor/importer/lexer/macros/collect_rust_block.deor)
            {
                // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                let mut kind_kw_rust: String = "KW_RUST".to_string();
                let mut kw_rust_val: String = "rust".to_string();
                let mut kind_newline: String = "NEWLINE".to_string();
                let mut kind_rust_block: String = "RUST_BLOCK".to_string();
                let mut empty_str: String = "".to_string();
                tokens.push(make_token(kind_kw_rust.clone(), kw_rust_val.clone(), meta.clone()).clone());
                tokens.push(make_token(kind_newline.clone(), empty_str.clone(), meta.clone()).clone());
                let mut rust_base: i64 = indent + 1;
                let mut rust_lines: Vec<String> = Vec::new();
                let mut scan_start: i64 = line_index + 1;
                for scan_line in scan_start..line_count {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    let mut rust_line: String = lines[scan_line as usize].clone();
                    let mut stripped: String = s_trim(rust_line.clone());
                    if is_empty(stripped.clone()) {
                        // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                        rust_lines.push("".to_string());
                        skip = skip + 1;
                    } else {
                        // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                        let mut line_indent: i64 = count_tabs(rust_line.clone());
                        if line_indent >= rust_base {
                            // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                            let mut content: String = s_from(rust_line.clone(), rust_base.clone());
                            rust_lines.push(s_rtrim(content.clone()).clone());
                            skip = skip + 1;
                        } else {
                            // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                            break;
                        }
                    }
                }
                let mut trimming: bool = true;
                while trimming {
                    // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                    let mut lines_len: i64 = (rust_lines.len() as i64);
                    if lines_len > 0 {
                        // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                        let mut last_index: i64 = lines_len - 1;
                        let mut last_line: String = rust_lines[last_index as usize].clone();
                        if last_line == "" {
                            // transpiler-deor/importer/lexer/macros/collect_rust_block.deor
                            rust_lines.remove(last_index as usize);
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
                tokens.push(make_token(kind_rust_block.clone(), block_content.clone(), meta.clone()).clone());
            }
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
                let mut string_result: ParseResult = scan_string_literal(chars.clone(), char_index.clone(), char_count.clone());
                let mut kind_string: String = "STRING".to_string();
                let mut str_val: String = pr_code(string_result.clone());
                tokens.push(make_token(kind_string.clone(), str_val.clone(), meta.clone()).clone());
                char_index = pr_pos(string_result.clone());
                continue;
            }
            if c_digit(character.clone()) {
                // transpiler-deor/importer/lexer/tokenizer.deor
                let mut number_result: ParseResult = scan_number(chars.clone(), char_index.clone(), char_count.clone());
                let mut num_str: String = pr_code(number_result.clone());
                char_index = pr_pos(number_result.clone());
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
                let mut word_result: ParseResult = scan_word(chars.clone(), char_index.clone(), char_count.clone());
                let mut word: String = pr_code(word_result.clone());
                char_index = pr_pos(word_result.clone());
                let mut word_kind: String = word_to_kind(word.clone());
                tokens.push(make_token(word_kind.clone(), word.clone(), meta.clone()).clone());
                continue;
            }
            // macro: emit_operator_token (transpiler-deor/importer/lexer/macros/emit_operator_token.deor)
            {
                // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
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
                let mut op_invalid_chars: Vec<String> = vec!["&".to_string(), "|".to_string(), "^".to_string(), "!".to_string(), "{".to_string(), "}".to_string()];
                let mut op_peek_idx: i64 = char_index + 1;
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
                } else if list_has(op_invalid_chars.clone(), character.clone()) {
                    // transpiler-deor/importer/lexer/macros/emit_operator_token.deor
                    let mut op_kind_inv: String = "INVALID".to_string();
                    tokens.push(make_token(op_kind_inv.clone(), character.clone(), meta.clone()).clone());
                }
                char_index = char_index + 1;
            }
        }
        tokens.push(make_token(kind_newline.clone(), empty_str.clone(), meta.clone()).clone());
    }
    let mut final_stack_len: i64 = (indent_stack.len() as i64);
    let mut tail_meta: TokenMeta = make_meta(cur_line.clone(), path.clone());
    let mut dedent_start: i64 = 1;
    for _ in dedent_start..final_stack_len {
        // transpiler-deor/importer/lexer/tokenizer.deor
        tokens.push(make_token(kind_dedent.clone(), empty_str.clone(), tail_meta.clone()).clone());
    }
    tokens.push(make_token(kind_eof.clone(), empty_str.clone(), tail_meta.clone()).clone());
    return tokens;
}

// transpiler-deor/importer/scan.deor
fn scan_import_new(tokens: Vec<Token>, pos: i64) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    fn locate_path(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_after_path(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    let mut path_pos: i64 = locate_path(pos.clone());
    let mut token_count: i64 = (tokens.len() as i64);
    if path_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut path_token: Token = tokens[path_pos as usize].clone();
        let kind = path_token.kind.clone();
        let value = path_token.value.clone();
        if kind == "STRING" {
            // transpiler-deor/importer/scan.deor
            let mut after_path: i64 = locate_after_path(pos.clone());
            return make_result(value.clone(), after_path.clone());
        }
    }
    let mut empty_str: String = "".to_string();
    return make_result(empty_str.clone(), pos.clone());
}

fn scan_import_where(tokens: Vec<Token>, pos: i64) -> ParseResult {
    // transpiler-deor/importer/scan.deor
    fn locate_replacement(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_equals(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    fn locate_concrete_type(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
    fn locate_after_where(kw_pos: i64) -> i64 {
        return kw_pos + 4;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut where_pos: i64 = pos.clone();
    let mut replacement_pos: i64 = locate_replacement(pos.clone());
    let mut eq_pos: i64 = locate_equals(pos.clone());
    let mut concrete_pos: i64 = locate_concrete_type(pos.clone());
    if concrete_pos < token_count {
        // transpiler-deor/importer/scan.deor
        let mut where_token: Token = tokens[where_pos as usize].clone();
        let kind = where_token.kind.clone();
        let value = where_token.value.clone();
        let mut is_where: bool = kind == "IDENT" && value == "where";
        if is_where {
            // transpiler-deor/importer/scan.deor
            let mut eq_token: Token = tokens[eq_pos as usize].clone();
            let mut concrete_token: Token = tokens[concrete_pos as usize].clone();
            let mut replacement_token: Token = tokens[replacement_pos as usize].clone();
            let kind = eq_token.kind.clone();
            let mut is_eq: bool = kind == "EQUALS";
            if is_eq {
                // transpiler-deor/importer/scan.deor
                let value = replacement_token.value.clone();
                let replacement_value = value.clone();
                let value = concrete_token.value.clone();
                let mut after_where: i64 = locate_after_where(pos.clone());
                let replace_with = [replacement_value.as_str(), "|", value.as_str()].concat();
                return make_result(replace_with.clone(), after_where.clone());
            }
        }
    }
    let mut empty_str: String = "".to_string();
    return make_result(empty_str.clone(), pos.clone());
}

// transpiler-deor/importer/t_substitute.deor
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
    let mut pascal_placeholder: String = s_pascal(placeholder.clone());
    let mut camel_placeholder: String = s_camel(placeholder.clone());
    let mut placeholder_len: i64 = (placeholder.len() as i64);
    let mut name_len: i64 = (name.len() as i64);
    if name_len > placeholder_len {
        // transpiler-deor/importer/t_substitute.deor
        let mut after_placeholder: String = s_from(name.clone(), placeholder_len.clone());
        let mut after_chars: Vec<String> = c_chars(after_placeholder.clone());
        if (after_chars.len() as i64) > 0 {
            // transpiler-deor/importer/t_substitute.deor
            let mut next_char: String = after_chars[0 as usize].clone();
            let mut next_is_upper: bool = s_upper_char(next_char.clone());
            let mut starts_pascal: bool = s_starts_with(name.clone(), pascal_placeholder.clone());
            if starts_pascal && next_is_upper {
                // transpiler-deor/importer/t_substitute.deor
                let mut pascal_concrete: String = s_pascal(concrete.clone());
                return s_cat(pascal_concrete.clone(), after_placeholder.clone());
            }
            let mut starts_camel: bool = s_starts_with(name.clone(), camel_placeholder.clone());
            if starts_camel && next_is_upper {
                // transpiler-deor/importer/t_substitute.deor
                let mut camel_concrete: String = s_camel(concrete.clone());
                return s_cat(camel_concrete.clone(), after_placeholder.clone());
            }
        }
    }
    let mut pascal_sep: String = ["_", pascal_placeholder.as_str(), "_"].concat();
    let mut camel_sep: String = ["_", camel_placeholder.as_str(), "_"].concat();
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
    		let pascal_placeholder = pascal_str(placeholder);
    		let camel_placeholder = camel_str(placeholder);
    		let pascal_c = pascal_str(concrete);
    		let camel_c = camel_str(concrete);
    		let ph_len = placeholder.len();
    		if word.len() > ph_len {
    			if word.starts_with(&pascal_placeholder) {
    				let rest = &word[ph_len..];
    				if rest.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
    					return format!("{}{}", pascal_c, rest);
    				}
    			}
    			if word.starts_with(&camel_placeholder) {
    				let rest = &word[ph_len..];
    				if rest.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
    					return format!("{}{}", camel_c, rest);
    				}
    			}
    		}
    		let pascal_sep = format!("_{}_", pascal_placeholder);
    		let camel_sep = format!("_{}_", camel_placeholder);
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
fn fatal(msg: String) {
    // transpiler-deor/importer/load.deor
    println!("{}", msg.clone());
    std::process::exit(1);
}

fn fatal_at(path: String, msg: String) {
    // transpiler-deor/importer/load.deor
    let mut err_pre: String = "[error] ".to_string();
    let mut err_sep: String = ": ".to_string();
    let mut err_parts: Vec<String> = vec![err_pre.clone(), path.clone(), err_sep.clone(), msg.clone()];
    fatal(s_join(err_parts.clone()));
}

fn fatal_missing_import(imp_path: String) {
    // transpiler-deor/importer/load.deor
    let mut err_pre: String = "[error] cannot find import: ".to_string();
    fatal(s_cat(err_pre.clone(), imp_path.clone()));
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
    let mut decl_phase: i64 = 0;
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
        let mut at_root_depth: bool = depth == 0;
        if at_root_depth {
            // transpiler-deor/importer/load.deor
            let mut is_structural_kw: bool = kind == "KW_STRUCT" || kind == "KW_ENUM" || kind == "KW_SHAPE" || kind == "KW_TYPE";
            if is_structural_kw {
                // transpiler-deor/importer/load.deor
                if decl_phase > 0 {
                    // transpiler-deor/importer/load.deor
                    let mut struct_order_msg: String = "struct/enum/type/shape declarations must come before macros and functions".to_string();
                    fatal_at(path.clone(), struct_order_msg.clone());
                }
            }
            if kind == "KW_MACRO" || kind == "KW_UNSAFE_MACRO" {
                // transpiler-deor/importer/load.deor
                if decl_phase == 2 {
                    // transpiler-deor/importer/load.deor
                    let mut macro_order_msg: String = "macros must be declared before functions".to_string();
                    fatal_at(path.clone(), macro_order_msg.clone());
                } else if decl_phase < 1 {
                    // transpiler-deor/importer/load.deor
                    decl_phase = 1;
                }
            }
            if kind == "KW_FN" {
                // transpiler-deor/importer/load.deor
                decl_phase = 2;
            }
        }
        let mut is_new_import: bool = kind == "KW_IMPORT" && at_root_depth;
        if is_new_import {
            // transpiler-deor/importer/load.deor
            let mut import_result: ParseResult = scan_import_new(tok_raw.clone(), pos.clone());
            let mut import_path: String = pr_code(import_result.clone());
            let mut import_end: i64 = pr_pos(import_result.clone());
            let mut import_type_concrete: String = "".to_string();
            let mut import_type_placeholder: String = "".to_string();
            let mut where_result: ParseResult = scan_import_where(tok_raw.clone(), import_end.clone());
            let import_type_code = pr_code(where_result.clone());
            if !is_empty(import_type_code.clone()) {
                // transpiler-deor/importer/load.deor
                import_end = pr_pos(where_result.clone());
                let PIPE: String = "|".to_string();
                let list_code = s_split(import_type_code.clone(), PIPE.clone());
                import_type_placeholder = list_code[0 as usize].clone();
                import_type_concrete = list_code[1 as usize].clone();
            }
            import_path = resolve_lib_path(import_path.clone());
            if !is_empty(import_path.clone()) {
                // transpiler-deor/importer/load.deor
                if seen_decl {
                    // transpiler-deor/importer/load.deor
                    let mut import_order_msg: String = "imports must appear at the top of the file before any declarations".to_string();
                    fatal_at(path.clone(), import_order_msg.clone());
                }
                let mut dedup_key: String = import_path.clone();
                if !is_empty(import_type_concrete.clone()) {
                    // transpiler-deor/importer/load.deor
                    dedup_key = [import_path.as_str(), "|", import_type_placeholder.as_str(), "=", import_type_concrete.as_str()].concat();
                }
                let mut is_new: bool = file_is_new_keyed(dedup_key.clone());
                if is_new {
                    // transpiler-deor/importer/load.deor
                    let mut exists: bool = f_exists(import_path.clone());
                    if !exists {
                        // transpiler-deor/importer/load.deor
                        fatal_missing_import(import_path.clone());
                    }
                    let mut import_tokens: Vec<Token> = load_file(import_path.clone());
                    if !is_empty(import_type_concrete.clone()) {
                        // transpiler-deor/importer/load.deor
                        import_tokens = apply_t_substitution(import_tokens.clone(), import_type_placeholder.clone(), import_type_concrete.clone());
                    }
                    let mut import_len: i64 = (import_tokens.len() as i64);
                    for import_index in 0..import_len {
                        // transpiler-deor/importer/load.deor
                        let mut import_token: Token = import_tokens[import_index as usize].clone();
                        let kind = import_token.kind.clone();
                        let mut import_is_eof: bool = kind == "EOF";
                        if !import_is_eof {
                            // transpiler-deor/importer/load.deor
                            result.push(import_token.clone());
                        }
                    }
                }
                pos = import_end;
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

// transpiler-deor/importer/macros/dedup/dd_error_duplicate.deor
fn dd_error_duplicate(decl_name: String, name_token: Token, origin_file: String) {
    // transpiler-deor/importer/macros/dedup/dd_error_duplicate.deor
    let file = name_token.file.clone();
    let line = name_token.line.clone();
    let mut line_str: String = n_to_str(line.clone());
    let mut error_prefix: String = "[error] ".to_string();
    let mut error_line: String = " line ".to_string();
    let mut error_mid: String = ": duplicate declaration '".to_string();
    let mut error_end: String = "' — already declared in ".to_string();
    let mut error_parts: Vec<String> = vec![error_prefix.clone(), file.clone(), error_line.clone(), line_str.clone(), error_mid.clone(), decl_name.clone(), error_end.clone(), origin_file.clone()];
    let mut error_msg: String = s_join(error_parts.clone());
    println!("{}", error_msg.clone());
    std::process::exit(1);
}

// transpiler-deor/importer/dedup.deor
fn deduplicate_decls(tokens_in: Vec<Token>) -> DedupResult {
    // transpiler-deor/importer/dedup.deor
    let mut tokens: Vec<Token> = tokens_in.clone();
    // macro: strip_enforce_pragmas (transpiler-deor/importer/macros/strip_enforce_pragmas.deor)
    let mut enforce_unique_file: bool = false;
    let mut enforce_unique_import: bool = false;
    let mut enforce_macro_file_depth: i64 = 0;
    let mut result_tokens: Vec<Token> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut scan_pos: i64 = 0;
    while scan_pos < token_count {
        // transpiler-deor/importer/macros/strip_enforce_pragmas.deor
        let mut scan_token: Token = tokens[scan_pos as usize].clone();
        let kind = scan_token.kind.clone();
        let mut is_main: bool = false;
        if kind == "KW_FN" {
            // macro: sep_check_main_signature (transpiler-deor/importer/macros/sep_check_main_signature.deor)
            {
                // transpiler-deor/importer/macros/sep_check_main_signature.deor
                fn locate_void(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_name(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_left_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                fn locate_right_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 4;
                }
                let mut void_pos: i64 = locate_void(scan_pos.clone());
                let mut name_pos: i64 = locate_name(scan_pos.clone());
                let mut left_paren_pos: i64 = locate_left_paren(scan_pos.clone());
                let mut right_paren_pos: i64 = locate_right_paren(scan_pos.clone());
                if right_paren_pos < token_count {
                    // transpiler-deor/importer/macros/sep_check_main_signature.deor
                    let mut void_token: Token = tokens[void_pos as usize].clone();
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                    let mut right_paren_token: Token = tokens[right_paren_pos as usize].clone();
                    let kind = void_token.kind.clone();
                    let mut is_void: bool = kind == "KW_VOID";
                    let kind = name_token.kind.clone();
                    let value = name_token.value.clone();
                    let mut is_main_name: bool = kind == "IDENT" && value == "main";
                    let kind = left_paren_token.kind.clone();
                    let mut is_lparen: bool = kind == "LPAREN";
                    let kind = right_paren_token.kind.clone();
                    let mut is_rparen: bool = kind == "RPAREN";
                    is_main = is_void && is_main_name && is_lparen && is_rparen;
                }
            }
        }
        if is_main {
            // transpiler-deor/importer/macros/strip_enforce_pragmas.deor
            let mut opener_ok: bool = false;
            // macro: sep_copy_main_opener (transpiler-deor/importer/macros/sep_copy_main_opener.deor)
            {
                // transpiler-deor/importer/macros/sep_copy_main_opener.deor
                let mut copy_index: i64 = 0;
                while copy_index < 5 {
                    // transpiler-deor/importer/macros/sep_copy_main_opener.deor
                    let mut copy_pos: i64 = scan_pos + copy_index;
                    let mut copy_token: Token = tokens[copy_pos as usize].clone();
                    result_tokens.push(copy_token.clone());
                    copy_index = copy_index + 1;
                }
                scan_pos = scan_pos + 5;
                let mut newline_present: bool = false;
                if scan_pos < token_count {
                    // transpiler-deor/importer/macros/sep_copy_main_opener.deor
                    let mut newline_token: Token = tokens[scan_pos as usize].clone();
                    let kind = newline_token.kind.clone();
                    newline_present = kind == "NEWLINE";
                }
                if newline_present {
                    // transpiler-deor/importer/macros/sep_copy_main_opener.deor
                    let mut newline_token2: Token = tokens[scan_pos as usize].clone();
                    result_tokens.push(newline_token2.clone());
                    scan_pos = scan_pos + 1;
                    let mut indent_present: bool = false;
                    if scan_pos < token_count {
                        // transpiler-deor/importer/macros/sep_copy_main_opener.deor
                        let mut indent_token: Token = tokens[scan_pos as usize].clone();
                        let kind = indent_token.kind.clone();
                        indent_present = kind == "INDENT";
                    }
                    if indent_present {
                        // transpiler-deor/importer/macros/sep_copy_main_opener.deor
                        let mut indent_token2: Token = tokens[scan_pos as usize].clone();
                        result_tokens.push(indent_token2.clone());
                        scan_pos = scan_pos + 1;
                        opener_ok = true;
                    }
                }
            }
            // transpiler-deor/importer/macros/strip_enforce_pragmas.deor
            if opener_ok {
                // macro: sep_scan_pragma_lines (transpiler-deor/importer/macros/sep_scan_pragma_lines.deor)
                {
                    // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                    fn locate_next_token(anchor: i64) -> i64 {
                        return anchor + 1;
                    }
                    let mut scanning: bool = true;
                    while scanning {
                        // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                        scanning = false;
                        if scan_pos < token_count {
                            // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                            let mut pragma_token: Token = tokens[scan_pos as usize].clone();
                            let kind = pragma_token.kind.clone();
                            let value = pragma_token.value.clone();
                            let mut is_file_flag: bool = kind == "IDENT" && value == "ENFORCE_UNIQUE_FILE_DECLARATIONS";
                            let mut is_import_flag: bool = kind == "IDENT" && value == "ENFORCE_UNIQUE_IMPORT_DECLARATIONS";
                            let mut is_depth_flag: bool = kind == "IDENT" && value == "ENFORCE_MACRO_FILE_DEPTH";
                            if is_file_flag || is_import_flag {
                                // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                let mut newline_pos: i64 = locate_next_token(scan_pos.clone());
                                let mut newline_ok: bool = false;
                                if newline_pos < token_count {
                                    // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                    let mut pragma_newline_token: Token = tokens[newline_pos as usize].clone();
                                    let kind = pragma_newline_token.kind.clone();
                                    newline_ok = kind == "NEWLINE";
                                }
                                if newline_ok {
                                    // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                    if is_file_flag {
                                        // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                        enforce_unique_file = true;
                                    } else {
                                        // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                        enforce_unique_import = true;
                                    }
                                    scan_pos = scan_pos + 2;
                                    scanning = true;
                                }
                            } else if is_depth_flag {
                                // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                let mut equals_pos: i64 = scan_pos + 1;
                                let mut value_pos: i64 = scan_pos + 2;
                                let mut depth_newline_pos: i64 = scan_pos + 3;
                                let mut depth_ok: bool = false;
                                let mut depth_str: String = "".to_string();
                                if depth_newline_pos < token_count {
                                    // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                    let mut equals_token: Token = tokens[equals_pos as usize].clone();
                                    let kind = equals_token.kind.clone();
                                    let mut eq_ok: bool = kind == "EQUALS";
                                    let mut value_token: Token = tokens[value_pos as usize].clone();
                                    let kind = value_token.kind.clone();
                                    let value = value_token.value.clone();
                                    let mut val_ok: bool = kind == "INT";
                                    depth_str = value;
                                    let mut depth_newline_token: Token = tokens[depth_newline_pos as usize].clone();
                                    let kind = depth_newline_token.kind.clone();
                                    let mut depth_newline_ok: bool = kind == "NEWLINE";
                                    depth_ok = eq_ok && val_ok && depth_newline_ok;
                                }
                                if depth_ok {
                                    // transpiler-deor/importer/macros/sep_scan_pragma_lines.deor
                                    enforce_macro_file_depth = n_parse(depth_str.clone());
                                    scan_pos = scan_pos + 4;
                                    scanning = true;
                                }
                            }
                        }
                    }
                }
            }
            for i in (scan_pos as usize)..(token_count as usize) {
            	result_tokens.push(tokens[i].clone());
            }
            scan_pos = token_count;
            break;
        } else {
            // transpiler-deor/importer/macros/strip_enforce_pragmas.deor
            result_tokens.push(scan_token.clone());
            scan_pos = scan_pos + 1;
        }
    }
    tokens = result_tokens;
    // transpiler-deor/importer/dedup.deor
    let mut result: Vec<Token> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut seen_files: Vec<String> = Vec::new();
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
        let mut is_fn: bool = kind == "KW_FN";
        let mut is_struct: bool = kind == "KW_STRUCT";
        let mut is_enum: bool = kind == "KW_ENUM";
        let mut is_shape: bool = kind == "KW_SHAPE";
        let mut is_type: bool = kind == "KW_TYPE";
        let mut is_macro: bool = kind == "KW_MACRO";
        let mut is_unsafe_macro: bool = kind == "KW_UNSAFE_MACRO";
        let mut is_raw: bool = kind == "KW_RAW";
        let mut is_rust_blk: bool = kind == "KW_RUST";
        let mut is_block_decl: bool = is_fn || is_struct || is_enum || is_type || is_macro || is_unsafe_macro;
        if is_block_decl {
            // macro: dd_handle_block_decl (transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor)
            {
                // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                let mut key_prefix: String = "".to_string();
                // macro: dd_check_duplicate (transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor)
                let mut name_offset: i64 = 1;
                if is_fn {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    name_offset = 2;
                }
                let mut name_pos: i64 = pos + name_offset;
                let mut decl_name: String = "".to_string();
                let mut name_token: Token = tokens[pos as usize].clone();
                if name_pos < token_count {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    name_token = tokens[name_pos as usize].clone();
                    let value = name_token.value.clone();
                    decl_name = value;
                }
                let mut lookup_name: String = [key_prefix.as_str(), decl_name.as_str()].concat();
                let mut already_seen: bool = false;
                let mut dup_index: i64 = 0;
                let mut seen_len: i64 = (seen.len() as i64);
                for seen_index in 0..seen_len {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    let mut seen_val: String = seen[seen_index as usize].clone();
                    if seen_val == lookup_name {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        already_seen = true;
                        dup_index = seen_index;
                        break;
                    }
                }
                if already_seen {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    let mut origin_file: String = seen_files[dup_index as usize].clone();
                    let file = name_token.file.clone();
                    let mut is_same_file: bool = origin_file == file;
                    let mut should_error: bool = false;
                    if is_same_file {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        should_error = enforce_unique_file;
                    } else {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        should_error = enforce_unique_import;
                    }
                    if should_error {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        dd_error_duplicate(decl_name.clone(), name_token.clone(), origin_file.clone());
                    }
                } else {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    seen.push(lookup_name.clone());
                    let file = name_token.file.clone();
                    seen_files.push(file.clone());
                }
                // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                let mut scan_pos: i64 = pos.clone();
                let mut depth: i64 = 0;
                let mut entered: bool = false;
                while scan_pos < token_count {
                    // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let kind = scan_token.kind.clone();
                    scan_pos = scan_pos + 1;
                    if kind == "INDENT" {
                        // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                        depth = depth + 1;
                        entered = true;
                    } else if kind == "DEDENT" {
                        // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                        depth = depth - 1;
                        if depth == 0 && entered {
                            // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                            break;
                        }
                    }
                }
                let mut end_pos: i64 = scan_pos.clone();
                if !already_seen {
                    // transpiler-deor/importer/macros/dedup/dd_handle_block_decl.deor
                    for i in (pos as usize)..(end_pos as usize) {
                    	result.push(tokens[i].clone());
                    }
                }
                pos = end_pos;
            }
        } else if is_shape {
            // macro: dd_handle_shape (transpiler-deor/importer/macros/dedup/dd_handle_shape.deor)
            {
                // transpiler-deor/importer/macros/dedup/dd_handle_shape.deor
                let mut key_prefix: String = "".to_string();
                // macro: dd_check_duplicate (transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor)
                let mut name_offset: i64 = 1;
                if is_fn {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    name_offset = 2;
                }
                let mut name_pos: i64 = pos + name_offset;
                let mut decl_name: String = "".to_string();
                let mut name_token: Token = tokens[pos as usize].clone();
                if name_pos < token_count {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    name_token = tokens[name_pos as usize].clone();
                    let value = name_token.value.clone();
                    decl_name = value;
                }
                let mut lookup_name: String = [key_prefix.as_str(), decl_name.as_str()].concat();
                let mut already_seen: bool = false;
                let mut dup_index: i64 = 0;
                let mut seen_len: i64 = (seen.len() as i64);
                for seen_index in 0..seen_len {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    let mut seen_val: String = seen[seen_index as usize].clone();
                    if seen_val == lookup_name {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        already_seen = true;
                        dup_index = seen_index;
                        break;
                    }
                }
                if already_seen {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    let mut origin_file: String = seen_files[dup_index as usize].clone();
                    let file = name_token.file.clone();
                    let mut is_same_file: bool = origin_file == file;
                    let mut should_error: bool = false;
                    if is_same_file {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        should_error = enforce_unique_file;
                    } else {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        should_error = enforce_unique_import;
                    }
                    if should_error {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        dd_error_duplicate(decl_name.clone(), name_token.clone(), origin_file.clone());
                    }
                } else {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    seen.push(lookup_name.clone());
                    let file = name_token.file.clone();
                    seen_files.push(file.clone());
                }
                // macro: dd_copy_single_line_decl (transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor)
                {
                    // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                    let mut scan_pos: i64 = pos.clone();
                    while scan_pos < token_count {
                        // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                        let kind = scan_token.kind.clone();
                        scan_pos = scan_pos + 1;
                        if kind == "NEWLINE" {
                            // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                            break;
                        }
                    }
                    let mut end_pos: i64 = scan_pos.clone();
                    if !already_seen {
                        // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                        let mut copy_len: i64 = end_pos - pos;
                        for copy_idx in 0..copy_len {
                            // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                            let mut copy_pos: i64 = pos + copy_idx;
                            let mut copy_token: Token = tokens[copy_pos as usize].clone();
                            result.push(copy_token.clone());
                        }
                    }
                    pos = end_pos;
                }
            }
        } else if is_raw {
            // macro: dd_handle_raw (transpiler-deor/importer/macros/dedup/dd_handle_raw.deor)
            {
                // transpiler-deor/importer/macros/dedup/dd_handle_raw.deor
                let mut key_prefix: String = "_raw_".to_string();
                // macro: dd_check_duplicate (transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor)
                let mut name_offset: i64 = 1;
                if is_fn {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    name_offset = 2;
                }
                let mut name_pos: i64 = pos + name_offset;
                let mut decl_name: String = "".to_string();
                let mut name_token: Token = tokens[pos as usize].clone();
                if name_pos < token_count {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    name_token = tokens[name_pos as usize].clone();
                    let value = name_token.value.clone();
                    decl_name = value;
                }
                let mut lookup_name: String = [key_prefix.as_str(), decl_name.as_str()].concat();
                let mut already_seen: bool = false;
                let mut dup_index: i64 = 0;
                let mut seen_len: i64 = (seen.len() as i64);
                for seen_index in 0..seen_len {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    let mut seen_val: String = seen[seen_index as usize].clone();
                    if seen_val == lookup_name {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        already_seen = true;
                        dup_index = seen_index;
                        break;
                    }
                }
                if already_seen {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    let mut origin_file: String = seen_files[dup_index as usize].clone();
                    let file = name_token.file.clone();
                    let mut is_same_file: bool = origin_file == file;
                    let mut should_error: bool = false;
                    if is_same_file {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        should_error = enforce_unique_file;
                    } else {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        should_error = enforce_unique_import;
                    }
                    if should_error {
                        // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                        dd_error_duplicate(decl_name.clone(), name_token.clone(), origin_file.clone());
                    }
                } else {
                    // transpiler-deor/importer/macros/dedup/dd_check_duplicate.deor
                    seen.push(lookup_name.clone());
                    let file = name_token.file.clone();
                    seen_files.push(file.clone());
                }
                // macro: dd_copy_single_line_decl (transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor)
                {
                    // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                    let mut scan_pos: i64 = pos.clone();
                    while scan_pos < token_count {
                        // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                        let kind = scan_token.kind.clone();
                        scan_pos = scan_pos + 1;
                        if kind == "NEWLINE" {
                            // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                            break;
                        }
                    }
                    let mut end_pos: i64 = scan_pos.clone();
                    if !already_seen {
                        // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                        let mut copy_len: i64 = end_pos - pos;
                        for copy_idx in 0..copy_len {
                            // transpiler-deor/importer/macros/dedup/dd_copy_single_line_decl.deor
                            let mut copy_pos: i64 = pos + copy_idx;
                            let mut copy_token: Token = tokens[copy_pos as usize].clone();
                            result.push(copy_token.clone());
                        }
                    }
                    pos = end_pos;
                }
            }
        } else if is_rust_blk {
            // macro: dd_handle_rust_block (transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor)
            {
                // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                fn locate_newline(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_content(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                let mut newline_pos: i64 = locate_newline(pos.clone());
                let mut content_pos: i64 = locate_content(pos.clone());
                let mut is_block: bool = false;
                let mut block_value: String = "".to_string();
                if content_pos < token_count {
                    // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                    let mut newline_token: Token = tokens[newline_pos as usize].clone();
                    let mut content_token: Token = tokens[content_pos as usize].clone();
                    let kind = newline_token.kind.clone();
                    let mut newline_ok: bool = kind == "NEWLINE";
                    let kind = content_token.kind.clone();
                    let value = content_token.value.clone();
                    let mut block_ok: bool = kind == "RUST_BLOCK";
                    is_block = newline_ok && block_ok;
                    block_value = value;
                }
                if is_block {
                    // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                    let mut rust_key_prefix: String = "_rust_".to_string();
                    let mut key_parts: Vec<String> = vec![rust_key_prefix.clone(), block_value.clone()];
                    let mut decl_name: String = s_join(key_parts.clone());
                    let mut already_seen: bool = false;
                    let mut seen_len: i64 = (seen.len() as i64);
                    for seen_index in 0..seen_len {
                        // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                        let mut seen_val: String = seen[seen_index as usize].clone();
                        if seen_val == decl_name {
                            // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                            already_seen = true;
                            break;
                        }
                    }
                    if !already_seen {
                        // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                        seen.push(decl_name.clone());
                        for copy_idx in 0..3 {
                            // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                            let mut copy_pos: i64 = pos + copy_idx;
                            let mut copy_token: Token = tokens[copy_pos as usize].clone();
                            result.push(copy_token.clone());
                        }
                    }
                    pos = pos + 3;
                } else {
                    // transpiler-deor/importer/macros/dedup/dd_handle_rust_block.deor
                    let mut fallback_token: Token = tokens[pos as usize].clone();
                    result.push(fallback_token.clone());
                    pos = pos + 1;
                }
            }
        } else {
            // transpiler-deor/importer/dedup.deor
            result.push(token.clone());
            pos = pos + 1;
        }
    }
    tokens = result;
    let mut dedup_out = DedupResult { tokens: tokens.clone(), enforce_macro_file_depth: enforce_macro_file_depth.clone() };
    return dedup_out;
}

// transpiler-deor/importer/importer.deor
fn collect_all_tokens_with_all_imports(path: String) -> DedupResult {
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
    let mut reg_index: i64 = 0;
    while reg_index < reg_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        let mut entry: String = reg[reg_index as usize].clone();
        if entry == name {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut val_pos: i64 = reg_index + 1;
            if val_pos < reg_count {
                // transpiler-deor/tokens_validator/arg_helpers.deor
                let mut fields: String = reg[val_pos as usize].clone();
                return fields;
            }
        }
        reg_index = reg_index + 2;
    }
    return "".to_string();
}

fn arg_is_named(tokens: TokensRef, scan_pos: i64, kind: String) -> bool {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    fn locate_after_move(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut check_pos: i64 = scan_pos.clone();
    let mut check_kind: String = kind.clone();
    if kind == "KW_MOVE" {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        check_pos = locate_after_move(scan_pos.clone());
        if check_pos < token_count {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut move_token: Token = tokens[check_pos as usize].clone();
            let kind = move_token.kind.clone();
            check_kind = kind;
        }
    }
    if check_kind != "IDENT" {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        return false;
    }
    let mut peek_pos: i64 = check_pos + 1;
    if peek_pos < token_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        let mut peek_token: Token = tokens[peek_pos as usize].clone();
        let kind = peek_token.kind.clone();
        let mut is_call: bool = kind == "LPAREN";
        let mut is_idx: bool = kind == "KW_AT";
        let mut is_plus: bool = kind == "PLUS";
        let mut is_minus: bool = kind == "MINUS";
        let mut is_star: bool = kind == "STAR";
        let mut is_slash: bool = kind == "SLASH";
        let mut is_pct: bool = kind == "PERCENT";
        let mut is_op: bool = is_plus || is_minus || is_star || is_slash || is_pct;
        if is_call || is_idx || is_op {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            return false;
        }
    }
    return true;
}

fn find_matching_rparen(tokens: TokensRef, lp_pos: i64) -> i64 {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    fn locate_first_token(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut cur: i64 = locate_first_token(lp_pos.clone());
    let mut depth: i64 = 0;
    while cur < token_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "LPAREN" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            depth = depth + 1;
        } else if kind == "RPAREN" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut at_root: bool = depth == 0;
            if at_root {
                // transpiler-deor/tokens_validator/arg_helpers.deor
                return cur;
            }
            depth = depth - 1;
        }
        cur = cur + 1;
    }
    return lp_pos;
}

fn count_call_args(tokens: TokensRef, lp_pos: i64) -> i64 {
    // transpiler-deor/tokens_validator/arg_helpers.deor
    fn locate_first_token(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut cur: i64 = locate_first_token(lp_pos.clone());
    let mut depth: i64 = 0;
    let mut comma_count: i64 = 0;
    let mut saw_token: bool = false;
    while cur < token_count {
        // transpiler-deor/tokens_validator/arg_helpers.deor
        let mut token: Token = tokens[cur as usize].clone();
        let kind = token.kind.clone();
        if kind == "RPAREN" {
            // transpiler-deor/tokens_validator/arg_helpers.deor
            let mut at_root: bool = depth == 0;
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
            let mut at_root: bool = depth == 0;
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
    let mut prefix: String = "[validation] ".to_string();
    let mut line_sep: String = " line ".to_string();
    let mut colon_sep: String = ": ".to_string();
    let mut quote_open: String = " '".to_string();
    let mut quote_close: String = "' - ".to_string();
    let mut parts: Vec<String> = vec![prefix.clone(), file.clone(), line_sep.clone(), line_str.clone(), colon_sep.clone(), label.clone(), quote_open.clone(), name.clone(), quote_close.clone(), rule.clone()];
    return s_join(parts.clone());
}

fn handle_errors(errors: Vec<String>) {
    // transpiler-deor/tokens_validator/error_handling.deor
    let mut error_count: i64 = (errors.len() as i64);
    if error_count > 0 {
        // transpiler-deor/tokens_validator/error_handling.deor
        let mut error_index: i64 = 0;
        while error_index < error_count {
            // transpiler-deor/tokens_validator/error_handling.deor
            let mut error_msg: String = errors[error_index as usize].clone();
            println!("{}", error_msg.clone());
            error_index = error_index + 1;
        }
        std::process::exit(1);
    }
}

// transpiler-deor/macro_builder/macro_expander.deor
fn expand_deor_macros(tokens: Vec<Token>, enforce_macro_file_depth: i64) -> Vec<Token> {
    // transpiler-deor/macro_builder/macro_expander.deor
    let mut macros: std::collections::HashMap<String, (Vec<Token>, i32, bool)> = std::collections::HashMap::new();
    let mut result: Vec<Token> = vec![];
    let mut queue: std::collections::VecDeque<Token> = tokens.into_iter().collect();
    let mut scope_depth: i32 = 0;
    let mut depth_stack: Vec<(String, bool, bool)> = vec![];
    let mut cross_file_depth: i64 = 1;
    let mut unsafe_open: Vec<i32> = vec![];
    while let Some(cur) = queue.pop_front() {
    	let kind = cur.kind.as_str();

    	// track scope depth for macro privacy
    	if kind == "INDENT" { scope_depth += 1; }
    	if kind == "DEDENT" {
    		scope_depth -= 1;
    		// remove any macros defined at the depth we are leaving
    		macros.retain(|_, (_, def_depth, _)| *def_depth <= scope_depth);
    	}

    	// collect macro definition
    	if kind == "KW_MACRO" || kind == "KW_UNSAFE_MACRO" {
    		let is_unsafe = kind == "KW_UNSAFE_MACRO";
    		let is_block_macro = !is_unsafe;
    		let def_label = if is_unsafe { "unsafe_macro" } else { "macro" }.to_string();
    		let name_tok = queue.pop_front();
    		let name = name_tok.as_ref().map(|t| t.value.clone()).unwrap_or_default();
    		// skip NEWLINE
    		while queue.front().map(|t| t.kind == "NEWLINE").unwrap_or(false) { queue.pop_front(); }
    		// a macro definition must have an indented body — without one, the body-collection
    		// loop below has no INDENT to balance against and runs away consuming the rest of
    		// the file as its "body" until it happens to hit an unrelated DEDENT
    		let has_body = queue.front().map(|t| t.kind == "INDENT").unwrap_or(false);
    		if !has_body {
    			let err_tok = name_tok.clone().unwrap_or(cur.clone());
    			handle_errors(vec![val_err(err_tok, def_label.clone(), format!("must have an indented body — an empty '{} <name>' with no block is not valid", def_label))]);
    		}
    		// skip INDENT
    		while queue.front().map(|t| t.kind == "INDENT").unwrap_or(false) { queue.pop_front(); }
    		// collect body tokens, excluding the outer INDENT/DEDENT pair
    		let mut body: Vec<Token> = vec![];
    		let mut depth: i32 = 1;
    		loop {
    			match queue.pop_front() {
    				None => break,
    				Some(t) => {
    					if t.kind == "KW_MACRO" || t.kind == "KW_UNSAFE_MACRO" {
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
    		if is_block_macro {
    			// wrap the body in synthetic 'block' tokens — identical to what the lexer
    			// would produce for a hand-written 'block' as the macro's first statement
    			let wrap_file = body.first().map(|t| t.file.clone()).unwrap_or_default();
    			let mut wrapped: Vec<Token> = vec![
    				Token { kind: "KW_BLOCK".to_string(), value: String::new(), line: 0, file: wrap_file.clone() },
    				Token { kind: "NEWLINE".to_string(), value: String::new(), line: 0, file: wrap_file.clone() },
    				Token { kind: "INDENT".to_string(), value: String::new(), line: 0, file: wrap_file.clone() },
    			];
    			wrapped.extend(body);
    			wrapped.push(Token { kind: "DEDENT".to_string(), value: String::new(), line: 0, file: wrap_file });
    			body = wrapped;
    		}
    		if !name.is_empty() { macros.insert(name, (body, scope_depth, is_unsafe)); }
    		// skip trailing NEWLINE after the definition block
    		while queue.front().map(|t| t.kind == "NEWLINE").unwrap_or(false) { queue.pop_front(); }
    		continue;
    	}

    	// expand macro_run / unsafe_macro_run call site — prepend body to queue for
    	// recursive expansion. The call-site keyword must match the target macro's
    	// own kind: macro_run for an ordinary macro, unsafe_macro_run for an
    	// unsafe_macro — so a reader never has to look up the definition to know
    	// whether a call site can leak.
    	if kind == "KW_MACRO_RUN" || kind == "KW_UNSAFE_MACRO_RUN" {
    		let is_unsafe_call = kind == "KW_UNSAFE_MACRO_RUN";
    		let name_tok = queue.pop_front();
    		let name = name_tok.as_ref().map(|t| t.value.clone()).unwrap_or_default();
    		// skip trailing NEWLINE after the call
    		if queue.front().map(|t| t.kind == "NEWLINE").unwrap_or(false) { queue.pop_front(); }
    		// prepend body tokens to front of queue so they are processed next
    		if let Some((body, _, target_is_unsafe)) = macros.get(&name) {
    			let target_is_unsafe = *target_is_unsafe;
    			if target_is_unsafe && !is_unsafe_call {
    				let err_tok = name_tok.clone().unwrap_or(cur.clone());
    				handle_errors(vec![val_err(err_tok, "macro_run".to_string(), "calls an unsafe_macro — use unsafe_macro_run instead".to_string())]);
    			}
    			if !target_is_unsafe && is_unsafe_call {
    				let err_tok = name_tok.clone().unwrap_or(cur.clone());
    				handle_errors(vec![val_err(err_tok, "unsafe_macro_run".to_string(), "calls an ordinary macro, not an unsafe_macro — use macro_run instead".to_string())]);
    			}
    			if target_is_unsafe {
    				if let Some(open_depth) = unsafe_open.last() {
    					if *open_depth == scope_depth {
    						let err_tok = name_tok.clone().unwrap_or(cur.clone());
    						handle_errors(vec![val_err(err_tok, "unsafe_macro".to_string(), "cannot be called here — an unsafe_macro cannot call, or be called from inside, another unsafe_macro".to_string())]);
    					}
    				}
    			}
    			let marker_file = body.first().map(|t| t.file.clone()).unwrap_or_default();
    			let mut did_increment = false;
    			if enforce_macro_file_depth > 0 {
    				let context_file = depth_stack.last().map(|e: &(String, bool, bool)| e.0.clone()).unwrap_or_else(|| cur.file.clone());
    				if marker_file != context_file {
    					let new_depth = cross_file_depth + 1;
    					if new_depth > enforce_macro_file_depth {
    						let err_tok = name_tok.clone().unwrap_or(cur.clone());
    						handle_errors(vec![val_err(err_tok, "macro_run".to_string(), format!("calling '{}' crosses into '{}', taking the cross-file macro chain to depth {} — exceeds ENFORCE_MACRO_FILE_DEPTH limit of {}", name, marker_file, new_depth, enforce_macro_file_depth))]);
    					}
    					cross_file_depth = new_depth;
    					did_increment = true;
    				}
    			}
    			depth_stack.push((marker_file.clone(), did_increment, target_is_unsafe));
    			if target_is_unsafe { unsafe_open.push(scope_depth); }
    			queue.push_front(Token { kind: "MACRO_FRAME_END".to_string(), value: String::new(), line: 0, file: marker_file.clone() });
    			for tok in body.iter().rev() { queue.push_front(tok.clone()); }
    			queue.push_front(Token { kind: "MACRO_MARKER".to_string(), value: name.clone(), line: 0, file: marker_file });
    		} else {
    			// validate_macros only checks that the name is defined *somewhere* in the
    			// file, not that it's still in scope here -- a macro defined inside a block
    			// that already closed (removed from `macros` by the DEDENT retain() above)
    			// passes that check but has nothing to expand to at this call site. Without
    			// this branch the call silently vanished: no error, and the tokens consumed
    			// above (name, trailing NEWLINE) were simply dropped from the output.
    			let err_tok = name_tok.clone().unwrap_or(cur.clone());
    			handle_errors(vec![val_err(err_tok, "macro_run".to_string(), format!("'{}' is defined locally but not visible here -- it went out of scope (a local macro is only visible inside the block it's declared in)", name))]);
    		}
    		continue;
    	}

    	// closes the bracket opened above when the spliced macro body is fully consumed
    	if kind == "MACRO_FRAME_END" {
    		if let Some((_, did_increment, was_unsafe)) = depth_stack.pop() {
    			if did_increment { cross_file_depth -= 1; }
    			if was_unsafe { unsafe_open.pop(); }
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
    fn locate_next_token(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut token_count: i64 = (raw_tokens.len() as i64);
    let mut errors: Vec<String> = Vec::new();
    let mut macro_names: Vec<String> = Vec::new();
    let mut def_index: i64 = 0;
    while def_index < token_count {
        // transpiler-deor/macro_builder/macro_validation.deor
        let mut tok: Token = raw_tokens[def_index as usize].clone();
        let kind = tok.kind.clone();
        if kind == "KW_MACRO" || kind == "KW_UNSAFE_MACRO" {
            // transpiler-deor/macro_builder/macro_validation.deor
            let mut name_pos: i64 = locate_next_token(def_index.clone());
            if name_pos < token_count {
                // transpiler-deor/macro_builder/macro_validation.deor
                let mut name_token: Token = raw_tokens[name_pos as usize].clone();
                let kind = name_token.kind.clone();
                let value = name_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/macro_builder/macro_validation.deor
                    macro_names.push(value.clone());
                }
            }
        }
        def_index = def_index + 1;
    }
    let mut lbl_macro: String = "macro_run".to_string();
    let mut rule_macro_run: String = "macro is not defined — check the name or add a 'macro <name>' definition".to_string();
    let mut run_index: i64 = 0;
    while run_index < token_count {
        // transpiler-deor/macro_builder/macro_validation.deor
        let mut tok: Token = raw_tokens[run_index as usize].clone();
        let kind = tok.kind.clone();
        if kind == "KW_MACRO_RUN" || kind == "KW_UNSAFE_MACRO_RUN" {
            // transpiler-deor/macro_builder/macro_validation.deor
            let mut name_pos: i64 = locate_next_token(run_index.clone());
            if name_pos < token_count {
                // transpiler-deor/macro_builder/macro_validation.deor
                let mut name_token: Token = raw_tokens[name_pos as usize].clone();
                let kind = name_token.kind.clone();
                let value = name_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/macro_builder/macro_validation.deor
                    if !list_has(macro_names.clone(), value.clone()) {
                        // transpiler-deor/macro_builder/macro_validation.deor
                        errors.push(val_err(name_token.clone(), lbl_macro.clone(), rule_macro_run.clone()).clone());
                    }
                }
            }
        }
        run_index = run_index + 1;
    }
    handle_errors(errors.clone());
    return raw_tokens;
}

// transpiler-deor/macro_builder/macro_builder.deor
fn build_macros(raw_tokens: Vec<Token>, enforce_macro_file_depth: i64) -> Vec<Token> {
    // transpiler-deor/macro_builder/macro_builder.deor
    let mut validated: Vec<Token> = validate_macros(raw_tokens.clone());
    return expand_deor_macros(validated.clone(), enforce_macro_file_depth.clone());
}

// transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
fn is_single_return_body(tokens: TokensRef, right_paren_pos: i64, token_count: i64) -> bool {
    // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
    let mut newline_pos: i64 = right_paren_pos + 1;
    if newline_pos >= token_count {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut newline_token: Token = tokens[newline_pos as usize].clone();
    let kind = newline_token.kind.clone();
    if kind != "NEWLINE" {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut indent_pos: i64 = newline_pos + 1;
    if indent_pos >= token_count {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut indent_token: Token = tokens[indent_pos as usize].clone();
    let kind = indent_token.kind.clone();
    if kind != "INDENT" {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut return_pos: i64 = indent_pos + 1;
    if return_pos >= token_count {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut return_token: Token = tokens[return_pos as usize].clone();
    let kind = return_token.kind.clone();
    if kind != "KW_RETURN" {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut scan_pos: i64 = return_pos + 1;
    while scan_pos < token_count {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        let mut scan_token: Token = tokens[scan_pos as usize].clone();
        let kind = scan_token.kind.clone();
        if kind == "NEWLINE" {
            // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
            break;
        }
        scan_pos = scan_pos + 1;
    }
    let mut after_stmt_pos: i64 = scan_pos + 1;
    if after_stmt_pos >= token_count {
        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
        return false;
    }
    let mut after_stmt_token: Token = tokens[after_stmt_pos as usize].clone();
    let kind = after_stmt_token.kind.clone();
    return kind == "DEDENT";
}

// transpiler-deor/tokens_validator/tokens_validation.deor
type FnTestRule = fn(String) -> bool;

#[derive(Clone, PartialEq, Debug)]
struct UamFrame {
    chain_depth: i64,
    pre_vars: Vec<String>,
    acc_vars: Vec<String>,
}

type FrameStack = Vec<UamFrame>;

#[derive(Clone, PartialEq, Debug)]
struct VoidFnFrame {
    depth: i64,
    prev_void: bool,
}

type VoidFrameStack = Vec<VoidFnFrame>;

fn validate_tokens(tokens: TokensRef) {
    // transpiler-deor/tokens_validator/tokens_validation.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut errors: Vec<String> = Vec::new();
    let mut pos: i64 = 0;
    let mut paren_depth: i64 = 0;
    let mut block_depth: i64 = 0;
    let mut in_void_fn: bool = false;
    let mut in_struct_body: bool = false;
    let mut in_enum_body: bool = false;
    let mut in_fn_body: bool = false;
    let mut was_already_in_fn_body: bool = false;
    let mut void_fn_stack: Vec<VoidFnFrame> = Vec::new();
    // macro: define_errors (transpiler-deor/tokens_validator/macros/define_errors.deor)
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
    let mut rule_min3: String = "name must be at least 3 characters".to_string();
    let mut rule_no_option: String = "Rust generic types (Option/Vec/Box/Rc/Arc/Result) are not valid in Deor — use shapes or validator types".to_string();
    let mut rule_pascal: String = "name must be PascalCase (start with uppercase letter)".to_string();
    let mut rule_camel: String = "name must be camelCase (start lowercase, no underscores)".to_string();
    let mut rule_snake: String = "name must be lower_snake_case (no uppercase letters)".to_string();
    let mut rule_screaming: String = "const name must be SCREAMING_SNAKE_CASE (all caps, underscores between words)".to_string();
    let mut rule_named_arg: String = "each arg must be a named variable when passing 2 or more args".to_string();
    let mut rule_enum_pascal: String = "enum variant must be PascalCase".to_string();
    let mut rule_enum_data: String = "enum variants cannot carry data — use a struct alongside the enum instead".to_string();
    let mut rule_typed_enum_eq: String = "typed enum variant must have a value — add '= value' after the variant name".to_string();
    let mut rule_untyped_enum_eq: String = "untyped enum variant cannot have a value — use 'enum string/int Name' to associate values with variants".to_string();
    let mut rule_non_primitive_validator: String = "validator base type must be a primitive (int, float, string, bool) — structs, list shapes, and other validator types cannot be used".to_string();
    let mut rule_validator_missing_body: String = "validator type is missing a predicate body — add an indented block that returns a bool, e.g. 'return val > 0'".to_string();
    let mut rule_max_params: String = "functions may have at most 3 parameters".to_string();
    let mut rule_param_shadow: String = "parameter name cannot be the same as its type — choose a descriptive name".to_string();
    let mut rule_type_param_shadow: String = "validator parameter name cannot be the same as the type name — use a descriptive name like 'val' or 'num'".to_string();
    let mut rule_no_ret: String = "missing return type — use 'fn void name()' for functions that return nothing".to_string();
    let mut rule_nested_fn: String = "functions may only be declared at the top level of a file or nested directly inside another fn — not inside a struct/enum/shape/macro body".to_string();
    let mut rule_nested_fn_body_shape: String = "a fn nested inside another fn must have a body of exactly one 'return expr' statement — no local variables, moves, or other statements".to_string();
    let mut rule_void_return: String = "void functions must not use return — remove the return statement and let the function fall through".to_string();
    let mut rule_return_empty: String = "cannot return 'empty' — declare a validator type variable without a value and return it to signal not-valid".to_string();
    let mut rule_return_none: String = "none is not a Deor keyword — declare a validator type variable without a value and return it to signal not-valid".to_string();
    let mut rule_void_var: String = "'void' is not a valid variable type — only functions can return void".to_string();
    let mut rule_crash: String = "crash takes exactly 1 string argument".to_string();
    let mut rule_print_args: String = "print takes 1 argument, or 2 arguments where the second replaces the trailing newline".to_string();
    let mut rule_range_args: String = "range takes 1 argument (range(count)) or 2 arguments (range(start, end))".to_string();
    let mut rule_len_args: String = "len takes exactly 1 argument".to_string();
    let mut rule_avow: String = "avow can only be used on a validator type variable".to_string();
    let mut rule_invalid_char: String = "character is not valid in Deor — use Deor operators and keywords; raw Rust syntax belongs inside a 'rust' block".to_string();
    let mut rule_validator_empty: String = "empty is not valid for validator types — declare without a value to start as not valid: 'Roll best'".to_string();
    let mut rule_bad_stmt: String = "literal cannot follow 'name ident' — capture in a named variable first".to_string();
    let mut rule_undeclared_reassign: String = "'name = expr' only works for reassigning an existing variable — this name was never declared. Prefix it with a type to declare it (e.g. 'string hello = expr'), or use 'as' instead of '=' if it's a fresh binding and moving/copying ownership is fine (e.g. 'hello as expr')".to_string();
    let mut rule_undefined_var: String = "used here but never declared anywhere — check for a typo, or declare it first ('string name = expr', 'name as expr', a function parameter, or a for-loop variable)".to_string();
    let mut rule_typed_as: String = "typed `as` bindings are not supported — use `a as b` to transfer ownership, or `Type a = move b` for an explicit typed move".to_string();
    let mut rule_as_move: String = "`as` already transfers ownership — use `a as b` instead of `a as move b`".to_string();
    let mut rule_bracket_index: String = "bracket indexing is not valid in Deor — use 'name at index' instead".to_string();
    let mut rule_empty_bracket: String = "use 'empty' to initialize an empty list — [] is only valid with items inside".to_string();
    let mut rule_move: String = "'move' can only precede a variable name — 'move 5' or 'move \"hello\"' are not valid".to_string();
    let mut rule_use_after_move: String = "used here but already moved earlier — the value was consumed by an earlier 'move' and cannot be read again".to_string();
    let mut rule_double_move: String = "already moved earlier — 'move' cannot consume the same variable twice".to_string();
    let mut rule_not_is: String = "use 'x is not y' instead of 'not x is y' — 'not' binds before 'is' resolves".to_string();
    let mut rule_kw_in_parens: String = "reserved keyword cannot be used as a name — choose a different variable name".to_string();
    let mut rule_valid: String = "'valid' can only appear after 'is' or 'is not' — it cannot be assigned or returned".to_string();
    let mut rule_end: String = "'end' can only appear directly after 'at' (list at end / list at end = val) — it cannot be used as a variable name or expression".to_string();
    let mut rule_with_parens: String = "'with' must be followed by a parenthesized field list — 'with (area)', not 'with area' — parens are required even for a single field".to_string();
    let mut rule_unmatched_open_paren: String = "'(' is never closed — every open paren needs a matching ')'".to_string();
    let mut rule_unmatched_close_paren: String = "')' has no matching '(' before it — remove the extra ')' or add the missing '('".to_string();
    let mut rule_empty_parens: String = "'()' is not valid — parens must contain at least one item, except when declaring or calling a zero-parameter function".to_string();
    let mut rule_unmatched_open_bracket: String = "'[' is never closed — every open bracket needs a matching ']'".to_string();
    let mut rule_unmatched_close_bracket: String = "']' has no matching '[' before it — remove the extra ']' or add the missing '['".to_string();
    let mut rule_builtin_shadow: String = "this name belongs to a built-in function (print, crash, len, range, args, input) and cannot be shadowed or redeclared".to_string();
    let mut rule_range_placement: String = "'range' can only be used as a for-loop's iterator expression ('for var in range(n)' or 'for in range(n)') — it cannot be assigned to a variable or passed as an argument".to_string();
    let mut rule_bare_tuple_range: String = "bare tuple range ('for var in (start, end)') is not valid — use 'for var in range(start, end)' instead".to_string();
    let mut rule_bare_truthiness: String = "only bool and validator types have truthiness — use an explicit comparison ('is not 0', 'is not \"\"', 'is valid', etc.)".to_string();
    let mut rule_func_shape_multi_param: String = "func shapes accept at most one input type and one output type — bundle multiple values into a struct instead".to_string();
    let mut rule_string_plus_banned: String = "'+' cannot be used with strings — use s_join([a, b, ...]) or s_join_with(list, sep) instead".to_string();
    let mut rule_double_equals: String = "'==' is not valid in Deor — use 'is' for equality comparison".to_string();
    let mut rule_const_reassign: String = "cannot reassign a const variable — const bindings are immutable".to_string();
    let mut rule_validator_reassign: String = "cannot reassign a validator type variable with 'as' — it skips the predicate check; use 'name = expr' to re-validate, or 'TypeName name = expr' for a fresh declaration".to_string();
    let mut rule_raw_in_expr: String = "raw variables cannot be used in Deor operators, builtins, or rebindings — pass them to a function or consume them inside a rust block".to_string();
    let mut rule_raw_reassign: String = "raw variables cannot be reassigned — declare a new 'raw name = expr' instead".to_string();
    let mut rule_raw_assignment: String = "raw variables can only be assigned from a function call — use 'raw name = some_function()', not a literal or an inline rust block".to_string();
    let mut rule_raw_as: String = "raw declarations must use '=', not 'as' — 'raw name = call()' declares an opaque value from a function call".to_string();
    let mut rule_no_func_field: String = "func shapes cannot be struct fields — pass the func shape as a function parameter instead".to_string();
    let mut rule_no_raw_field: String = "raw cannot be a struct field — raw values are opaque and cannot be stored in structs".to_string();
    let mut rule_struct_field_count: String = "wrong number of fields in struct construction — all fields must be provided".to_string();
    let mut rule_struct_field_name: String = "unknown field name in struct construction — variable name does not match any field in this struct".to_string();
    // macro: check_paren_balance (transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor)
    {
        // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
        let mut depth: i64 = 0;
        let mut open_line: i64 = 0;
        let mut open_file: String = "".to_string();
        let mut scan_i: i64 = 0;
        while scan_i < token_count {
            // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
            let mut scan_token: Token = tokens[scan_i as usize].clone();
            let mut kind = scan_token.kind.clone();
            let mut line = scan_token.line.clone();
            let mut file = scan_token.file.clone();
            if kind == "LPAREN" {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
                if depth == 0 {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
                    open_line = line;
                    open_file = file;
                }
                depth = depth + 1;
            } else if kind == "RPAREN" {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
                if depth == 0 {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
                    errors.push(val_err(scan_token.clone(), lbl_var.clone(), rule_unmatched_close_paren.clone()).clone());
                } else {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
                    depth = depth - 1;
                }
            }
            scan_i = scan_i + 1;
        }
        if depth > 0 {
            // transpiler-deor/tokens_validator/macros/brackets_parens/check_paren_balance.deor
            let mut kind: String = "LPAREN".to_string();
            let mut value: String = "(".to_string();
            let mut line: i64 = open_line.clone();
            let mut file: String = open_file.clone();
            let mut open_token = Token { kind: kind.clone(), value: value.clone(), line: line.clone(), file: file.clone() };
            errors.push(val_err(open_token.clone(), lbl_var.clone(), rule_unmatched_open_paren.clone()).clone());
        }
    }
    // macro: check_bracket_balance (transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor)
    {
        // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
        let mut depth: i64 = 0;
        let mut open_line: i64 = 0;
        let mut open_file: String = "".to_string();
        let mut scan_i: i64 = 0;
        while scan_i < token_count {
            // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
            let mut scan_token: Token = tokens[scan_i as usize].clone();
            let mut kind = scan_token.kind.clone();
            let mut line = scan_token.line.clone();
            let mut file = scan_token.file.clone();
            if kind == "LBRACKET" {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
                if depth == 0 {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
                    open_line = line;
                    open_file = file;
                }
                depth = depth + 1;
            } else if kind == "RBRACKET" {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
                if depth == 0 {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
                    errors.push(val_err(scan_token.clone(), lbl_var.clone(), rule_unmatched_close_bracket.clone()).clone());
                } else {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
                    depth = depth - 1;
                }
            }
            scan_i = scan_i + 1;
        }
        if depth > 0 {
            // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_balance.deor
            let mut kind: String = "LBRACKET".to_string();
            let mut value: String = "[".to_string();
            let mut line: i64 = open_line.clone();
            let mut file: String = open_file.clone();
            let mut open_token = Token { kind: kind.clone(), value: value.clone(), line: line.clone(), file: file.clone() };
            errors.push(val_err(open_token.clone(), lbl_var.clone(), rule_unmatched_open_bracket.clone()).clone());
        }
    }
    // transpiler-deor/tokens_validator/tokens_validation.deor
    handle_errors(errors.clone());
    // macro: define_lookup_tables (transpiler-deor/tokens_validator/macros/define_lookup_tables.deor)
    let mut forbidden_in_parens: Vec<String> = vec!["KW_LIST".to_string(), "KW_STRUCT".to_string(), "KW_SHAPE".to_string(), "KW_ENUM".to_string(), "KW_TYPE".to_string(), "KW_FN".to_string(), "KW_OF".to_string(), "KW_FOR".to_string(), "KW_IF".to_string(), "KW_ELSE".to_string(), "KW_RETURN".to_string(), "KW_BREAK".to_string(), "KW_CONTINUE".to_string(), "KW_REMOVE".to_string(), "KW_RUST".to_string(), "KW_IMPORT".to_string(), "KW_MACRO".to_string(), "KW_UNSAFE_MACRO".to_string(), "KW_VOID".to_string(), "KW_RAW".to_string()];
    let mut reserved_keywords: Vec<String> = vec!["KW_AND".to_string(), "KW_AS".to_string(), "KW_AT".to_string(), "KW_AVOW".to_string(), "KW_BLOCK".to_string(), "KW_BREAK".to_string(), "KW_CONST".to_string(), "KW_CONTINUE".to_string(), "KW_ELSE".to_string(), "KW_EMPTY".to_string(), "KW_ENUM".to_string(), "KW_FALSE".to_string(), "KW_FN".to_string(), "KW_FOR".to_string(), "KW_FUNC".to_string(), "KW_IF".to_string(), "KW_IMPORT".to_string(), "KW_IN".to_string(), "KW_IS".to_string(), "KW_LIST".to_string(), "KW_MACRO".to_string(), "KW_MACRO_RUN".to_string(), "KW_UNSAFE_MACRO".to_string(), "KW_UNSAFE_MACRO_RUN".to_string(), "KW_MOVE".to_string(), "KW_NONE".to_string(), "KW_NOT".to_string(), "KW_OF".to_string(), "KW_OR".to_string(), "KW_RAW".to_string(), "KW_REMOVE".to_string(), "KW_RETURN".to_string(), "KW_RUST".to_string(), "KW_SHAPE".to_string(), "KW_STRUCT".to_string(), "KW_TO".to_string(), "KW_TRUE".to_string(), "KW_TYPE".to_string(), "KW_VALID".to_string(), "KW_VOID".to_string(), "KW_WITH".to_string()];
    let mut builtin_names: Vec<String> = vec!["print".to_string(), "crash".to_string(), "len".to_string(), "range".to_string(), "args".to_string(), "input".to_string()];
    let mut primitive_type_names: Vec<String> = vec!["string".to_string(), "int".to_string(), "float".to_string(), "bool".to_string()];
    // macro: prescan_shapes_and_validator_types (transpiler-deor/tokens_validator/macros/prescan/prescan_shapes_and_validator_types.deor)
    let mut func_shape_names: Vec<String> = Vec::new();
    let mut validator_type_names: Vec<String> = Vec::new();
    let mut pre_i: i64 = 0;
    while pre_i < token_count {
        // transpiler-deor/tokens_validator/macros/prescan/prescan_shapes_and_validator_types.deor
        let mut pre_tok: Token = tokens[pre_i as usize].clone();
        let mut kind = pre_tok.kind.clone();
        // macro: prescan_collect_func_shapes (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor
            fn locate_form(kw_pos: i64) -> i64 {
                return kw_pos + 3;
            }
            fn locate_name(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            if kind == "KW_SHAPE" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor
                let mut form_pos: i64 = locate_form(pre_i.clone());
                if form_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor
                    let mut form_token: Token = tokens[form_pos as usize].clone();
                    let mut kind = form_token.kind.clone();
                    if kind == "KW_FUNC" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor
                        let mut name_pos: i64 = locate_name(pre_i.clone());
                        if name_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            let mut kind = name_token.kind.clone();
                            let mut value = name_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_func_shapes.deor
                                func_shape_names.push(value.clone());
                            }
                        }
                    }
                }
            }
        }
        // macro: prescan_collect_validator_types (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_validator_types.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_validator_types.deor
            fn locate_name(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            if kind == "KW_TYPE" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_validator_types.deor
                let mut name_pos: i64 = locate_name(pre_i.clone());
                if name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_validator_types.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_validator_types.deor
                        validator_type_names.push(value.clone());
                    }
                }
            }
        }
        // transpiler-deor/tokens_validator/macros/prescan/prescan_shapes_and_validator_types.deor
        pre_i = pre_i + 1;
    }
    // macro: prescan_declared_state (transpiler-deor/tokens_validator/macros/prescan/prescan_declared_state.deor)
    let mut struct_field_reg: Vec<String> = Vec::new();
    let mut validator_vars: Vec<String> = Vec::new();
    let mut raw_var_names: Vec<String> = Vec::new();
    let mut const_var_names: Vec<String> = Vec::new();
    let mut non_bool_var_names: Vec<String> = Vec::new();
    let mut string_var_names: Vec<String> = Vec::new();
    let mut copy_var_names: Vec<String> = Vec::new();
    let mut moved_vars: Vec<String> = Vec::new();
    let mut moved_fields: Vec<String> = Vec::new();
    let mut move_if_stack: Vec<UamFrame> = Vec::new();
    let mut declared_var_names: Vec<String> = Vec::new();
    let mut fn_names: Vec<String> = Vec::new();
    let mut enum_variant_names: Vec<String> = Vec::new();
    let mut enum_names: Vec<String> = Vec::new();
    let mut pre_i: i64 = 0;
    while pre_i < token_count {
        // transpiler-deor/tokens_validator/macros/prescan/prescan_declared_state.deor
        let mut pre_tok: Token = tokens[pre_i as usize].clone();
        let mut kind = pre_tok.kind.clone();
        // macro: prescan_collect_const_names (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_const_names.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_const_names.deor
            fn locate_name(kw_pos: i64) -> i64 {
                return kw_pos + 2;
            }
            if kind == "KW_CONST" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_const_names.deor
                let mut name_pos: i64 = locate_name(pre_i.clone());
                if name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_const_names.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_const_names.deor
                        if !list_has(const_var_names.clone(), value.clone()) {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_const_names.deor
                            const_var_names.push(value.clone());
                        }
                    }
                }
            }
        }
        // macro: prescan_collect_declared_vars (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars.deor)
        {
            // macro: prescan_collect_declared_vars_binding (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                fn locate_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_equals(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                if kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                    let mut name_pos: i64 = locate_name(pre_i.clone());
                    let mut equals_pos: i64 = locate_equals(pre_i.clone());
                    if equals_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                        let mut name_token: Token = tokens[name_pos as usize].clone();
                        let mut equals_token: Token = tokens[equals_pos as usize].clone();
                        let mut kind = name_token.kind.clone();
                        let mut value = name_token.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_value: String = value.clone();
                        let mut kind = equals_token.kind.clone();
                        if name_kind == "IDENT" && kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                            declared_var_names.push(name_value.clone());
                        }
                    }
                    if name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                        let mut as_token: Token = tokens[name_pos as usize].clone();
                        let mut kind = as_token.kind.clone();
                        if kind == "KW_AS" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                            let mut value = pre_tok.value.clone();
                            declared_var_names.push(value.clone());
                        }
                        let mut value = pre_tok.value.clone();
                        let mut is_vtype: bool = list_has(validator_type_names.clone(), value.clone());
                        if is_vtype {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_binding.deor
                                let mut value = as_token.value.clone();
                                declared_var_names.push(value.clone());
                            }
                        }
                    }
                }
            }
            // macro: prescan_collect_declared_vars_raw (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_raw.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_raw.deor
                fn locate_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_equals(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                if kind == "KW_RAW" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_raw.deor
                    let mut name_pos: i64 = locate_name(pre_i.clone());
                    let mut equals_pos: i64 = locate_equals(pre_i.clone());
                    if equals_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_raw.deor
                        let mut name_token: Token = tokens[name_pos as usize].clone();
                        let mut equals_token: Token = tokens[equals_pos as usize].clone();
                        let mut kind = name_token.kind.clone();
                        let mut value = name_token.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_value: String = value.clone();
                        let mut kind = equals_token.kind.clone();
                        if name_kind == "IDENT" && kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_raw.deor
                            declared_var_names.push(name_value.clone());
                        }
                    }
                }
            }
            // macro: prescan_collect_declared_vars_validator_param (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_validator_param.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_validator_param.deor
                fn locate_left_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_param_name(kw_pos: i64) -> i64 {
                    return kw_pos + 4;
                }
                if kind == "KW_TYPE" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_validator_param.deor
                    let mut left_paren_pos: i64 = locate_left_paren(pre_i.clone());
                    let mut param_name_pos: i64 = locate_param_name(pre_i.clone());
                    if param_name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_validator_param.deor
                        let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                        let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                        let mut kind = left_paren_token.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_validator_param.deor
                            let mut kind = param_name_token.kind.clone();
                            let mut value = param_name_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_validator_param.deor
                                declared_var_names.push(value.clone());
                            }
                        }
                    }
                }
            }
            // macro: prescan_collect_declared_vars_for (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                fn locate_loop_var(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_in_keyword(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_move_keyword(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_move_lparen(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_move_var(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                fn locate_move_in(kw_pos: i64) -> i64 {
                    return kw_pos + 4;
                }
                if kind == "KW_FOR" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                    let mut loop_var_pos: i64 = locate_loop_var(pre_i.clone());
                    let mut in_pos: i64 = locate_in_keyword(pre_i.clone());
                    if in_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                        let mut loop_var_token: Token = tokens[loop_var_pos as usize].clone();
                        let mut in_token: Token = tokens[in_pos as usize].clone();
                        let mut kind = loop_var_token.kind.clone();
                        let mut value = loop_var_token.value.clone();
                        let mut loop_var_kind: String = kind.clone();
                        let mut loop_var_value: String = value.clone();
                        let mut kind = in_token.kind.clone();
                        if loop_var_kind == "IDENT" && kind == "KW_IN" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                            declared_var_names.push(loop_var_value.clone());
                        }
                    }
                    let mut move_pos: i64 = locate_move_keyword(pre_i.clone());
                    if move_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                        let mut move_token: Token = tokens[move_pos as usize].clone();
                        let mut kind = move_token.kind.clone();
                        if kind == "KW_MOVE" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                            let mut move_lparen_pos: i64 = locate_move_lparen(pre_i.clone());
                            let mut move_var_pos: i64 = locate_move_var(pre_i.clone());
                            let mut move_in_pos: i64 = locate_move_in(pre_i.clone());
                            if move_in_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                                let mut move_lparen_token: Token = tokens[move_lparen_pos as usize].clone();
                                let mut move_var_token: Token = tokens[move_var_pos as usize].clone();
                                let mut move_in_token: Token = tokens[move_in_pos as usize].clone();
                                let mut kind = move_lparen_token.kind.clone();
                                let mut move_lparen_ok: bool = kind == "LPAREN";
                                let mut kind = move_var_token.kind.clone();
                                let mut value = move_var_token.value.clone();
                                let mut move_var_kind: String = kind.clone();
                                let mut move_var_value: String = value.clone();
                                let mut kind = move_in_token.kind.clone();
                                if move_lparen_ok && move_var_kind == "IDENT" && kind == "KW_IN" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_for.deor
                                    declared_var_names.push(move_var_value.clone());
                                }
                            }
                        }
                    }
                }
            }
            // macro: prescan_collect_declared_vars_fn_params (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                fn locate_left_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                if kind == "KW_FN" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                    let mut left_paren_pos: i64 = locate_left_paren(pre_i.clone());
                    if left_paren_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                        let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                        let mut kind = left_paren_token.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                            let mut param_scan_pos: i64 = left_paren_pos + 1;
                            while param_scan_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                                let mut param_scan_token: Token = tokens[param_scan_pos as usize].clone();
                                let mut kind = param_scan_token.kind.clone();
                                let mut value = param_scan_token.value.clone();
                                if kind == "RPAREN" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                                    break;
                                }
                                if kind == "COMMA" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                                    param_scan_pos = param_scan_pos + 1;
                                    continue;
                                }
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                                    let mut param_name_pos: i64 = param_scan_pos + 1;
                                    if param_name_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                                        let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                                        let mut kind = param_name_token.kind.clone();
                                        let mut value = param_name_token.value.clone();
                                        if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_fn_params.deor
                                            declared_var_names.push(value.clone());
                                            param_scan_pos = param_name_pos;
                                        }
                                    }
                                }
                                param_scan_pos = param_scan_pos + 1;
                            }
                        }
                    }
                }
            }
            // macro: prescan_collect_declared_vars_enum (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                fn locate_type_keyword(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if kind == "KW_ENUM" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                    let mut type_keyword_pos: i64 = locate_type_keyword(pre_i.clone());
                    if type_keyword_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                        let mut type_keyword_token: Token = tokens[type_keyword_pos as usize].clone();
                        let mut kind = type_keyword_token.kind.clone();
                        let mut value = type_keyword_token.value.clone();
                        let mut is_typed: bool = false;
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                            is_typed = list_has(primitive_type_names.clone(), value.clone());
                        }
                        if is_typed {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                            let mut indent_scan_pos: i64 = type_keyword_pos + 2;
                            while indent_scan_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                let mut indent_scan_token: Token = tokens[indent_scan_pos as usize].clone();
                                let mut kind = indent_scan_token.kind.clone();
                                if kind == "INDENT" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                    break;
                                }
                                indent_scan_pos = indent_scan_pos + 1;
                            }
                            let mut body_pos: i64 = indent_scan_pos + 1;
                            let mut depth: i64 = 1;
                            while body_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                let mut body_token: Token = tokens[body_pos as usize].clone();
                                let mut kind = body_token.kind.clone();
                                let mut value = body_token.value.clone();
                                if kind == "INDENT" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                    depth = depth + 1;
                                } else if kind == "DEDENT" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                    depth = depth - 1;
                                    if depth == 0 {
                                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                        break;
                                    }
                                } else if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                    let mut equals_check_pos: i64 = body_pos + 1;
                                    if equals_check_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                        let mut equals_check_token: Token = tokens[equals_check_pos as usize].clone();
                                        let mut kind = equals_check_token.kind.clone();
                                        if kind == "EQUALS" {
                                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_enum.deor
                                            declared_var_names.push(value.clone());
                                        }
                                    }
                                }
                                body_pos = body_pos + 1;
                            }
                        }
                    }
                }
            }
            // macro: prescan_collect_declared_vars_destructure (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor)
            {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                fn locate_prev_token(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                    let mut is_call: bool = false;
                    if pre_i > 0 {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                        let mut prev_pos: i64 = locate_prev_token(pre_i.clone());
                        let mut prev_token: Token = tokens[prev_pos as usize].clone();
                        let mut kind = prev_token.kind.clone();
                        is_call = kind == "IDENT";
                    }
                    if !is_call {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                        let mut scan_pos: i64 = locate_next_token(pre_i.clone());
                        let mut depth: i64 = 1;
                        while scan_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                            let mut scan_token: Token = tokens[scan_pos as usize].clone();
                            let mut kind = scan_token.kind.clone();
                            if kind == "LPAREN" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                depth = depth + 1;
                            } else if kind == "RPAREN" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                depth = depth - 1;
                                if depth == 0 {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                    break;
                                }
                            }
                            scan_pos = scan_pos + 1;
                        }
                        let mut after_pos: i64 = scan_pos + 1;
                        let mut is_destructure: bool = false;
                        if after_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                            let mut after_token: Token = tokens[after_pos as usize].clone();
                            let mut kind = after_token.kind.clone();
                            is_destructure = kind == "KW_IN";
                        }
                        if is_destructure {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                            let mut name_pos: i64 = locate_next_token(pre_i.clone());
                            let mut at_name_slot: bool = true;
                            while name_pos < scan_pos {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                let mut name_token: Token = tokens[name_pos as usize].clone();
                                let mut kind = name_token.kind.clone();
                                let mut value = name_token.value.clone();
                                if kind == "COMMA" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                    at_name_slot = true;
                                } else if at_name_slot {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_declared_vars_destructure.deor
                                        declared_var_names.push(value.clone());
                                    }
                                    at_name_slot = false;
                                }
                                name_pos = name_pos + 1;
                            }
                        }
                    }
                }
            }
        }
        // macro: prescan_collect_fn_names (transpiler-deor/tokens_validator/macros/prescan/prescan_collect_fn_names.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_fn_names.deor
            fn locate_name(kw_pos: i64) -> i64 {
                return kw_pos + 2;
            }
            if kind == "KW_FN" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_fn_names.deor
                let mut name_pos: i64 = locate_name(pre_i.clone());
                if name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_fn_names.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_collect_fn_names.deor
                        fn_names.push(value.clone());
                    }
                }
            }
        }
        // macro: prescan_check_duplicate_decls (transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
            fn locate_decl_name(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            fn locate_fn_name(kw_pos: i64) -> i64 {
                return kw_pos + 2;
            }
            let mut is_kw_struct: bool = kind == "KW_STRUCT";
            let mut is_kw_enum: bool = kind == "KW_ENUM";
            let mut is_kw_shape: bool = kind == "KW_SHAPE";
            let mut is_kw_type: bool = kind == "KW_TYPE";
            let mut is_named_decl: bool = is_kw_struct || is_kw_enum || is_kw_shape || is_kw_type;
            if is_named_decl {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                let mut name_pos: i64 = locate_decl_name(pre_i.clone());
                if name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                        let mut is_typed_enum_kw: bool = list_has(primitive_type_names.clone(), value.clone());
                        if is_kw_enum {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                            if is_typed_enum_kw {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                let mut typed_name_pos: i64 = name_pos + 1;
                                if typed_name_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                    let mut typed_name_token: Token = tokens[typed_name_pos as usize].clone();
                                    let mut kind = typed_name_token.kind.clone();
                                    let mut value = typed_name_token.value.clone();
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                        if list_has(builtin_names.clone(), value.clone()) {
                                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                            errors.push(val_err(typed_name_token.clone(), lbl_decl.clone(), rule_builtin_shadow.clone()).clone());
                                        }
                                    }
                                }
                            } else {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                if list_has(builtin_names.clone(), value.clone()) {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                    errors.push(val_err(name_token.clone(), lbl_decl.clone(), rule_builtin_shadow.clone()).clone());
                                }
                            }
                        } else {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                            if list_has(builtin_names.clone(), value.clone()) {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                                errors.push(val_err(name_token.clone(), lbl_decl.clone(), rule_builtin_shadow.clone()).clone());
                            }
                        }
                    }
                }
            }
            if kind == "KW_FN" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                let mut fn_name_pos: i64 = locate_fn_name(pre_i.clone());
                if fn_name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                    let mut fn_name_token: Token = tokens[fn_name_pos as usize].clone();
                    let mut kind = fn_name_token.kind.clone();
                    let mut value = fn_name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                        if list_has(builtin_names.clone(), value.clone()) {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_duplicate_decls.deor
                            errors.push(val_err(fn_name_token.clone(), lbl_decl.clone(), rule_builtin_shadow.clone()).clone());
                        }
                    }
                }
            }
        }
        // macro: prescan_check_struct_fields (transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
            fn locate_struct_name(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            if kind == "KW_STRUCT" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                let mut struct_name: String = "".to_string();
                let mut name_pos: i64 = locate_struct_name(pre_i.clone());
                if name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                        struct_name = value;
                    }
                }
                let mut scan_pos: i64 = locate_struct_name(pre_i.clone());
                while scan_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let mut kind = scan_token.kind.clone();
                    if kind == "INDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                        break;
                    }
                    scan_pos = scan_pos + 1;
                }
                scan_pos = scan_pos + 1;
                let mut fields: Vec<String> = Vec::new();
                let mut depth: i64 = 0;
                while scan_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let mut kind = scan_token.kind.clone();
                    if kind == "INDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                        depth = depth + 1;
                    }
                    if kind == "DEDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                        if depth == 0 {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                            break;
                        }
                        depth = depth - 1;
                    }
                    if kind == "KW_RAW" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                        let mut raw_name_pos: i64 = scan_pos + 1;
                        if raw_name_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                            let mut raw_name_token: Token = tokens[raw_name_pos as usize].clone();
                            let mut kind = raw_name_token.kind.clone();
                            let mut value = raw_name_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                errors.push(val_err(raw_name_token.clone(), lbl_field.clone(), rule_no_raw_field.clone()).clone());
                            }
                        }
                    }
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                        let mut value = scan_token.value.clone();
                        let mut field_type: String = value.clone();
                        let mut field_name_pos: i64 = scan_pos + 1;
                        if field_name_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                            let mut field_name_token: Token = tokens[field_name_pos as usize].clone();
                            let mut kind = field_name_token.kind.clone();
                            let mut value = field_name_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                if (value.len() as i64) < 3 {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                    errors.push(val_err(field_name_token.clone(), lbl_field.clone(), rule_min3.clone()).clone());
                                }
                                if !is_snake(value.clone()) {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                    errors.push(val_err(field_name_token.clone(), lbl_field.clone(), rule_snake.clone()).clone());
                                }
                                if list_has(builtin_names.clone(), value.clone()) {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                    errors.push(val_err(field_name_token.clone(), lbl_field.clone(), rule_builtin_shadow.clone()).clone());
                                }
                                let mut is_func_field: bool = list_has(func_shape_names.clone(), field_type.clone());
                                if is_func_field {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                    errors.push(val_err(field_name_token.clone(), lbl_field.clone(), rule_no_func_field.clone()).clone());
                                }
                                fields.push(value.clone());
                            } else if list_has(reserved_keywords.clone(), kind.clone()) {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                                errors.push(val_err(field_name_token.clone(), lbl_field.clone(), rule_kw_in_parens.clone()).clone());
                            }
                        }
                    }
                    scan_pos = scan_pos + 1;
                }
                let mut has_name: bool = struct_name != "";
                if has_name {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_struct_fields.deor
                    let mut sep: String = ",".to_string();
                    let mut fields_str: String = s_join_with(fields.clone(), sep.clone());
                    struct_field_reg.push(struct_name.clone());
                    struct_field_reg.push(fields_str.clone());
                }
            }
        }
        // macro: prescan_check_enum_variants (transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor)
        {
            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
            fn locate_enum_type_or_name_slot(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            fn locate_enum_typed_name(kw_pos: i64) -> i64 {
                return kw_pos + 2;
            }
            if kind == "KW_ENUM" {
                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                let mut is_typed: bool = false;
                let mut scan_pos: i64 = locate_enum_type_or_name_slot(pre_i.clone());
                if scan_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                    let mut type_keyword_token: Token = tokens[scan_pos as usize].clone();
                    let mut value = type_keyword_token.value.clone();
                    is_typed = list_has(primitive_type_names.clone(), value.clone());
                }
                let mut name_pos: i64 = locate_enum_type_or_name_slot(pre_i.clone());
                if is_typed {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                    name_pos = locate_enum_typed_name(pre_i.clone());
                }
                if name_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                        enum_names.push(value.clone());
                    }
                }
                while scan_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let mut kind = scan_token.kind.clone();
                    if kind == "INDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                        break;
                    }
                    scan_pos = scan_pos + 1;
                }
                scan_pos = scan_pos + 1;
                while scan_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let mut kind = scan_token.kind.clone();
                    if kind == "DEDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                        break;
                    }
                    if kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                        let mut value = scan_token.value.clone();
                        enum_variant_names.push(value.clone());
                        if (value.len() as i64) < 3 {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                            errors.push(val_err(scan_token.clone(), lbl_variant.clone(), rule_min3.clone()).clone());
                        }
                        if !is_pascal(value.clone()) {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                            errors.push(val_err(scan_token.clone(), lbl_variant.clone(), rule_enum_pascal.clone()).clone());
                        }
                        let mut after_variant_pos: i64 = scan_pos + 1;
                        if after_variant_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                            let mut after_variant_token: Token = tokens[after_variant_pos as usize].clone();
                            let mut kind = after_variant_token.kind.clone();
                            if is_typed {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                                if kind != "EQUALS" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                                    errors.push(val_err(scan_token.clone(), lbl_variant.clone(), rule_typed_enum_eq.clone()).clone());
                                }
                            } else {
                                // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                                    errors.push(val_err(scan_token.clone(), lbl_variant.clone(), rule_enum_data.clone()).clone());
                                }
                                if kind == "EQUALS" {
                                    // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                                    errors.push(val_err(scan_token.clone(), lbl_variant.clone(), rule_untyped_enum_eq.clone()).clone());
                                }
                            }
                        }
                    } else if list_has(reserved_keywords.clone(), kind.clone()) {
                        // transpiler-deor/tokens_validator/macros/prescan/prescan_check_enum_variants.deor
                        errors.push(val_err(scan_token.clone(), lbl_variant.clone(), rule_kw_in_parens.clone()).clone());
                    }
                    scan_pos = scan_pos + 1;
                }
            }
        }
        // transpiler-deor/tokens_validator/macros/prescan/prescan_declared_state.deor
        pre_i = pre_i + 1;
    }
    // transpiler-deor/tokens_validator/tokens_validation.deor
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
        // macro: check_common_token_rules (transpiler-deor/tokens_validator/macros/check_common_token_rules.deor)
        {
            // macro: track_paren_depth (transpiler-deor/tokens_validator/macros/track/track_paren_depth.deor)
            {
                // transpiler-deor/tokens_validator/macros/track/track_paren_depth.deor
                if cur_kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/macros/track/track_paren_depth.deor
                    paren_depth = paren_depth + 1;
                }
                if cur_kind == "RPAREN" {
                    // transpiler-deor/tokens_validator/macros/track/track_paren_depth.deor
                    paren_depth = paren_depth - 1;
                }
            }
            // macro: track_block_scope (transpiler-deor/tokens_validator/macros/track/track_block_scope.deor)
            {
                // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                fn locate_return_type(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "INDENT" {
                    // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                    block_depth = block_depth + 1;
                }
                if cur_kind == "DEDENT" {
                    // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                    block_depth = block_depth - 1;
                    if block_depth == 0 {
                        // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                        in_struct_body = false;
                        in_enum_body = false;
                        in_fn_body = false;
                    }
                    let mut stack_len: i64 = (void_fn_stack.len() as i64);
                    if stack_len > 0 {
                        // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                        let mut top_idx: i64 = stack_len - 1;
                        let mut top_frame: VoidFnFrame = void_fn_stack[top_idx as usize].clone();
                        let mut depth = top_frame.depth.clone();
                        let mut prev_void = top_frame.prev_void.clone();
                        if depth == block_depth {
                            // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                            in_void_fn = prev_void;
                            void_fn_stack.remove(top_idx as usize);
                        }
                    }
                }
                if cur_kind == "KW_STRUCT" {
                    // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                    if block_depth == 0 {
                        // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                        in_struct_body = true;
                    }
                }
                if cur_kind == "KW_ENUM" {
                    // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                    if block_depth == 0 {
                        // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                        in_enum_body = true;
                    }
                }
                if cur_kind == "KW_FN" {
                    // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                    was_already_in_fn_body = in_fn_body;
                    in_fn_body = true;
                    if block_depth == 0 {
                        // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                        validator_vars.clear();
                        non_bool_var_names.clear();
                        string_var_names.clear();
                        copy_var_names.clear();
                        moved_vars.clear();
                        moved_fields.clear();
                    }
                    let mut depth: i64 = block_depth.clone();
                    let mut prev_void: bool = in_void_fn.clone();
                    let mut new_frame = VoidFnFrame { depth: depth.clone(), prev_void: prev_void.clone() };
                    void_fn_stack.push(new_frame.clone());
                    let mut return_type_pos: i64 = locate_return_type(pos.clone());
                    let mut is_void: bool = false;
                    if return_type_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                        let mut return_type_token: Token = tokens[return_type_pos as usize].clone();
                        let mut kind = return_type_token.kind.clone();
                        if kind == "KW_VOID" {
                            // transpiler-deor/tokens_validator/macros/track/track_block_scope.deor
                            is_void = true;
                        }
                    }
                    in_void_fn = is_void;
                }
            }
            // macro: check_void_return (transpiler-deor/tokens_validator/macros/syntax_rules/check_void_return.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_void_return.deor
                if cur_kind == "KW_RETURN" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_void_return.deor
                    if in_void_fn {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_void_return.deor
                        errors.push(val_err(tok.clone(), lbl_fn.clone(), rule_void_return.clone()).clone());
                    }
                }
            }
            // macro: check_return_invalid (transpiler-deor/tokens_validator/macros/syntax_rules/check_return_invalid.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_return_invalid.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_RETURN" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_return_invalid.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_return_invalid.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        if kind == "KW_EMPTY" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_return_invalid.deor
                            errors.push(val_err(next_token.clone(), lbl_fn.clone(), rule_return_empty.clone()).clone());
                        }
                        if kind == "KW_NONE" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_return_invalid.deor
                            errors.push(val_err(next_token.clone(), lbl_fn.clone(), rule_return_none.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_move_target (transpiler-deor/tokens_validator/macros/use_after_move/check_move_target.deor)
            {
                // transpiler-deor/tokens_validator/macros/use_after_move/check_move_target.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_MOVE" {
                    // transpiler-deor/tokens_validator/macros/use_after_move/check_move_target.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_move_target.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        let mut is_ident: bool = kind == "IDENT";
                        let mut is_lparen: bool = kind == "LPAREN";
                        let mut is_valid_target: bool = is_ident || is_lparen;
                        if !is_valid_target {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_move_target.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_move.clone()).clone());
                        }
                    }
                }
            }
            // macro: track_copy_vars (transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor)
            {
                // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                fn locate_var_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_equals(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_left_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                if cur_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                    let mut is_copy_kw: bool = cur_val == "int";
                    if !is_copy_kw {
                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                        is_copy_kw = cur_val == "float";
                    }
                    if !is_copy_kw {
                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                        is_copy_kw = cur_val == "bool";
                    }
                    if is_copy_kw {
                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                        let mut name_pos: i64 = locate_var_name(pos.clone());
                        let mut equals_pos: i64 = locate_equals(pos.clone());
                        if equals_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            let mut equals_token: Token = tokens[equals_pos as usize].clone();
                            let mut kind = name_token.kind.clone();
                            let mut name_kind: String = kind.clone();
                            let mut kind = equals_token.kind.clone();
                            if name_kind == "IDENT" && kind == "EQUALS" {
                                // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                let mut value = name_token.value.clone();
                                copy_var_names.push(value.clone());
                            }
                        }
                    }
                }
                if cur_kind == "KW_FN" {
                    // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                    let mut left_paren_pos: i64 = locate_left_paren(pos.clone());
                    if left_paren_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                        let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                        let mut kind = left_paren_token.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                            let mut param_scan_pos: i64 = left_paren_pos + 1;
                            while param_scan_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                let mut param_scan_token: Token = tokens[param_scan_pos as usize].clone();
                                let mut kind = param_scan_token.kind.clone();
                                let mut value = param_scan_token.value.clone();
                                if kind == "RPAREN" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                    break;
                                }
                                if kind == "COMMA" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                    param_scan_pos = param_scan_pos + 1;
                                    continue;
                                }
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                    let mut param_type: String = value.clone();
                                    let mut param_is_copy: bool = param_type == "int";
                                    if !param_is_copy {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                        param_is_copy = param_type == "float";
                                    }
                                    if !param_is_copy {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                        param_is_copy = param_type == "bool";
                                    }
                                    let mut param_name_pos: i64 = param_scan_pos + 1;
                                    if param_name_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                        let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                                        let mut kind = param_name_token.kind.clone();
                                        let mut value = param_name_token.value.clone();
                                        if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                            if param_is_copy {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/track_copy_vars.deor
                                                copy_var_names.push(value.clone());
                                            }
                                            param_scan_pos = param_name_pos;
                                        }
                                    }
                                }
                                param_scan_pos = param_scan_pos + 1;
                            }
                        }
                    }
                }
            }
            // macro: check_use_after_move (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move.deor)
            {
                // macro: check_use_after_move_chain (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor)
                {
                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                    fn locate_prev_token(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    if cur_kind == "KW_IF" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                        let mut is_elseif: bool = false;
                        if pos > 0 {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                            let mut prev_pos: i64 = locate_prev_token(pos.clone());
                            let mut prev_token: Token = tokens[prev_pos as usize].clone();
                            let mut kind = prev_token.kind.clone();
                            if kind == "KW_ELSE" {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                is_elseif = true;
                            }
                        }
                        if !is_elseif {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                            let mut chain_depth: i64 = block_depth.clone();
                            let mut pre_vars: Vec<String> = moved_vars.clone();
                            let mut acc_vars: Vec<String> = moved_vars.clone();
                            let mut new_frame = UamFrame { chain_depth: chain_depth.clone(), pre_vars: pre_vars.clone(), acc_vars: acc_vars.clone() };
                            move_if_stack.push(new_frame.clone());
                        }
                    }
                    if cur_kind == "KW_ELSE" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                        let mut stack_len: i64 = (move_if_stack.len() as i64);
                        if stack_len > 0 {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                            let mut top_idx: i64 = stack_len - 1;
                            let mut top_frame: UamFrame = move_if_stack[top_idx as usize].clone();
                            let mut chain_depth = top_frame.chain_depth.clone();
                            let mut pre_vars = top_frame.pre_vars.clone();
                            let mut acc_vars = top_frame.acc_vars.clone();
                            if chain_depth == block_depth {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                let mut arm_is_terminal: bool = false;
                                let mut line_scan: i64 = locate_prev_token(pos.clone());
                                while line_scan >= 0 {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                    let mut line_scan_token: Token = tokens[line_scan as usize].clone();
                                    let mut kind = line_scan_token.kind.clone();
                                    if kind == "NEWLINE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        line_scan = line_scan - 1;
                                    } else {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        break;
                                    }
                                }
                                let mut line_end: i64 = line_scan.clone();
                                while line_scan >= 0 {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                    let mut line_scan_token2: Token = tokens[line_scan as usize].clone();
                                    let mut kind = line_scan_token2.kind.clone();
                                    if kind == "NEWLINE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        break;
                                    }
                                    if kind == "INDENT" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        break;
                                    }
                                    if kind == "DEDENT" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        break;
                                    }
                                    line_scan = line_scan - 1;
                                }
                                let mut line_start: i64 = line_scan + 1;
                                if line_start <= line_end {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                    let mut line_first_token: Token = tokens[line_start as usize].clone();
                                    let mut kind = line_first_token.kind.clone();
                                    if kind == "KW_RETURN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        arm_is_terminal = true;
                                    }
                                    if kind == "KW_BREAK" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        arm_is_terminal = true;
                                    }
                                    if kind == "KW_CONTINUE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        arm_is_terminal = true;
                                    }
                                }
                                if !arm_is_terminal {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                    let mut move_index: i64 = 0;
                                    let mut move_count: i64 = (moved_vars.len() as i64);
                                    while move_index < move_count {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        let mut moved_name: String = moved_vars[move_index as usize].clone();
                                        let mut already_acc: bool = list_has(acc_vars.clone(), moved_name.clone());
                                        if !already_acc {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            acc_vars.push(moved_name.clone());
                                        }
                                        move_index = move_index + 1;
                                    }
                                }
                                move_if_stack.remove(top_idx as usize);
                                let mut updated_frame = UamFrame { chain_depth: chain_depth.clone(), pre_vars: pre_vars.clone(), acc_vars: acc_vars.clone() };
                                move_if_stack.push(updated_frame.clone());
                                moved_vars = pre_vars;
                            }
                        }
                    }
                    if cur_kind == "DEDENT" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                        let mut stack_len: i64 = (move_if_stack.len() as i64);
                        if stack_len > 0 {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                            let mut top_idx: i64 = stack_len - 1;
                            let mut top_frame: UamFrame = move_if_stack[top_idx as usize].clone();
                            let mut chain_depth = top_frame.chain_depth.clone();
                            let mut acc_vars = top_frame.acc_vars.clone();
                            if chain_depth == block_depth {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                let mut next_is_else: bool = false;
                                let mut after_dedent: i64 = locate_next_token(pos.clone());
                                if after_dedent < token_count {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                    let mut after_dedent_token: Token = tokens[after_dedent as usize].clone();
                                    let mut kind = after_dedent_token.kind.clone();
                                    if kind == "KW_ELSE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        next_is_else = true;
                                    }
                                }
                                if !next_is_else {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                    let mut arm_is_terminal: bool = false;
                                    let mut line_scan: i64 = locate_prev_token(pos.clone());
                                    while line_scan >= 0 {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        let mut line_scan_token: Token = tokens[line_scan as usize].clone();
                                        let mut kind = line_scan_token.kind.clone();
                                        if kind == "NEWLINE" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            line_scan = line_scan - 1;
                                        } else {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            break;
                                        }
                                    }
                                    let mut line_end: i64 = line_scan.clone();
                                    while line_scan >= 0 {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        let mut line_scan_token2: Token = tokens[line_scan as usize].clone();
                                        let mut kind = line_scan_token2.kind.clone();
                                        if kind == "NEWLINE" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            break;
                                        }
                                        if kind == "INDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            break;
                                        }
                                        if kind == "DEDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            break;
                                        }
                                        line_scan = line_scan - 1;
                                    }
                                    let mut line_start: i64 = line_scan + 1;
                                    if line_start <= line_end {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        let mut line_first_token: Token = tokens[line_start as usize].clone();
                                        let mut kind = line_first_token.kind.clone();
                                        if kind == "KW_RETURN" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            arm_is_terminal = true;
                                        }
                                        if kind == "KW_BREAK" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            arm_is_terminal = true;
                                        }
                                        if kind == "KW_CONTINUE" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            arm_is_terminal = true;
                                        }
                                    }
                                    if !arm_is_terminal {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                        let mut move_index: i64 = 0;
                                        let mut move_count: i64 = (moved_vars.len() as i64);
                                        while move_index < move_count {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                            let mut moved_name: String = moved_vars[move_index as usize].clone();
                                            let mut already_acc: bool = list_has(acc_vars.clone(), moved_name.clone());
                                            if !already_acc {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_chain.deor
                                                acc_vars.push(moved_name.clone());
                                            }
                                            move_index = move_index + 1;
                                        }
                                    }
                                    move_if_stack.remove(top_idx as usize);
                                    moved_vars = acc_vars;
                                }
                            }
                        }
                    }
                }
                // macro: check_use_after_move_field (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor)
                {
                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_prev_token(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    fn locate_backscan_start(kw_pos: i64) -> i64 {
                        return kw_pos - 2;
                    }
                    if cur_kind == "INDENT" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                        moved_fields.clear();
                    }
                    if cur_kind == "DEDENT" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                        moved_fields.clear();
                    }
                    if cur_kind == "KW_ELSE" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                        moved_fields.clear();
                    }
                    if cur_kind == "KW_MOVE" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                        let mut move_next_pos: i64 = locate_next_token(pos.clone());
                        if move_next_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                            let mut move_next_token: Token = tokens[move_next_pos as usize].clone();
                            let mut kind = move_next_token.kind.clone();
                            if kind == "LPAREN" {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                let mut move_fields: Vec<String> = Vec::new();
                                let mut scan_pos: i64 = locate_next_token(move_next_pos.clone());
                                let mut scanning: bool = true;
                                while scanning {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                    if scan_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                                        let mut kind = scan_token.kind.clone();
                                        let mut value = scan_token.value.clone();
                                        if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                            move_fields.push(value.clone());
                                        }
                                        if kind == "RPAREN" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                            scanning = false;
                                        }
                                        scan_pos = scan_pos + 1;
                                    } else {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                        scanning = false;
                                    }
                                }
                                if scan_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                    let mut in_token: Token = tokens[scan_pos as usize].clone();
                                    let mut kind = in_token.kind.clone();
                                    if kind == "KW_IN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                        let mut source_pos: i64 = scan_pos + 1;
                                        if source_pos < token_count {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                            let mut source_token: Token = tokens[source_pos as usize].clone();
                                            let mut kind = source_token.kind.clone();
                                            let mut value = source_token.value.clone();
                                            if kind == "IDENT" {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                let mut source_name: String = value.clone();
                                                let mut line = source_token.line.clone();
                                                let mut file = source_token.file.clone();
                                                let mut line_str: String = n_to_str(line.clone());
                                                let mut field_index: i64 = 0;
                                                let mut field_count: i64 = (move_fields.len() as i64);
                                                while field_index < field_count {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                    let mut field_name: String = move_fields[field_index as usize].clone();
                                                    let mut field_key: String = [source_name.as_str(), ".", field_name.as_str()].concat();
                                                    let mut already_moved: bool = list_has(moved_fields.clone(), field_key.clone());
                                                    if already_moved {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                        let mut msg_parts: Vec<String> = vec!["[validation] ".to_string(), file.clone(), " line ".to_string(), line_str.clone(), ": ".to_string(), lbl_var.clone(), " '".to_string(), field_key.clone(), "' - ".to_string(), rule_double_move.clone()];
                                                        errors.push(s_join(msg_parts.clone()).clone());
                                                    } else {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                        moved_fields.push(field_key.clone());
                                                    }
                                                    field_index = field_index + 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if cur_kind == "KW_IN" {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                        if pos > 0 {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                            let mut prev_pos: i64 = locate_prev_token(pos.clone());
                            let mut prev_token: Token = tokens[prev_pos as usize].clone();
                            let mut kind = prev_token.kind.clone();
                            if kind == "RPAREN" {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                let mut next_pos: i64 = locate_next_token(pos.clone());
                                if next_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                    let mut next_token: Token = tokens[next_pos as usize].clone();
                                    let mut kind = next_token.kind.clone();
                                    let mut value = next_token.value.clone();
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                        let mut after_pos: i64 = locate_next_token(next_pos.clone());
                                        let mut is_bare: bool = true;
                                        if after_pos < token_count {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                            let mut after_token: Token = tokens[after_pos as usize].clone();
                                            let mut kind = after_token.kind.clone();
                                            if kind == "LPAREN" {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                is_bare = false;
                                            }
                                        }
                                        if is_bare {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                            let mut source_name: String = value.clone();
                                            let mut fields: Vec<String> = Vec::new();
                                            let mut backscan_pos: i64 = locate_backscan_start(pos.clone());
                                            let mut paren_depth: i64 = 0;
                                            while backscan_pos >= 0 {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                let mut backscan_token: Token = tokens[backscan_pos as usize].clone();
                                                let mut kind = backscan_token.kind.clone();
                                                let mut value = backscan_token.value.clone();
                                                if kind == "RPAREN" {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                    paren_depth = paren_depth + 1;
                                                } else if kind == "LPAREN" {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                    if paren_depth == 0 {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                        break;
                                                    }
                                                    paren_depth = paren_depth - 1;
                                                } else if kind == "IDENT" {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                    if paren_depth == 0 {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                        fields.push(value.clone());
                                                    }
                                                }
                                                backscan_pos = backscan_pos - 1;
                                            }
                                            let mut is_move_stmt: bool = false;
                                            if backscan_pos > 0 {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                let mut before_lparen_pos: i64 = backscan_pos - 1;
                                                let mut before_lparen_token: Token = tokens[before_lparen_pos as usize].clone();
                                                let mut kind = before_lparen_token.kind.clone();
                                                if kind == "KW_MOVE" {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                    is_move_stmt = true;
                                                }
                                            }
                                            if !is_move_stmt {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                let mut field_index: i64 = 0;
                                                let mut field_count: i64 = (fields.len() as i64);
                                                while field_index < field_count {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                    let mut field_name: String = fields[field_index as usize].clone();
                                                    let mut field_key: String = [source_name.as_str(), ".", field_name.as_str()].concat();
                                                    let mut was_moved: bool = list_has(moved_fields.clone(), field_key.clone());
                                                    if was_moved {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_field.deor
                                                        let mut line = next_token.line.clone();
                                                        let mut file = next_token.file.clone();
                                                        let mut line_str: String = n_to_str(line.clone());
                                                        let mut msg_parts: Vec<String> = vec!["[validation] ".to_string(), file.clone(), " line ".to_string(), line_str.clone(), ": ".to_string(), lbl_var.clone(), " '".to_string(), field_key.clone(), "' - ".to_string(), rule_use_after_move.clone()];
                                                        errors.push(s_join(msg_parts.clone()).clone());
                                                    }
                                                    field_index = field_index + 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // macro: check_use_after_move_var (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var.deor)
                {
                    // macro: check_use_after_move_var_for (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor)
                    {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                        fn locate_next_token(kw_pos: i64) -> i64 {
                            return kw_pos + 1;
                        }
                        if cur_kind == "KW_FOR" {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                            let mut move_pos: i64 = locate_next_token(pos.clone());
                            if move_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                let mut move_token: Token = tokens[move_pos as usize].clone();
                                let mut kind = move_token.kind.clone();
                                if kind == "KW_MOVE" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                    let mut var_pos: i64 = move_pos + 2;
                                    let mut in_pos: i64 = locate_next_token(var_pos.clone());
                                    let mut collection_pos: i64 = locate_next_token(in_pos.clone());
                                    if collection_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                        let mut collection_token: Token = tokens[collection_pos as usize].clone();
                                        let mut kind = collection_token.kind.clone();
                                        let mut value = collection_token.value.clone();
                                        if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                            let mut after_collection: i64 = locate_next_token(collection_pos.clone());
                                            let mut collection_is_bare: bool = true;
                                            if after_collection < token_count {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                                let mut after_token: Token = tokens[after_collection as usize].clone();
                                                let mut kind = after_token.kind.clone();
                                                if kind == "LPAREN" {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                                    collection_is_bare = false;
                                                }
                                            }
                                            if collection_is_bare {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                                let mut already_moved: bool = list_has(moved_vars.clone(), value.clone());
                                                if already_moved {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                                    errors.push(val_err(collection_token.clone(), lbl_var.clone(), rule_use_after_move.clone()).clone());
                                                } else {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_for.deor
                                                    moved_vars.push(value.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // macro: check_use_after_move_var_move (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor)
                    {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                        fn locate_next_token(kw_pos: i64) -> i64 {
                            return kw_pos + 1;
                        }
                        fn locate_prev_token(kw_pos: i64) -> i64 {
                            return kw_pos - 1;
                        }
                        if cur_kind == "KW_MOVE" {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                            let mut next_pos: i64 = locate_next_token(pos.clone());
                            if next_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                let mut next_token: Token = tokens[next_pos as usize].clone();
                                let mut kind = next_token.kind.clone();
                                let mut value = next_token.value.clone();
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                    let mut in_sjoin: bool = false;
                                    let mut scan_pos: i64 = locate_prev_token(pos.clone());
                                    let mut bracket_depth: i64 = 0;
                                    while scan_pos >= 0 {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                                        let mut kind = scan_token.kind.clone();
                                        if kind == "RBRACKET" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            bracket_depth = bracket_depth + 1;
                                        } else if kind == "LBRACKET" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            if bracket_depth == 0 {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                if scan_pos >= 2 {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                    let mut lparen_pos: i64 = scan_pos - 1;
                                                    let mut func_pos: i64 = scan_pos - 2;
                                                    let mut lparen_token: Token = tokens[lparen_pos as usize].clone();
                                                    let mut func_token: Token = tokens[func_pos as usize].clone();
                                                    let mut kind = lparen_token.kind.clone();
                                                    let mut lparen_ok: bool = kind == "LPAREN";
                                                    let mut kind = func_token.kind.clone();
                                                    let mut value = func_token.value.clone();
                                                    if lparen_ok {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                        if kind == "IDENT" {
                                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                            if value == "s_join" {
                                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                                in_sjoin = true;
                                                            }
                                                        }
                                                    }
                                                }
                                                break;
                                            } else {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                bracket_depth = bracket_depth - 1;
                                            }
                                        }
                                        scan_pos = scan_pos - 1;
                                    }
                                    let mut value = next_token.value.clone();
                                    let mut move_name: String = value.clone();
                                    let mut self_reassign: bool = false;
                                    let mut line_scan_pos: i64 = locate_prev_token(pos.clone());
                                    while line_scan_pos >= 0 {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                        let mut line_scan_token: Token = tokens[line_scan_pos as usize].clone();
                                        let mut kind = line_scan_token.kind.clone();
                                        if kind == "NEWLINE" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            break;
                                        }
                                        if kind == "INDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            break;
                                        }
                                        if kind == "DEDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            break;
                                        }
                                        line_scan_pos = line_scan_pos - 1;
                                    }
                                    let mut line_first_pos: i64 = line_scan_pos + 1;
                                    if line_first_pos < pos {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                        let mut line_first_token: Token = tokens[line_first_pos as usize].clone();
                                        let mut kind = line_first_token.kind.clone();
                                        let mut value = line_first_token.value.clone();
                                        if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            let mut line_first_name: String = value.clone();
                                            let mut line_first_next: i64 = line_first_pos + 1;
                                            if line_first_next < token_count {
                                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                let mut line_first_next_token: Token = tokens[line_first_next as usize].clone();
                                                let mut kind = line_first_next_token.kind.clone();
                                                if kind == "EQUALS" {
                                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                    if line_first_name == move_name {
                                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                                        self_reassign = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    let mut value = next_token.value.clone();
                                    let mut is_copy: bool = list_has(copy_var_names.clone(), value.clone());
                                    if in_sjoin {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                        is_copy = true;
                                    }
                                    if self_reassign {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                        is_copy = true;
                                    }
                                    if !is_copy {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                        let mut already_moved: bool = list_has(moved_vars.clone(), value.clone());
                                        if already_moved {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            errors.push(val_err(next_token.clone(), lbl_var.clone(), rule_double_move.clone()).clone());
                                        } else {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_move.deor
                                            moved_vars.push(value.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // macro: check_use_after_move_var_ident (transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor)
                    {
                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                        fn locate_next_token(kw_pos: i64) -> i64 {
                            return kw_pos + 1;
                        }
                        fn locate_prev_token(kw_pos: i64) -> i64 {
                            return kw_pos - 1;
                        }
                        if cur_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                            let mut next_pos: i64 = locate_next_token(pos.clone());
                            let mut is_binding: bool = false;
                            if next_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                let mut next_token: Token = tokens[next_pos as usize].clone();
                                let mut kind = next_token.kind.clone();
                                if kind == "EQUALS" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                    is_binding = true;
                                }
                                if kind == "KW_AS" {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                    is_binding = true;
                                }
                            }
                            if is_binding {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                let mut name: String = cur_val.clone();
                                let mut field_prefix: String = [name.as_str(), "."].concat();
                                moved_vars.retain(|x| x != &name);
                                moved_fields.retain(|x| !x.starts_with(&field_prefix));
                            } else {
                                // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                let mut skip_read: bool = in_struct_body.clone();
                                if in_enum_body {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                    skip_read = true;
                                }
                                let mut next_is_rparen: bool = false;
                                if next_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                    let mut next_token2: Token = tokens[next_pos as usize].clone();
                                    let mut kind = next_token2.kind.clone();
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "LPAREN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_IN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "RPAREN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        next_is_rparen = true;
                                    }
                                }
                                if pos > 0 {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                    let mut prev_pos: i64 = locate_prev_token(pos.clone());
                                    let mut prev_token: Token = tokens[prev_pos as usize].clone();
                                    let mut kind = prev_token.kind.clone();
                                    if kind == "KW_MOVE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_IN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        if next_is_rparen {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                            skip_read = true;
                                        }
                                    }
                                    if kind == "KW_STRUCT" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_ENUM" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_SHAPE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_TYPE" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_FN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_MACRO" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_MACRO_RUN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_UNSAFE_MACRO_RUN" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_UNSAFE_MACRO" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_OF" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_TO" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                    if kind == "KW_RAW" {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        skip_read = true;
                                    }
                                }
                                if !skip_read {
                                    // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                    let mut was_moved: bool = list_has(moved_vars.clone(), cur_val.clone());
                                    if was_moved {
                                        // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                        let mut is_copy: bool = list_has(copy_var_names.clone(), cur_val.clone());
                                        if !is_copy {
                                            // transpiler-deor/tokens_validator/macros/use_after_move/check_use_after_move_var_ident.deor
                                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_use_after_move.clone()).clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_keyword_in_parens (transpiler-deor/tokens_validator/macros/brackets_parens/check_keyword_in_parens.deor)
            {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_keyword_in_parens.deor
                if paren_depth > 0 {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_keyword_in_parens.deor
                    let mut is_forbidden: bool = list_has(forbidden_in_parens.clone(), cur_kind.clone());
                    if is_forbidden {
                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_keyword_in_parens.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
                    }
                }
            }
            // macro: check_kw_as_name (transpiler-deor/tokens_validator/macros/idents/check_kw_as_name.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/check_kw_as_name.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                let mut is_reserved: bool = list_has(reserved_keywords.clone(), cur_kind.clone());
                if is_reserved {
                    // transpiler-deor/tokens_validator/macros/idents/check_kw_as_name.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/check_kw_as_name.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        let mut next_is_eq: bool = kind == "EQUALS";
                        let mut next_is_as: bool = kind == "KW_AS";
                        if next_is_eq || next_is_as {
                            // transpiler-deor/tokens_validator/macros/idents/check_kw_as_name.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_destructure_binding_kw (transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                fn locate_prev_token(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                    let mut is_call: bool = false;
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                        let mut prev_pos: i64 = locate_prev_token(pos.clone());
                        let mut prev_token: Token = tokens[prev_pos as usize].clone();
                        let mut kind = prev_token.kind.clone();
                        is_call = kind == "IDENT";
                    }
                    if !is_call {
                        // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                        let mut scan_pos: i64 = locate_next_token(pos.clone());
                        let mut depth: i64 = 1;
                        while scan_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                            let mut scan_token: Token = tokens[scan_pos as usize].clone();
                            let mut kind = scan_token.kind.clone();
                            if kind == "LPAREN" {
                                // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                depth = depth + 1;
                            } else if kind == "RPAREN" {
                                // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                depth = depth - 1;
                                if depth == 0 {
                                    // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                    break;
                                }
                            }
                            scan_pos = scan_pos + 1;
                        }
                        let mut after_pos: i64 = scan_pos + 1;
                        let mut is_destructure: bool = false;
                        if after_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                            let mut after_token: Token = tokens[after_pos as usize].clone();
                            let mut kind = after_token.kind.clone();
                            is_destructure = kind == "KW_IN";
                        }
                        if is_destructure {
                            // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                            let mut name_pos: i64 = locate_next_token(pos.clone());
                            let mut at_name_slot: bool = true;
                            while name_pos < scan_pos {
                                // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                let mut name_token: Token = tokens[name_pos as usize].clone();
                                let mut kind = name_token.kind.clone();
                                let mut value = name_token.value.clone();
                                if kind == "COMMA" {
                                    // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                    at_name_slot = true;
                                } else if at_name_slot {
                                    // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                        if list_has(builtin_names.clone(), value.clone()) {
                                            // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                            errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_builtin_shadow.clone()).clone());
                                        }
                                    } else if list_has(reserved_keywords.clone(), kind.clone()) {
                                        // transpiler-deor/tokens_validator/macros/idents/check_destructure_binding_kw.deor
                                        errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_kw_in_parens.clone()).clone());
                                    }
                                    at_name_slot = false;
                                }
                                name_pos = name_pos + 1;
                            }
                        }
                    }
                }
            }
            // macro: check_with_parens (transpiler-deor/tokens_validator/macros/brackets_parens/check_with_parens.deor)
            {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_with_parens.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_WITH" {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_with_parens.deor
                    let mut with_ok: bool = false;
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_with_parens.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        with_ok = kind == "LPAREN";
                    }
                    if !with_ok {
                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_with_parens.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_with_parens.clone()).clone());
                    }
                }
            }
            // macro: check_empty_parens (transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor)
            {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_prev_token(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                fn locate_prev2(kw_pos: i64) -> i64 {
                    return kw_pos - 2;
                }
                if cur_kind == "LPAREN" {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                            let mut is_fn: bool = false;
                            if pos > 0 {
                                // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                                let mut prev_pos: i64 = locate_prev_token(pos.clone());
                                let mut prev_token: Token = tokens[prev_pos as usize].clone();
                                let mut kind = prev_token.kind.clone();
                                let mut prev_is_ident: bool = kind == "IDENT";
                                if prev_is_ident {
                                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                                    let mut is_type_decl: bool = false;
                                    if pos > 1 {
                                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                                        let mut prev2_pos: i64 = locate_prev2(pos.clone());
                                        let mut prev2_token: Token = tokens[prev2_pos as usize].clone();
                                        let mut kind = prev2_token.kind.clone();
                                        is_type_decl = kind == "KW_TYPE";
                                    }
                                    is_fn = !is_type_decl;
                                }
                            }
                            if !is_fn {
                                // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_parens.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_empty_parens.clone()).clone());
                            }
                        }
                    }
                }
            }
            // macro: skip_rust_block (transpiler-deor/tokens_validator/macros/skip_rust_block.deor)
            {
                // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
                fn locate_newline(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_rust_block(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                if cur_kind == "KW_RUST" {
                    // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
                    let mut newline_pos: i64 = locate_newline(pos.clone());
                    let mut block_pos: i64 = locate_rust_block(pos.clone());
                    let mut is_block: bool = false;
                    if block_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
                        let mut newline_token: Token = tokens[newline_pos as usize].clone();
                        let mut block_token: Token = tokens[block_pos as usize].clone();
                        let mut kind = newline_token.kind.clone();
                        let mut newline_ok: bool = kind == "NEWLINE";
                        let mut kind = block_token.kind.clone();
                        let mut block_ok: bool = kind == "RUST_BLOCK";
                        is_block = newline_ok && block_ok;
                    }
                    if is_block {
                        // transpiler-deor/tokens_validator/macros/skip_rust_block.deor
                        pos = block_pos + 1;
                        continue;
                    }
                }
            }
            // macro: check_not_is_order (transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_NOT" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        let mut end_pos: i64 = next_pos.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                            end_pos = find_matching_rparen(tokens.clone(), next_pos.clone());
                        } else if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                            let mut peek_pos: i64 = locate_next_token(next_pos.clone());
                            if peek_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                                let mut peek_token: Token = tokens[peek_pos as usize].clone();
                                let mut kind = peek_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                                    end_pos = find_matching_rparen(tokens.clone(), peek_pos.clone());
                                }
                            }
                        }
                        let mut after_pos: i64 = end_pos + 1;
                        if after_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                            let mut after_token: Token = tokens[after_pos as usize].clone();
                            let mut kind = after_token.kind.clone();
                            if kind == "KW_IS" {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_not_is_order.deor
                                errors.push(val_err(next_token.clone(), lbl_var.clone(), rule_not_is.clone()).clone());
                            }
                        }
                    }
                }
            }
            // macro: check_bare_truthiness (transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_IF" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        let mut value = next_token.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                            let mut after_pos: i64 = locate_next_token(next_pos.clone());
                            if after_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                                let mut after_token: Token = tokens[after_pos as usize].clone();
                                let mut kind = after_token.kind.clone();
                                if kind == "NEWLINE" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                                    let mut is_non_bool: bool = list_has(non_bool_var_names.clone(), value.clone());
                                    let mut is_validator_var: bool = list_has(validator_vars.clone(), value.clone());
                                    if is_non_bool || is_validator_var {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_truthiness.deor
                                        errors.push(val_err(next_token.clone(), lbl_var.clone(), rule_bare_truthiness.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_string_plus_banned (transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_prev_token(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                if cur_kind == "PLUS" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                        let mut right_pos: i64 = locate_next_token(pos.clone());
                        if right_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                            let mut left_pos: i64 = locate_prev_token(pos.clone());
                            let mut left_token: Token = tokens[left_pos as usize].clone();
                            let mut right_token: Token = tokens[right_pos as usize].clone();
                            let mut kind = left_token.kind.clone();
                            let mut value = left_token.value.clone();
                            let mut left_kind: String = kind.clone();
                            let mut left_value: String = value.clone();
                            let mut kind = right_token.kind.clone();
                            let mut value = right_token.value.clone();
                            let mut right_kind: String = kind.clone();
                            let mut right_value: String = value.clone();
                            let mut left_is_str: bool = left_kind == "STRING";
                            if !left_is_str {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                                if left_kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                                    left_is_str = list_has(string_var_names.clone(), left_value.clone());
                                }
                            }
                            let mut right_is_str: bool = right_kind == "STRING";
                            if !right_is_str {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                                if right_kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                                    right_is_str = list_has(string_var_names.clone(), right_value.clone());
                                }
                            }
                            if left_is_str || right_is_str {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_string_plus_banned.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_string_plus_banned.clone()).clone());
                            }
                        }
                    }
                }
            }
            // macro: check_double_equals (transpiler-deor/tokens_validator/macros/syntax_rules/check_double_equals.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_double_equals.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "EQUALS" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_double_equals.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_double_equals.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        if kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_double_equals.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_double_equals.clone()).clone());
                        }
                    }
                }
            }
        }
        // macro: check_struct_decl (transpiler-deor/tokens_validator/macros/declarations/check_struct_decl.deor)
        {
            // transpiler-deor/tokens_validator/macros/declarations/check_struct_decl.deor
            let mut validate_indent_offset: i64 = 1;
            let mut keyword: String = "KW_STRUCT".to_string();
            let mut lbl: String = lbl_struct.clone();
            let mut rule: String = rule_pascal.clone();
            let mut test_rule: fn(String) -> bool = is_pascal.clone();
            // macro: validate_ident (transpiler-deor/tokens_validator/macros/idents/validate_ident.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                if cur_kind == keyword {
                    // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                    let mut name_pos: i64 = pos + validate_indent_offset;
                    if name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                        let mut name_tok: Token = tokens[name_pos as usize].clone();
                        let mut kind = name_tok.kind.clone();
                        let mut value = name_tok.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_val: String = value.clone();
                        if name_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            if (name_val.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                            }
                            if !test_rule(name_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                            }
                        } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                    pos = pos + 1;
                    continue;
                }
            }
        }
        // macro: check_enum_decl (transpiler-deor/tokens_validator/macros/declarations/check_enum_decl.deor)
        {
            // transpiler-deor/tokens_validator/macros/declarations/check_enum_decl.deor
            fn locate_enum_type_keyword(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            let mut keyword: String = "KW_ENUM".to_string();
            let mut lbl: String = lbl_enum.clone();
            let mut rule: String = rule_pascal.clone();
            let mut test_rule: fn(String) -> bool = is_pascal.clone();
            let mut validate_indent_offset: i64 = 1;
            if cur_kind == "KW_ENUM" {
                // transpiler-deor/tokens_validator/macros/declarations/check_enum_decl.deor
                let mut type_keyword_pos: i64 = locate_enum_type_keyword(pos.clone());
                if type_keyword_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/declarations/check_enum_decl.deor
                    let mut type_keyword_token: Token = tokens[type_keyword_pos as usize].clone();
                    let mut value = type_keyword_token.value.clone();
                    if list_has(primitive_type_names.clone(), value.clone()) {
                        // transpiler-deor/tokens_validator/macros/declarations/check_enum_decl.deor
                        validate_indent_offset = 2;
                    }
                }
            }
            // macro: validate_ident (transpiler-deor/tokens_validator/macros/idents/validate_ident.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                if cur_kind == keyword {
                    // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                    let mut name_pos: i64 = pos + validate_indent_offset;
                    if name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                        let mut name_tok: Token = tokens[name_pos as usize].clone();
                        let mut kind = name_tok.kind.clone();
                        let mut value = name_tok.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_val: String = value.clone();
                        if name_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            if (name_val.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                            }
                            if !test_rule(name_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                            }
                        } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                    pos = pos + 1;
                    continue;
                }
            }
        }
        // macro: check_shape_decl (transpiler-deor/tokens_validator/macros/declarations/check_shape_decl.deor)
        {
            // macro: check_func_shape_multi_param (transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor)
            {
                // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                fn locate_shape_form(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                fn locate_func_of_or_to(kw_pos: i64) -> i64 {
                    return kw_pos + 4;
                }
                fn locate_func_in_type(kw_pos: i64) -> i64 {
                    return kw_pos + 5;
                }
                fn locate_func_to_after_of(kw_pos: i64) -> i64 {
                    return kw_pos + 6;
                }
                fn locate_func_out_type_after_of(kw_pos: i64) -> i64 {
                    return kw_pos + 7;
                }
                fn locate_func_out_type_after_to(kw_pos: i64) -> i64 {
                    return kw_pos + 5;
                }
                if cur_kind == "KW_SHAPE" {
                    // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                    let mut form_pos: i64 = locate_shape_form(pos.clone());
                    if form_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                        let mut form_token: Token = tokens[form_pos as usize].clone();
                        let mut kind = form_token.kind.clone();
                        if kind == "KW_FUNC" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                            let mut of_or_to_pos: i64 = locate_func_of_or_to(pos.clone());
                            if of_or_to_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                let mut of_or_to_token: Token = tokens[of_or_to_pos as usize].clone();
                                let mut kind = of_or_to_token.kind.clone();
                                if kind == "KW_OF" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                    let mut in_type_pos: i64 = locate_func_in_type(pos.clone());
                                    if in_type_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                        let mut in_type_token: Token = tokens[in_type_pos as usize].clone();
                                        let mut kind = in_type_token.kind.clone();
                                        if kind == "LPAREN" {
                                            // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                            errors.push(val_err(tok.clone(), lbl_shape.clone(), rule_func_shape_multi_param.clone()).clone());
                                        }
                                    }
                                    let mut to_pos: i64 = locate_func_to_after_of(pos.clone());
                                    if to_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                        let mut to_token: Token = tokens[to_pos as usize].clone();
                                        let mut kind = to_token.kind.clone();
                                        if kind == "KW_TO" {
                                            // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                            let mut out_type_pos: i64 = locate_func_out_type_after_of(pos.clone());
                                            if out_type_pos < token_count {
                                                // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                                let mut out_type_token: Token = tokens[out_type_pos as usize].clone();
                                                let mut kind = out_type_token.kind.clone();
                                                if kind == "LPAREN" {
                                                    // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                                    errors.push(val_err(tok.clone(), lbl_shape.clone(), rule_func_shape_multi_param.clone()).clone());
                                                }
                                            }
                                        }
                                    }
                                } else if kind == "KW_TO" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                    let mut out_type_pos: i64 = locate_func_out_type_after_to(pos.clone());
                                    if out_type_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                        let mut out_type_token: Token = tokens[out_type_pos as usize].clone();
                                        let mut kind = out_type_token.kind.clone();
                                        if kind == "LPAREN" {
                                            // transpiler-deor/tokens_validator/macros/declarations/check_func_shape_multi_param.deor
                                            errors.push(val_err(tok.clone(), lbl_shape.clone(), rule_func_shape_multi_param.clone()).clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // transpiler-deor/tokens_validator/macros/declarations/check_shape_decl.deor
            let mut keyword: String = "KW_SHAPE".to_string();
            let mut lbl: String = lbl_shape.clone();
            let mut rule: String = rule_camel.clone();
            let mut test_rule: fn(String) -> bool = is_camel.clone();
            let mut validate_indent_offset: i64 = 1;
            // macro: validate_ident (transpiler-deor/tokens_validator/macros/idents/validate_ident.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                if cur_kind == keyword {
                    // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                    let mut name_pos: i64 = pos + validate_indent_offset;
                    if name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                        let mut name_tok: Token = tokens[name_pos as usize].clone();
                        let mut kind = name_tok.kind.clone();
                        let mut value = name_tok.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_val: String = value.clone();
                        if name_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            if (name_val.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                            }
                            if !test_rule(name_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                            }
                        } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                    pos = pos + 1;
                    continue;
                }
            }
        }
        // macro: check_type_decl (transpiler-deor/tokens_validator/macros/declarations/check_type_decl.deor)
        {
            // macro: check_type_base_primitive (transpiler-deor/tokens_validator/macros/declarations/check_type_base_primitive.deor)
            {
                // transpiler-deor/tokens_validator/macros/declarations/check_type_base_primitive.deor
                fn locate_base_type(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                fn locate_type_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_TYPE" {
                    // transpiler-deor/tokens_validator/macros/declarations/check_type_base_primitive.deor
                    let mut base_type_pos: i64 = locate_base_type(pos.clone());
                    if base_type_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/declarations/check_type_base_primitive.deor
                        let mut base_type_token: Token = tokens[base_type_pos as usize].clone();
                        let mut value = base_type_token.value.clone();
                        let mut is_primitive: bool = list_has(primitive_type_names.clone(), value.clone());
                        if !is_primitive {
                            // transpiler-deor/tokens_validator/macros/declarations/check_type_base_primitive.deor
                            let mut name_pos: i64 = locate_type_name(pos.clone());
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            errors.push(val_err(name_token.clone(), lbl_type.clone(), rule_non_primitive_validator.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_validator_declaration (transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor)
            {
                // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                fn locate_type_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_type_left_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_type_param_type(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                fn locate_type_param_name(kw_pos: i64) -> i64 {
                    return kw_pos + 4;
                }
                fn locate_type_newline(kw_pos: i64) -> i64 {
                    return kw_pos + 6;
                }
                fn locate_type_body(kw_pos: i64) -> i64 {
                    return kw_pos + 7;
                }
                if cur_kind == "KW_TYPE" {
                    // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                    let mut type_name_pos: i64 = locate_type_name(pos.clone());
                    let mut left_paren_pos: i64 = locate_type_left_paren(pos.clone());
                    let mut param_type_pos: i64 = locate_type_param_type(pos.clone());
                    let mut param_name_pos: i64 = locate_type_param_name(pos.clone());
                    if param_name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                        let mut type_name_token: Token = tokens[type_name_pos as usize].clone();
                        let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                        let mut param_type_token: Token = tokens[param_type_pos as usize].clone();
                        let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                        let mut kind = type_name_token.kind.clone();
                        let mut value = type_name_token.value.clone();
                        let mut type_name: String = value.clone();
                        let mut kind = left_paren_token.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                            let mut value = param_type_token.value.clone();
                            let mut param_type: String = value.clone();
                            let mut kind = param_name_token.kind.clone();
                            let mut value = param_name_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                if value == type_name {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                    errors.push(val_err(param_name_token.clone(), lbl_type.clone(), rule_type_param_shadow.clone()).clone());
                                }
                                if value == param_type {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                    errors.push(val_err(param_name_token.clone(), lbl_type.clone(), rule_param_shadow.clone()).clone());
                                }
                                if list_has(builtin_names.clone(), value.clone()) {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                    errors.push(val_err(param_name_token.clone(), lbl_type.clone(), rule_builtin_shadow.clone()).clone());
                                }
                            }
                            let mut newline_pos: i64 = locate_type_newline(pos.clone());
                            let mut body_pos: i64 = locate_type_body(pos.clone());
                            let mut has_body: bool = false;
                            if body_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                let mut newline_token: Token = tokens[newline_pos as usize].clone();
                                let mut kind = newline_token.kind.clone();
                                if kind == "NEWLINE" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                    let mut body_token: Token = tokens[body_pos as usize].clone();
                                    let mut kind = body_token.kind.clone();
                                    if kind == "INDENT" {
                                        // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                        has_body = true;
                                    }
                                }
                            }
                            if !has_body {
                                // transpiler-deor/tokens_validator/macros/declarations/check_validator_declaration.deor
                                errors.push(val_err(type_name_token.clone(), lbl_type.clone(), rule_validator_missing_body.clone()).clone());
                            }
                        }
                    }
                }
            }
            // transpiler-deor/tokens_validator/macros/declarations/check_type_decl.deor
            let mut keyword: String = "KW_TYPE".to_string();
            let mut lbl: String = lbl_type.clone();
            let mut rule: String = rule_pascal.clone();
            let mut test_rule: fn(String) -> bool = is_pascal.clone();
            let mut validate_indent_offset: i64 = 1;
            // macro: validate_ident (transpiler-deor/tokens_validator/macros/idents/validate_ident.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                if cur_kind == keyword {
                    // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                    let mut name_pos: i64 = pos + validate_indent_offset;
                    if name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                        let mut name_tok: Token = tokens[name_pos as usize].clone();
                        let mut kind = name_tok.kind.clone();
                        let mut value = name_tok.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_val: String = value.clone();
                        if name_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            if (name_val.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                            }
                            if !test_rule(name_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                            }
                        } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                    pos = pos + 1;
                    continue;
                }
            }
        }
        // macro: track_non_bool_vars (transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor)
        {
            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
            fn locate_var_name(kw_pos: i64) -> i64 {
                return kw_pos + 1;
            }
            fn locate_equals(kw_pos: i64) -> i64 {
                return kw_pos + 2;
            }
            fn locate_left_paren(kw_pos: i64) -> i64 {
                return kw_pos + 3;
            }
            if cur_kind == "IDENT" {
                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                let mut name_pos: i64 = locate_var_name(pos.clone());
                let mut equals_pos: i64 = locate_equals(pos.clone());
                if equals_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let mut equals_token: Token = tokens[equals_pos as usize].clone();
                    let mut kind = name_token.kind.clone();
                    let mut name_kind: String = kind.clone();
                    let mut kind = equals_token.kind.clone();
                    let mut equals_kind: String = kind.clone();
                    if name_kind == "IDENT" && equals_kind == "EQUALS" {
                        // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                        let mut is_bool: bool = cur_val == "bool";
                        let mut is_vtype: bool = list_has(validator_type_names.clone(), cur_val.clone());
                        if !is_bool {
                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                            if !is_vtype {
                                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                let mut value = name_token.value.clone();
                                non_bool_var_names.push(value.clone());
                            }
                        }
                        if cur_val == "string" {
                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                            let mut value = name_token.value.clone();
                            string_var_names.push(value.clone());
                        }
                    }
                    if name_kind == "KW_AS" {
                        // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                        let mut as_is_str: bool = equals_kind == "STRING";
                        if !as_is_str {
                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                            if equals_kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                let mut value = equals_token.value.clone();
                                as_is_str = list_has(string_var_names.clone(), value.clone());
                            }
                        }
                        if as_is_str {
                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                            string_var_names.push(cur_val.clone());
                        }
                    }
                }
            }
            if cur_kind == "KW_FN" {
                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                let mut left_paren_pos: i64 = locate_left_paren(pos.clone());
                if left_paren_pos < token_count {
                    // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                    let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                    let mut kind = left_paren_token.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                        let mut param_scan_pos: i64 = left_paren_pos + 1;
                        while param_scan_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                            let mut param_scan_token: Token = tokens[param_scan_pos as usize].clone();
                            let mut kind = param_scan_token.kind.clone();
                            let mut value = param_scan_token.value.clone();
                            if kind == "RPAREN" {
                                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                break;
                            }
                            if kind == "COMMA" {
                                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                param_scan_pos = param_scan_pos + 1;
                                continue;
                            }
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                let mut param_type: String = value.clone();
                                let mut param_name_pos: i64 = param_scan_pos + 1;
                                if param_name_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                    let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                                    let mut kind = param_name_token.kind.clone();
                                    let mut value = param_name_token.value.clone();
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                        let mut param_is_bool: bool = param_type == "bool";
                                        let mut param_is_vtype: bool = list_has(validator_type_names.clone(), param_type.clone());
                                        if !param_is_bool {
                                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                            if !param_is_vtype {
                                                // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                                non_bool_var_names.push(value.clone());
                                            }
                                        }
                                        if param_type == "string" {
                                            // transpiler-deor/tokens_validator/macros/track/track_non_bool_vars.deor
                                            string_var_names.push(value.clone());
                                        }
                                        param_scan_pos = param_name_pos;
                                    }
                                }
                            }
                            param_scan_pos = param_scan_pos + 1;
                        }
                    }
                }
            }
        }
        // macro: check_fn_decl (transpiler-deor/tokens_validator/macros/declarations/check_fn_decl.deor)
        {
            // macro: check_fn_declaration (transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor)
            {
                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                fn locate_fn_return_type(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_fn_name(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_fn_left_paren(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                if cur_kind == "KW_FN" {
                    // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                    let mut is_nested: bool = block_depth > 0;
                    if is_nested {
                        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                        if !was_already_in_fn_body {
                            // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                            let mut name_pos: i64 = locate_fn_name(pos.clone());
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            errors.push(val_err(name_token.clone(), lbl_fn.clone(), rule_nested_fn.clone()).clone());
                        }
                    }
                    let mut left_paren_pos: i64 = locate_fn_left_paren(pos.clone());
                    if left_paren_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                        let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                        let mut kind = left_paren_token.kind.clone();
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                            let mut param_count: i64 = count_call_args(tokens.clone(), left_paren_pos.clone());
                            if param_count > 3 {
                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                let mut name_pos: i64 = locate_fn_name(pos.clone());
                                let mut name_token: Token = tokens[name_pos as usize].clone();
                                errors.push(val_err(name_token.clone(), lbl_fn.clone(), rule_max_params.clone()).clone());
                            }
                            let mut param_scan_pos: i64 = left_paren_pos + 1;
                            let mut right_paren_pos: i64 = left_paren_pos.clone();
                            while param_scan_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                let mut param_scan_token: Token = tokens[param_scan_pos as usize].clone();
                                let mut kind = param_scan_token.kind.clone();
                                let mut value = param_scan_token.value.clone();
                                if kind == "RPAREN" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                    right_paren_pos = param_scan_pos;
                                    break;
                                }
                                if kind == "COMMA" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                    param_scan_pos = param_scan_pos + 1;
                                    continue;
                                }
                                if kind == "IDENT" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                    let mut param_type_val: String = value.clone();
                                    let mut param_name_pos: i64 = param_scan_pos + 1;
                                    if param_name_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                        let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
                                        let mut kind = param_name_token.kind.clone();
                                        let mut value = param_name_token.value.clone();
                                        if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                            if value == param_type_val {
                                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                                errors.push(val_err(param_name_token.clone(), lbl_fn.clone(), rule_param_shadow.clone()).clone());
                                            }
                                            if (value.len() as i64) < 3 {
                                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                                errors.push(val_err(param_name_token.clone(), lbl_fn.clone(), rule_min3.clone()).clone());
                                            }
                                            if !is_snake(value.clone()) {
                                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                                errors.push(val_err(param_name_token.clone(), lbl_fn.clone(), rule_snake.clone()).clone());
                                            }
                                            if list_has(builtin_names.clone(), value.clone()) {
                                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                                errors.push(val_err(param_name_token.clone(), lbl_fn.clone(), rule_builtin_shadow.clone()).clone());
                                            }
                                            param_scan_pos = param_name_pos;
                                        } else if list_has(reserved_keywords.clone(), kind.clone()) {
                                            // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                            errors.push(val_err(param_name_token.clone(), lbl_fn.clone(), rule_kw_in_parens.clone()).clone());
                                            param_scan_pos = param_name_pos;
                                        }
                                    }
                                }
                                param_scan_pos = param_scan_pos + 1;
                            }
                            if is_nested {
                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                if was_already_in_fn_body {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                    let mut has_valid_body: bool = is_single_return_body(tokens.clone(), right_paren_pos.clone(), token_count.clone());
                                    if !has_valid_body {
                                        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                        let mut name_pos: i64 = locate_fn_name(pos.clone());
                                        let mut name_token: Token = tokens[name_pos as usize].clone();
                                        errors.push(val_err(name_token.clone(), lbl_fn.clone(), rule_nested_fn_body_shape.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                    let mut return_type_pos: i64 = locate_fn_return_type(pos.clone());
                    let mut after_return_type_pos: i64 = locate_fn_name(pos.clone());
                    if after_return_type_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                        let mut return_type_token: Token = tokens[return_type_pos as usize].clone();
                        let mut after_return_type_token: Token = tokens[after_return_type_pos as usize].clone();
                        let mut kind = return_type_token.kind.clone();
                        let mut return_type_kind: String = kind.clone();
                        let mut kind = after_return_type_token.kind.clone();
                        if return_type_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                            if kind == "LPAREN" {
                                // transpiler-deor/tokens_validator/macros/declarations/check_fn_declaration.deor
                                errors.push(val_err(return_type_token.clone(), lbl_fn.clone(), rule_no_ret.clone()).clone());
                            }
                        }
                    }
                }
            }
            // transpiler-deor/tokens_validator/macros/declarations/check_fn_decl.deor
            let mut keyword: String = "KW_FN".to_string();
            let mut lbl: String = lbl_fn.clone();
            let mut rule: String = rule_snake.clone();
            let mut test_rule: fn(String) -> bool = is_snake.clone();
            let mut validate_indent_offset: i64 = 2;
            // macro: validate_ident (transpiler-deor/tokens_validator/macros/idents/validate_ident.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                if cur_kind == keyword {
                    // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                    let mut name_pos: i64 = pos + validate_indent_offset;
                    if name_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                        let mut name_tok: Token = tokens[name_pos as usize].clone();
                        let mut kind = name_tok.kind.clone();
                        let mut value = name_tok.value.clone();
                        let mut name_kind: String = kind.clone();
                        let mut name_val: String = value.clone();
                        if name_kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            if (name_val.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule_min3.clone()).clone());
                            }
                            if !test_rule(name_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                                errors.push(val_err(name_tok.clone(), lbl.clone(), rule.clone()).clone());
                            }
                        } else if list_has(reserved_keywords.clone(), name_kind.clone()) {
                            // transpiler-deor/tokens_validator/macros/idents/validate_ident.deor
                            errors.push(val_err(name_tok.clone(), lbl.clone(), rule_kw_in_parens.clone()).clone());
                        }
                    }
                    pos = pos + 1;
                    continue;
                }
            }
        }
        // macro: check_misc_token_rules (transpiler-deor/tokens_validator/macros/check_misc_token_rules.deor)
        {
            // macro: check_invalid_char (transpiler-deor/tokens_validator/macros/check_invalid_char.deor)
            {
                // transpiler-deor/tokens_validator/macros/check_invalid_char.deor
                if cur_kind == "INVALID" {
                    // transpiler-deor/tokens_validator/macros/check_invalid_char.deor
                    errors.push(val_err(tok.clone(), lbl_rust.clone(), rule_invalid_char.clone()).clone());
                }
            }
            // macro: check_raw_assignment (transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor)
            {
                // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                fn locate_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_equals(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                fn locate_call_target(kw_pos: i64) -> i64 {
                    return kw_pos + 3;
                }
                fn locate_call_lparen(kw_pos: i64) -> i64 {
                    return kw_pos + 4;
                }
                if cur_kind == "KW_RAW" {
                    // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                    let mut equals_pos: i64 = locate_equals(pos.clone());
                    if equals_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                        let mut equals_token: Token = tokens[equals_pos as usize].clone();
                        let mut kind = equals_token.kind.clone();
                        if kind == "KW_AS" {
                            // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                            let mut as_name_pos: i64 = locate_name(pos.clone());
                            let mut as_name_token: Token = tokens[as_name_pos as usize].clone();
                            errors.push(val_err(as_name_token.clone(), lbl_var.clone(), rule_raw_as.clone()).clone());
                        } else if kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                            let mut name_pos: i64 = locate_name(pos.clone());
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            let mut kind = name_token.kind.clone();
                            let mut value = name_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                                raw_var_names.push(value.clone());
                                let mut call_pos: i64 = locate_call_target(pos.clone());
                                let mut is_call: bool = false;
                                if call_pos < token_count {
                                    // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                                    let mut call_token: Token = tokens[call_pos as usize].clone();
                                    let mut kind = call_token.kind.clone();
                                    if kind == "IDENT" {
                                        // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                                        let mut lparen_pos: i64 = locate_call_lparen(pos.clone());
                                        if lparen_pos < token_count {
                                            // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                                            let mut lparen_token: Token = tokens[lparen_pos as usize].clone();
                                            let mut kind = lparen_token.kind.clone();
                                            is_call = kind == "LPAREN";
                                        }
                                    }
                                }
                                if !is_call {
                                    // transpiler-deor/tokens_validator/macros/raw/check_raw_assignment.deor
                                    errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_raw_assignment.clone()).clone());
                                }
                            }
                        }
                    }
                }
            }
            // macro: track_validator_vars (transpiler-deor/tokens_validator/macros/track/track_validator_vars.deor)
            {
                // transpiler-deor/tokens_validator/macros/track/track_validator_vars.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "IDENT" {
                    // transpiler-deor/tokens_validator/macros/track/track_validator_vars.deor
                    let mut is_vtype: bool = list_has(validator_type_names.clone(), cur_val.clone());
                    if is_vtype {
                        // transpiler-deor/tokens_validator/macros/track/track_validator_vars.deor
                        let mut next_pos: i64 = locate_next_token(pos.clone());
                        if next_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/track/track_validator_vars.deor
                            let mut next_token: Token = tokens[next_pos as usize].clone();
                            let mut kind = next_token.kind.clone();
                            let mut value = next_token.value.clone();
                            if kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/track/track_validator_vars.deor
                                validator_vars.push(value.clone());
                            }
                        }
                    }
                }
            }
            // macro: check_crash_args (transpiler-deor/tokens_validator/macros/builtins/check_crash_args.deor)
            {
                // transpiler-deor/tokens_validator/macros/builtins/check_crash_args.deor
                let mut builtin_name: String = "crash".to_string();
                let mut min_args: i64 = 1;
                let mut max_args: i64 = 1;
                let mut arg_count_rule: String = rule_crash.clone();
                // macro: check_builtin_arg_count (transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor)
                {
                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                        let mut is_target: bool = cur_val == builtin_name;
                        if is_target {
                            // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                            let mut left_paren_pos: i64 = locate_next_token(pos.clone());
                            if left_paren_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                                let mut kind = left_paren_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                    let mut arg_count: i64 = count_call_args(tokens.clone(), left_paren_pos.clone());
                                    let mut count_ok: bool = arg_count >= min_args && arg_count <= max_args;
                                    if !count_ok {
                                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                        errors.push(val_err(tok.clone(), lbl_call.clone(), arg_count_rule.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_print_args (transpiler-deor/tokens_validator/macros/builtins/check_print_args.deor)
            {
                // transpiler-deor/tokens_validator/macros/builtins/check_print_args.deor
                let mut builtin_name: String = "print".to_string();
                let mut min_args: i64 = 1;
                let mut max_args: i64 = 2;
                let mut arg_count_rule: String = rule_print_args.clone();
                // macro: check_builtin_arg_count (transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor)
                {
                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                        let mut is_target: bool = cur_val == builtin_name;
                        if is_target {
                            // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                            let mut left_paren_pos: i64 = locate_next_token(pos.clone());
                            if left_paren_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                                let mut kind = left_paren_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                    let mut arg_count: i64 = count_call_args(tokens.clone(), left_paren_pos.clone());
                                    let mut count_ok: bool = arg_count >= min_args && arg_count <= max_args;
                                    if !count_ok {
                                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                        errors.push(val_err(tok.clone(), lbl_call.clone(), arg_count_rule.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_range_args (transpiler-deor/tokens_validator/macros/builtins/check_range_args.deor)
            {
                // transpiler-deor/tokens_validator/macros/builtins/check_range_args.deor
                let mut builtin_name: String = "range".to_string();
                let mut min_args: i64 = 1;
                let mut max_args: i64 = 2;
                let mut arg_count_rule: String = rule_range_args.clone();
                // macro: check_builtin_arg_count (transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor)
                {
                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                        let mut is_target: bool = cur_val == builtin_name;
                        if is_target {
                            // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                            let mut left_paren_pos: i64 = locate_next_token(pos.clone());
                            if left_paren_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                                let mut kind = left_paren_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                    let mut arg_count: i64 = count_call_args(tokens.clone(), left_paren_pos.clone());
                                    let mut count_ok: bool = arg_count >= min_args && arg_count <= max_args;
                                    if !count_ok {
                                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                        errors.push(val_err(tok.clone(), lbl_call.clone(), arg_count_rule.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_len_args (transpiler-deor/tokens_validator/macros/builtins/check_len_args.deor)
            {
                // transpiler-deor/tokens_validator/macros/builtins/check_len_args.deor
                let mut builtin_name: String = "len".to_string();
                let mut min_args: i64 = 1;
                let mut max_args: i64 = 1;
                let mut arg_count_rule: String = rule_len_args.clone();
                // macro: check_builtin_arg_count (transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor)
                {
                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                        let mut is_target: bool = cur_val == builtin_name;
                        if is_target {
                            // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                            let mut left_paren_pos: i64 = locate_next_token(pos.clone());
                            if left_paren_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                                let mut kind = left_paren_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                    let mut arg_count: i64 = count_call_args(tokens.clone(), left_paren_pos.clone());
                                    let mut count_ok: bool = arg_count >= min_args && arg_count <= max_args;
                                    if !count_ok {
                                        // transpiler-deor/tokens_validator/macros/builtins/check_builtin_arg_count.deor
                                        errors.push(val_err(tok.clone(), lbl_call.clone(), arg_count_rule.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_avow_target (transpiler-deor/tokens_validator/macros/check_avow_target.deor)
            {
                // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_AVOW" {
                    // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                        let mut target_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = target_token.kind.clone();
                        let mut value = target_token.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                            let mut is_valid: bool = list_has(validator_vars.clone(), value.clone());
                            if !is_valid {
                                // transpiler-deor/tokens_validator/macros/check_avow_target.deor
                                errors.push(val_err(target_token.clone(), lbl_var.clone(), rule_avow.clone()).clone());
                            }
                        }
                    }
                }
            }
            // macro: check_validator_empty (transpiler-deor/tokens_validator/macros/declarations/check_validator_empty.deor)
            {
                // transpiler-deor/tokens_validator/macros/declarations/check_validator_empty.deor
                fn locate_type_before_empty(kw_pos: i64) -> i64 {
                    return kw_pos - 3;
                }
                if cur_kind == "KW_EMPTY" {
                    // transpiler-deor/tokens_validator/macros/declarations/check_validator_empty.deor
                    if pos > 2 {
                        // transpiler-deor/tokens_validator/macros/declarations/check_validator_empty.deor
                        let mut type_pos: i64 = locate_type_before_empty(pos.clone());
                        let mut type_token: Token = tokens[type_pos as usize].clone();
                        let mut kind = type_token.kind.clone();
                        let mut value = type_token.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_validator_empty.deor
                            let mut is_validator: bool = list_has(validator_type_names.clone(), value.clone());
                            if is_validator {
                                // transpiler-deor/tokens_validator/macros/declarations/check_validator_empty.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_validator_empty.clone()).clone());
                            }
                        }
                    }
                }
            }
            // macro: check_empty_bracket (transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_bracket.deor)
            {
                // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_bracket.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "LBRACKET" {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_bracket.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_bracket.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        if kind == "RBRACKET" {
                            // transpiler-deor/tokens_validator/macros/brackets_parens/check_empty_bracket.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_empty_bracket.clone()).clone());
                        }
                    }
                }
            }
            // macro: check_for_loop_var_name (transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor)
            {
                // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_FOR" {
                    // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        let mut value = next_token.value.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                            if (value.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                                errors.push(val_err(next_token.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                            }
                            if !is_snake(value.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                                errors.push(val_err(next_token.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                            }
                            if list_has(builtin_names.clone(), value.clone()) {
                                // transpiler-deor/tokens_validator/macros/idents/check_for_loop_var_name.deor
                                errors.push(val_err(next_token.clone(), lbl_var.clone(), rule_builtin_shadow.clone()).clone());
                            }
                        }
                    }
                }
            }
            // macro: check_bare_tuple_range (transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                fn locate_next_token(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                if cur_kind == "KW_FOR" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        let mut applies: bool = true;
                        if kind == "KW_IF" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                            applies = false;
                        }
                        if kind == "KW_MOVE" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                            applies = false;
                        }
                        if applies {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                            let mut iter_pos: i64 = 0;
                            if kind == "KW_IN" {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                                iter_pos = locate_next_token(next_pos.clone());
                            } else {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                                let mut in_pos: i64 = locate_next_token(next_pos.clone());
                                iter_pos = locate_next_token(in_pos.clone());
                            }
                            if iter_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                                let mut iter_token: Token = tokens[iter_pos as usize].clone();
                                let mut kind = iter_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bare_tuple_range.deor
                                    errors.push(val_err(iter_token.clone(), lbl_call.clone(), rule_bare_tuple_range.clone()).clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        // macro: check_ident_token_rules (transpiler-deor/tokens_validator/macros/check_ident_token_rules.deor)
        {
            // transpiler-deor/tokens_validator/macros/check_ident_token_rules.deor
            fn locate_prev2(kw_pos: i64) -> i64 {
                return kw_pos - 2;
            }
            if cur_kind == "IDENT" {
                // transpiler-deor/tokens_validator/macros/check_ident_token_rules.deor
                let mut is_fn_decl_name: bool = false;
                if pos > 1 {
                    // transpiler-deor/tokens_validator/macros/check_ident_token_rules.deor
                    let mut prev2_pos: i64 = locate_prev2(pos.clone());
                    let mut prev2_token: Token = tokens[prev2_pos as usize].clone();
                    let mut kind = prev2_token.kind.clone();
                    is_fn_decl_name = kind == "KW_FN";
                }
                if !is_fn_decl_name {
                    // macro: check_call_args (transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor)
                    {
                        // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                        fn locate_next_token(kw_pos: i64) -> i64 {
                            return kw_pos + 1;
                        }
                        let mut left_paren_pos: i64 = locate_next_token(pos.clone());
                        if left_paren_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                            let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                            let mut kind = left_paren_token.kind.clone();
                            if kind == "LPAREN" {
                                // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                let mut arg_count: i64 = count_call_args(tokens.clone(), left_paren_pos.clone());
                                if arg_count >= 2 {
                                    // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                    let mut scan_pos: i64 = left_paren_pos + 1;
                                    let mut scan_depth: i64 = 0;
                                    let mut at_arg_start: bool = true;
                                    while scan_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                                        let mut kind = scan_token.kind.clone();
                                        if kind == "RPAREN" {
                                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                            let mut at_root: bool = scan_depth == 0;
                                            if at_root {
                                                // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                                break;
                                            }
                                            scan_depth = scan_depth - 1;
                                            at_arg_start = false;
                                        } else if kind == "LPAREN" {
                                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                            scan_depth = scan_depth + 1;
                                            at_arg_start = false;
                                        } else if kind == "LBRACKET" {
                                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                            scan_depth = scan_depth + 1;
                                            at_arg_start = false;
                                        } else if kind == "RBRACKET" {
                                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                            scan_depth = scan_depth - 1;
                                            at_arg_start = false;
                                        } else if kind == "COMMA" {
                                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                            let mut at_root: bool = scan_depth == 0;
                                            if at_root {
                                                // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                                at_arg_start = true;
                                            }
                                        } else if at_arg_start {
                                            // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
                                            let mut named: bool = arg_is_named(tokens.clone(), scan_pos.clone(), kind.clone());
                                            if !named {
                                                // transpiler-deor/tokens_validator/macros/builtins/check_call_args.deor
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
                }
                // macro: check_rust_generic (transpiler-deor/tokens_validator/macros/syntax_rules/check_rust_generic.deor)
                {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_rust_generic.deor
                    let mut rust_generic_names: Vec<String> = vec!["Option".to_string(), "Vec".to_string(), "Box".to_string(), "Rc".to_string(), "Arc".to_string(), "Result".to_string()];
                    let mut is_rust_generic: bool = list_has(rust_generic_names.clone(), cur_val.clone());
                    if is_rust_generic {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_rust_generic.deor
                        errors.push(val_err(tok.clone(), lbl_rust.clone(), rule_no_option.clone()).clone());
                    }
                }
                // macro: check_raw_in_binding (transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor)
                {
                    // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                    fn locate_binding_name(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_binding_equals(kw_pos: i64) -> i64 {
                        return kw_pos + 2;
                    }
                    fn locate_binding_value(kw_pos: i64) -> i64 {
                        return kw_pos + 3;
                    }
                    fn locate_as_keyword(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_as_value(kw_pos: i64) -> i64 {
                        return kw_pos + 2;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                        let mut name_pos: i64 = locate_binding_name(pos.clone());
                        let mut equals_pos: i64 = locate_binding_equals(pos.clone());
                        let mut value_pos: i64 = locate_binding_value(pos.clone());
                        if value_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            let mut equals_token: Token = tokens[equals_pos as usize].clone();
                            let mut value_token: Token = tokens[value_pos as usize].clone();
                            let mut kind = name_token.kind.clone();
                            let mut name_kind: String = kind.clone();
                            let mut kind = equals_token.kind.clone();
                            let mut equals_kind: String = kind.clone();
                            let mut kind = value_token.kind.clone();
                            let mut value = value_token.value.clone();
                            let mut is_binding: bool = name_kind == "IDENT";
                            is_binding = is_binding && equals_kind == "EQUALS";
                            let mut value_is_ident: bool = kind == "IDENT";
                            if is_binding && value_is_ident {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                                let mut is_raw: bool = list_has(raw_var_names.clone(), value.clone());
                                if is_raw {
                                    // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                                    errors.push(val_err(value_token.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                                }
                            }
                        }
                        let mut as_pos: i64 = locate_as_keyword(pos.clone());
                        let mut as_value_pos: i64 = locate_as_value(pos.clone());
                        if as_value_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                            let mut as_token: Token = tokens[as_pos as usize].clone();
                            let mut as_value_token: Token = tokens[as_value_pos as usize].clone();
                            let mut kind = as_token.kind.clone();
                            let mut is_as: bool = kind == "KW_AS";
                            let mut kind = as_value_token.kind.clone();
                            let mut value = as_value_token.value.clone();
                            let mut as_value_is_ident: bool = kind == "IDENT";
                            if is_as && as_value_is_ident {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                                let mut as_value_is_raw: bool = list_has(raw_var_names.clone(), value.clone());
                                if as_value_is_raw {
                                    // transpiler-deor/tokens_validator/macros/raw/check_raw_in_binding.deor
                                    errors.push(val_err(as_value_token.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                                }
                            }
                        }
                    }
                }
                // macro: check_raw_reassign (transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor)
                {
                    // transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor
                    fn locate_after_name(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_before_name(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    let mut is_tracked: bool = list_has(raw_var_names.clone(), cur_val.clone());
                    if is_tracked {
                        // transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor
                        let mut after_pos: i64 = locate_after_name(pos.clone());
                        if after_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor
                            let mut after_token: Token = tokens[after_pos as usize].clone();
                            let mut kind = after_token.kind.clone();
                            let mut is_eq: bool = kind == "EQUALS";
                            let mut is_as: bool = kind == "KW_AS";
                            if is_eq || is_as {
                                // transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor
                                let mut is_decl: bool = false;
                                if pos > 0 {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor
                                    let mut before_pos: i64 = locate_before_name(pos.clone());
                                    let mut before_token: Token = tokens[before_pos as usize].clone();
                                    let mut kind = before_token.kind.clone();
                                    is_decl = kind == "KW_RAW";
                                }
                                if !is_decl {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_raw_reassign.deor
                                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_raw_reassign.clone()).clone());
                                }
                            }
                        }
                    }
                }
                // macro: check_raw_operator_use (transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor)
                {
                    // transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_prev_token(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor
                        let mut is_raw: bool = list_has(raw_var_names.clone(), cur_val.clone());
                        if is_raw {
                            // transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor
                            let mut op_kinds: Vec<String> = vec!["PLUS".to_string(), "MINUS".to_string(), "STAR".to_string(), "SLASH".to_string(), "PERCENT".to_string(), "GT".to_string(), "LT".to_string(), "GTE".to_string(), "LTE".to_string(), "KW_IS".to_string(), "KW_AND".to_string(), "KW_OR".to_string()];
                            let mut next_is_op: bool = false;
                            let mut next_pos: i64 = locate_next_token(pos.clone());
                            if next_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor
                                let mut next_token: Token = tokens[next_pos as usize].clone();
                                let mut kind = next_token.kind.clone();
                                next_is_op = list_has(op_kinds.clone(), kind.clone());
                            }
                            let mut prev_is_op: bool = false;
                            if pos > 0 {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor
                                let mut prev_pos: i64 = locate_prev_token(pos.clone());
                                let mut prev_token: Token = tokens[prev_pos as usize].clone();
                                let mut kind = prev_token.kind.clone();
                                prev_is_op = list_has(op_kinds.clone(), kind.clone());
                            }
                            if next_is_op || prev_is_op {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_operator_use.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                            }
                        }
                    }
                }
                // macro: check_raw_in_special_builtin (transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor)
                {
                    // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                    fn locate_left_paren(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                        let mut target_names: Vec<String> = vec!["len".to_string(), "crash".to_string(), "s_join".to_string()];
                        let mut is_target: bool = list_has(target_names.clone(), cur_val.clone());
                        if is_target {
                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                            let mut left_paren_pos: i64 = locate_left_paren(pos.clone());
                            if left_paren_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                let mut left_paren_token: Token = tokens[left_paren_pos as usize].clone();
                                let mut kind = left_paren_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                    let mut scan_pos: i64 = left_paren_pos + 1;
                                    let mut depth: i64 = 0;
                                    while scan_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                                        let mut kind = scan_token.kind.clone();
                                        let mut value = scan_token.value.clone();
                                        if kind == "RPAREN" {
                                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                            let mut at_root: bool = depth == 0;
                                            if at_root {
                                                // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                                break;
                                            }
                                            depth = depth - 1;
                                        } else if kind == "LPAREN" {
                                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                            depth = depth + 1;
                                        } else if kind == "LBRACKET" {
                                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                            depth = depth + 1;
                                        } else if kind == "RBRACKET" {
                                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                            depth = depth - 1;
                                        } else if kind == "IDENT" {
                                            // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                            let mut is_raw: bool = list_has(raw_var_names.clone(), value.clone());
                                            if is_raw {
                                                // transpiler-deor/tokens_validator/macros/raw/check_raw_in_special_builtin.deor
                                                errors.push(val_err(scan_token.clone(), lbl_var.clone(), rule_raw_in_expr.clone()).clone());
                                            }
                                        }
                                        scan_pos = scan_pos + 1;
                                    }
                                }
                            }
                        }
                    }
                }
                // macro: check_const_reassign (transpiler-deor/tokens_validator/macros/reassign/check_const_reassign.deor)
                {
                    // transpiler-deor/tokens_validator/macros/reassign/check_const_reassign.deor
                    let mut bare_reassign_names: Vec<String> = const_var_names.clone();
                    let mut allow_eq: bool = true;
                    let mut allow_as: bool = false;
                    let mut bare_reassign_rule: String = rule_const_reassign.clone();
                    // macro: check_bare_reassign (transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor)
                    {
                        // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                        fn locate_after_name(kw_pos: i64) -> i64 {
                            return kw_pos + 1;
                        }
                        fn locate_before_name(kw_pos: i64) -> i64 {
                            return kw_pos - 1;
                        }
                        let mut is_tracked: bool = list_has(bare_reassign_names.clone(), cur_val.clone());
                        if is_tracked {
                            // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                            let mut after_pos: i64 = locate_after_name(pos.clone());
                            if after_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                let mut after_token: Token = tokens[after_pos as usize].clone();
                                let mut kind = after_token.kind.clone();
                                let mut is_eq: bool = false;
                                if allow_eq {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                    is_eq = kind == "EQUALS";
                                }
                                let mut is_as: bool = false;
                                if allow_as {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                    is_as = kind == "KW_AS";
                                }
                                if is_eq || is_as {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                    let mut is_decl: bool = false;
                                    if pos > 0 {
                                        // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                        let mut before_pos: i64 = locate_before_name(pos.clone());
                                        let mut before_token: Token = tokens[before_pos as usize].clone();
                                        let mut kind = before_token.kind.clone();
                                        is_decl = kind == "IDENT";
                                    }
                                    if !is_decl {
                                        // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                        errors.push(val_err(tok.clone(), lbl_var.clone(), bare_reassign_rule.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
                // macro: check_validator_reassign (transpiler-deor/tokens_validator/macros/reassign/check_validator_reassign.deor)
                {
                    // transpiler-deor/tokens_validator/macros/reassign/check_validator_reassign.deor
                    let mut bare_reassign_names: Vec<String> = validator_vars.clone();
                    let mut allow_eq: bool = false;
                    let mut allow_as: bool = true;
                    let mut bare_reassign_rule: String = rule_validator_reassign.clone();
                    // macro: check_bare_reassign (transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor)
                    {
                        // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                        fn locate_after_name(kw_pos: i64) -> i64 {
                            return kw_pos + 1;
                        }
                        fn locate_before_name(kw_pos: i64) -> i64 {
                            return kw_pos - 1;
                        }
                        let mut is_tracked: bool = list_has(bare_reassign_names.clone(), cur_val.clone());
                        if is_tracked {
                            // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                            let mut after_pos: i64 = locate_after_name(pos.clone());
                            if after_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                let mut after_token: Token = tokens[after_pos as usize].clone();
                                let mut kind = after_token.kind.clone();
                                let mut is_eq: bool = false;
                                if allow_eq {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                    is_eq = kind == "EQUALS";
                                }
                                let mut is_as: bool = false;
                                if allow_as {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                    is_as = kind == "KW_AS";
                                }
                                if is_eq || is_as {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                    let mut is_decl: bool = false;
                                    if pos > 0 {
                                        // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                        let mut before_pos: i64 = locate_before_name(pos.clone());
                                        let mut before_token: Token = tokens[before_pos as usize].clone();
                                        let mut kind = before_token.kind.clone();
                                        is_decl = kind == "IDENT";
                                    }
                                    if !is_decl {
                                        // transpiler-deor/tokens_validator/macros/reassign/check_bare_reassign.deor
                                        errors.push(val_err(tok.clone(), lbl_var.clone(), bare_reassign_rule.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
                // macro: check_var_decl (transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor)
                {
                    // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                    fn locate_var_name(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_var_equals(kw_pos: i64) -> i64 {
                        return kw_pos + 2;
                    }
                    fn locate_const_check_pos(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    let mut name_pos: i64 = locate_var_name(pos.clone());
                    let mut equals_pos: i64 = locate_var_equals(pos.clone());
                    if equals_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                        let mut name_token: Token = tokens[name_pos as usize].clone();
                        let mut equals_token: Token = tokens[equals_pos as usize].clone();
                        let mut kind = name_token.kind.clone();
                        let mut name_kind: String = kind.clone();
                        let mut kind = equals_token.kind.clone();
                        let mut equals_kind: String = kind.clone();
                        if name_kind == "IDENT" && equals_kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                            let mut value = name_token.value.clone();
                            let mut line = name_token.line.clone();
                            let mut file = name_token.file.clone();
                            let mut var_name: String = value.clone();
                            let mut var_line: i64 = line.clone();
                            let mut var_file: String = file.clone();
                            if (var_name.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                            }
                            if list_has(builtin_names.clone(), var_name.clone()) {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_builtin_shadow.clone()).clone());
                            }
                            let mut is_const: bool = false;
                            if pos > 0 {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                let mut const_check_pos: i64 = locate_const_check_pos(pos.clone());
                                let mut const_check_token: Token = tokens[const_check_pos as usize].clone();
                                let mut kind = const_check_token.kind.clone();
                                is_const = kind == "KW_CONST";
                            }
                            if is_const {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                if !is_screaming_snake(var_name.clone()) {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                    errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_screaming.clone()).clone());
                                }
                            }
                            if !is_const {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                if !is_snake(var_name.clone()) {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                    errors.push(val_err(name_token.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                                }
                            }
                        }
                        if name_kind == "KW_AS" {
                            // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                            if (cur_val.len() as i64) < 3 {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_min3.clone()).clone());
                            }
                            if !is_snake(cur_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_snake.clone()).clone());
                            }
                            if list_has(builtin_names.clone(), cur_val.clone()) {
                                // transpiler-deor/tokens_validator/macros/declarations/check_var_decl.deor
                                errors.push(val_err(tok.clone(), lbl_var.clone(), rule_builtin_shadow.clone()).clone());
                            }
                        }
                    }
                }
                // macro: check_bad_stmt (transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor)
                {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                    fn locate_second_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_third_token(kw_pos: i64) -> i64 {
                        return kw_pos + 2;
                    }
                    if paren_depth == 0 {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                        let mut second_pos: i64 = locate_second_token(pos.clone());
                        let mut third_pos: i64 = locate_third_token(pos.clone());
                        if third_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                            let mut second_token: Token = tokens[second_pos as usize].clone();
                            let mut third_token: Token = tokens[third_pos as usize].clone();
                            let mut kind = second_token.kind.clone();
                            let mut second_kind: String = kind.clone();
                            let mut kind = third_token.kind.clone();
                            let mut third_kind: String = kind.clone();
                            if second_kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                                let mut third_is_str: bool = third_kind == "STRING";
                                let mut third_is_int: bool = third_kind == "INT";
                                let mut third_is_flt: bool = third_kind == "FLOAT";
                                let mut third_is_lit: bool = third_is_str || third_is_int || third_is_flt;
                                if third_is_lit {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_bad_stmt.clone()).clone());
                                }
                                if third_kind == "KW_AS" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_typed_as.clone()).clone());
                                }
                            }
                            if second_kind == "KW_AS" {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                                if third_kind == "KW_MOVE" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_bad_stmt.deor
                                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_as_move.clone()).clone());
                                }
                            }
                        }
                    }
                }
                // macro: check_undeclared_reassign (transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor)
                {
                    // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                    fn locate_after_name(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_before_name(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    let mut after_pos: i64 = locate_after_name(pos.clone());
                    if after_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                        let mut after_token: Token = tokens[after_pos as usize].clone();
                        let mut kind = after_token.kind.clone();
                        if kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                            let mut is_decl: bool = false;
                            if pos > 0 {
                                // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                                let mut before_pos: i64 = locate_before_name(pos.clone());
                                let mut before_token: Token = tokens[before_pos as usize].clone();
                                let mut kind = before_token.kind.clone();
                                let mut before_kind: String = kind.clone();
                                is_decl = before_kind == "IDENT";
                                if before_kind == "KW_RAW" {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                                    is_decl = true;
                                }
                                if before_kind == "KW_SHAPE" {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                                    is_decl = true;
                                }
                                if before_kind == "KW_TYPE" {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                                    is_decl = true;
                                }
                            }
                            if !is_decl {
                                // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                                let mut is_known: bool = list_has(declared_var_names.clone(), cur_val.clone());
                                if !is_known {
                                    // transpiler-deor/tokens_validator/macros/reassign/check_undeclared_reassign.deor
                                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_undeclared_reassign.clone()).clone());
                                }
                            }
                        }
                    }
                }
                // macro: check_undefined_var_read (transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor)
                {
                    // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_prev_token(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    let mut skip: bool = in_struct_body.clone();
                    if in_enum_body {
                        // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                        skip = true;
                    }
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        if kind == "IDENT" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "EQUALS" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_AS" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "LPAREN" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_IN" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                    }
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                        let mut prev_pos: i64 = locate_prev_token(pos.clone());
                        let mut prev_token: Token = tokens[prev_pos as usize].clone();
                        let mut kind = prev_token.kind.clone();
                        if kind == "KW_STRUCT" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_ENUM" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_SHAPE" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_TYPE" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_FN" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_MACRO" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_MACRO_RUN" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_UNSAFE_MACRO_RUN" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_UNSAFE_MACRO" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_OF" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_TO" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                        if kind == "KW_RAW" {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            skip = true;
                        }
                    }
                    if !skip {
                        // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                        let mut known: bool = list_has(declared_var_names.clone(), cur_val.clone());
                        if !known {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            known = list_has(enum_variant_names.clone(), cur_val.clone());
                        }
                        if !known {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            known = list_has(enum_names.clone(), cur_val.clone());
                        }
                        if !known {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            known = list_has(fn_names.clone(), cur_val.clone());
                        }
                        if !known {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            known = list_has(builtin_names.clone(), cur_val.clone());
                        }
                        if !known {
                            // transpiler-deor/tokens_validator/macros/check_undefined_var_read.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_undefined_var.clone()).clone());
                        }
                    }
                }
                // macro: check_bracket_indexing (transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_indexing.deor)
                {
                    // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_indexing.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    let mut next_pos: i64 = locate_next_token(pos.clone());
                    if next_pos < token_count {
                        // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_indexing.deor
                        let mut next_token: Token = tokens[next_pos as usize].clone();
                        let mut kind = next_token.kind.clone();
                        if kind == "LBRACKET" {
                            // transpiler-deor/tokens_validator/macros/brackets_parens/check_bracket_indexing.deor
                            errors.push(val_err(tok.clone(), lbl_var.clone(), rule_bracket_index.clone()).clone());
                        }
                    }
                }
                // macro: check_struct_construction (transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor)
                {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                    fn locate_var_name(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_equals(kw_pos: i64) -> i64 {
                        return kw_pos + 2;
                    }
                    fn locate_value_or_move(kw_pos: i64) -> i64 {
                        return kw_pos + 3;
                    }
                    fn locate_paren_after_move(kw_pos: i64) -> i64 {
                        return kw_pos + 4;
                    }
                    if paren_depth == 0 {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                        let mut name_pos: i64 = locate_var_name(pos.clone());
                        let mut equals_pos: i64 = locate_equals(pos.clone());
                        let mut value_pos: i64 = locate_value_or_move(pos.clone());
                        if value_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                            let mut name_token: Token = tokens[name_pos as usize].clone();
                            let mut equals_token: Token = tokens[equals_pos as usize].clone();
                            let mut value_token: Token = tokens[value_pos as usize].clone();
                            let mut kind = name_token.kind.clone();
                            let mut name_kind: String = kind.clone();
                            let mut kind = equals_token.kind.clone();
                            let mut equals_kind: String = kind.clone();
                            let mut kind = value_token.kind.clone();
                            let mut value_kind: String = kind.clone();
                            let mut is_var: bool = name_kind == "IDENT";
                            let mut is_eq: bool = equals_kind == "EQUALS";
                            let mut prefix_ok: bool = is_var && is_eq;
                            let mut matched: bool = false;
                            let mut paren_pos: i64 = value_pos.clone();
                            if prefix_ok {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                if value_kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                    matched = true;
                                } else if value_kind == "KW_MOVE" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                    let mut move_paren_pos: i64 = locate_paren_after_move(pos.clone());
                                    if move_paren_pos < token_count {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                        let mut move_paren_token: Token = tokens[move_paren_pos as usize].clone();
                                        let mut kind = move_paren_token.kind.clone();
                                        if kind == "LPAREN" {
                                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                            matched = true;
                                            paren_pos = move_paren_pos;
                                        }
                                    }
                                }
                            }
                            if matched {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                let mut sep: String = ",".to_string();
                                let mut fields_str: String = find_struct_field_str(struct_field_reg.clone(), cur_val.clone());
                                let mut is_struct: bool = fields_str != "";
                                if is_struct {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                    let mut expected_fields: Vec<String> = s_split(fields_str.clone(), sep.clone());
                                    let mut expected_count: i64 = (expected_fields.len() as i64);
                                    let mut provided_fields: Vec<String> = Vec::new();
                                    let mut scan_pos: i64 = paren_pos + 1;
                                    let mut scanning: bool = true;
                                    while scanning {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                        if scan_pos >= token_count {
                                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                            scanning = false;
                                        } else {
                                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                            let mut scan_token: Token = tokens[scan_pos as usize].clone();
                                            let mut kind = scan_token.kind.clone();
                                            let mut value = scan_token.value.clone();
                                            if kind == "RPAREN" {
                                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                                scanning = false;
                                            } else if kind == "IDENT" {
                                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                                provided_fields.push(value.clone());
                                            }
                                            scan_pos = scan_pos + 1;
                                        }
                                    }
                                    let mut provided_count: i64 = (provided_fields.len() as i64);
                                    let mut wrong_count: bool = provided_count != expected_count;
                                    if wrong_count {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                        errors.push(val_err(tok.clone(), lbl_struct.clone(), rule_struct_field_count.clone()).clone());
                                    } else {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                        let mut field_index: i64 = 0;
                                        while field_index < provided_count {
                                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                            let mut field_name: String = provided_fields[field_index as usize].clone();
                                            if !list_has(expected_fields.clone(), field_name.clone()) {
                                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_struct_construction.deor
                                                errors.push(val_err(tok.clone(), lbl_struct.clone(), rule_struct_field_name.clone()).clone());
                                            }
                                            field_index = field_index + 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // macro: check_range_placement (transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor)
                {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                    fn locate_next_token(kw_pos: i64) -> i64 {
                        return kw_pos + 1;
                    }
                    fn locate_prev_token(kw_pos: i64) -> i64 {
                        return kw_pos - 1;
                    }
                    fn locate_prev2(kw_pos: i64) -> i64 {
                        return kw_pos - 2;
                    }
                    fn locate_prev3(kw_pos: i64) -> i64 {
                        return kw_pos - 3;
                    }
                    if cur_kind == "IDENT" {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                        if cur_val == "range" {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                            let mut next_pos: i64 = locate_next_token(pos.clone());
                            if next_pos < token_count {
                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                let mut next_token: Token = tokens[next_pos as usize].clone();
                                let mut kind = next_token.kind.clone();
                                if kind == "LPAREN" {
                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                    let mut is_valid: bool = false;
                                    if pos > 1 {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                        let mut prev_pos: i64 = locate_prev_token(pos.clone());
                                        let mut prev_token: Token = tokens[prev_pos as usize].clone();
                                        let mut kind = prev_token.kind.clone();
                                        if kind == "KW_IN" {
                                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                            let mut prev2_pos: i64 = locate_prev2(pos.clone());
                                            let mut prev2_token: Token = tokens[prev2_pos as usize].clone();
                                            let mut kind = prev2_token.kind.clone();
                                            if kind == "KW_FOR" {
                                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                                is_valid = true;
                                            } else if kind == "IDENT" {
                                                // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                                if pos > 2 {
                                                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                                    let mut prev3_pos: i64 = locate_prev3(pos.clone());
                                                    let mut prev3_token: Token = tokens[prev3_pos as usize].clone();
                                                    let mut kind = prev3_token.kind.clone();
                                                    if kind == "KW_FOR" {
                                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                                        is_valid = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if !is_valid {
                                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_range_placement.deor
                                        errors.push(val_err(tok.clone(), lbl_call.clone(), rule_range_placement.clone()).clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // macro: check_closing_token_rules (transpiler-deor/tokens_validator/macros/check_closing_token_rules.deor)
        {
            // macro: check_void_var (transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor)
            {
                // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                fn locate_before_void(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                fn locate_void_name(kw_pos: i64) -> i64 {
                    return kw_pos + 1;
                }
                fn locate_void_equals(kw_pos: i64) -> i64 {
                    return kw_pos + 2;
                }
                if cur_kind == "KW_VOID" {
                    // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                    let mut preceded_by_fn: bool = false;
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                        let mut before_void_pos: i64 = locate_before_void(pos.clone());
                        let mut before_void_token: Token = tokens[before_void_pos as usize].clone();
                        let mut kind = before_void_token.kind.clone();
                        preceded_by_fn = kind == "KW_FN";
                    }
                    if !preceded_by_fn {
                        // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                        let mut void_name_pos: i64 = locate_void_name(pos.clone());
                        let mut void_equals_pos: i64 = locate_void_equals(pos.clone());
                        if void_equals_pos < token_count {
                            // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                            let mut void_name_token: Token = tokens[void_name_pos as usize].clone();
                            let mut void_equals_token: Token = tokens[void_equals_pos as usize].clone();
                            let mut kind = void_name_token.kind.clone();
                            let mut void_name_kind: String = kind.clone();
                            let mut kind = void_equals_token.kind.clone();
                            if void_name_kind == "IDENT" {
                                // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                                if kind == "EQUALS" {
                                    // transpiler-deor/tokens_validator/macros/declarations/check_void_var.deor
                                    errors.push(val_err(tok.clone(), lbl_var.clone(), rule_void_var.clone()).clone());
                                }
                            }
                        }
                    }
                }
            }
            // macro: check_valid_placement (transpiler-deor/tokens_validator/macros/syntax_rules/check_valid_placement.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_valid_placement.deor
                fn locate_prev_token(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                if cur_kind == "KW_VALID" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_valid_placement.deor
                    let mut is_valid: bool = false;
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_valid_placement.deor
                        let mut prev_pos: i64 = locate_prev_token(pos.clone());
                        let mut prev_token: Token = tokens[prev_pos as usize].clone();
                        let mut kind = prev_token.kind.clone();
                        is_valid = kind == "KW_IS";
                        if !is_valid {
                            // transpiler-deor/tokens_validator/macros/syntax_rules/check_valid_placement.deor
                            is_valid = kind == "KW_NOT";
                        }
                    }
                    if !is_valid {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_valid_placement.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_valid.clone()).clone());
                    }
                }
            }
            // macro: check_end_placement (transpiler-deor/tokens_validator/macros/syntax_rules/check_end_placement.deor)
            {
                // transpiler-deor/tokens_validator/macros/syntax_rules/check_end_placement.deor
                fn locate_prev_token(kw_pos: i64) -> i64 {
                    return kw_pos - 1;
                }
                if cur_kind == "KW_END" {
                    // transpiler-deor/tokens_validator/macros/syntax_rules/check_end_placement.deor
                    let mut is_valid: bool = false;
                    if pos > 0 {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_end_placement.deor
                        let mut prev_pos: i64 = locate_prev_token(pos.clone());
                        let mut prev_token: Token = tokens[prev_pos as usize].clone();
                        let mut kind = prev_token.kind.clone();
                        is_valid = kind == "KW_AT";
                    }
                    if !is_valid {
                        // transpiler-deor/tokens_validator/macros/syntax_rules/check_end_placement.deor
                        errors.push(val_err(tok.clone(), lbl_var.clone(), rule_end.clone()).clone());
                    }
                }
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
    let mut empty_str: String = "".to_string();
    return make_result(empty_str.clone(), cur.clone());
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
            let mut field_name_pos: i64 = field_index + 1;
            if field_name_pos < token_count {
                // transpiler-deor/registry/struct.deor
                let mut field_name_token: Token = tokens[field_name_pos as usize].clone();
                let kind = field_name_token.kind.clone();
                let value = field_name_token.value.clone();
                if kind == "IDENT" {
                    // transpiler-deor/registry/struct.deor
                    fields.push(value.clone());
                    cur = field_name_pos + 1;
                }
            }
        }
    }
    let mut comma: String = ",".to_string();
    let mut fields_joined: String = s_join_with(fields.clone(), comma.clone());
    return make_result(fields_joined.clone(), cur.clone());
}

fn try_struct_entry(tokens: TokensRef, pos: i64) -> Reg2Scan {
    // transpiler-deor/registry/struct.deor
    fn locate_struct_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_struct_body_start(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut matched: bool = false;
    let mut key: String = "".to_string();
    let mut val: String = "".to_string();
    let mut new_pos: i64 = locate_struct_name(pos.clone());
    let mut name_pos: i64 = locate_struct_name(pos.clone());
    if name_pos < token_count {
        // transpiler-deor/registry/struct.deor
        let mut name_token: Token = tokens[name_pos as usize].clone();
        let kind = name_token.kind.clone();
        let value = name_token.value.clone();
        if kind == "IDENT" {
            // transpiler-deor/registry/struct.deor
            matched = true;
            key = value;
            let mut block_start: i64 = locate_struct_body_start(pos.clone());
            let mut block_result: ParseResult = skip_to_block_start(tokens.clone(), block_start.clone());
            let mut field_start: i64 = pr_pos(block_result.clone());
            let mut fields_result: ParseResult = collect_struct_fields(tokens.clone(), field_start.clone());
            val = pr_code(fields_result.clone());
            new_pos = pr_pos(fields_result.clone());
        }
    }
    let mut result = Reg2Scan { matched: matched.clone(), key: key.clone(), val: val.clone(), new_pos: new_pos.clone() };
    return result;
}

// transpiler-deor/registry/shape.deor
fn try_shape_entry(tokens: TokensRef, pos: i64) -> Reg2Scan {
    // transpiler-deor/registry/shape.deor
    fn locate_shape_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_shape_form(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
    fn locate_func_of_or_to(kw_pos: i64) -> i64 {
        return kw_pos + 4;
    }
    fn locate_list_elem(kw_pos: i64) -> i64 {
        return kw_pos + 5;
    }
    fn locate_func_in_type(kw_pos: i64) -> i64 {
        return kw_pos + 5;
    }
    fn locate_func_to_after_of(kw_pos: i64) -> i64 {
        return kw_pos + 6;
    }
    fn locate_func_out_type_after_of(kw_pos: i64) -> i64 {
        return kw_pos + 7;
    }
    fn locate_func_out_type_after_to(kw_pos: i64) -> i64 {
        return kw_pos + 5;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut matched: bool = false;
    let mut key: String = "".to_string();
    let mut val: String = "".to_string();
    let mut new_pos: i64 = locate_shape_name(pos.clone());
    let mut name_pos: i64 = locate_shape_name(pos.clone());
    let mut form_pos: i64 = locate_shape_form(pos.clone());
    let mut of_or_to_pos: i64 = locate_func_of_or_to(pos.clone());
    if of_or_to_pos < token_count {
        // transpiler-deor/registry/shape.deor
        let mut name_token: Token = tokens[name_pos as usize].clone();
        let mut form_token: Token = tokens[form_pos as usize].clone();
        let value = name_token.value.clone();
        let mut shape_name: String = value.clone();
        let kind = form_token.kind.clone();
        if kind == "KW_LIST" {
            // transpiler-deor/registry/shape.deor
            let mut elem_pos: i64 = locate_list_elem(pos.clone());
            if elem_pos < token_count {
                // transpiler-deor/registry/shape.deor
                let mut elem_token: Token = tokens[elem_pos as usize].clone();
                let value = elem_token.value.clone();
                matched = true;
                key = shape_name;
                val = value;
            }
        } else {
            // transpiler-deor/registry/shape.deor
            let mut of_or_to_token: Token = tokens[of_or_to_pos as usize].clone();
            let kind = of_or_to_token.kind.clone();
            let value = of_or_to_token.value.clone();
            let mut is_of: bool = kind == "KW_OF";
            let mut is_to: bool = kind == "KW_TO";
            let mut in_type: String = "".to_string();
            let mut out_type: String = "".to_string();
            if is_of {
                // transpiler-deor/registry/shape.deor
                let mut in_type_pos: i64 = locate_func_in_type(pos.clone());
                if in_type_pos < token_count {
                    // transpiler-deor/registry/shape.deor
                    let mut in_type_token: Token = tokens[in_type_pos as usize].clone();
                    let value = in_type_token.value.clone();
                    in_type = value;
                }
                let mut to_pos: i64 = locate_func_to_after_of(pos.clone());
                if to_pos < token_count {
                    // transpiler-deor/registry/shape.deor
                    let mut to_token: Token = tokens[to_pos as usize].clone();
                    let kind = to_token.kind.clone();
                    let value = to_token.value.clone();
                    let mut has_to: bool = kind == "KW_TO";
                    if has_to {
                        // transpiler-deor/registry/shape.deor
                        let mut out_type_pos: i64 = locate_func_out_type_after_of(pos.clone());
                        if out_type_pos < token_count {
                            // transpiler-deor/registry/shape.deor
                            let mut out_type_token: Token = tokens[out_type_pos as usize].clone();
                            let value = out_type_token.value.clone();
                            out_type = value;
                        }
                    }
                }
            } else if is_to {
                // transpiler-deor/registry/shape.deor
                let mut out_type_pos: i64 = locate_func_out_type_after_to(pos.clone());
                if out_type_pos < token_count {
                    // transpiler-deor/registry/shape.deor
                    let mut out_type_token: Token = tokens[out_type_pos as usize].clone();
                    let value = out_type_token.value.clone();
                    out_type = value;
                }
            }
            matched = true;
            key = shape_name;
            val = ["fn:", in_type.as_str(), ":", out_type.as_str()].concat();
        }
    }
    let mut result = Reg2Scan { matched: matched.clone(), key: key.clone(), val: val.clone(), new_pos: new_pos.clone() };
    return result;
}

fn try_raw_entry(tokens: TokensRef, pos: i64) -> Reg2Scan {
    // transpiler-deor/registry/shape.deor
    fn locate_raw_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut matched: bool = false;
    let mut key: String = "".to_string();
    let mut val: String = "".to_string();
    let mut new_pos: i64 = locate_raw_name(pos.clone());
    let mut name_pos: i64 = locate_raw_name(pos.clone());
    if name_pos < token_count {
        // transpiler-deor/registry/shape.deor
        let mut name_token: Token = tokens[name_pos as usize].clone();
        let value = name_token.value.clone();
        matched = true;
        key = value;
        val = "raw:".to_string();
    }
    let mut result = Reg2Scan { matched: matched.clone(), key: key.clone(), val: val.clone(), new_pos: new_pos.clone() };
    return result;
}

// transpiler-deor/registry/enum.deor
#[derive(Clone, PartialEq, Debug)]
struct PairsScan {
    pairs: Vec<String>,
    new_pos: i64,
}

#[derive(Clone, PartialEq, Debug)]
struct EnumScan {
    matched: bool,
    is_typed: bool,
    name: String,
    rust_name: String,
    val_type: String,
    pairs: Vec<String>,
    new_pos: i64,
}

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

fn ps_pairs(scan: PairsScan) -> Vec<String> {
    // transpiler-deor/registry/enum.deor
    scan.pairs
}

fn ps_new_pos(scan: PairsScan) -> i64 {
    // transpiler-deor/registry/enum.deor
    scan.new_pos
}

fn collect_variant_pairs(tokens: TokensRef, start: i64, rust_name: String) -> PairsScan {
    // transpiler-deor/registry/enum.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut pairs: Vec<String> = Vec::new();
    let mut new_pos: i64 = start.clone();
    let mut scanning: bool = true;
    while scanning {
        // transpiler-deor/registry/enum.deor
        if new_pos >= token_count {
            // transpiler-deor/registry/enum.deor
            scanning = false;
        } else {
            // transpiler-deor/registry/enum.deor
            let mut variant_token: Token = tokens[new_pos as usize].clone();
            let kind = variant_token.kind.clone();
            let value = variant_token.value.clone();
            new_pos = new_pos + 1;
            if kind == "DEDENT" {
                // transpiler-deor/registry/enum.deor
                scanning = false;
            } else if kind == "IDENT" {
                // transpiler-deor/registry/enum.deor
                pairs.push(value.clone());
                pairs.push(rust_name.clone());
            }
        }
    }
    let mut result = PairsScan { pairs: pairs.clone(), new_pos: new_pos.clone() };
    return result;
}

fn collect_typed_variant_pairs(tokens: TokensRef, start: i64, enum_name: String) -> PairsScan {
    // transpiler-deor/registry/enum.deor
    let mut token_count: i64 = (tokens.len() as i64);
    let mut pairs: Vec<String> = Vec::new();
    let mut new_pos: i64 = start.clone();
    let mut scanning: bool = true;
    while scanning {
        // transpiler-deor/registry/enum.deor
        if new_pos >= token_count {
            // transpiler-deor/registry/enum.deor
            scanning = false;
        } else {
            // transpiler-deor/registry/enum.deor
            let mut variant_token: Token = tokens[new_pos as usize].clone();
            let kind = variant_token.kind.clone();
            let value = variant_token.value.clone();
            new_pos = new_pos + 1;
            if kind == "DEDENT" {
                // transpiler-deor/registry/enum.deor
                scanning = false;
            } else if kind == "IDENT" {
                // transpiler-deor/registry/enum.deor
                let mut variant_name: String = value.clone();
                if new_pos < token_count {
                    // transpiler-deor/registry/enum.deor
                    let mut equals_token: Token = tokens[new_pos as usize].clone();
                    let kind = equals_token.kind.clone();
                    new_pos = new_pos + 1;
                    if kind == "EQUALS" {
                        // transpiler-deor/registry/enum.deor
                        if new_pos < token_count {
                            // transpiler-deor/registry/enum.deor
                            let mut literal_token: Token = tokens[new_pos as usize].clone();
                            let value = literal_token.value.clone();
                            new_pos = new_pos + 1;
                            let mut dot: String = ".".to_string();
                            let mut variant_key: String = [enum_name.as_str(), dot.as_str(), variant_name.as_str()].concat();
                            pairs.push(variant_key.clone());
                            pairs.push(value.clone());
                        }
                    }
                }
            }
        }
    }
    let mut result = PairsScan { pairs: pairs.clone(), new_pos: new_pos.clone() };
    return result;
}

fn try_enum_entry(tokens: TokensRef, pos: i64) -> EnumScan {
    // transpiler-deor/registry/enum.deor
    fn locate_type_or_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_typed_name(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    fn locate_typed_body_start(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
    fn locate_untyped_body_start(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut matched: bool = false;
    let mut is_typed: bool = false;
    let mut name: String = "".to_string();
    let mut rust_name: String = "".to_string();
    let mut val_type: String = "".to_string();
    let mut pairs: Vec<String> = Vec::new();
    let mut new_pos: i64 = locate_type_or_name(pos.clone());
    let mut type_pos: i64 = locate_type_or_name(pos.clone());
    if type_pos < token_count {
        // transpiler-deor/registry/enum.deor
        let mut type_token: Token = tokens[type_pos as usize].clone();
        let kind = type_token.kind.clone();
        let value = type_token.value.clone();
        if kind == "IDENT" {
            // transpiler-deor/registry/enum.deor
            let mut word_is_typed: bool = is_typed_enum_type(value.clone());
            if word_is_typed {
                // transpiler-deor/registry/enum.deor
                matched = true;
                is_typed = true;
                val_type = value;
                let mut name_pos: i64 = locate_typed_name(pos.clone());
                if name_pos < token_count {
                    // transpiler-deor/registry/enum.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let value = name_token.value.clone();
                    name = value;
                    let mut body_start: i64 = locate_typed_body_start(pos.clone());
                    let mut block_result: ParseResult = skip_to_block_start(tokens.clone(), body_start.clone());
                    let mut scan_start: i64 = pr_pos(block_result.clone());
                    let mut pairs_result: PairsScan = collect_typed_variant_pairs(tokens.clone(), scan_start.clone(), name.clone());
                    pairs = ps_pairs(pairs_result.clone());
                    new_pos = ps_new_pos(pairs_result.clone());
                }
            } else {
                // transpiler-deor/registry/enum.deor
                matched = true;
                is_typed = false;
                name = value;
                rust_name = s_pascal(name.clone());
                let mut body_start: i64 = locate_untyped_body_start(pos.clone());
                let mut block_result: ParseResult = skip_to_block_start(tokens.clone(), body_start.clone());
                let mut scan_start: i64 = pr_pos(block_result.clone());
                let mut pairs_result: PairsScan = collect_variant_pairs(tokens.clone(), scan_start.clone(), rust_name.clone());
                pairs = ps_pairs(pairs_result.clone());
                new_pos = ps_new_pos(pairs_result.clone());
            }
        }
    }
    let mut result = EnumScan { matched: matched.clone(), is_typed: is_typed.clone(), name: name.clone(), rust_name: rust_name.clone(), val_type: val_type.clone(), pairs: pairs.clone(), new_pos: new_pos.clone() };
    return result;
}

// transpiler-deor/registry/validator_type.deor
fn try_type_entry(tokens: TokensRef, pos: i64) -> Reg3Scan {
    // transpiler-deor/registry/validator_type.deor
    fn locate_type_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_type_param_type(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
    fn locate_type_param_name(kw_pos: i64) -> i64 {
        return kw_pos + 4;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut matched: bool = false;
    let mut key: String = "".to_string();
    let mut val: String = "".to_string();
    let mut val2: String = "".to_string();
    let mut new_pos: i64 = locate_type_name(pos.clone());
    let mut name_pos: i64 = locate_type_name(pos.clone());
    let mut param_type_pos: i64 = locate_type_param_type(pos.clone());
    let mut param_name_pos: i64 = locate_type_param_name(pos.clone());
    if param_name_pos < token_count {
        // transpiler-deor/registry/validator_type.deor
        let mut name_token: Token = tokens[name_pos as usize].clone();
        let mut param_type_token: Token = tokens[param_type_pos as usize].clone();
        let mut param_name_token: Token = tokens[param_name_pos as usize].clone();
        matched = true;
        let value = name_token.value.clone();
        key = value;
        let value = param_type_token.value.clone();
        val = value;
        let value = param_name_token.value.clone();
        val2 = value;
    }
    let mut result = Reg3Scan { matched: matched.clone(), key: key.clone(), val: val.clone(), val2: val2.clone(), new_pos: new_pos.clone() };
    return result;
}

fn collect_validator_var_types(tokens: Vec<Token>, type_reg: Vec<String>) -> Vec<String> {
    // transpiler-deor/registry/validator_type.deor
    let mut result: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens.len() as i64);
    for index in 0..token_count {
        // transpiler-deor/registry/validator_type.deor
        let mut token: Token = tokens[index as usize].clone();
        let kind = token.kind.clone();
        let value = token.value.clone();
        if kind == "IDENT" {
            // transpiler-deor/registry/validator_type.deor
            let mut maybe_type: String = value.clone();
            let mut is_vtype: bool = reg3_has(type_reg.clone(), maybe_type.clone());
            if is_vtype {
                // transpiler-deor/registry/validator_type.deor
                let mut next_pos: i64 = index + 1;
                if next_pos < token_count {
                    // transpiler-deor/registry/validator_type.deor
                    let mut next_token: Token = tokens[next_pos as usize].clone();
                    let kind = next_token.kind.clone();
                    let value = next_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/registry/validator_type.deor
                        let mut var_name: String = value.clone();
                        result.push(var_name.clone());
                        result.push(maybe_type.clone());
                    }
                }
            }
        }
    }
    return result;
}

// transpiler-deor/registry/type_resolve.deor
fn resolve_type(type_name: String, ctx: RcCtx) -> String {
    // transpiler-deor/registry/type_resolve.deor
    let shape_reg = ctx.shape_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let type_reg = ctx.type_reg.clone();
    let mut is_validator: bool = reg3_has(type_reg.clone(), type_name.clone());
    if is_validator {
        // transpiler-deor/registry/type_resolve.deor
        let mut opt_parts: Vec<String> = vec!["Option<".to_string(), type_name.clone(), ">".to_string()];
        return s_join(opt_parts.clone());
    }
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
fn find_block_end_ref(tokens: TokensRef, indent_pos: i64) -> i64 {
    // transpiler-deor/registry/mut_scan.deor
    fn locate_block_body_start(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut token_count: i64 = (tokens.len() as i64);
    let mut depth: i64 = 1;
    let mut result: i64 = indent_pos.clone();
    let mut start: i64 = locate_block_body_start(indent_pos.clone());
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
            let mut const_name_pos: i64 = raw_i + 2;
            if const_name_pos < end_pos {
                // transpiler-deor/registry/mut_scan.deor
                let mut const_name_token: Token = tokens[const_name_pos as usize].clone();
                let value = const_name_token.value.clone();
                if !list_has(const_names.clone(), value.clone()) {
                    // transpiler-deor/registry/mut_scan.deor
                    const_names.push(value.clone());
                }
            }
        }
        if kind == "EQUALS" {
            // transpiler-deor/registry/mut_scan.deor
            let mut prev_pos: i64 = raw_i - 1;
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
    let mut struct_reg: Vec<String> = Vec::new();
    let mut shape_reg: Vec<String> = Vec::new();
    let mut enum_reg: Vec<String> = Vec::new();
    let mut variant_reg: Vec<String> = Vec::new();
    let mut type_reg: Vec<String> = Vec::new();
    let mut typed_enum_reg: Vec<String> = Vec::new();
    let mut typed_variant_reg: Vec<String> = Vec::new();
    let mut token_count: i64 = (tokens_ref.len() as i64);
    let mut pos: i64 = 0;
    while pos < token_count {
        // transpiler-deor/registry/registry.deor
        let mut token: Token = tokens_ref[pos as usize].clone();
        let kind = token.kind.clone();
        if kind == "KW_STRUCT" {
            // transpiler-deor/registry/registry.deor
            let mut scan: Reg2Scan = try_struct_entry(tokens_ref.clone(), pos.clone());
            let matched = scan.matched;
            let key = scan.key;
            let val = scan.val;
            let new_pos = scan.new_pos;
            if matched {
                // transpiler-deor/registry/registry.deor
                struct_reg.push(key.clone());
                struct_reg.push(val.clone());
            }
            pos = new_pos;
            continue;
        }
        if kind == "KW_SHAPE" {
            // transpiler-deor/registry/registry.deor
            let mut scan: Reg2Scan = try_shape_entry(tokens_ref.clone(), pos.clone());
            let matched = scan.matched;
            let key = scan.key;
            let val = scan.val;
            let new_pos = scan.new_pos;
            if matched {
                // transpiler-deor/registry/registry.deor
                shape_reg.push(key.clone());
                shape_reg.push(val.clone());
            }
            pos = new_pos;
            continue;
        }
        if kind == "KW_RAW" {
            // transpiler-deor/registry/registry.deor
            let mut scan: Reg2Scan = try_raw_entry(tokens_ref.clone(), pos.clone());
            let matched = scan.matched;
            let key = scan.key;
            let val = scan.val;
            let new_pos = scan.new_pos;
            if matched {
                // transpiler-deor/registry/registry.deor
                shape_reg.push(key.clone());
                shape_reg.push(val.clone());
            }
            pos = new_pos;
            continue;
        }
        if kind == "KW_TYPE" {
            // transpiler-deor/registry/registry.deor
            let mut scan: Reg3Scan = try_type_entry(tokens_ref.clone(), pos.clone());
            let matched = scan.matched;
            let key = scan.key;
            let val = scan.val;
            let val2 = scan.val2;
            let new_pos = scan.new_pos;
            if matched {
                // transpiler-deor/registry/registry.deor
                type_reg.push(key.clone());
                type_reg.push(val.clone());
                type_reg.push(val2.clone());
            }
            pos = new_pos;
            continue;
        }
        if kind == "KW_ENUM" {
            // transpiler-deor/registry/registry.deor
            let mut scan: EnumScan = try_enum_entry(tokens_ref.clone(), pos.clone());
            let matched = scan.matched;
            let is_typed = scan.is_typed;
            let name = scan.name;
            let rust_name = scan.rust_name;
            let val_type = scan.val_type;
            let pairs = scan.pairs;
            let new_pos = scan.new_pos;
            if matched {
                // transpiler-deor/registry/registry.deor
                if is_typed {
                    // transpiler-deor/registry/registry.deor
                    typed_enum_reg.push(name.clone());
                    typed_enum_reg.push(val_type.clone());
                    let mut typed_variant_count: i64 = (pairs.len() as i64);
                    for typed_variant_index in 0..typed_variant_count {
                        // transpiler-deor/registry/registry.deor
                        typed_variant_reg.push(pairs[typed_variant_index as usize].clone().clone());
                    }
                } else {
                    // transpiler-deor/registry/registry.deor
                    enum_reg.push(name.clone());
                    enum_reg.push(rust_name.clone());
                    let mut variant_count: i64 = (pairs.len() as i64);
                    for variant_index in 0..variant_count {
                        // transpiler-deor/registry/registry.deor
                        variant_reg.push(pairs[variant_index as usize].clone().clone());
                    }
                }
            }
            pos = new_pos;
            continue;
        }
        pos = pos + 1;
    }
    let mut mut_names: Vec<String> = Vec::new();
    let mut validator_var_reg: Vec<String> = Vec::new();
    let mut placeholder: Vec<Token> = Vec::new();
    let mut tokens: TokensRef = tokens_wrap(placeholder);
    let mut ctx_raw: GenCtx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, tokens, typed_enum_reg, typed_variant_reg, validator_var_reg };
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
        let mut arg_code = code.clone();
        let arg_pos = new_pos.clone();
        let mut ca_is_chain: bool = is_expr_chain(tokens.clone(), cur.clone(), arg_pos.clone());
        let mut start_token: Token = tokens[cur as usize].clone();
        let kind = start_token.kind.clone();
        if !ca_is_chain {
            // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
            if kind == "STRING" {
                // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
                arg_code = [arg_code.as_str(), RS_TOS.as_str()].concat();
            } else if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/expr/call_args.deor
                let mut next_cur: i64 = cur + 1;
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
        let mut item_code = code.clone();
        let item_pos = new_pos.clone();
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
        let mut item_code = code.clone();
        let item_pos = new_pos.clone();
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
    let mut inner_result: ParseResult = gen_expr(tokens.clone(), args_pos.clone(), ctx.clone());
    let code = inner_result.code;
    let new_pos = inner_result.new_pos;
    let inner_code = code.clone();
    let close = new_pos + 1;
    let mut result_code: String = [inner_code.as_str(), suffix.as_str()].concat();
    return make_result(result_code, close.clone());
}

fn gen_primary(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/expr/primary.deor
    fn locate_next_token(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
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
    {
        // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
        fn locate_next_token(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        if kind == "INT" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
            let mut int_value: String = value.clone();
            let mut in_float_ctx: bool = float_ctx_get();
            if in_float_ctx {
                // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
                let mut float_suffix: String = ".0".to_string();
                int_value = s_cat(int_value.clone(), float_suffix.clone());
            }
            let mut next_pos: i64 = locate_next_token(pos.clone());
            return make_result(int_value, next_pos.clone());
        }
        if kind == "FLOAT" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
            let mut next_pos: i64 = locate_next_token(pos.clone());
            return make_result(value, next_pos.clone());
        }
        if kind == "STRING" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
            let mut string_debug: String = s_debug(value.clone());
            let mut next_pos: i64 = locate_next_token(pos.clone());
            return make_result(string_debug, next_pos.clone());
        }
        if kind == "KW_TRUE" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
            let mut true_value: String = "true".to_string();
            let mut next_pos: i64 = locate_next_token(pos.clone());
            return make_result(true_value, next_pos.clone());
        }
        if kind == "KW_FALSE" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/literals.deor
            let mut false_value: String = "false".to_string();
            let mut next_pos: i64 = locate_next_token(pos.clone());
            return make_result(false_value, next_pos.clone());
        }
    }
    // macro: primary_list_literal (transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor)
    {
        // transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor
        fn locate_next_token(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        if kind == "LBRACKET" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/list_literal.deor
            let mut inner_pos: i64 = locate_next_token(pos.clone());
            let mut items_result: ParseResult = gen_list_items(tokens.clone(), inner_pos.clone(), ctx.clone());
            let code = items_result.code;
            let new_pos = items_result.new_pos;
            let items_code = code.clone();
            let items_pos = new_pos.clone();
            let mut list_open: String = "vec![".to_string();
            let mut list_close: String = "]".to_string();
            let mut list_code: String = [list_open.as_str(), items_code.as_str(), list_close.as_str()].concat();
            let mut after_pos: i64 = items_pos + 1;
            return make_result(list_code, after_pos.clone());
        }
    }
    // macro: primary_paren_expr (transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor)
    {
        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
        fn locate_next_token(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
            let mut peek_pos: i64 = locate_next_token(pos.clone());
            if peek_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                let mut peek_token: Token = tokens[peek_pos as usize].clone();
                let kind = peek_token.kind.clone();
                if kind == "KW_AVOW" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    let mut avow_expr_pos: i64 = locate_next_token(peek_pos.clone());
                    let mut avow_expr_result: ParseResult = gen_expr(tokens.clone(), avow_expr_pos.clone(), ctx.clone());
                    let code = avow_expr_result.code;
                    let new_pos = avow_expr_result.new_pos;
                    let avow_expr_code = code.clone();
                    let avow_after = new_pos + 1;
                    let mut unwrap_suffix: String = ".unwrap().0".to_string();
                    let mut avow_unwrap_code: String = [avow_expr_code.as_str(), unwrap_suffix.as_str()].concat();
                    return make_result(avow_unwrap_code, avow_after.clone());
                }
                let mut is_struct: bool = true;
                let mut scan_pos: i64 = peek_pos.clone();
                while scan_pos < token_count {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let kind = scan_token.kind.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        break;
                    }
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        scan_pos = scan_pos + 1;
                        continue;
                    }
                    if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        scan_pos = scan_pos + 1;
                        continue;
                    }
                    is_struct = false;
                    break;
                }
                if is_struct {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                    let mut fields: Vec<String> = Vec::new();
                    let mut field_scan_pos: i64 = peek_pos.clone();
                    while field_scan_pos < token_count {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                        let mut field_token: Token = tokens[field_scan_pos as usize].clone();
                        let kind = field_token.kind.clone();
                        let value = field_token.value.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                            field_scan_pos = field_scan_pos + 1;
                            break;
                        } else if kind == "COMMA" {
                            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                            field_scan_pos = field_scan_pos + 1;
                        } else if kind == "IDENT" {
                            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                            fields.push(value.clone());
                            field_scan_pos = field_scan_pos + 1;
                        } else {
                            // transpiler-deor/codegen/decl/stmt/expr/macros/paren_expr.deor
                            field_scan_pos = field_scan_pos + 1;
                        }
                    }
                    let mut struct_name: String = find_struct_for_fields(struct_reg.clone(), fields.clone());
                    let mut fields_separator: String = ", ".to_string();
                    let mut fields_code: String = s_join_with(fields.clone(), fields_separator.clone());
                    let mut open_brace: String = " { ".to_string();
                    let mut close_brace: String = " }".to_string();
                    let mut struct_code: String = [struct_name.as_str(), open_brace.as_str(), fields_code.as_str(), close_brace.as_str()].concat();
                    return make_result(struct_code, field_scan_pos.clone());
                }
                let mut grouped_result: ParseResult = gen_expr(tokens.clone(), peek_pos.clone(), ctx.clone());
                let code = grouped_result.code;
                let new_pos = grouped_result.new_pos;
                let grouped_inner_code = code.clone();
                let grouped_after = new_pos + 1;
                let mut paren_open: String = "(".to_string();
                let mut paren_close: String = ")".to_string();
                let mut grouped_code: String = [paren_open.as_str(), grouped_inner_code.as_str(), paren_close.as_str()].concat();
                return make_result(grouped_code, grouped_after.clone());
            }
        }
    }
    // macro: primary_prefix_ops (transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor)
    {
        // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
        fn locate_next_token(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        if kind == "KW_MOVE" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
            let mut inner_pos: i64 = locate_next_token(pos.clone());
            let mut result: ParseResult = gen_primary(tokens.clone(), inner_pos.clone(), ctx.clone());
            return result;
        }
        if kind == "KW_AVOW" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
            let mut avow_inner_pos: i64 = locate_next_token(pos.clone());
            let mut avow_result: ParseResult = gen_primary(tokens.clone(), avow_inner_pos.clone(), ctx.clone());
            let code = avow_result.code;
            let new_pos = avow_result.new_pos;
            let avow_code = code.clone();
            let avow_end = new_pos.clone();
            let mut avow_suffix: String = ".unwrap().0".to_string();
            let mut avow_unwrap_code: String = [avow_code.as_str(), avow_suffix.as_str()].concat();
            return make_result(avow_unwrap_code, avow_end.clone());
        }
        if kind == "KW_NOT" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/prefix_ops.deor
            let mut operand_pos: i64 = locate_next_token(pos.clone());
            let mut operand_result: ParseResult = gen_primary(tokens.clone(), operand_pos.clone(), ctx.clone());
            let code = operand_result.code;
            let new_pos = operand_result.new_pos;
            let operand_code = code.clone();
            let operand_end = new_pos.clone();
            let mut bang: String = "!".to_string();
            let mut not_code: String = [bang.as_str(), operand_code.as_str()].concat();
            return make_result(not_code, operand_end.clone());
        }
    }
    // macro: primary_ident_expr (transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor)
    {
        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
        fn locate_next_token(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
            let mut next_pos: i64 = locate_next_token(pos.clone());
            if next_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                let mut next_token: Token = tokens[next_pos as usize].clone();
                let kind = next_token.kind.clone();
                if kind == "LPAREN" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut func_name: String = value.clone();
                    let mut args_pos: i64 = locate_next_token(next_pos.clone());
                    if func_name == "len" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                        let mut len_suffix: String = ".len() as i64".to_string();
                        let mut len_result: ParseResult = gen_unary_method(args_pos.clone(), len_suffix.clone(), ctx.clone());
                        let code = len_result.code;
                        let new_pos = len_result.new_pos;
                        let len_code = code.clone();
                        let len_end = new_pos.clone();
                        let mut paren_open: String = "(".to_string();
                        let mut paren_close: String = ")".to_string();
                        let mut len_wrapped: String = [paren_open.as_str(), len_code.as_str(), paren_close.as_str()].concat();
                        return make_result(len_wrapped, len_end.clone());
                    } else if func_name == "crash" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                        let mut crash_result: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
                        let code = crash_result.code;
                        let new_pos = crash_result.new_pos;
                        let crash_code = code.clone();
                        let crash_end = new_pos.clone();
                        let mut panic_prefix: String = "panic!(\"{}\", ".to_string();
                        let mut panic_suffix: String = ")".to_string();
                        let mut panic_code: String = [panic_prefix.as_str(), crash_code.as_str(), panic_suffix.as_str()].concat();
                        let mut after_crash: i64 = crash_end + 1;
                        return make_result(panic_code, after_crash.clone());
                    }
                    if func_name == "s_join" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                        let mut join_arg_token: Token = tokens[args_pos as usize].clone();
                        let kind = join_arg_token.kind.clone();
                        if kind == "LBRACKET" {
                            // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                            let mut join_inner_pos: i64 = locate_next_token(args_pos.clone());
                            let mut join_result: ParseResult = gen_join_items(tokens.clone(), join_inner_pos.clone(), ctx.clone());
                            let code = join_result.code;
                            let new_pos = join_result.new_pos;
                            let join_items_code = code.clone();
                            let join_end = new_pos.clone();
                            let mut join_after: i64 = join_end + 2;
                            let mut join_open: String = "[".to_string();
                            let mut join_close: String = "].concat()".to_string();
                            let mut join_code: String = [join_open.as_str(), join_items_code.as_str(), join_close.as_str()].concat();
                            return make_result(join_code, join_after.clone());
                        }
                    }
                    let mut call_args_result: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
                    let code = call_args_result.code;
                    let new_pos = call_args_result.new_pos;
                    let call_args_code = code.clone();
                    let call_args_end = new_pos.clone();
                    let mut call_after: i64 = call_args_end + 1;
                    let mut call_paren_open: String = "(".to_string();
                    let mut call_paren_close: String = ")".to_string();
                    let mut call_code: String = [func_name.as_str(), call_paren_open.as_str(), call_args_code.as_str(), call_paren_close.as_str()].concat();
                    return make_result(call_code, call_after.clone());
                }
                if kind == "KW_AT" {
                    // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                    let mut index_pos: i64 = locate_next_token(next_pos.clone());
                    let mut index_result: ParseResult = gen_primary(tokens.clone(), index_pos.clone(), ctx.clone());
                    let code = index_result.code;
                    let new_pos = index_result.new_pos;
                    let index_code = code.clone();
                    let index_end = new_pos.clone();
                    let mut index_mid: String = "[".to_string();
                    let mut index_suffix: String = " as usize].clone()".to_string();
                    let mut index_expr: String = [value.as_str(), index_mid.as_str(), index_code.as_str(), index_suffix.as_str()].concat();
                    return make_result(index_expr, index_end.clone());
                }
            }
            let mut variant_enum: String = reg_get(variant_reg.clone(), value.clone());
            if !is_empty(variant_enum.clone()) {
                // transpiler-deor/codegen/decl/stmt/expr/macros/ident_expr.deor
                let mut double_colon: String = "::".to_string();
                let mut variant_code: String = [variant_enum.as_str(), double_colon.as_str(), value.as_str()].concat();
                return make_result(variant_code, next_pos.clone());
            }
            return make_result(value, next_pos.clone());
        }
    }
    // transpiler-deor/codegen/decl/stmt/expr/primary.deor
    let mut unknown: String = "/* unknown_primary */".to_string();
    let mut next: i64 = locate_next_token(pos.clone());
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
    let mut primary_result: ParseResult = gen_primary(tokens.clone(), pos.clone(), ctx.clone());
    let code = primary_result.code;
    let new_pos = primary_result.new_pos;
    let mut left_code: String = code.clone();
    let mut cur_pos: i64 = new_pos.clone();
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
        let mut after_op: i64 = cur_pos + 1;
        // macro: expr_is_special (transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor)
        {
            // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
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
                        let mut is_empty_suffix: String = ".is_empty()".to_string();
                        left_code = s_cat(left_code, is_empty_suffix.clone());
                        cur_pos = after_op + 1;
                        continue;
                    }
                    if kind == "KW_VALID" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                        let mut is_valid_suffix: String = ".is_some()".to_string();
                        left_code = s_cat(left_code, is_valid_suffix.clone());
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
                        let mut not_empty_prefix: String = "!".to_string();
                        let mut not_empty_suffix: String = ".is_empty()".to_string();
                        left_code = s_cat(not_empty_prefix.clone(), left_code);
                        left_code = s_cat(left_code, not_empty_suffix.clone());
                        cur_pos = after_op + 1;
                        continue;
                    }
                    if kind == "KW_VALID" {
                        // transpiler-deor/codegen/decl/stmt/expr/macros/expr_is_special.deor
                        let mut not_valid_prefix: String = "!(".to_string();
                        let mut not_valid_suffix: String = ".is_some())".to_string();
                        left_code = s_cat(not_valid_prefix.clone(), left_code);
                        left_code = s_cat(left_code, not_valid_suffix.clone());
                        cur_pos = after_op + 1;
                        continue;
                    }
                }
            }
        }
        // transpiler-deor/codegen/decl/stmt/expr/expr.deor
        let mut rhs_result: ParseResult = gen_primary(tokens.clone(), after_op.clone(), ctx.clone());
        let code = rhs_result.code;
        let new_pos = rhs_result.new_pos;
        let rhs_code = code.clone();
        let rhs_pos = new_pos.clone();
        let mut rust_op: String = map_op(operator_str.clone());
        let mut operator_space: String = " ".to_string();
        left_code = s_cat(left_code, operator_space.clone());
        left_code = s_cat(left_code, rust_op.clone());
        left_code = s_cat(left_code, operator_space.clone());
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

fn is_expr_chain(tokens: TokensRef, start: i64, stop: i64) -> bool {
    // transpiler-deor/codegen/decl/stmt/helpers.deor
    let mut depth: i64 = 0;
    let mut scan: i64 = start.clone();
    while scan < stop {
        // transpiler-deor/codegen/decl/stmt/helpers.deor
        let mut scan_tok: Token = tokens[scan as usize].clone();
        let kind = scan_tok.kind.clone();
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/helpers.deor
            depth = depth + 1;
        }
        if kind == "RPAREN" {
            // transpiler-deor/codegen/decl/stmt/helpers.deor
            if depth > 0 {
                // transpiler-deor/codegen/decl/stmt/helpers.deor
                depth = depth - 1;
            }
        }
        if depth == 0 {
            // transpiler-deor/codegen/decl/stmt/helpers.deor
            if is_binary_op(kind.clone()) {
                // transpiler-deor/codegen/decl/stmt/helpers.deor
                return true;
            }
        }
        scan = scan + 1;
    }
    return false;
}

// transpiler-deor/codegen/decl/stmt/destructure.deor
fn gen_destructure(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // macro: initialize_gen_destructure (transpiler-deor/codegen/decl/stmt/macros/initialize_gen_destructure.deor)
    fn locate_first_field(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i64 = locate_first_field(pos.clone());
    // macro: for_collect_fields_into_fields_list (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
        while cur < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
            let mut field_token: Token = tokens[cur as usize].clone();
            let kind = field_token.kind.clone();
            let value = field_token.value.clone();
            if kind == "RPAREN" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                cur = cur + 1;
                break;
            } else if kind == "COMMA" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                cur = cur + 1;
            } else if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                fields.push(value.clone());
                cur = cur + 1;
            } else {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                cur = cur + 1;
            }
        }
    }
    // macro: gen_input_check (transpiler-deor/codegen/decl/stmt/macros/input_destructure/gen_input_check.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gen_input_check.deor
        fn locate_after_call(kw_pos: i64) -> i64 {
            return kw_pos + 4;
        }
        let mut matched: bool = false;
        let mut is_args: bool = false;
        let mut name_ok: bool = false;
        let mut lparen_pos: i64 = 0;
        // macro: gic_match_kw_and_name (transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
            fn locate_next_token(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut in_token: Token = tokens[cur as usize].clone();
            let kind = in_token.kind.clone();
            if kind == "KW_IN" {
                // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
                let mut name_pos: i64 = locate_next_token(cur.clone());
                if name_pos < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let kind = name_token.kind.clone();
                    let value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
                        if value == "input" {
                            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
                            name_ok = true;
                        } else if value == "args" {
                            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
                            name_ok = true;
                            is_args = true;
                        }
                        if name_ok {
                            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_kw_and_name.deor
                            lparen_pos = locate_next_token(name_pos.clone());
                        }
                    }
                }
            }
        }
        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gen_input_check.deor
        if name_ok {
            // macro: gic_match_parens (transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_parens.deor)
            {
                // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_parens.deor
                fn locate_next_token(anchor: i64) -> i64 {
                    return anchor + 1;
                }
                if lparen_pos < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_parens.deor
                    let mut lparen_token: Token = tokens[lparen_pos as usize].clone();
                    let kind = lparen_token.kind.clone();
                    if kind == "LPAREN" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_parens.deor
                        let mut rparen_pos: i64 = locate_next_token(lparen_pos.clone());
                        if rparen_pos < token_count {
                            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_parens.deor
                            let mut rparen_token: Token = tokens[rparen_pos as usize].clone();
                            let kind = rparen_token.kind.clone();
                            if kind == "RPAREN" {
                                // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_match_parens.deor
                                matched = true;
                            }
                        }
                    }
                }
            }
        }
        if matched {
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
            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gen_input_check.deor
            let mut input_pad: String = s_repeat(RS_IND.clone(), depth.clone());
            let mut output_lines: Vec<String> = Vec::new();
            // macro: gic_emit_header (transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_header.deor)
            {
                // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_header.deor
                if is_args {
                    // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_header.deor
                    let mut args_decl: String = "let _deor_args: Vec<String> = std::env::args().skip(1).collect();".to_string();
                    let mut args_line: String = [input_pad.as_str(), args_decl.as_str(), RS_NL.as_str()].concat();
                    output_lines.push(args_line.clone());
                } else {
                    // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_header.deor
                    let mut raw_decl: String = "let mut _deor_raw: String = String::new();".to_string();
                    let mut raw_line: String = [input_pad.as_str(), raw_decl.as_str(), RS_NL.as_str()].concat();
                    output_lines.push(raw_line.clone());
                    let mut flush_decl: String = "std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());".to_string();
                    let mut flush_line: String = [input_pad.as_str(), flush_decl.as_str(), RS_NL.as_str()].concat();
                    output_lines.push(flush_line.clone());
                    let mut read_decl: String = "std::io::stdin().read_line(&mut _deor_raw).unwrap_or_default();".to_string();
                    let mut read_line: String = [input_pad.as_str(), read_decl.as_str(), RS_NL.as_str()].concat();
                    output_lines.push(read_line.clone());
                    let mut split_decl: String = "let _deor_args: Vec<String> = _deor_raw.split_whitespace().map(|s| s.to_string()).collect();".to_string();
                    let mut split_line: String = [input_pad.as_str(), split_decl.as_str(), RS_NL.as_str()].concat();
                    output_lines.push(split_line.clone());
                }
            }
            // macro: gic_emit_bindings (transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor)
            {
                // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                let mut field_count: i64 = (fields.len() as i64);
                for field_index in 0..field_count {
                    // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                    let mut field_name: String = fields[field_index as usize].clone();
                    let mut binding: String = "".to_string();
                    if field_name == "first" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                        binding = "let first: String = _deor_args.get(0).cloned().unwrap_or_default();".to_string();
                    } else if field_name == "second" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                        binding = "let second: String = _deor_args.get(1).cloned().unwrap_or_default();".to_string();
                    } else if field_name == "third" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                        binding = "let third: String = _deor_args.get(2).cloned().unwrap_or_default();".to_string();
                    } else if field_name == "input_string" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                        binding = "let input_string: String = _deor_args.join(\" \");".to_string();
                    } else if field_name == "input_list" {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                        binding = "let input_list: Vec<String> = _deor_args.clone();".to_string();
                    }
                    if !is_empty(binding.clone()) {
                        // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gic_emit_bindings.deor
                        let mut binding_line: String = [input_pad.as_str(), binding.as_str(), RS_NL.as_str()].concat();
                        output_lines.push(binding_line.clone());
                    }
                }
            }
            // transpiler-deor/codegen/decl/stmt/macros/input_destructure/gen_input_check.deor
            let mut result_code: String = s_join(output_lines.clone());
            let mut after_pos: i64 = locate_after_call(cur.clone());
            return make_nl_result(result_code, after_pos.clone(), tokens.clone());
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut field_count: i64 = (fields.len() as i64);
    // macro: gen_enum_extract_check (transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
        fn locate_next_token(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        fn locate_after_name(kw_pos: i64) -> i64 {
            return kw_pos + 2;
        }
        let typed_enum_reg = ctx.typed_enum_reg.clone();
        let typed_variant_reg = ctx.typed_variant_reg.clone();
        let mut matched: bool = false;
        if cur < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
            let mut in_token: Token = tokens[cur as usize].clone();
            let kind = in_token.kind.clone();
            if kind == "KW_IN" {
                // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                let mut name_pos: i64 = locate_next_token(cur.clone());
                if name_pos < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                    let mut name_token: Token = tokens[name_pos as usize].clone();
                    let kind = name_token.kind.clone();
                    let value = name_token.value.clone();
                    if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                        let mut val_type: String = reg_get(typed_enum_reg.clone(), value.clone());
                        if !is_empty(val_type.clone()) {
                            // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                            matched = true;
                            let mut enum_name: String = value.clone();
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
                            let mut pad: String = s_repeat(RS_IND.clone(), depth.clone());
                            let mut output_lines: Vec<String> = Vec::new();
                            for field_index in 0..field_count {
                                // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                let mut field_name: String = fields[field_index as usize].clone();
                                let mut dot: String = ".".to_string();
                                let mut key: String = [enum_name.as_str(), dot.as_str(), field_name.as_str()].concat();
                                let mut literal: String = reg_get(typed_variant_reg.clone(), key.clone());
                                let mut is_mut: bool = list_has(mut_names.clone(), field_name.clone());
                                let mut mut_kw: String = "".to_string();
                                if is_mut {
                                    // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                    mut_kw = "mut ".to_string();
                                }
                                let mut line: String = "".to_string();
                                if val_type == "string" {
                                    // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                    let mut debug_str: String = s_debug(literal.clone());
                                    line = [pad.as_str(), "let ", mut_kw.as_str(), field_name.as_str(), ": String = ", debug_str.as_str(), ".to_string();"].concat();
                                } else {
                                    // transpiler-deor/codegen/decl/stmt/macros/gen_enum_extract_check.deor
                                    let mut rust_type: String = render_rust_type(val_type.clone());
                                    line = [pad.as_str(), "let ", mut_kw.as_str(), field_name.as_str(), ": ", rust_type.as_str(), " = ", literal.as_str(), ";"].concat();
                                }
                                output_lines.push(line.clone());
                            }
                            let mut result_code: String = s_join_nl(output_lines.clone());
                            let mut after_pos: i64 = locate_after_name(cur.clone());
                            return make_nl_result(result_code, after_pos.clone(), tokens.clone());
                        }
                    }
                }
            }
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut val_pos: i64 = cur + 1;
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = expr_result.code;
    let new_pos = expr_result.new_pos;
    let val_code = code.clone();
    let val_end = new_pos.clone();
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
    // macro: for_build_fields (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor
        for field_index in 0..field_count {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor
            let mut field: String = fields[field_index as usize].clone();
            let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
            let mut mut_kw: String = "".to_string();
            if is_mut {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor
                mut_kw = "mut ".to_string();
            }
            let mut let_kw: String = "let ".to_string();
            let mut equals_kw: String = " = ".to_string();
            let mut dot: String = ".".to_string();
            dest_lines.push([pad.as_str(), let_kw.as_str(), mut_kw.as_str(), field.as_str(), equals_kw.as_str(), val_code.as_str(), dot.as_str(), field.as_str(), field_suffix.as_str()].concat().clone());
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut after: i64 = adv_nl_ref(val_end.clone(), tokens.clone());
    let mut dest_code: String = s_join_nl(dest_lines.clone());
    dest_code = s_cat(dest_code.clone(), RS_NL.clone());
    return make_result(dest_code, after.clone());
}

fn gen_move_destructure(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // macro: initialize_gen_destructure (transpiler-deor/codegen/decl/stmt/macros/initialize_gen_destructure.deor)
    fn locate_first_field(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let tokens = ctx.tokens.clone();
    let mut token_count: i64 = (tokens.len() as i64);
    let mut fields: Vec<String> = Vec::new();
    let mut cur: i64 = locate_first_field(pos.clone());
    // macro: for_collect_fields_into_fields_list (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
        while cur < token_count {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
            let mut field_token: Token = tokens[cur as usize].clone();
            let kind = field_token.kind.clone();
            let value = field_token.value.clone();
            if kind == "RPAREN" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                cur = cur + 1;
                break;
            } else if kind == "COMMA" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                cur = cur + 1;
            } else if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                fields.push(value.clone());
                cur = cur + 1;
            } else {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_collect_fields.deor
                cur = cur + 1;
            }
        }
    }
    // transpiler-deor/codegen/decl/stmt/destructure.deor
    let mut val_pos: i64 = cur + 1;
    // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
    let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
    let code = expr_result.code;
    let new_pos = expr_result.new_pos;
    let val_code = code.clone();
    let val_end = new_pos.clone();
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
    // macro: for_build_fields (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor
        for field_index in 0..field_count {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor
            let mut field: String = fields[field_index as usize].clone();
            let mut is_mut: bool = list_has(mut_names.clone(), field.clone());
            let mut mut_kw: String = "".to_string();
            if is_mut {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_build_fields.deor
                mut_kw = "mut ".to_string();
            }
            let mut let_kw: String = "let ".to_string();
            let mut equals_kw: String = " = ".to_string();
            let mut dot: String = ".".to_string();
            dest_lines.push([pad.as_str(), let_kw.as_str(), mut_kw.as_str(), field.as_str(), equals_kw.as_str(), val_code.as_str(), dot.as_str(), field.as_str(), field_suffix.as_str()].concat().clone());
        }
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
            let mut macro_prefix: String = "// macro: ".to_string();
            let mut paren_open: String = " (".to_string();
            let mut paren_close: String = ")".to_string();
            let mut macro_comment: String = [pad.as_str(), macro_prefix.as_str(), value.as_str(), paren_open.as_str(), file.as_str(), paren_close.as_str(), RS_NL.as_str()].concat();
            stmts.push(macro_comment.clone());
            last_file = file;
            cur = cur + 1;
            continue;
        }
        if file != last_file {
            // transpiler-deor/codegen/decl/stmt/block.deor
            let mut comment_prefix: String = "// ".to_string();
            let mut file_comment: String = [pad.as_str(), comment_prefix.as_str(), file.as_str(), RS_NL.as_str()].concat();
            stmts.push(file_comment.clone());
            last_file = file;
        }
        let mut stmt_result: ParseResult = gen_stmt(cur.clone(), depth.clone(), ctx.clone());
        let code = stmt_result.code;
        let new_pos = stmt_result.new_pos;
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
    let mut cond_result: ParseResult = gen_expr(tokens.clone(), cond_pos.clone(), ctx.clone());
    let code = cond_result.code;
    let new_pos = cond_result.new_pos;
    let cond_code = code.clone();
    let cond_end = new_pos.clone();
    let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), cond_end.clone());
    // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
    let mut blk_depth: i64 = depth + 1;
    let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
    let code = blk_r.code;
    let new_pos = blk_r.new_pos;
    let blk_code = code.clone();
    let blk_end = new_pos.clone();
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
    fn locate_if_cond(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
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
    let mut if_cond_pos: i64 = locate_if_cond(pos.clone());
    let mut then_result: ParseResult = gen_if_branch(if_cond_pos.clone(), depth.clone(), ctx.clone());
    let code = then_result.code;
    let new_pos = then_result.new_pos;
    let then_code = code.clone();
    let mut if_kw: String = "if ".to_string();
    let mut if_close: String = "}".to_string();
    let mut result_code: String = [pad.as_str(), if_kw.as_str(), then_code.as_str(), pad.as_str(), if_close.as_str()].concat();
    let mut cur = new_pos.clone();
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
        let mut after_else: i64 = cur + 1;
        if after_else >= token_count {
            // transpiler-deor/codegen/decl/stmt/if.deor
            break;
        }
        let mut after_else_token: Token = tokens[after_else as usize].clone();
        let kind = after_else_token.kind.clone();
        if kind == "KW_IF" {
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut else_if_cond_pos: i64 = after_else + 1;
            let mut else_if_result: ParseResult = gen_if_branch(else_if_cond_pos.clone(), depth.clone(), ctx.clone());
            let code = else_if_result.code;
            let new_pos = else_if_result.new_pos;
            let else_if_code = code.clone();
            let mut else_if_kw: String = " else if ".to_string();
            let mut else_if_close: String = "}".to_string();
            result_code = s_cat(result_code, else_if_kw.clone());
            result_code = s_cat(result_code, else_if_code.clone());
            result_code = s_cat(result_code, pad.clone());
            result_code = s_cat(result_code, else_if_close.clone());
            cur = new_pos;
        } else {
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), after_else.clone());
            // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
            let mut blk_depth: i64 = depth + 1;
            let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
            let code = blk_r.code;
            let new_pos = blk_r.new_pos;
            let blk_code = code.clone();
            let blk_end = new_pos.clone();
            // transpiler-deor/codegen/decl/stmt/if.deor
            let mut else_kw: String = " else {\n".to_string();
            let mut else_close: String = "}".to_string();
            result_code = s_cat(result_code, else_kw.clone());
            result_code = s_cat(result_code, blk_code.clone());
            result_code = s_cat(result_code, pad.clone());
            result_code = s_cat(result_code, else_close.clone());
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
    fn locate_next_token(anchor: i64) -> i64 {
        return anchor + 1;
    }
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
    let mut next_pos: i64 = locate_next_token(pos.clone());
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_IF" {
        // macro: for_while (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_while.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_while.deor
            fn locate_next_token(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut cond_pos: i64 = locate_next_token(next_pos.clone());
            let mut val_pos = cond_pos.clone();
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), val_end.clone());
            let mut blk_depth: i64 = depth + 1;
            let mut blk_result: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
            let code = blk_result.code;
            let new_pos = blk_result.new_pos;
            let blk_code = code.clone();
            let blk_end = new_pos.clone();
            let mut while_kw: String = "while ".to_string();
            let mut while_head: String = [while_kw.as_str(), val_code.as_str()].concat();
            if val_code == "true" {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_while.deor
                while_head = "loop".to_string();
            }
            let mut while_code: String = [pad.as_str(), while_head.as_str(), RS_OB.as_str(), blk_code.as_str(), pad.as_str(), RS_CB.as_str()].concat();
            return make_result(while_code, blk_end.clone());
        }
    }
    if kind == "KW_MOVE" {
        // macro: for_move (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_move.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_move.deor
            fn locate_next_token(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut lparen_pos: i64 = locate_next_token(next_pos.clone());
            let mut var_pos: i64 = locate_next_token(lparen_pos.clone());
            let mut var_tok: Token = tokens[var_pos as usize].clone();
            let value = var_tok.value.clone();
            let mut move_var: String = value.clone();
            let mut in_pos: i64 = locate_next_token(var_pos.clone());
            let mut iter_pos: i64 = locate_next_token(in_pos.clone());
            let mut val_pos = iter_pos.clone();
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            let mut iter_next: i64 = val_end + 1;
            let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), iter_next.clone());
            let mut blk_depth: i64 = depth + 1;
            let mut blk_result: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
            let code = blk_result.code;
            let new_pos = blk_result.new_pos;
            let blk_code = code.clone();
            let blk_end = new_pos.clone();
            let mut for_kw: String = "for ".to_string();
            let mut for_in_kw: String = " in ".to_string();
            let mut for_code: String = [pad.as_str(), for_kw.as_str(), move_var.as_str(), for_in_kw.as_str(), val_code.as_str(), RS_OB.as_str(), blk_code.as_str(), pad.as_str(), RS_CB.as_str()].concat();
            return make_result(for_code, blk_end.clone());
        }
    }
    let mut var_name: String = "_".to_string();
    let mut iter_pos: i64 = 0;
    if kind == "KW_IN" {
        // transpiler-deor/codegen/decl/stmt/for.deor
        iter_pos = locate_next_token(next_pos.clone());
    } else {
        // transpiler-deor/codegen/decl/stmt/for.deor
        let value = next_token.value.clone();
        var_name = value;
        let mut in_pos: i64 = locate_next_token(next_pos.clone());
        iter_pos = locate_next_token(in_pos.clone());
    }
    let mut iter_token: Token = tokens[iter_pos as usize].clone();
    let kind = iter_token.kind.clone();
    let value = iter_token.value.clone();
    let mut range_expr: String = "".to_string();
    let mut body_tok_pos: i64 = 0;
    // macro: for_iter_expr (transpiler-deor/codegen/decl/stmt/macros/for_loop/for_iter_expr.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_iter_expr.deor
        fn locate_next_token(anchor: i64) -> i64 {
            return anchor + 1;
        }
        if kind == "IDENT" && value == "range" {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_iter_expr.deor
            let mut lparen: i64 = locate_next_token(iter_pos.clone());
            let mut first_pos: i64 = locate_next_token(lparen.clone());
            let mut val_pos = first_pos.clone();
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            let mut comma_token: Token = tokens[val_end as usize].clone();
            let kind = comma_token.kind.clone();
            let mut has_start: bool = kind == "COMMA";
            if has_start {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_iter_expr.deor
                let mut first_code: String = val_code;
                let mut val_pos: i64 = val_end + 1;
                let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                let code = expr_result.code;
                let new_pos = expr_result.new_pos;
                let val_code = code.clone();
                let val_end = new_pos.clone();
                let mut range_dot: String = "..".to_string();
                range_expr = [first_code.as_str(), range_dot.as_str(), val_code.as_str()].concat();
                body_tok_pos = val_end + 1;
            } else {
                // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_iter_expr.deor
                let mut range_zero_prefix: String = "0..".to_string();
                range_expr = [range_zero_prefix.as_str(), val_code.as_str()].concat();
                body_tok_pos = val_end + 1;
            }
        } else {
            // transpiler-deor/codegen/decl/stmt/macros/for_loop/for_iter_expr.deor
            let mut val_pos = iter_pos.clone();
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            let mut amp: String = "&".to_string();
            let mut collection_ref: String = s_cat(amp.clone(), val_code.clone());
            range_expr = collection_ref;
            body_tok_pos = val_end;
        }
    }
    // transpiler-deor/codegen/decl/stmt/for.deor
    let mut blk_start: i64 = skip_to_body_ref(tokens.clone(), body_tok_pos.clone());
    // macro: gen_block_r (transpiler-deor/codegen/decl/stmt/macros/gen_block_r.deor)
    let mut blk_depth: i64 = depth + 1;
    let mut blk_r: ParseResult = gen_block(blk_start.clone(), blk_depth.clone(), ctx.clone());
    let code = blk_r.code;
    let new_pos = blk_r.new_pos;
    let blk_code = code.clone();
    let blk_end = new_pos.clone();
    // transpiler-deor/codegen/decl/stmt/for.deor
    let mut for_kw: String = "for ".to_string();
    let mut for_in_kw: String = " in ".to_string();
    let mut for_code: String = [pad.as_str(), for_kw.as_str(), var_name.as_str(), for_in_kw.as_str(), range_expr.as_str(), RS_OB.as_str(), blk_code.as_str(), pad.as_str(), RS_CB.as_str()].concat();
    return make_result(for_code, blk_end.clone());
}

// transpiler-deor/codegen/decl/stmt/as_binding.deor
fn gen_as_binding(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/as_binding.deor
    fn locate_after_as(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
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
    let mut after_as: i64 = locate_after_as(pos.clone());
    let mut after_as_token: Token = tokens[after_as as usize].clone();
    let kind = after_as_token.kind.clone();
    let value = after_as_token.value.clone();
    let mut after_as_value: String = value.clone();
    if kind == "LPAREN" {
        // macro: aas_struct (transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
            fn locate_after_lparen(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut is_struct: bool = true;
            let mut peek_pos: i64 = locate_after_lparen(after_as.clone());
            while peek_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                let mut peek_token: Token = tokens[peek_pos as usize].clone();
                let kind = peek_token.kind.clone();
                if kind == "RPAREN" {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                    break;
                }
                if kind == "IDENT" {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                    peek_pos = peek_pos + 1;
                    continue;
                }
                if kind == "COMMA" {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                    peek_pos = peek_pos + 1;
                    continue;
                }
                is_struct = false;
                break;
            }
            if is_struct {
                // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                let mut fields: Vec<String> = Vec::new();
                let mut scan_pos: i64 = locate_after_lparen(after_as.clone());
                while scan_pos < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                    let mut scan_token: Token = tokens[scan_pos as usize].clone();
                    let kind = scan_token.kind.clone();
                    let value = scan_token.value.clone();
                    if kind == "RPAREN" {
                        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                        scan_pos = scan_pos + 1;
                        break;
                    } else if kind == "COMMA" {
                        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                        scan_pos = scan_pos + 1;
                    } else if kind == "IDENT" {
                        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                        fields.push(value.clone());
                        scan_pos = scan_pos + 1;
                    }
                }
                let mut matched_struct_name: String = find_struct_for_fields(struct_reg.clone(), fields.clone());
                let mut var_name: String = ident_name.clone();
                // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
                let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                let mut mut_kw: String = "".to_string();
                if mg_is_mut {
                    // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                    mut_kw = "mut ".to_string();
                }
                // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                let mut field_count: i64 = (fields.len() as i64);
                let mut field_pairs: Vec<String> = Vec::new();
                for field_index in 0..field_count {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_struct.deor
                    let mut field_name: String = fields[field_index as usize].clone();
                    let mut field_sep: String = ": ".to_string();
                    let mut field_clone_suffix: String = ".clone()".to_string();
                    field_pairs.push([field_name.as_str(), field_sep.as_str(), field_name.as_str(), field_clone_suffix.as_str()].concat().clone());
                }
                let mut fields_separator: String = ", ".to_string();
                let mut fields_code: String = s_join_with(field_pairs.clone(), fields_separator.clone());
                let mut let_kw: String = "let ".to_string();
                let mut equals_kw: String = " = ".to_string();
                let mut open_brace: String = " { ".to_string();
                let mut close_brace: String = " };\n".to_string();
                let mut struct_code: String = [pad.as_str(), let_kw.as_str(), mut_kw.as_str(), ident_name.as_str(), equals_kw.as_str(), matched_struct_name.as_str(), open_brace.as_str(), fields_code.as_str(), close_brace.as_str()].concat();
                return make_nl_result(struct_code, scan_pos.clone(), tokens.clone());
            }
        }
    }
    if kind == "KW_EMPTY" {
        // macro: aas_empty (transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_empty.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_empty.deor
            fn locate_after_empty(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut empty_prefix: String = "let mut ".to_string();
            let mut empty_suffix: String = " = Vec::new();\n".to_string();
            let mut empty_code: String = [pad.as_str(), empty_prefix.as_str(), ident_name.as_str(), empty_suffix.as_str()].concat();
            let mut after_empty: i64 = locate_after_empty(after_as.clone());
            return make_nl_result(empty_code, after_empty.clone(), tokens.clone());
        }
    }
    if kind == "IDENT" {
        // macro: aas_with (transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
            fn locate_with_keyword(anchor: i64) -> i64 {
                return anchor + 1;
            }
            fn locate_with_lparen(anchor: i64) -> i64 {
                return anchor + 2;
            }
            let mut with_keyword_pos: i64 = locate_with_keyword(after_as.clone());
            if with_keyword_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                let mut with_keyword_token: Token = tokens[with_keyword_pos as usize].clone();
                let kind = with_keyword_token.kind.clone();
                if kind == "KW_WITH" {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                    let mut source_name: String = after_as_value.clone();
                    let mut with_lparen_pos: i64 = locate_with_lparen(after_as.clone());
                    let mut override_fields: Vec<String> = Vec::new();
                    let mut scan_pos: i64 = with_lparen_pos + 1;
                    while scan_pos < token_count {
                        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                        let kind = scan_token.kind.clone();
                        let value = scan_token.value.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                            scan_pos = scan_pos + 1;
                            break;
                        }
                        if kind == "COMMA" {
                            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                            scan_pos = scan_pos + 1;
                            continue;
                        }
                        if kind == "IDENT" {
                            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                            override_fields.push(value.clone());
                            scan_pos = scan_pos + 1;
                        }
                    }
                    let mut first_field: String = override_fields[0 as usize].clone();
                    let mut matched_struct_name: String = find_struct_for_field(struct_reg.clone(), first_field.clone());
                    let mut fields_separator: String = ", ".to_string();
                    let mut fields_code: String = s_join_with(override_fields.clone(), fields_separator.clone());
                    let mut var_name: String = ident_name.clone();
                    // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
                    let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                    let mut mut_kw: String = "".to_string();
                    if mg_is_mut {
                        // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                        mut_kw = "mut ".to_string();
                    }
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_with.deor
                    let mut let_kw: String = "let ".to_string();
                    let mut equals_kw: String = " = ".to_string();
                    let mut open_brace: String = " { ".to_string();
                    let mut spread_prefix: String = ", ..".to_string();
                    let mut clone_suffix: String = ".clone()".to_string();
                    let mut close_brace: String = " };\n".to_string();
                    let mut with_code: String = [pad.as_str(), let_kw.as_str(), mut_kw.as_str(), ident_name.as_str(), equals_kw.as_str(), matched_struct_name.as_str(), open_brace.as_str(), fields_code.as_str(), spread_prefix.as_str(), source_name.as_str(), clone_suffix.as_str(), close_brace.as_str()].concat();
                    return make_nl_result(with_code, scan_pos.clone(), tokens.clone());
                }
            }
        }
    }
    // macro: aas_default (transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
        fn locate_after_value(anchor: i64) -> i64 {
            return anchor + 1;
        }
        let kind = after_as_token.kind.clone();
        let val_pos = after_as.clone();
        let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = expr_result.code;
        let new_pos = expr_result.new_pos;
        let val_code = code.clone();
        let val_end = new_pos.clone();
        let mut is_chain: bool = is_expr_chain(tokens.clone(), val_pos.clone(), val_end.clone());
        let mut var_name: String = ident_name.clone();
        // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
        let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
        let mut mut_kw: String = "".to_string();
        if mg_is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
            mut_kw = "mut ".to_string();
        }
        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
        let mut suffix: String = "".to_string();
        if !is_chain {
            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
            if kind == "STRING" {
                // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
                suffix = ".to_string()".to_string();
            } else if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
                let mut next_idx: i64 = locate_after_value(val_pos.clone());
                if next_idx < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
                    let mut next_token: Token = tokens[next_idx as usize].clone();
                    let kind = next_token.kind.clone();
                    let mut is_call: bool = kind == "LPAREN";
                    let mut is_idx: bool = kind == "KW_AT";
                    if !is_call {
                        // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
                        if !is_idx {
                            // transpiler-deor/codegen/decl/stmt/macros/as_binding/aas_default.deor
                            suffix = ".clone()".to_string();
                        }
                    }
                }
            }
        }
        let mut let_kw: String = "let ".to_string();
        let mut equals_kw: String = " = ".to_string();
        let mut statement_close: String = ";\n".to_string();
        let mut bind_code: String = [pad.as_str(), let_kw.as_str(), mut_kw.as_str(), ident_name.as_str(), equals_kw.as_str(), val_code.as_str(), suffix.as_str(), statement_close.as_str()].concat();
        return make_nl_result(bind_code, val_end.clone(), tokens.clone());
    }
}

// transpiler-deor/codegen/decl/stmt/call_stmt.deor
fn gen_call_stmt(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/call_stmt.deor
    fn locate_next_token(anchor: i64) -> i64 {
        return anchor + 1;
    }
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
    let mut next_pos: i64 = locate_next_token(pos.clone());
    let mut args_pos: i64 = locate_next_token(next_pos.clone());
    let mut args_result: ParseResult = gen_call_args(tokens.clone(), args_pos.clone(), ctx.clone());
    let code = args_result.code;
    let new_pos = args_result.new_pos;
    let args_code = code.clone();
    let args_end = new_pos.clone();
    let mut after_paren: i64 = args_end + 1;
    let mut call_code: String = "".to_string();
    if ident_name == "print" {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut print_arg_count: i64 = count_call_args(tokens.clone(), next_pos.clone());
        if print_arg_count == 2 {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut print_2arg_prefix: String = "print!(\"{}{}\", ".to_string();
            call_code = [pad.as_str(), print_2arg_prefix.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
        } else {
            // transpiler-deor/codegen/decl/stmt/call_stmt.deor
            let mut println_prefix: String = "println!(\"{}\", ".to_string();
            call_code = [pad.as_str(), println_prefix.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
        }
    } else if ident_name == "crash" {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        let mut crash_prefix: String = "panic!(\"{}\", ".to_string();
        call_code = [pad.as_str(), crash_prefix.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
    } else {
        // transpiler-deor/codegen/decl/stmt/call_stmt.deor
        call_code = [pad.as_str(), ident_name.as_str(), RS_LP.as_str(), args_code.as_str(), RS_RP_SC.as_str()].concat();
    }
    return make_nl_result(call_code, after_paren.clone(), tokens.clone());
}

// transpiler-deor/codegen/decl/stmt/list_mutation.deor
fn gen_list_mutation_stmt(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/list_mutation.deor
    fn locate_next_token(anchor: i64) -> i64 {
        return anchor + 1;
    }
    fn locate_value_after_end(anchor: i64) -> i64 {
        return anchor + 2;
    }
    fn locate_index_after_remove(anchor: i64) -> i64 {
        return anchor + 2;
    }
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
    let mut next_pos: i64 = locate_next_token(pos.clone());
    let mut next_token: Token = tokens[next_pos as usize].clone();
    let kind = next_token.kind.clone();
    if kind == "KW_AT" {
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut after_at: i64 = locate_next_token(next_pos.clone());
        if after_at < token_count {
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
            let mut at_next_token: Token = tokens[after_at as usize].clone();
            let kind = at_next_token.kind.clone();
            let value = at_next_token.value.clone();
            if kind == "KW_END" {
                // transpiler-deor/codegen/decl/stmt/list_mutation.deor
                let mut val_pos: i64 = locate_value_after_end(after_at.clone());
                // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
                let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                let code = expr_result.code;
                let new_pos = expr_result.new_pos;
                let val_code = code.clone();
                let val_end = new_pos.clone();
                // transpiler-deor/codegen/decl/stmt/list_mutation.deor
                let mut val_tok: Token = tokens[val_pos as usize].clone();
                let kind = val_tok.kind.clone();
                let mut push_val: String = emit_val(val_code.clone(), kind.clone());
                let mut push_prefix: String = ".push(".to_string();
                let mut push_code: String = [pad.as_str(), ident_name.as_str(), push_prefix.as_str(), push_val.as_str(), RS_RP_SC.as_str()].concat();
                return make_nl_result(push_code, val_end.clone(), tokens.clone());
            }
            let mut val_pos = after_at.clone();
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
            let mut idx_code: String = val_code;
            let mut val_pos: i64 = val_end + 1;
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            // transpiler-deor/codegen/decl/stmt/list_mutation.deor
            let mut val_tok: Token = tokens[val_pos as usize].clone();
            let kind = val_tok.kind.clone();
            let mut idx_val: String = emit_val(val_code.clone(), kind.clone());
            let mut index_open: String = "[".to_string();
            let mut index_mid: String = " as usize] = ".to_string();
            let mut index_suffix: String = ";\n".to_string();
            let mut index_code: String = [pad.as_str(), ident_name.as_str(), index_open.as_str(), idx_code.as_str(), index_mid.as_str(), idx_val.as_str(), index_suffix.as_str()].concat();
            return make_nl_result(index_code, val_end.clone(), tokens.clone());
        }
    }
    if kind == "KW_REMOVE" {
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut idx_pos: i64 = locate_index_after_remove(next_pos.clone());
        let mut val_pos = idx_pos.clone();
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = expr_result.code;
        let new_pos = expr_result.new_pos;
        let val_code = code.clone();
        let val_end = new_pos.clone();
        // transpiler-deor/codegen/decl/stmt/list_mutation.deor
        let mut remove_prefix: String = ".remove(".to_string();
        let mut remove_suffix: String = " as usize);\n".to_string();
        let mut remove_code: String = [pad.as_str(), ident_name.as_str(), remove_prefix.as_str(), val_code.as_str(), remove_suffix.as_str()].concat();
        return make_nl_result(remove_code, val_end.clone(), tokens.clone());
    }
    let mut unhandled_prefix: String = "/* unhandled_list_mut(".to_string();
    let mut unhandled_suffix: String = ") */\n".to_string();
    let mut unhandled: String = [unhandled_prefix.as_str(), kind.as_str(), unhandled_suffix.as_str()].concat();
    let mut unhandled_next: i64 = locate_next_token(pos.clone());
    return make_result(unhandled, unhandled_next.clone());
}

// transpiler-deor/codegen/decl/stmt/typed_binding.deor
fn gen_typed_binding(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/typed_binding.deor
    fn locate_var_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_value(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
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
    let mut name_pos: i64 = locate_var_name(pos.clone());
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let value = name_token.value.clone();
    let mut var_name: String = value.clone();
    let mut val_pos: i64 = locate_value(pos.clone());
    let mut rust_type: String = resolve_type(var_type.clone(), ctx.clone());
    let mut val_token: Token = tokens[val_pos as usize].clone();
    let kind = val_token.kind.clone();
    if kind == "KW_EMPTY" {
        // macro: tb_empty (transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_empty.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_empty.deor
            fn locate_after_empty(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut is_shape: bool = reg_has(shape_reg.clone(), var_type.clone());
            let mut val_next_pos: i64 = locate_after_empty(val_pos.clone());
            let mut after_empty: i64 = adv_nl_ref(val_next_pos.clone(), tokens.clone());
            if is_shape {
                // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_empty.deor
                let mut list_prefix: String = "let mut ".to_string();
                let mut list_mid: String = ": ".to_string();
                let mut list_suffix: String = " = Vec::new();\n".to_string();
                let mut list_code: String = [pad.as_str(), list_prefix.as_str(), var_name.as_str(), list_mid.as_str(), rust_type.as_str(), list_suffix.as_str()].concat();
                return make_result(list_code, after_empty.clone());
            }
            let mut err_msg: String = "/* error: empty is only valid for list shapes */\n".to_string();
            let mut err_code: String = [pad.as_str(), err_msg.as_str()].concat();
            return make_result(err_code, after_empty.clone());
        }
    }
    if kind == "LPAREN" {
        // macro: tb_paren (transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
            fn locate_after_lparen(anchor: i64) -> i64 {
                return anchor + 1;
            }
            let mut peek_pos: i64 = locate_after_lparen(val_pos.clone());
            let mut peek_token: Token = tokens[peek_pos as usize].clone();
            let kind = peek_token.kind.clone();
            let mut is_avow_expr: bool = kind == "KW_AVOW";
            let mut is_struct_type: bool = reg_has(struct_reg.clone(), var_type.clone());
            if !is_avow_expr {
                // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                if is_struct_type {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                    let mut paren_fields: Vec<String> = Vec::new();
                    let mut scan_pos: i64 = locate_after_lparen(val_pos.clone());
                    while scan_pos < token_count {
                        // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                        let kind = scan_token.kind.clone();
                        let value = scan_token.value.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                            scan_pos = scan_pos + 1;
                            break;
                        } else if kind == "COMMA" {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                            scan_pos = scan_pos + 1;
                        } else if kind == "IDENT" {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                            paren_fields.push(value.clone());
                            scan_pos = scan_pos + 1;
                        } else {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                            scan_pos = scan_pos + 1;
                        }
                    }
                    let mut field_pairs: Vec<String> = Vec::new();
                    let mut field_count: i64 = (paren_fields.len() as i64);
                    for field_index in 0..field_count {
                        // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                        let mut field_name: String = paren_fields[field_index as usize].clone();
                        let mut field_sep: String = ": ".to_string();
                        let mut field_clone_suffix: String = ".clone()".to_string();
                        field_pairs.push([field_name.as_str(), field_sep.as_str(), field_name.as_str(), field_clone_suffix.as_str()].concat().clone());
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
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                    let mut struct_let: String = "let ".to_string();
                    let mut struct_eq: String = " = ".to_string();
                    let mut struct_open_brace: String = " { ".to_string();
                    let mut struct_close_brace: String = " };\n".to_string();
                    let mut struct_code: String = [pad.as_str(), struct_let.as_str(), mut_kw.as_str(), var_name.as_str(), struct_eq.as_str(), var_type.as_str(), struct_open_brace.as_str(), fields_code.as_str(), struct_close_brace.as_str()].concat();
                    return make_nl_result(struct_code, scan_pos.clone(), tokens.clone());
                }
            }
            if is_avow_expr {
                // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                let mut inner_pos: i64 = peek_pos + 1;
                let mut val_pos = inner_pos.clone();
                let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
                let code = expr_result.code;
                let new_pos = expr_result.new_pos;
                let val_code = code.clone();
                let val_end = new_pos.clone();
                let mut after_rparen: i64 = val_end + 1;
                let mut unwrap_suffix: String = ".unwrap()".to_string();
                let mut unwrap_field0_suffix: String = ".unwrap().0".to_string();
                let mut unwrap_expr: String = s_cat(val_code.clone(), unwrap_suffix.clone());
                if var_type == "int" {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                    unwrap_expr = s_cat(val_code.clone(), unwrap_field0_suffix.clone());
                }
                if var_type == "string" {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                    unwrap_expr = s_cat(val_code.clone(), unwrap_field0_suffix.clone());
                }
                if var_type == "bool" {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                    unwrap_expr = s_cat(val_code.clone(), unwrap_field0_suffix.clone());
                }
                if var_type == "float" {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_paren.deor
                    unwrap_expr = s_cat(val_code.clone(), unwrap_field0_suffix.clone());
                }
                let mut avow_let: String = "let ".to_string();
                let mut avow_colon: String = ": ".to_string();
                let mut avow_eq: String = " = ".to_string();
                let mut avow_sc: String = ";\n".to_string();
                let mut avow_code: String = [pad.as_str(), avow_let.as_str(), var_name.as_str(), avow_colon.as_str(), rust_type.as_str(), avow_eq.as_str(), unwrap_expr.as_str(), avow_sc.as_str()].concat();
                return make_nl_result(avow_code, after_rparen.clone(), tokens.clone());
            }
        }
    }
    if kind == "LBRACKET" {
        // macro: tb_list_literal (transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_list_literal.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_list_literal.deor
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            let mut list_prefix: String = "let mut ".to_string();
            let mut list_colon: String = ": ".to_string();
            let mut list_eq: String = " = ".to_string();
            let mut list_sc: String = ";\n".to_string();
            let mut list_code: String = [pad.as_str(), list_prefix.as_str(), var_name.as_str(), list_colon.as_str(), rust_type.as_str(), list_eq.as_str(), val_code.as_str(), list_sc.as_str()].concat();
            return make_nl_result(list_code, val_end.clone(), tokens.clone());
        }
    }
    if kind == "IDENT" {
        // macro: tb_with (transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
            fn locate_with_keyword(anchor: i64) -> i64 {
                return anchor + 1;
            }
            fn locate_with_lparen(anchor: i64) -> i64 {
                return anchor + 2;
            }
            let mut with_keyword_pos: i64 = locate_with_keyword(val_pos.clone());
            if with_keyword_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                let mut with_keyword_token: Token = tokens[with_keyword_pos as usize].clone();
                let kind = with_keyword_token.kind.clone();
                if kind == "KW_WITH" {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                    let value = val_token.value.clone();
                    let mut source_name: String = value.clone();
                    let mut with_lparen_pos: i64 = locate_with_lparen(val_pos.clone());
                    let mut override_fields: Vec<String> = Vec::new();
                    let mut scan_pos: i64 = with_lparen_pos + 1;
                    while scan_pos < token_count {
                        // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                        let mut scan_token: Token = tokens[scan_pos as usize].clone();
                        let kind = scan_token.kind.clone();
                        let value = scan_token.value.clone();
                        if kind == "RPAREN" {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                            scan_pos = scan_pos + 1;
                            break;
                        }
                        if kind == "COMMA" {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                            scan_pos = scan_pos + 1;
                            continue;
                        }
                        if kind == "IDENT" {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                            override_fields.push(value.clone());
                            scan_pos = scan_pos + 1;
                        }
                    }
                    let mut fields_separator: String = ", ".to_string();
                    let mut fields_code: String = s_join_with(override_fields.clone(), fields_separator.clone());
                    // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
                    let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
                    let mut mut_kw: String = "".to_string();
                    if mg_is_mut {
                        // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                        mut_kw = "mut ".to_string();
                    }
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_with.deor
                    let mut let_kw: String = "let ".to_string();
                    let mut equals_kw: String = " = ".to_string();
                    let mut open_brace: String = " { ".to_string();
                    let mut spread_prefix: String = ", ..".to_string();
                    let mut clone_suffix: String = ".clone()".to_string();
                    let mut close_brace: String = " };\n".to_string();
                    let mut with_code: String = [pad.as_str(), let_kw.as_str(), mut_kw.as_str(), var_name.as_str(), equals_kw.as_str(), rust_type.as_str(), open_brace.as_str(), fields_code.as_str(), spread_prefix.as_str(), source_name.as_str(), clone_suffix.as_str(), close_brace.as_str()].concat();
                    return make_nl_result(with_code, scan_pos.clone(), tokens.clone());
                }
            }
        }
    }
    let mut is_validator: bool = reg3_has(type_reg.clone(), var_type.clone());
    if is_validator {
        // macro: tb_validator (transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_validator.deor)
        {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_validator.deor
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
            let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
            let mut mut_kw: String = "".to_string();
            if mg_is_mut {
                // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
                mut_kw = "mut ".to_string();
            }
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_validator.deor
            let mut validator_let: String = "let ".to_string();
            let mut validator_option_open: String = ": Option<".to_string();
            let mut validator_option_close: String = "> = ".to_string();
            let mut validator_new_open: String = "::new(".to_string();
            let mut validator_sc: String = ");\n".to_string();
            let mut validator_code: String = [pad.as_str(), validator_let.as_str(), mut_kw.as_str(), var_name.as_str(), validator_option_open.as_str(), var_type.as_str(), validator_option_close.as_str(), var_type.as_str(), validator_new_open.as_str(), val_code.as_str(), validator_sc.as_str()].concat();
            return make_nl_result(validator_code, val_end.clone(), tokens.clone());
        }
    }
    // macro: tb_default (transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
        fn locate_after_value(anchor: i64) -> i64 {
            return anchor + 1;
        }
        let mut is_float: bool = var_type == "float";
        if is_float {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
            float_ctx_enable();
        }
        let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        if is_float {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
            float_ctx_disable();
        }
        let code = expr_result.code;
        let new_pos = expr_result.new_pos;
        let val_code = code.clone();
        let val_end = new_pos.clone();
        // macro: mut_guard (transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor)
        let mut mg_is_mut: bool = list_has(mut_names.clone(), var_name.clone());
        let mut mut_kw: String = "".to_string();
        if mg_is_mut {
            // transpiler-deor/codegen/decl/stmt/macros/mut_guard.deor
            mut_kw = "mut ".to_string();
        }
        // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
        let mut is_chain: bool = is_expr_chain(tokens.clone(), val_pos.clone(), val_end.clone());
        let mut suffix: String = "".to_string();
        if !is_chain {
            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
            if kind == "STRING" {
                // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
                suffix = ".to_string()".to_string();
            } else if kind == "IDENT" {
                // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
                let mut val_next_idx: i64 = locate_after_value(val_pos.clone());
                if val_next_idx < token_count {
                    // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
                    let mut next_val_token: Token = tokens[val_next_idx as usize].clone();
                    let kind = next_val_token.kind.clone();
                    let mut val_is_call: bool = kind == "LPAREN";
                    let mut val_is_idx: bool = kind == "KW_AT";
                    if !val_is_call {
                        // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
                        if !val_is_idx {
                            // transpiler-deor/codegen/decl/stmt/macros/typed_binding/tb_default.deor
                            suffix = ".clone()".to_string();
                        }
                    }
                }
            }
        }
        let mut bind_code: String = [pad.as_str(), RS_LET.as_str(), mut_kw.as_str(), var_name.as_str(), RS_COL.as_str(), rust_type.as_str(), RS_EQ.as_str(), val_code.as_str(), suffix.as_str(), RS_SC.as_str()].concat();
        return make_nl_result(bind_code, val_end.clone(), tokens.clone());
    }
}

// transpiler-deor/codegen/decl/cursor.deor
fn c_at_end(cur: TokenCursor) -> bool {
    // transpiler-deor/codegen/decl/cursor.deor
    let token_count = cur.token_count.clone();
    let pos = cur.pos.clone();
    return pos >= token_count;
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
    let mut pos: i64 = pos + 1;
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

// transpiler-deor/codegen/decl/stmt/stmt.deor
fn gen_stmt(pos: i64, depth: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    fn locate_next_token(anchor: i64) -> i64 {
        return anchor + 1;
    }
    fn locate_raw_value(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let tokens = ctx.tokens.clone();
    let validator_var_reg = ctx.validator_var_reg.clone();
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
    {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
        fn locate_next_token(anchor: i64) -> i64 {
            return anchor + 1;
        }
        if kind == "KW_RETURN" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
            let mut val_pos: i64 = locate_next_token(pos.clone());
            let mut val_token: Token = tokens[val_pos as usize].clone();
            let kind = val_token.kind.clone();
            let mut val_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = val_result.code;
            let new_pos = val_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            let mut is_chain: bool = is_expr_chain(tokens.clone(), val_pos.clone(), val_end.clone());
            let mut suffix: String = "".to_string();
            if !is_chain {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
                if kind == "STRING" {
                    // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
                    suffix = ".to_string()".to_string();
                }
            }
            let mut return_kw: String = "return ".to_string();
            let mut return_sc: String = ";\n".to_string();
            let mut return_code: String = [pad.as_str(), return_kw.as_str(), val_code.as_str(), suffix.as_str(), return_sc.as_str()].concat();
            return make_nl_result(return_code, val_end.clone(), tokens.clone());
        }
        if kind == "KW_BREAK" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
            let mut break_kw: String = "break;\n".to_string();
            let mut break_code: String = [pad.as_str(), break_kw.as_str()].concat();
            let mut break_next: i64 = locate_next_token(pos.clone());
            return make_nl_result(break_code, break_next.clone(), tokens.clone());
        }
        if kind == "KW_CONTINUE" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_flow.deor
            let mut continue_kw: String = "continue;\n".to_string();
            let mut continue_code: String = [pad.as_str(), continue_kw.as_str()].concat();
            let mut continue_next: i64 = locate_next_token(pos.clone());
            return make_nl_result(continue_code, continue_next.clone(), tokens.clone());
        }
    }
    // macro: stmt_blocks (transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
        fn locate_next_token(anchor: i64) -> i64 {
            return anchor + 1;
        }
        fn locate_rust_block_content(kw_pos: i64) -> i64 {
            return kw_pos + 2;
        }
        if kind == "KW_BLOCK" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
            let mut newline_pos: i64 = locate_next_token(pos.clone());
            let mut body_start: i64 = skip_to_body_ref(tokens.clone(), newline_pos.clone());
            let mut body_depth: i64 = depth + 1;
            let mut body_result: ParseResult = gen_block(body_start.clone(), body_depth.clone(), ctx.clone());
            let code = body_result.code;
            let new_pos = body_result.new_pos;
            let body_code = code.clone();
            let body_end = new_pos.clone();
            let mut open_brace: String = "{\n".to_string();
            let mut close_brace: String = "}\n".to_string();
            let mut block_open: String = s_cat(pad.clone(), open_brace.clone());
            let mut block_close: String = s_cat(pad.clone(), close_brace.clone());
            let mut block_code: String = [block_open.as_str(), body_code.as_str(), block_close.as_str()].concat();
            return make_result(block_code, body_end.clone());
        }
        if kind == "KW_RUST" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
            let mut rust_block_pos: i64 = locate_rust_block_content(pos.clone());
            let mut rust_block_token: Token = tokens[rust_block_pos as usize].clone();
            let value = rust_block_token.value.clone();
            let mut rust_content: String = value.clone();
            let mut rust_lines: Vec<String> = s_split(rust_content.clone(), RS_NL.clone());
            let mut padded_lines: Vec<String> = Vec::new();
            let mut line_count: i64 = (rust_lines.len() as i64);
            for line_index in 0..line_count {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
                let mut rust_line: String = rust_lines[line_index as usize].clone();
                if is_empty(rust_line.clone()) {
                    // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
                    let mut empty_line: String = "".to_string();
                    padded_lines.push(empty_line.clone());
                } else {
                    // transpiler-deor/codegen/decl/stmt/macros/stmt_blocks.deor
                    padded_lines.push([pad.as_str(), rust_line.as_str()].concat().clone());
                }
            }
            let mut rust_block_code: String = s_join_nl(padded_lines.clone());
            rust_block_code = s_cat(rust_block_code.clone(), RS_NL.clone());
            let mut rust_block_next: i64 = locate_next_token(rust_block_pos.clone());
            return make_result(rust_block_code, rust_block_next.clone());
        }
    }
    // macro: stmt_structural (transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor)
    {
        // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
        fn locate_next_token(anchor: i64) -> i64 {
            return anchor + 1;
        }
        if kind == "KW_MOVE" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
            let mut move_next: i64 = locate_next_token(pos.clone());
            let mut move_next_token: Token = tokens[move_next as usize].clone();
            let kind = move_next_token.kind.clone();
            if kind == "LPAREN" {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
                return gen_move_destructure(move_next.clone(), depth.clone(), ctx.clone());
            }
        }
        if kind == "LPAREN" {
            // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
            let mut peek_pos: i64 = locate_next_token(pos.clone());
            if peek_pos < token_count {
                // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
                let mut peek_token: Token = tokens[peek_pos as usize].clone();
                let kind = peek_token.kind.clone();
                if kind == "KW_AVOW" {
                    // transpiler-deor/codegen/decl/stmt/macros/stmt_structural.deor
                    let mut expr_pos: i64 = locate_next_token(peek_pos.clone());
                    let mut expr_result: ParseResult = gen_expr(tokens.clone(), expr_pos.clone(), ctx.clone());
                    let code = expr_result.code;
                    let new_pos = expr_result.new_pos;
                    let expr_code = code.clone();
                    let after_rparen = new_pos + 1;
                    let mut unwrap_suffix: String = ".unwrap();\n".to_string();
                    let mut avow_code: String = [pad.as_str(), expr_code.as_str(), unwrap_suffix.as_str()].concat();
                    return make_nl_result(avow_code, after_rparen.clone(), tokens.clone());
                }
            }
            return gen_destructure(pos.clone(), depth.clone(), ctx.clone());
        }
    }
    // transpiler-deor/codegen/decl/stmt/stmt.deor
    if kind == "KW_RAW" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut raw_name_pos: i64 = locate_next_token(pos.clone());
        let mut raw_name_tok: Token = tokens[raw_name_pos as usize].clone();
        let value = raw_name_tok.value.clone();
        let mut raw_var_name: String = value.clone();
        let mut val_pos: i64 = locate_raw_value(pos.clone());
        // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
        let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
        let code = expr_result.code;
        let new_pos = expr_result.new_pos;
        let val_code = code.clone();
        let val_end = new_pos.clone();
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
        let mut const_type_pos: i64 = locate_next_token(pos.clone());
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
    if kind == "KW_FN" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        fn locate_fn_return_type(kw_pos: i64) -> i64 {
            return kw_pos + 1;
        }
        let mut fn_tokens: TokensRef = tokens.clone();
        let mut return_type_pos: i64 = locate_fn_return_type(pos.clone());
        // macro: fn_parse_signature (transpiler-deor/codegen/decl/macros/fn_parse_signature.deor)
        let mut cur: TokenCursor = cur_at_ref(fn_tokens.clone(), return_type_pos.clone());
        let current = cur.current.clone();
        let value = current.value.clone();
        let mut ret_type: String = resolve_type(value.clone(), ctx.clone());
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
                let mut rust_param_type: String = resolve_type(param_type.clone(), ctx.clone());
                let mut param_separator: String = ": ".to_string();
                param_strs.push([param_name.as_str(), param_separator.as_str(), rust_param_type.as_str()].concat().clone());
                cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            } else {
                // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
                cur = cur_next_ref(cur.clone(), fn_tokens.clone());
            }
        }
        let pos = cur.pos.clone();
        let mut indent_pos: i64 = pos + 1;
        cur = cur_skip_to_body_ref(cur.clone(), fn_tokens.clone());
        let pos = cur.pos.clone();
        let mut body_start: i64 = pos.clone();
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut nested_fn_body_depth: i64 = depth + 1;
        let mut nested_fn_body_result: ParseResult = gen_stmt(body_start.clone(), nested_fn_body_depth.clone(), ctx.clone());
        let code = nested_fn_body_result.code;
        let new_pos = nested_fn_body_result.new_pos;
        let nested_fn_body_code = code.clone();
        let nested_fn_body_end = new_pos.clone();
        let mut nested_fn_after_pos: i64 = nested_fn_body_end + 1;
        let mut nested_fn_params_code: String = s_join_with(param_strs.clone(), RS_CSEP.clone());
        let mut nested_fn_ret_suffix: String = "".to_string();
        if !is_empty(ret_type.clone()) {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            nested_fn_ret_suffix = [RS_ARR.as_str(), ret_type.as_str()].concat();
        }
        let mut nested_fn_kw: String = "fn ".to_string();
        let mut nested_fn_close: String = "}\n".to_string();
        let mut nested_fn_parts: Vec<String> = vec![pad.clone(), nested_fn_kw.clone(), fn_name.clone(), RS_LP.clone(), nested_fn_params_code.clone(), RS_RP.clone(), nested_fn_ret_suffix.clone(), RS_OB.clone(), nested_fn_body_code.clone(), pad.clone(), nested_fn_close.clone()];
        let mut nested_fn_code: String = s_join(nested_fn_parts.clone());
        return make_result(nested_fn_code, nested_fn_after_pos.clone());
    }
    if kind == "IDENT" {
        // transpiler-deor/codegen/decl/stmt/stmt.deor
        let mut ident_name: String = value.clone();
        let mut next_pos: i64 = locate_next_token(pos.clone());
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
            let mut val_pos: i64 = locate_next_token(next_pos.clone());
            let mut eq_val_token: Token = tokens[val_pos as usize].clone();
            let kind = eq_val_token.kind.clone();
            // macro: gen_expr_r (transpiler-deor/codegen/decl/stmt/macros/gen_expr_r.deor)
            let mut expr_result: ParseResult = gen_expr(tokens.clone(), val_pos.clone(), ctx.clone());
            let code = expr_result.code;
            let new_pos = expr_result.new_pos;
            let val_code = code.clone();
            let val_end = new_pos.clone();
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut assign_suffix: String = "".to_string();
            if kind == "STRING" {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                assign_suffix = ".to_string()".to_string();
            }
            let mut reassign_type: String = reg_get(validator_var_reg.clone(), ident_name.clone());
            let mut asgn_code: String = "".to_string();
            if !is_empty(reassign_type.clone()) {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                let mut reassign_new_open: String = "::new(".to_string();
                let mut reassign_sc: String = ");\n".to_string();
                let mut reassign_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), RS_EQ.clone(), reassign_type.clone(), reassign_new_open.clone(), val_code.clone(), reassign_sc.clone()];
                asgn_code = s_join(reassign_parts.clone());
            } else {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                let mut assign_parts: Vec<String> = vec![pad.clone(), ident_name.clone(), RS_EQ.clone(), val_code.clone(), assign_suffix.clone(), RS_SC.clone()];
                asgn_code = s_join(assign_parts.clone());
            }
            return make_nl_result(asgn_code, val_end.clone(), tokens.clone());
        }
        if kind == "IDENT" {
            // transpiler-deor/codegen/decl/stmt/stmt.deor
            let mut eq_pos: i64 = locate_next_token(next_pos.clone());
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
            let mut bare_rust_type: String = resolve_type(ident_name.clone(), ctx.clone());
            let mut bare_is_validator: bool = reg3_has(type_reg.clone(), ident_name.clone());
            if bare_is_validator {
                // transpiler-deor/codegen/decl/stmt/stmt.deor
                let mut bare_decl_suffix: String = " = None;\n".to_string();
                let mut bare_decl_code: String = [pad.as_str(), RS_LETM.as_str(), bare_var_name.as_str(), RS_COL.as_str(), bare_rust_type.as_str(), bare_decl_suffix.as_str()].concat();
                let mut bare_decl_after: i64 = locate_next_token(next_pos.clone());
                return make_nl_result(bare_decl_code, bare_decl_after.clone(), tokens.clone());
            }
        }
    }
    let mut unhandled_prefix: String = "/* unhandled(".to_string();
    let mut unhandled_suffix: String = ") */\n".to_string();
    let mut unhandled_parts: Vec<String> = vec![unhandled_prefix.clone(), kind.clone(), unhandled_suffix.clone()];
    let mut unhandled: String = s_join(unhandled_parts.clone());
    let mut unhandled_next: i64 = locate_next_token(pos.clone());
    return make_result(unhandled, unhandled_next.clone());
}

// transpiler-deor/codegen/decl/struct.deor
fn gen_struct_decl(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/struct.deor
    fn locate_struct_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut name_pos: i64 = locate_struct_name(pos.clone());
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), name_pos.clone());
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
            let mut rust_type: String = resolve_type(field_type.clone(), ctx.clone());
            field_lines.push([RS_IND.as_str(), field_name.as_str(), RS_COL.as_str(), rust_type.as_str(), RS_COM.as_str()].concat().clone());
            cur = cur_next_ref(cur.clone(), tokens.clone());
        }
    }
    let mut fields_code: String = s_join_nl(field_lines.clone());
    let mut struct_prefix: String = "#[derive(Clone, PartialEq, Debug)]\nstruct ".to_string();
    let mut decl: String = [struct_prefix.as_str(), struct_name.as_str(), RS_OB.as_str(), fields_code.as_str(), RS_CB2.as_str()].concat();
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
}

// transpiler-deor/codegen/decl/enum.deor
fn gen_enum_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/enum.deor
    fn locate_enum_type_or_name_slot(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let mut name_pos: i64 = locate_enum_type_or_name_slot(pos.clone());
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), name_pos.clone());
    let current = cur.current.clone();
    let kind = current.kind.clone();
    let value = current.value.clone();
    let mut is_typed: bool = is_typed_enum_type(value.clone());
    if is_typed {
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
    let mut enum_prefix: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nenum ".to_string();
    let mut decl: String = [enum_prefix.as_str(), rust_name.as_str(), RS_OB.as_str(), variants_code.as_str(), RS_CB2.as_str()].concat();
    let pos = cur.pos.clone();
    return make_result(decl, pos.clone());
}

// transpiler-deor/codegen/decl/shape.deor
fn gen_list_shape_code(rust_name: String, rust_elem: String) -> String {
    // transpiler-deor/codegen/decl/shape.deor
    let mut list_prefix: String = "type ".to_string();
    let mut list_mid: String = " = Vec<".to_string();
    let mut list_suffix: String = ">;\n\n".to_string();
    return [list_prefix.as_str(), rust_name.as_str(), list_mid.as_str(), rust_elem.as_str(), list_suffix.as_str()].concat();
}

fn gen_func_shape_code(rust_name: String, rust_in: String, rust_out: String) -> String {
    // transpiler-deor/codegen/decl/shape.deor
    let mut out_suffix: String = "".to_string();
    if !is_empty(rust_out.clone()) {
        // transpiler-deor/codegen/decl/shape.deor
        let mut out_prefix: String = " -> ".to_string();
        out_suffix = [out_prefix.as_str(), rust_out.as_str()].concat();
    }
    let mut func_prefix: String = "type ".to_string();
    let mut func_mid: String = " = fn(".to_string();
    let mut func_paren_close: String = ")".to_string();
    let mut func_suffix: String = ";\n\n".to_string();
    return [func_prefix.as_str(), rust_name.as_str(), func_mid.as_str(), rust_in.as_str(), func_paren_close.as_str(), out_suffix.as_str(), func_suffix.as_str()].concat();
}

fn gen_shape_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/shape.deor
    fn locate_shape_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    fn locate_shape_form(kw_pos: i64) -> i64 {
        return kw_pos + 3;
    }
    let mut name_pos: i64 = locate_shape_name(pos.clone());
    let mut form_pos: i64 = locate_shape_form(pos.clone());
    let mut name_token: Token = tokens[name_pos as usize].clone();
    let mut form_token: Token = tokens[form_pos as usize].clone();
    let value = name_token.value.clone();
    let mut shape_name: String = value.clone();
    let kind = form_token.kind.clone();
    let mut rust_name: String = s_pascal(shape_name.clone());
    // macro: shape_list (transpiler-deor/codegen/decl/macros/shape_list.deor)
    {
        // transpiler-deor/codegen/decl/macros/shape_list.deor
        fn locate_list_elem(kw_pos: i64) -> i64 {
            return kw_pos + 5;
        }
        if kind == "KW_LIST" {
            // transpiler-deor/codegen/decl/macros/shape_list.deor
            let mut elem_pos: i64 = locate_list_elem(pos.clone());
            let mut elem_token: Token = tokens[elem_pos as usize].clone();
            let value = elem_token.value.clone();
            let mut elem_type: String = value.clone();
            let mut rust_elem: String = render_rust_type(elem_type.clone());
            let mut decl: String = gen_list_shape_code(rust_name.clone(), rust_elem.clone());
            let mut after: i64 = elem_pos + 1;
            return make_nl_result(decl, after.clone(), tokens.clone());
        }
    }
    // macro: shape_func (transpiler-deor/codegen/decl/macros/shape_func.deor)
    {
        // transpiler-deor/codegen/decl/macros/shape_func.deor
        fn locate_func_of_or_to(kw_pos: i64) -> i64 {
            return kw_pos + 4;
        }
        fn locate_func_in_type(kw_pos: i64) -> i64 {
            return kw_pos + 5;
        }
        fn locate_func_to_after_of(kw_pos: i64) -> i64 {
            return kw_pos + 6;
        }
        fn locate_func_out_type_after_of(kw_pos: i64) -> i64 {
            return kw_pos + 7;
        }
        fn locate_func_out_type_after_to(kw_pos: i64) -> i64 {
            return kw_pos + 5;
        }
        let mut of_or_to_pos: i64 = locate_func_of_or_to(pos.clone());
        let mut of_or_to_token: Token = tokens[of_or_to_pos as usize].clone();
        let kind = of_or_to_token.kind.clone();
        let value = of_or_to_token.value.clone();
        let mut is_of: bool = kind == "KW_OF";
        let mut is_to: bool = kind == "KW_TO";
        let mut in_type: String = "".to_string();
        let mut out_type: String = "".to_string();
        let mut func_end: i64 = of_or_to_pos.clone();
        if is_of {
            // transpiler-deor/codegen/decl/macros/shape_func.deor
            let mut in_type_pos: i64 = locate_func_in_type(pos.clone());
            let mut in_type_token: Token = tokens[in_type_pos as usize].clone();
            let value = in_type_token.value.clone();
            in_type = value;
            let mut to_pos: i64 = locate_func_to_after_of(pos.clone());
            let mut to_token: Token = tokens[to_pos as usize].clone();
            let kind = to_token.kind.clone();
            let value = to_token.value.clone();
            let mut has_to: bool = kind == "KW_TO";
            func_end = to_pos;
            if has_to {
                // transpiler-deor/codegen/decl/macros/shape_func.deor
                let mut out_type_pos: i64 = locate_func_out_type_after_of(pos.clone());
                let mut out_type_token: Token = tokens[out_type_pos as usize].clone();
                let value = out_type_token.value.clone();
                out_type = value;
                func_end = out_type_pos;
            }
        } else if is_to {
            // transpiler-deor/codegen/decl/macros/shape_func.deor
            let mut out_type_pos: i64 = locate_func_out_type_after_to(pos.clone());
            let mut out_type_token: Token = tokens[out_type_pos as usize].clone();
            let value = out_type_token.value.clone();
            out_type = value;
            func_end = out_type_pos;
        }
        let mut rust_in: String = render_rust_type(in_type.clone());
        let mut rust_out: String = render_rust_type(out_type.clone());
        let mut decl: String = gen_func_shape_code(rust_name.clone(), rust_in.clone(), rust_out.clone());
        let mut after: i64 = func_end + 1;
        return make_nl_result(decl, after.clone(), tokens.clone());
    }
}

// transpiler-deor/codegen/decl/validator_type.deor
fn gen_type_decl(tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/validator_type.deor
    fn locate_type_name(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let type_reg = ctx.type_reg.clone();
    let typed_enum_reg = ctx.typed_enum_reg.clone();
    let typed_variant_reg = ctx.typed_variant_reg.clone();
    let mut type_name_pos: i64 = locate_type_name(pos.clone());
    let mut cur: TokenCursor = cur_at_ref(tokens.clone(), type_name_pos.clone());
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
    let mut indent_pos: i64 = pos + 1;
    cur = cur_skip_to_body_ref(cur.clone(), tokens.clone());
    let pos = cur.pos.clone();
    let mut body_start: i64 = pos.clone();
    let mut pred_r: ParseResult = gen_expr(tokens.clone(), body_start.clone(), ctx.clone());
    let code = pred_r.code;
    let new_pos = pred_r.new_pos;
    let pred_code = code.clone();
    let pred_end = new_pos.clone();
    let mut peek: TokenCursor = cur_at_ref(tokens.clone(), pred_end.clone());
    while !c_at_end(peek.clone()) {
        // transpiler-deor/codegen/decl/validator_type.deor
        let current = peek.current.clone();
        let kind = current.kind.clone();
        if kind == "NEWLINE" {
            // transpiler-deor/codegen/decl/validator_type.deor
            peek = cur_next_ref(peek.clone(), tokens.clone());
            continue;
        }
        break;
    }
    let mut is_single: bool = true;
    if !c_at_end(peek.clone()) {
        // transpiler-deor/codegen/decl/validator_type.deor
        let current = peek.current.clone();
        let kind = current.kind.clone();
        is_single = kind == "DEDENT";
    }
    let mut final_pred_code: String = pred_code.clone();
    let mut final_pos: i64 = 0;
    if is_single {
        // transpiler-deor/codegen/decl/validator_type.deor
        let mut scan: TokenCursor = cur_at_ref(tokens.clone(), pred_end.clone());
        while !c_at_end(scan.clone()) {
            // transpiler-deor/codegen/decl/validator_type.deor
            let current = scan.current.clone();
            let kind = current.kind.clone();
            scan = cur_next_ref(scan.clone(), tokens.clone());
            if kind == "DEDENT" {
                // transpiler-deor/codegen/decl/validator_type.deor
                break;
            }
        }
        let pos = scan.pos.clone();
        final_pos = pos;
    } else {
        // transpiler-deor/codegen/decl/validator_type.deor
        let mut body_end_pos: i64 = find_block_end_ref(tokens.clone(), indent_pos.clone());
        let mut body_slice_end: i64 = body_end_pos + 1;
        let mut body_tokens: Vec<Token> = l_slice_ref(tokens.clone(), body_start.clone(), body_slice_end.clone());
        let mut zero: i64 = 0;
        let mut last: i64 = (body_tokens.len() as i64) - 1;
        let mut mut_names: Vec<String> = collect_mut_names(body_tokens.clone(), zero.clone(), last.clone());
        let mut validator_var_reg: Vec<String> = collect_validator_var_types(body_tokens.clone(), type_reg.clone());
        let mut tokens: TokensRef = tokens_wrap(body_tokens);
        let mut ctx_raw: GenCtx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, tokens, typed_enum_reg, typed_variant_reg, validator_var_reg };
        let mut pred_ctx: RcCtx = make_rctx(ctx_raw);
        let mut block_depth: i64 = 2;
        let mut block_r: ParseResult = gen_block(zero.clone(), block_depth.clone(), pred_ctx.clone());
        let code = block_r.code;
        let new_pos = block_r.new_pos;
        let block_code = code.clone();
        let block_new_pos = new_pos.clone();
        let mut closure_open: String = "(|| -> bool {\n".to_string();
        let mut closure_close: String = "        })()".to_string();
        final_pred_code = [closure_open.as_str(), block_code.as_str(), closure_close.as_str()].concat();
        final_pos = body_start + block_new_pos;
    }
    let mut struct_prefix: String = "#[derive(Clone, Copy, PartialEq, Debug)]\nstruct ".to_string();
    let mut struct_paren_open: String = "(".to_string();
    let mut struct_suffix: String = ");\n\n".to_string();
    let mut struct_code: String = [struct_prefix.as_str(), type_name.as_str(), struct_paren_open.as_str(), rust_param_type.as_str(), struct_suffix.as_str()].concat();
    let mut impl_prefix: String = "impl ".to_string();
    let mut impl_fn_new_open: String = " {\n    fn new(".to_string();
    let mut impl_colon: String = ": ".to_string();
    let mut impl_return_if_open: String = ") -> Option<Self> {\n        if ".to_string();
    let mut impl_some_open: String = " {\n            Some(".to_string();
    let mut impl_inner_paren_open: String = "(".to_string();
    let mut impl_suffix: String = "))\n        } else {\n            None\n        }\n    }\n}\n\n".to_string();
    let mut impl_code: String = [impl_prefix.as_str(), type_name.as_str(), impl_fn_new_open.as_str(), param_name.as_str(), impl_colon.as_str(), rust_param_type.as_str(), impl_return_if_open.as_str(), final_pred_code.as_str(), impl_some_open.as_str(), type_name.as_str(), impl_inner_paren_open.as_str(), param_name.as_str(), impl_suffix.as_str()].concat();
    let mut type_code: String = s_cat(struct_code, impl_code);
    return make_result(type_code, final_pos.clone());
}

// transpiler-deor/codegen/decl/function.deor
fn gen_fn_decl(fn_tokens: TokensRef, pos: i64, ctx: RcCtx) -> ParseResult {
    // transpiler-deor/codegen/decl/function.deor
    fn locate_fn_return_type(kw_pos: i64) -> i64 {
        return kw_pos + 1;
    }
    let variant_reg = ctx.variant_reg.clone();
    let shape_reg = ctx.shape_reg.clone();
    let struct_reg = ctx.struct_reg.clone();
    let enum_reg = ctx.enum_reg.clone();
    let mut mut_names = ctx.mut_names.clone();
    let type_reg = ctx.type_reg.clone();
    let mut return_type_pos: i64 = locate_fn_return_type(pos.clone());
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
    let mut cur: TokenCursor = cur_at_ref(fn_tokens.clone(), return_type_pos.clone());
    let current = cur.current.clone();
    let value = current.value.clone();
    let mut ret_type: String = resolve_type(value.clone(), ctx.clone());
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
            let mut rust_param_type: String = resolve_type(param_type.clone(), ctx.clone());
            let mut param_separator: String = ": ".to_string();
            param_strs.push([param_name.as_str(), param_separator.as_str(), rust_param_type.as_str()].concat().clone());
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
        } else {
            // transpiler-deor/codegen/decl/macros/fn_parse_signature.deor
            cur = cur_next_ref(cur.clone(), fn_tokens.clone());
        }
    }
    let pos = cur.pos.clone();
    let mut indent_pos: i64 = pos + 1;
    cur = cur_skip_to_body_ref(cur.clone(), fn_tokens.clone());
    let pos = cur.pos.clone();
    let mut body_start: i64 = pos.clone();
    // macro: fn_build_body_ctx (transpiler-deor/codegen/decl/macros/fn_build_body_ctx.deor)
    let typed_enum_reg = ctx.typed_enum_reg.clone();
    let typed_variant_reg = ctx.typed_variant_reg.clone();
    let mut body_end_pos: i64 = find_block_end_ref(fn_tokens.clone(), indent_pos.clone());
    let mut body_slice_end: i64 = body_end_pos + 1;
    let mut body_tokens_raw: Vec<Token> = l_slice_ref(fn_tokens.clone(), body_start.clone(), body_slice_end.clone());
    let mut body_len: i64 = (body_tokens_raw.len() as i64);
    let mut zero: i64 = 0;
    let mut body_last: i64 = body_len - 1;
    let mut mut_names: Vec<String> = collect_mut_names(body_tokens_raw.clone(), zero.clone(), body_last.clone());
    let mut validator_var_reg: Vec<String> = collect_validator_var_types(body_tokens_raw.clone(), type_reg.clone());
    let mut tokens: TokensRef = tokens_wrap(body_tokens_raw);
    let mut body_ctx_raw: GenCtx = GenCtx { variant_reg, shape_reg, struct_reg, enum_reg, mut_names, type_reg, tokens, typed_enum_reg, typed_variant_reg, validator_var_reg };
    let mut body_ctx: RcCtx = make_rctx(body_ctx_raw);
    // macro: fn_emit (transpiler-deor/codegen/decl/macros/fn_emit.deor)
    {
        // transpiler-deor/codegen/decl/macros/fn_emit.deor
        let mut body_pos: i64 = 0;
        let mut body_depth: i64 = 1;
        let mut body_r: ParseResult = gen_block(body_pos.clone(), body_depth.clone(), body_ctx);
        let code = body_r.code;
        let new_pos = body_r.new_pos;
        let body_code = code.clone();
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
}

// transpiler-deor/codegen/decl/raw.deor
fn gen_raw_decl(tokens: TokensRef, pos: i64) -> ParseResult {
    // transpiler-deor/codegen/decl/raw.deor
    fn locate_raw_newline(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    let mut after: i64 = locate_raw_newline(pos.clone());
    let mut empty_str: String = "".to_string();
    return make_nl_result(empty_str, after.clone(), tokens.clone());
}

// transpiler-deor/codegen/codegen.deor
fn generate_rust_from_tokens(all_ref: TokensRef, ctx: RcCtx) -> String {
    // transpiler-deor/codegen/codegen.deor
    fn locate_rust_block_content(kw_pos: i64) -> i64 {
        return kw_pos + 2;
    }
    let mut parts: Vec<String> = Vec::new();
    let mut token_count: i64 = (all_ref.len() as i64);
    if verbose_get() {
        // transpiler-deor/codegen/codegen.deor
        println!("{}", ["[diag] token_count: ", n_to_str(token_count.clone()).as_str()].concat());
    }
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
            let mut comment_prefix: String = "// ".to_string();
            let mut newline: String = "\n".to_string();
            let mut prefixed_file: String = s_cat(comment_prefix.clone(), file.clone());
            let mut file_comment: String = s_cat(prefixed_file.clone(), newline.clone());
            parts.push(file_comment.clone());
            last_file = file;
        }
        if kind == "KW_STRUCT" {
            // transpiler-deor/codegen/codegen.deor
            let mut result: ParseResult = gen_struct_decl(all_ref.clone(), pos.clone(), ctx.clone());
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
            let mut block_pos: i64 = locate_rust_block_content(pos.clone());
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
    {
        // transpiler-deor/macros/timer.deor
        let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
        if verbose_get() {
            // transpiler-deor/macros/timer.deor
            let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
            let mut _timer_sfx: String = "ms".to_string();
            println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
        }
    }
    // transpiler-deor/codegen/codegen.deor
    return s_join(parts.clone());
}

// transpiler-deor/main.deor
fn main() {
    // transpiler-deor/main.deor
    let mut raw_args: Vec<String> = f_args();
    let mut verbose_long: String = "--verbose".to_string();
    let mut verbose_short: String = "-v".to_string();
    let mut verbose: bool = list_has(raw_args.clone(), verbose_long.clone()) || list_has(raw_args.clone(), verbose_short.clone());
    if verbose {
        // transpiler-deor/main.deor
        verbose_enable();
    }
    let mut cli_args: Vec<String> = Vec::new();
    let mut raw_count: i64 = (raw_args.len() as i64);
    let mut raw_i: i64 = 0;
    while raw_i < raw_count {
        // transpiler-deor/main.deor
        let mut raw_arg: String = raw_args[raw_i as usize].clone();
        let mut is_verbose_flag: bool = raw_arg == verbose_long || raw_arg == verbose_short;
        if !is_verbose_flag {
            // transpiler-deor/main.deor
            cli_args.push(raw_arg.clone());
        }
        raw_i = raw_i + 1;
    }
    let mut arg_count: i64 = (cli_args.len() as i64);
    if arg_count < 2 {
        // transpiler-deor/main.deor
        println!("{}", "usage: deor input.deor output.rs [--verbose|-v]".to_string());
    } else {
        // transpiler-deor/main.deor
        let mut input_path: String = cli_args[0 as usize].clone();
        let mut output_path: String = cli_args[1 as usize].clone();
        let mut _timer_label: String = "[timer] load+dedup: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let mut dedup_r: DedupResult = collect_all_tokens_with_all_imports(input_path.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        {
            // transpiler-deor/macros/timer.deor
            let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
            if verbose_get() {
                // transpiler-deor/macros/timer.deor
                let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
                let mut _timer_sfx: String = "ms".to_string();
                println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
            }
        }
        // transpiler-deor/main.deor
        let mut tokens = dedup_r.tokens.clone();
        let enforce_macro_file_depth = dedup_r.enforce_macro_file_depth.clone();
        _timer_label = "[timer] macro-build: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        tokens = build_macros(tokens.clone(), enforce_macro_file_depth.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        {
            // transpiler-deor/macros/timer.deor
            let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
            if verbose_get() {
                // transpiler-deor/macros/timer.deor
                let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
                let mut _timer_sfx: String = "ms".to_string();
                println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
            }
        }
        // transpiler-deor/main.deor
        let mut tokens_ref: TokensRef = tokens_wrap(tokens);
        _timer_label = "[timer] validate: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        validate_tokens(tokens_ref.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        {
            // transpiler-deor/macros/timer.deor
            let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
            if verbose_get() {
                // transpiler-deor/macros/timer.deor
                let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
                let mut _timer_sfx: String = "ms".to_string();
                println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
            }
        }
        // transpiler-deor/main.deor
        _timer_label = "[timer] registry: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let ctx = build_registry(tokens_ref.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        {
            // transpiler-deor/macros/timer.deor
            let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
            if verbose_get() {
                // transpiler-deor/macros/timer.deor
                let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
                let mut _timer_sfx: String = "ms".to_string();
                println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
            }
        }
        // transpiler-deor/main.deor
        _timer_label = "[timer] total-codegen: ".to_string();
        // macro: timer_start (transpiler-deor/macros/timer.deor)
        let mut _timer_start: i64 = now_ms();
        // transpiler-deor/main.deor
        let mut rust_code: String = generate_rust_from_tokens(tokens_ref.clone(), ctx.clone());
        // macro: timer_end (transpiler-deor/macros/timer.deor)
        {
            // transpiler-deor/macros/timer.deor
            let mut _timer_elapsed: i64 = elapsed_ms(_timer_start.clone());
            if verbose_get() {
                // transpiler-deor/macros/timer.deor
                let mut _timer_str: String = n_to_str(_timer_elapsed.clone());
                let mut _timer_sfx: String = "ms".to_string();
                println!("{}", [_timer_label.as_str(), _timer_str.as_str(), _timer_sfx.as_str()].concat());
            }
        }
        // transpiler-deor/main.deor
        let mut allow_warnings: String = "#![allow(warnings)]\n".to_string();
        rust_code = s_cat(allow_warnings.clone(), rust_code.clone());
        f_write(output_path.clone(), rust_code.clone());
    }
}

