# Meta Tags

A meta tag is an ordinary comment that other tools can read. The transpiler never looks at it: a tagged file transpiles to exactly the same Rust as an untagged one. Tags are for tools that work beside the compiler — documentation generators, test runners, component frameworks.

```
# meta(component: page)
# meta(route: /settings)
fn string settings_page()
	int val = 5  # meta(note: example)
	return "x"
```

## Form

- A tag is exactly `# meta(` (one space after the `#`) up to the last `)` on the line. `#meta(` and `#  meta(` are ordinary comments.
- Recommended inside: up to 3 `key: value` pairs, such as `# meta(name: Nate, last: Hoffman, date: 2026)`. What the keys and values mean is up to the tool that reads them.
- At the end of a line, a tag describes that line. On its own line, it describes the next line with code, so several tags can stack above one declaration.

## What a tag describes

The reader reports the kind and name of what each tag is attached to: `function`, `struct`, `enum`, `shape`, `type`, `macro`, `raw`, `import`, `field` (inside a struct), `variant` (inside an enum), `statement` (any other code line), or `none` (a tag at the end of a file).

## Reading tags

`deor meta [path]` lists the tags in every `.deor` file under the current directory (or `path`), skipping gitignored files. It must run inside a git repository.

```
profile.deor line 14  component: page  -> function settings_page
profile.deor line 17  note: example  -> statement (line 17)
```

`deor meta --json` prints the same information as JSON.

Deor tools import `lib/meta.deor` and call `read_meta_tags(source)`, which returns each tag with its line, attached line, fields, kind, name and warnings.

## Warnings

The reader never fails; it attaches warnings to a tag, and the tool decides what to do with them.

| Warning | Meaning | Hide with |
|---|---|---|
| `malformed:` | the tag has no closing `)` | `--ignore-malformed` |
| `inner:` | the inside is not `key: value` pairs (including a comma inside a value), or has more than 3 fields | `--ignore-inner` |

The compiler's own tests treat any warning as a failure.

## Rule of thumb

If the transpiler would ever need to act on a tag, it has stopped being a comment: it should become real syntax instead.
