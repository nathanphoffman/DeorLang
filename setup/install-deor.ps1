$ErrorActionPreference = "Stop"

$DeorHome = "$env:USERPROFILE\.deor"
$BinDir   = "$DeorHome\bin"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "Installing Deor..."

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

Write-Host "  Compiling transpiler..."
& rustc -O "$ScriptDir\out.rs" -o "$BinDir\deor.exe"
if ($LASTEXITCODE -ne 0) { throw "rustc failed" }

$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$BinDir;$userPath", "User")
    Write-Host "  Added $BinDir to user PATH"
}

$InstallExt = Read-Host "Install the Deor VS Code extension? [y/N]"
if ($InstallExt -match '^[yY]') {
    if (Get-Command code -ErrorAction SilentlyContinue) {
        Write-Host "  Installing VS Code extension..."
        & code --install-extension "$ScriptDir\deor-lang.vsix"
    } else {
        Write-Host "  VS Code 'code' command not found in PATH; skipping extension install."
    }
}

Write-Host ""
Write-Host "Done! Restart your terminal for PATH changes to take effect."
