export function renderAsBinding(name: string, value: string, isString: boolean, isMut = false): string {
  const mut = isMut ? 'mut ' : '';
  return isString
    ? `let ${mut}${name} = ${value}.to_string();`
    : `let ${mut}${name} = ${value};`;
}

export function renderTypedBinding(name: string, rustType: string, value: string, isMut = false): string {
  const mut = isMut ? 'mut ' : '';
  return `let ${mut}${name}: ${rustType} = ${value};`;
}
