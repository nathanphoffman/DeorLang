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

Write-Host ""
Write-Host "Done! Restart your terminal for PATH changes to take effect."
