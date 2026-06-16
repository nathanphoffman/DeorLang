// field extraction from a struct — emits one field-access binding per field
// single:   `area in room`         → let area = room.area;
// multi:    `(area, name) in room` → let area = room.area; let name = room.name;
export function renderDestructure(fields: string[], source: string, pad: string): string {
  return fields.map(f => `${pad}let ${f} = ${source}.${f};\n`).join('');
}
