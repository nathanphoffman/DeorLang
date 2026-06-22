$ErrorActionPreference = "Stop"

$DeorHome = "$env:USERPROFILE\.deor"
$BinDir   = "$DeorHome\bin"
$LibDir   = "$DeorHome\lib"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir

Write-Host "Installing Deor..."

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $LibDir | Out-Null

Write-Host "  Compiling transpiler..."
& rustc -O -A warnings "$ScriptDir\out.rs" -o "$BinDir\deor.exe"
if ($LASTEXITCODE -ne 0) { throw "rustc failed" }

Write-Host "  Installing lib\..."
Copy-Item -Recurse -Force "$RepoRoot\lib\*" "$LibDir\"

$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$BinDir;$userPath", "User")
    Write-Host "  Added $BinDir to user PATH"
}

[Environment]::SetEnvironmentVariable("DEOR_LIB", $LibDir, "User")
Write-Host "  Set DEOR_LIB=$LibDir"

Write-Host ""
Write-Host "Done! Restart your terminal for PATH changes to take effect."
