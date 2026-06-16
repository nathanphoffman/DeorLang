import { FieldDecl } from '../ast';

export function renderStructDecl(
  name: string,
  fields: FieldDecl[],
  resolveType: (t: string) => string,
): string {
  const fieldLines = fields.map(f => `    ${f.name}: ${resolveType(f.type)},`).join('\n');
  return `#[derive(Clone, PartialEq, Debug)]\nstruct ${name} {\n${fieldLines}\n}\n`;
}

export function renderStructConstruct(varName: string, structName: string, fields: string[], isMut: boolean): string {
  const mut = isMut ? 'mut ' : '';
  return `let ${mut}${varName} = ${structName} { ${fields.join(', ')} };`;
}
