export function renderAsBinding(name: string, value: string, isString: boolean): string {
  return isString
    ? `let ${name} = ${value}.to_string();`
    : `let ${name} = ${value};`;
}

export function renderTypedBinding(name: string, rustType: string, value: string): string {
  return `let ${name}: ${rustType} = ${value};`;
}
