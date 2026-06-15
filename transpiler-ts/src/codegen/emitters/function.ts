export function renderRustType(deorType: string): string {
  switch (deorType) {
    case 'int':    return 'i32';
    case 'float':  return 'f64';
    case 'bool':   return 'bool';
    case 'string': return 'String';
    default:       return deorType;
  }
}

export function renderParam(name: string, deorType: string): string {
  return `${name}: ${renderRustType(deorType)}`;
}

export function renderFunction(name: string, params: string, returnType: string, body: string): string {
  const ret = returnType ? ` -> ${renderRustType(returnType)}` : '';
  return `fn ${name}(${params})${ret} {\n${body}}\n`;
}
