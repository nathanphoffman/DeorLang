install-ext:
    cd deor-vscode && vsce package --allow-missing-repository --skip-license
    code --install-extension deor-vscode/deor-lang-0.0.1.vsix
    @echo "Done — reload VS Code window to apply."

run-spec:
    cd specification && npm run start
