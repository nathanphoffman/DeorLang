export function renderIf(
  condition: string,
  thenBlock: string,
  elseIfClauses: { condition: string; block: string }[],
  elseBlock: string | null,
  pad: string,
): string {
  let out = `${pad}if ${condition} {\n${thenBlock}`;
  for (const clause of elseIfClauses) {
    out += `${pad}} else if ${clause.condition} {\n${clause.block}`;
  }
  if (elseBlock !== null) {
    out += `${pad}} else {\n${elseBlock}`;
  }
  out += `${pad}}\n`;
  return out;
}

// maps Deor operators to their Rust equivalents
export function mapOperator(op: string): string {
  switch (op) {
    case 'is':     return '==';
    case 'is not': return '!=';
    case 'and':    return '&&';
    case 'or':     return '||';
    default:       return op;
  }
}
