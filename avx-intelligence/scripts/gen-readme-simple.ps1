# Generate READMEs for ARXIS crates
param([int]$First = 8)

$crates = Get-ChildItem -Directory | Where-Object { ($_.Name -like "avila-*") -and (Test-Path "$($_.FullName)\Cargo.toml") -and !(Test-Path "$($_.FullName)\README.md") } | Select-Object -First $First

foreach ($crate in $crates) {
    $readme = "# $($crate.Name)`n`nPart of ARXIS ecosystem.`n`n## License`n`nMIT OR Apache-2.0"
    $readme | Out-File -FilePath "$($crate.FullName)\README.md" -Encoding UTF8
    Write-Host "Created: $($crate.Name)/README.md"
}

Write-Host "Done: $($crates.Count) READMEs created"
