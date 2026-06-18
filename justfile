run file="examples/hello.deor":
    cd transpiler && go build -o ./bin/deor .
    ./transpiler/bin/deor {{file}} output/out.rs
    rustc output/out.rs -o output/out
    ./output/out

run-deor file="examples/hello.deor":
    ./output/out transpiler-deor/main.deor output/out.rs
    rustc -A warnings output/out.rs -o output/out
    ./output/out {{file}} output/run.rs
    rustc -A warnings output/run.rs -o output/run
    ./output/run

run-ts file="examples/hello.deor":
    cd transpiler-ts && npm install --silent
    cd transpiler-ts && npx tsx src/main.ts ../{{file}} ../output/out.rs
    rustc output/out.rs -o output/out
    ./output/out

install-ext:
    cd deor-vscode && npm install --silent && npm run compile
    cd deor-vscode && vsce package --allow-missing-repository --skip-license
    code --install-extension $(ls deor-vscode/*.vsix | tail -1)
    @echo "Done — reload VS Code window to apply."

run-spec:
    cd specification && npm run start

run-splash:
    cd ./site && bun run dev

