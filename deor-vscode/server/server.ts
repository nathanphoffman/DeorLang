import { spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import { fileURLToPath } from 'url';
import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  InitializeParams,
  InitializeResult,
  TextDocumentSyncKind,
  Diagnostic,
  DiagnosticSeverity,
  Location,
  Range,
} from 'vscode-languageserver/node';
import { TextDocument } from 'vscode-languageserver-textdocument';

const connection = createConnection(ProposedFeatures.all);
const documents  = new TextDocuments(TextDocument);

let workspaceRoot: string | null = null;
let prevDiagnosticUris: Set<string> = new Set();
let running = false;

connection.onInitialize((params: InitializeParams): InitializeResult => {
  if (params.rootUri) {
    workspaceRoot = fileURLToPath(params.rootUri);
  } else if (params.rootPath) {
    workspaceRoot = params.rootPath;
  }
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      definitionProvider: true,
    },
  };
});

// ─── Go to Definition ────────────────────────────────────────────────────────

connection.onDefinition((params) => {
  const doc = documents.get(params.textDocument.uri);
  if (!doc || !workspaceRoot) return null;

  const lines = doc.getText().split('\n');
  const line  = lines[params.position.line] ?? '';
  const word  = wordAt(line, params.position.character);
  if (!word) return null;

  return findDefinition(word, workspaceRoot);
});

function wordAt(line: string, char: number): string {
  let start = char;
  while (start > 0 && /[a-zA-Z0-9_]/.test(line[start - 1])) start--;
  let end = char;
  while (end < line.length && /[a-zA-Z0-9_]/.test(line[end])) end++;
  return line.slice(start, end);
}

// Patterns for every Deor declaration kind. All top-level so no leading tabs.
function definitionPatterns(name: string): RegExp[] {
  const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  return [
    new RegExp(`^fn \\S+ ${escaped}\\(`),   // fn <type> name(
    new RegExp(`^fn ${escaped}\\(`),         // fn name( (void shorthand if ever used)
    new RegExp(`^struct ${escaped}\\b`),
    new RegExp(`^enum ${escaped}\\b`),
    new RegExp(`^shape ${escaped}\\b`),
    new RegExp(`^type ${escaped}\\b`),
    new RegExp(`^macro ${escaped}\\b`),
    new RegExp(`^raw ${escaped}\\b`),
  ];
}

function findDefinition(name: string, root: string): Location | null {
  const patterns = definitionPatterns(name);

  for (const file of allDeorFiles(root)) {
    const content = fs.readFileSync(file, 'utf8');
    const lines   = content.split('\n');
    for (let i = 0; i < lines.length; i++) {
      for (const pat of patterns) {
        if (pat.test(lines[i])) {
          const uri: string = `file://${file}`;
          const range: Range = {
            start: { line: i, character: 0 },
            end:   { line: i, character: lines[i].length },
          };
          return Location.create(uri, range);
        }
      }
    }
  }
  return null;
}

function allDeorFiles(dir: string): string[] {
  const results: string[] = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === 'node_modules' || entry.name === 'target') continue;
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...allDeorFiles(full));
    } else if (entry.isFile() && entry.name.endsWith('.deor')) {
      results.push(full);
    }
  }
  return results;
}

// ─── Diagnostics ─────────────────────────────────────────────────────────────

function findEntryPoint(savedPath: string, root: string): string {
  let dir = path.dirname(savedPath);
  while (true) {
    const candidate = path.join(dir, 'main.deor');
    if (fs.existsSync(candidate)) return candidate;
    if (dir === root || dir === path.dirname(dir)) break;
    dir = path.dirname(dir);
  }
  return savedPath;
}

documents.onDidSave((event) => { runValidation(event.document.uri); });

function runValidation(savedUri: string): void {
  if (!workspaceRoot || running) return;

  const transpilerBin = path.join(workspaceRoot, 'output', 'out');
  if (!fs.existsSync(transpilerBin)) return;

  const savedPath = fileURLToPath(savedUri);
  const entryAbs  = findEntryPoint(savedPath, workspaceRoot);
  const relEntry  = path.relative(workspaceRoot, entryAbs);

  running = true;
  let output = '';

  const proc = spawn(transpilerBin, [relEntry, '/dev/null'], {
    cwd: workspaceRoot,
  });

  proc.stdout.on('data', (chunk: Buffer) => { output += chunk.toString(); });
  proc.stderr.on('data', (chunk: Buffer) => { output += chunk.toString(); });

  proc.on('close', () => { running = false; publishDiagnostics(output); });
  proc.on('error', () => { running = false; });
}

const VALIDATION_RE = /^\[validation\] (.+) line (\d+): (.+)$/;

function publishDiagnostics(output: string): void {
  const byUri = new Map<string, Diagnostic[]>();

  for (const line of output.split('\n')) {
    const m = line.match(VALIDATION_RE);
    if (!m) continue;

    const [, relFile, lineStr, msg] = m;
    const absPath = path.resolve(workspaceRoot!, relFile);
    const uri     = `file://${absPath}`;
    const ln      = Math.max(0, parseInt(lineStr, 10) - 1);

    const diag: Diagnostic = {
      range: {
        start: { line: ln, character: 0 },
        end:   { line: ln, character: Number.MAX_SAFE_INTEGER },
      },
      message:  msg.trim(),
      severity: DiagnosticSeverity.Error,
      source:   'deor',
    };

    if (!byUri.has(uri)) byUri.set(uri, []);
    byUri.get(uri)!.push(diag);
  }

  for (const uri of prevDiagnosticUris) {
    if (!byUri.has(uri)) connection.sendDiagnostics({ uri, diagnostics: [] });
  }

  prevDiagnosticUris = new Set();
  for (const [uri, diags] of byUri) {
    connection.sendDiagnostics({ uri, diagnostics: diags });
    prevDiagnosticUris.add(uri);
  }
}

documents.listen(connection);
connection.listen();
