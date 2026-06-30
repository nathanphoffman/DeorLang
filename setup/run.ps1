param(
    [string]$Input = "setup\main.deor"
)

$ErrorActionPreference = "Stop"

$OutRs  = "$env:TEMP\_deor_out.rs"
$OutBin = "$env:TEMP\_deor_run.exe"

& deor $Input $OutRs
if ($LASTEXITCODE -ne 0) { throw "deor transpile failed" }

& rustc -O -A warnings $OutRs -o $OutBin
if ($LASTEXITCODE -ne 0) { throw "rustc compile failed" }

& $OutBin
