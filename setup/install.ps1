$ErrorActionPreference = "Stop"

$DeorHome = "$env:USERPROFILE\.deor"
$BinDir   = "$DeorHome\bin"
$LibDir   = "$DeorHome\lib"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir

$DefaultProject = "$(Get-Location)\hello-deor"
while ($true) {
    $Input = Read-Host "Where would you like to create your starter project? (default: $DefaultProject)"
    if ([string]::IsNullOrWhiteSpace($Input)) {
        $ProjectDir = $DefaultProject
    } else {
        $ProjectDir = $Input -replace '^~', $HOME
    }

    if (Test-Path -PathType Leaf $ProjectDir) {
        Write-Host "  Error: '$ProjectDir' is a file, not a directory. Please choose a different path."
        continue
    }

    $ParentDir = Split-Path -Parent $ProjectDir
    if (-not (Test-Path -PathType Container $ParentDir)) {
        Write-Host "  Error: parent directory '$ParentDir' does not exist. Please choose a different path."
        continue
    }

    if (Test-Path -PathType Container $ProjectDir) {
        $Confirm = Read-Host "  '$ProjectDir' already exists. Install hello.deor there anyway? [Y/n]"
        if ($Confirm -match '^[nN]') { Write-Host "  Aborted."; exit 0 }
    }

    break
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
