$ErrorActionPreference = "Stop"

$DeorHome = "$env:USERPROFILE\.deor"
$BinDir   = "$DeorHome\bin"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir

$ProjectNameInput = Read-Host "Project name (default: hello)"
$ProjectName = if ([string]::IsNullOrWhiteSpace($ProjectNameInput)) { "hello" } else { $ProjectNameInput }

$DefaultProject = "$(Get-Location)\$ProjectName"
while ($true) {
    $Input = Read-Host "Where would you like to create your project? (default: $DefaultProject)"
    if ([string]::IsNullOrWhiteSpace($Input)) {
        $ProjectDir = $DefaultProject
    } else {
        $ProjectDir = $Input -replace '^~', $HOME
    }

    if (Test-Path -PathType Leaf $ProjectDir) {
        Write-Host "  Error: '$ProjectDir' is a file, not a directory. Please choose a different path."
        continue
    }

    if (Test-Path -PathType Container $ProjectDir) {
        $Confirm = Read-Host "  '$ProjectDir' already exists. Install project there anyway? [Y/n]"
        if ($Confirm -match '^[nN]') { Write-Host "  Aborted."; exit 0 }
    }

    break
}

Write-Host "Installing Deor..."

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $ProjectDir | Out-Null

if (-not (Get-Command just -ErrorAction SilentlyContinue)) {
    Write-Host "  Installing just..."
    & cargo install just
    if ($LASTEXITCODE -ne 0) { throw "cargo install just failed" }
}

Write-Host "  Compiling transpiler..."
& rustc -O -A warnings "$ScriptDir\out.rs" -o "$BinDir\deor.exe"
if ($LASTEXITCODE -ne 0) { throw "rustc failed" }

Write-Host "  Creating starter project..."
Copy-Item "$ScriptDir\hello.deor"   "$ProjectDir\hello.deor"
Copy-Item "$ScriptDir\.gitignore"   "$ProjectDir\.gitignore"
Copy-Item "$ScriptDir\justfile"     "$ProjectDir\justfile"
(Get-Content "$ScriptDir\Cargo.toml") -replace '{{PROJECT_NAME}}', $ProjectName |
    Set-Content "$ProjectDir\Cargo.toml"
Copy-Item -Recurse -Force "$RepoRoot\lib\*" "$ProjectDir\lib\"

$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$BinDir;$userPath", "User")
    Write-Host "  Added $BinDir to user PATH"
}

Write-Host ""
Write-Host "Done! Restart your terminal for PATH changes to take effect."
Write-Host ""
Write-Host "To run your hello world:"
Write-Host "  cd `"$ProjectDir`""
Write-Host "  just run"
Write-Host ""
Write-Host "  (Without just: set DEOR_LIB=lib && deor hello.deor build\main.rs && cargo run)"
