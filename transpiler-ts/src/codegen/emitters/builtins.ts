export function renderCallStmt(func: string, args: string[]): string {
  if (func === 'print') {
    return args.length === 1
      ? `println!("{}", ${args[0]});`
      : `println!(${args.join(', ')});`;
  }
  return `${func}(${args.join(', ')});`;
}
