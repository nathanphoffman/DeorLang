import {
  createConnection,
  TextDocuments,
  Diagnostic,
  DiagnosticSeverity,
  ProposedFeatures,
  InitializeResult,
  TextDocumentSyncKind,
} from 'vscode-languageserver/node';
import { TextDocument } from 'vscode-languageserver-textdocument';
import { Lexer } from '../../transpiler-ts_old_dont_use/src/lexer';
import { Parser } from '../../transpiler-ts_old_dont_use/src/parser';

const connection = createConnection(ProposedFeatures.all);
const documents  = new TextDocuments(TextDocument);

connection.onInitialize((): InitializeResult => ({
  capabilities: {
    textDocumentSync: TextDocumentSyncKind.Incremental,
  },
}));

documents.onDidChangeContent(change => validate(change.document));

function validate(doc: TextDocument): void {
  const diagnostics: Diagnostic[] = [];

  try {
    const lexer  = new Lexer(doc.getText());
    const parser = new Parser(lexer);
    parser.parseProgram();
  } catch (e) {
    if (e instanceof Error) {
      // parser errors are formatted as "line N: message"
      const match   = e.message.match(/^line (\d+): (.+)$/);
      const line    = match ? parseInt(match[1], 10) - 1 : 0;
      const message = match ? match[2] : e.message;

      diagnostics.push({
        severity: DiagnosticSeverity.Error,
        range: {
          start: { line, character: 0 },
          end:   { line, character: Number.MAX_SAFE_INTEGER },
        },
        message,
        source: 'deor',
      });
    }
  }

  connection.sendDiagnostics({ uri: doc.uri, diagnostics });
}

documents.listen(connection);
connection.listen();
