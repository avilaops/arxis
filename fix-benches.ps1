# Fix missing bench files by commenting them out
$crates = @(
    "avila-clustering",
    "avila-dataframe",
    "avila-linalg",
    "avila-math",
    "avila-ml",
    "avila-telemetry",
    "avila-reduction",
    "avila-tokenizer",
    "avila-arrow",
    "aviladb",
    "avl-loadbalancer"
)

foreach ($crate in $crates) {
    $cargoFile = Join-Path $PSScriptRoot "$crate\Cargo.toml"

    if (Test-Path $cargoFile) {
        Write-Host "Processing $crate..." -ForegroundColor Cyan

        $lines = Get-Content $cargoFile
        $newLines = @()
        $inBench = $false

        for ($i = 0; $i -lt $lines.Count; $i++) {
            $line = $lines[$i]

            if ($line -match '^\[\[bench\]\]') {
                $inBench = $true
                $newLines += "# [[bench]]"
            }
            elseif ($inBench -and ($line -match '^(name|harness|path)\s*=')) {
                $newLines += "# $line"
            }
            elseif ($inBench -and ($line -match '^\s*$' -or $line -match '^\[')) {
                # Linha vazia ou nova seção - sair do modo bench
                $inBench = $false
                $newLines += $line
            }
            else {
                $newLines += $line
            }
        }

        $newLines | Set-Content $cargoFile
        Write-Host "  Fixed $crate" -ForegroundColor Green
    }
}

Write-Host "`nAll benches commented!" -ForegroundColor Green
