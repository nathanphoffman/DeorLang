$ErrorActionPreference = "Stop"

$DeorHome = "$env:USERPROFILE\.deor"
$BinDir   = "$DeorHome\bin"
$LibDir   = "$DeorHome\lib"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir

$DefaultProject = "$(Get-Location)\hello-deor"
$Input = Read-Host "Where would you like to create your starter project? (default: $DefaultProject)"
if ([string]::IsNullOrWhiteSpace($Input)) {
    $ProjectDir = $DefaultProject
} else {
    $ProjectDir = $Input
}

Write-Host "Installing Deor..."

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $LibDir | Out-Null
New-Item -ItemType Directory -Force -Path $ProjectDir | Out-Null

Write-Host "  Compiling transpiler..."
& rustc -O -A warnings "$ScriptDir\out.rs" -o "$BinDir\deor.exe"
if ($LASTEXITCODE -ne 0) { throw "rustc failed" }

Write-Host "  Installing lib\..."
Copy-Item -Recurse -Force "$RepoRoot\lib\*" "$LibDir\"

Write-Host "  Creating starter project..."
Copy-Item "$ScriptDir\hello.deor" "$ProjectDir\hello.deor"

$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$BinDir;$userPath", "User")
    Write-Host "  Added $BinDir to user PATH"
}

[Environment]::SetEnvironmentVariable("DEOR_LIB", $LibDir, "User")
Write-Host "  Set DEOR_LIB=$LibDir"

Write-Host ""
Write-Host "Done! Restart your terminal for PATH changes to take effect."
Write-Host ""
Write-Host "To run your hello world:"
Write-Host "  cd `"$ProjectDir`""
Write-Host "  deor hello.deor hello.rs; rustc hello.rs -o hello.exe; .\hello.exe"
