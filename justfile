run file="examples/hello.deor":
    cd transpiler && go build -o ./bin/deor .
    ./transpiler/bin/deor {{file}} output/out.rs
    rustc output/out.rs -o output/out
    ./output/out

run-ts file="examples/hello.deor":
    cd transpiler-ts && npm install --silent
    cd transpiler-ts && npx tsx src/main.ts ../{{file}} ../output/out.rs
    rustc output/out.rs -o output/out
    ./output/out

install-ext:
    cd deor-vscode && vsce package --allow-missing-repository --skip-license
    code --install-extension deor-vscode/deor-lang-0.0.1.vsix
    @echo "Done — reload VS Code window to apply."

run-spec:
    cd specification && npm run start
