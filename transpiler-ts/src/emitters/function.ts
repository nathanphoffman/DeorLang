export function renderRustType(deorType: string): string {
  switch (deorType) {
    case 'int':    return 'i32';
    case 'float':  return 'f64';
    case 'bool':   return 'bool';
    case 'string': return 'String';
    default:       return deorType;
  }
}

export function renderParam(
  name: string,
  deorType: string,
  shapeRegistry: Map<string, string>,
  enumRegistry: Map<string, string>,
): string {
  const elemType = shapeRegistry.get(deorType);
  if (elemType) {
    // pass lists by value — caller moves the Vec in; simpler than references for bootstrap
    return `${name}: Vec<${renderRustType(elemType)}>`;
  }
  const enumRustName = enumRegistry.get(deorType);
  if (enumRustName) return `${name}: ${enumRustName}`;
  return `${name}: ${renderRustType(deorType)}`;
}

export function renderFunction(name: string, params: string, returnType: string, body: string): string {
  const ret = returnType ? ` -> ${returnType}` : '';
  return `fn ${name}(${params})${ret} {\n${body}}\n`;
}
