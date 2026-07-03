$ErrorActionPreference = "Stop"

$DeorHome = "$env:USERPROFILE\.deor"
$BinDir   = "$DeorHome\bin"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir

Write-Host "Updating Deor..."

Write-Host "  Recompiling transpiler..."
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
& rustc -O "$ScriptDir\out.rs" -o "$BinDir\deor.exe"
if ($LASTEXITCODE -ne 0) { throw "rustc failed" }
Write-Host "  Transpiler updated at $BinDir\deor.exe"

# Determine project directory to update lib/spec files in
if ($args.Count -gt 0) {
    $ProjectDir = $args[0] -replace '^~', $HOME
} elseif (Test-Path "$(Get-Location)\lib\*.deor") {
    $ProjectDir = (Get-Location).Path
} else {
    $ProjectDir = ""
}

if ($ProjectDir -ne "") {
    if (-not (Test-Path -PathType Container $ProjectDir)) {
        throw "'$ProjectDir' is not a directory."
    }

    Write-Host "  Updating lib files in $ProjectDir\lib\ ..."
    New-Item -ItemType Directory -Force -Path "$ProjectDir\lib" | Out-Null
    Copy-Item -Recurse -Force "$RepoRoot\lib\*" "$ProjectDir\lib\"
    Write-Host "  Libs updated."

    Write-Host "  Updating spec docs in $ProjectDir\deor_specification\ ..."
    Remove-Item -Recurse -Force "$ProjectDir\deor_specification" -ErrorAction SilentlyContinue
    New-Item -ItemType Directory -Force -Path "$ProjectDir\deor_specification" | Out-Null
    Copy-Item -Recurse -Force "$ScriptDir\deor_specification\*" "$ProjectDir\deor_specification\"
    Write-Host "  Spec docs updated."
} else {
    Write-Host ""
    Write-Host "  No project directory detected. To also update a project's lib files and spec docs, run this from inside your project directory, or pass a path:"
    Write-Host "    .\setup\update.ps1 C:\path\to\your\project"
}

Write-Host ""
Write-Host "Done! Run 'deor --version' or 'just run' in your project to verify."
