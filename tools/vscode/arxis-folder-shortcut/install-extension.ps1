param(
    [string]$DestinationRoot = "$env:USERPROFILE\.vscode\extensions"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$extensionRoot = Resolve-Path $scriptDir
$packageJsonPath = Join-Path $extensionRoot 'package.json'

if (-not (Test-Path $packageJsonPath)) {
    throw "Não foi possível localizar package.json em $extensionRoot"
}

$packageJson = Get-Content $packageJsonPath | ConvertFrom-Json
$publisher = $packageJson.publisher
$extensionName = $packageJson.name
$version = $packageJson.version

if ([string]::IsNullOrWhiteSpace($publisher) -or [string]::IsNullOrWhiteSpace($extensionName) -or [string]::IsNullOrWhiteSpace($version)) {
    throw 'package.json precisa conter os campos publisher, name e version.'
}

$destinationPath = Join-Path $DestinationRoot "$publisher.$extensionName-$version"

if (Test-Path $destinationPath) {
    Write-Host "Removendo versão existente em $destinationPath" -ForegroundColor Yellow
    Remove-Item -Path $destinationPath -Recurse -Force
}

Write-Host "Copiando extensão para $destinationPath" -ForegroundColor Cyan
New-Item -ItemType Directory -Path $DestinationRoot -Force | Out-Null
Copy-Item -Path $extensionRoot -Destination $destinationPath -Recurse -Force

Write-Host 'Extensão instalada com sucesso. Reinicie ou recarregue o VS Code para concluir.' -ForegroundColor Green
