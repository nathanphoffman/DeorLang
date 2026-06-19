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
    },
  };
});

// Walk up from the saved file looking for a main.deor to use as the entry point.
// Falls back to the saved file itself if none is found before leaving the workspace.
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

  proc.on('close', () => {
    running = false;
    publishDiagnostics(output);
  });

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

  // Clear files that were previously red but are clean now
  for (const uri of prevDiagnosticUris) {
    if (!byUri.has(uri)) {
      connection.sendDiagnostics({ uri, diagnostics: [] });
    }
  }

  prevDiagnosticUris = new Set();
  for (const [uri, diags] of byUri) {
    connection.sendDiagnostics({ uri, diagnostics: diags });
    prevDiagnosticUris.add(uri);
  }
}

documents.listen(connection);
connection.listen();
