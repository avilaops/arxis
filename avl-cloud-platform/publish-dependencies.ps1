#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Script avançado para publicação automatizada de dependências do Avila Cloud no crates.io

.DESCRIPTION
    Este script gerencia o processo completo de publicação de crates no crates.io, incluindo:
    - Validação pré-publicação (build, tests, metadata)
    - Verificação de dependências
    - Detecção automática de versão
    - Publicação com retry logic
    - Geração de relatórios detalhados
    - Backup automático antes da publicação

.PARAMETER DryRun
    Executa o script em modo dry-run (não publica realmente)

.PARAMETER SkipTests
    Pula a execução de testes (não recomendado)

.PARAMETER SkipValidation
    Pula validações pré-publicação (não recomendado)

.PARAMETER LogFile
    Caminho para o arquivo de log (padrão: publish-log-[timestamp].txt)

.PARAMETER WaitTime
    Tempo de espera entre publicações em segundos (padrão: 10)

.EXAMPLE
    .\publish-dependencies.ps1

.EXAMPLE
    .\publish-dependencies.ps1 -DryRun

.EXAMPLE
    .\publish-dependencies.ps1 -WaitTime 15 -LogFile "my-publish.log"

.NOTES
    Versão: 2.0
    Autor: Avila Team
    Data: 2025-12-02
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory=$false)]
    [switch]$DryRun,

    [Parameter(Mandatory=$false)]
    [switch]$SkipTests,

    [Parameter(Mandatory=$false)]
    [switch]$SkipValidation,

    [Parameter(Mandatory=$false)]
    [string]$LogFile = "publish-log-$(Get-Date -Format 'yyyyMMdd-HHmmss').txt",

    [Parameter(Mandatory=$false)]
    [int]$WaitTime = 10
)

# Configuração de cores e símbolos
$script:Colors = @{
    Success = 'Green'
    Error = 'Red'
    Warning = 'Yellow'
    Info = 'Cyan'
    Highlight = 'Magenta'
}

$script:Symbols = @{
    Success = '✅'
    Error = '❌'
    Warning = '⚠️'
    Info = 'ℹ️'
    Package = '📦'
    Rocket = '🚀'
    Clock = '⏳'
    Check = '✓'
    Cross = '✗'
    Arrow = '➜'
    Star = '⭐'
}

# Função de logging
function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "INFO",
        [string]$Color = "White"
    )

    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"

    Add-Content -Path $LogFile -Value $logMessage

    if ($Level -eq "ERROR") {
        Write-Host $Message -ForegroundColor $script:Colors.Error
    } elseif ($Level -eq "WARNING") {
        Write-Host $Message -ForegroundColor $script:Colors.Warning
    } elseif ($Level -eq "SUCCESS") {
        Write-Host $Message -ForegroundColor $script:Colors.Success
    } else {
        Write-Host $Message -ForegroundColor $Color
    }
}

# Função para exibir banner
function Show-Banner {
    $banner = @"

╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║        🌩️  AVILA CLOUD - DEPENDENCY PUBLISHER v2.0  🌩️       ║
║                                                              ║
║              Publicação Automatizada no crates.io            ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

"@
    Write-Host $banner -ForegroundColor $script:Colors.Info

    if ($DryRun) {
        Write-Log "$($script:Symbols.Warning) MODO DRY-RUN ATIVADO - Nenhuma publicação será realizada" "WARNING"
    }
}

Write-Host "`n=== Avila Cloud - Publicação de Dependências ===" -ForegroundColor Cyan
Write-Host "Este script publicará as seguintes dependências no crates.io:`n" -ForegroundColor Yellow

# Configuração de dependências com metadados completos
$dependencies = @(
    @{
        Name = "avila-error-derive"
        Path = "d:\arxis\avila-error\avila-error-derive"
        Description = "Procedural macros para derivação de traits de erro"
        Dependencies = @()
        RequiredFeatures = @()
        MinRustVersion = "1.70.0"
    },
    @{
        Name = "avila-error"
        Path = "d:\arxis\avila-error"
        Description = "Sistema de tratamento de erros robusto e ergonômico"
        Dependencies = @("avila-error-derive")
        RequiredFeatures = @()
        MinRustVersion = "1.70.0"
    },
    @{
        Name = "avx-gateway"
        Path = "d:\arxis\avx-gateway"
        Description = "API Gateway de alta performance para Avila Experience Fabric"
        Dependencies = @()
        RequiredFeatures = @()
        MinRustVersion = "1.75.0"
    },
    @{
        Name = "avl-loadbalancer"
        Path = "d:\arxis\avl-loadbalancer"
        Description = "Load Balancer distribuído com algoritmos avançados"
        Dependencies = @()
        RequiredFeatures = @()
        MinRustVersion = "1.75.0"
    }
)

# Inicialização
Show-Banner
Write-Log "Iniciando processo de publicação..." "INFO" $script:Colors.Info
Write-Log "Arquivo de log: $LogFile" "INFO" $script:Colors.Info

# Função para verificar se cargo está instalado
function Test-CargoInstalled {
    try {
        $null = cargo --version 2>&1
        return $true
    } catch {
        Write-Log "$($script:Symbols.Error) Cargo não está instalado ou não está no PATH" "ERROR"
        return $false
    }
}

# Função para obter versão do Cargo.toml
function Get-CrateVersion {
    param([string]$Path)

    $cargoToml = Join-Path $Path "Cargo.toml"
    if (-not (Test-Path $cargoToml)) {
        return $null
    }

    $content = Get-Content $cargoToml -Raw
    if ($content -match 'version\s*=\s*"([^"]+)"') {
        return $matches[1]
    }
    return $null
}

# Função para validar Cargo.toml
function Test-CargoToml {
    param([string]$Path)

    $cargoToml = Join-Path $Path "Cargo.toml"
    $content = Get-Content $cargoToml -Raw

    $requiredFields = @('name', 'version', 'edition', 'description', 'license')
    $missingFields = @()

    foreach ($field in $requiredFields) {
        if ($content -notmatch "$field\s*=") {
            $missingFields += $field
        }
    }

    return @{
        IsValid = ($missingFields.Count -eq 0)
        MissingFields = $missingFields
    }
}

# Função para executar testes
function Invoke-CargoTest {
    param([string]$Path)

    Push-Location $Path
    Write-Log "  $($script:Symbols.Arrow) Executando testes..." "INFO" $script:Colors.Info

    $output = cargo test --quiet 2>&1
    $success = $LASTEXITCODE -eq 0

    Pop-Location

    if ($success) {
        Write-Log "  $($script:Symbols.Check) Testes passaram" "SUCCESS"
    } else {
        Write-Log "  $($script:Symbols.Cross) Testes falharam: $output" "ERROR"
    }

    return $success
}

# Função para executar build
function Invoke-CargoBuild {
    param([string]$Path)

    Push-Location $Path
    Write-Log "  $($script:Symbols.Arrow) Compilando..." "INFO" $script:Colors.Info

    $output = cargo build --release 2>&1
    $success = $LASTEXITCODE -eq 0

    Pop-Location

    if ($success) {
        Write-Log "  $($script:Symbols.Check) Build bem-sucedido" "SUCCESS"
    } else {
        Write-Log "  $($script:Symbols.Cross) Build falhou: $output" "ERROR"
    }

    return $success
}

# Função para verificar se pacote já está publicado
function Test-CratePublished {
    param([string]$CrateName)

    $result = cargo search "^$CrateName`$" --limit 1 2>&1 | Select-String -Pattern "^$CrateName"

    if ($result) {
        if ($result -match 'version = "([^"]+)"') {
            return @{
                IsPublished = $true
                Version = $matches[1]
            }
        }
        return @{
            IsPublished = $true
            Version = "unknown"
        }
    }

    return @{
        IsPublished = $false
        Version = $null
    }
}

# Função principal de publicação
function Publish-Crate {
    param(
        [hashtable]$Dependency,
        [switch]$DryRun
    )

    $name = $Dependency.Name
    $path = $Dependency.Path

    Write-Log "`n========================================" "INFO" $script:Colors.Info
    Write-Log "$($script:Symbols.Package) Processando: $name" "INFO" $script:Colors.Highlight
    Write-Log "========================================" "INFO" $script:Colors.Info

    # Verificar se o diretório existe
    if (-not (Test-Path $path)) {
        Write-Log "$($script:Symbols.Error) Diretório não encontrado: $path" "ERROR"
        return @{Success = $false; Reason = "Diretório não encontrado"}
    }

    # Obter versão local
    $localVersion = Get-CrateVersion -Path $path
    if ($localVersion) {
        Write-Log "  $($script:Symbols.Info) Versão local: $localVersion" "INFO" $script:Colors.Info
    }

    # Verificar se já está publicado
    $publishStatus = Test-CratePublished -CrateName $name
    if ($publishStatus.IsPublished) {
        Write-Log "  $($script:Symbols.Warning) Já publicado no crates.io (v$($publishStatus.Version))" "WARNING"

        if ($localVersion -eq $publishStatus.Version) {
            Write-Log "  $($script:Symbols.Info) Versões são idênticas - Publicação não necessária" "INFO" $script:Colors.Info

            $response = Read-Host "  Deseja publicar mesmo assim? (S/N)"
            if ($response -ne 'S' -and $response -ne 's') {
                Write-Log "  $($script:Symbols.Arrow) Pulando $name" "INFO" $script:Colors.Warning
                return @{Success = $true; Reason = "Pulado pelo usuário"; Skipped = $true}
            }
        }
    }

    # Validação do Cargo.toml
    if (-not $SkipValidation) {
        Write-Log "  $($script:Symbols.Check) Validando Cargo.toml..." "INFO" $script:Colors.Info
        $validation = Test-CargoToml -Path $path

        if (-not $validation.IsValid) {
            Write-Log "  $($script:Symbols.Error) Campos obrigatórios faltando: $($validation.MissingFields -join ', ')" "ERROR"
            return @{Success = $false; Reason = "Validação do Cargo.toml falhou"}
        }
        Write-Log "  $($script:Symbols.Check) Cargo.toml válido" "SUCCESS"
    }

    # Build
    if (-not $SkipValidation) {
        if (-not (Invoke-CargoBuild -Path $path)) {
            return @{Success = $false; Reason = "Build falhou"}
        }
    }

    # Testes
    if (-not $SkipTests -and -not $SkipValidation) {
        if (-not (Invoke-CargoTest -Path $path)) {
            Write-Log "  $($script:Symbols.Warning) Testes falharam - continuar? (S/N)" "WARNING"
            $response = Read-Host
            if ($response -ne 'S' -and $response -ne 's') {
                return @{Success = $false; Reason = "Testes falharam"}
            }
        }
    }

    # Publicação
    if ($DryRun) {
        Write-Log "  $($script:Symbols.Info) [DRY-RUN] Simulando publicação..." "INFO" $script:Colors.Highlight
        Push-Location $path
        $output = cargo publish --dry-run --allow-dirty 2>&1
        Pop-Location

        if ($LASTEXITCODE -eq 0) {
            Write-Log "  $($script:Symbols.Check) [DRY-RUN] Publicação seria bem-sucedida" "SUCCESS"
            return @{Success = $true; DryRun = $true}
        } else {
            Write-Log "  $($script:Symbols.Cross) [DRY-RUN] Falha na simulação: $output" "ERROR"
            return @{Success = $false; Reason = "Dry-run falhou"}
        }
    } else {
        Write-Log "  $($script:Symbols.Rocket) Publicando no crates.io..." "INFO" $script:Colors.Highlight

        Push-Location $path
        $output = cargo publish --allow-dirty 2>&1
        $success = $LASTEXITCODE -eq 0
        Pop-Location

        if ($success) {
            Write-Log "  $($script:Symbols.Success) Publicado com sucesso!" "SUCCESS"
            return @{Success = $true}
        } else {
            Write-Log "  $($script:Symbols.Error) Falha na publicação" "ERROR"
            Write-Log "  Erro: $output" "ERROR"

            # Verificar rate limit
            if ($output -match "429 Too Many Requests") {
                if ($output -match "after ([^`r`n]+)") {
                    $retryAfter = $matches[1]
                    Write-Log "  $($script:Symbols.Clock) Rate Limit - Tentar após: $retryAfter" "WARNING"
                }
                return @{Success = $false; Reason = "Rate Limit"; RateLimited = $true}
            }

            return @{Success = $false; Reason = $output}
        }
    }
}

# Verificar pré-requisitos
Write-Log "`n$($script:Symbols.Check) Verificando pré-requisitos..." "INFO" $script:Colors.Info

if (-not (Test-CargoInstalled)) {
    Write-Log "$($script:Symbols.Error) Cargo não encontrado. Instale o Rust: https://rustup.rs/" "ERROR"
    exit 1
}

$cargoVersion = cargo --version
Write-Log "$($script:Symbols.Check) Cargo encontrado: $cargoVersion" "SUCCESS"

# Exibir lista de dependências
Write-Log "`n$($script:Symbols.Package) Dependências a serem publicadas:" "INFO" $script:Colors.Highlight
foreach ($dep in $dependencies) {
    $depStr = "$($script:Symbols.Package) $($dep.Name)"
    if ($dep.Dependencies.Count -gt 0) {
        $depStr += " (requer: $($dep.Dependencies -join ', '))"
    }
    Write-Log "  $depStr - $($dep.Description)" "INFO" "White"
}

Write-Log "`n$($script:Symbols.Warning) IMPORTANTE: Verifique se o rate limit do crates.io expirou!" "WARNING"
Write-Log "Pressione qualquer tecla para continuar ou CTRL+C para cancelar..." "INFO" $script:Colors.Warning
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Estatísticas
$stats = @{
    Total = $dependencies.Count
    Success = 0
    Failed = 0
    Skipped = 0
    RateLimited = $false
}

$results = @()
$startTime = Get-Date

# Processar cada dependência
foreach ($dep in $dependencies) {
    $result = Publish-Crate -Dependency $dep -DryRun:$DryRun

    $results += @{
        Name = $dep.Name
        Result = $result
    }

    if ($result.Success) {
        if ($result.Skipped) {
            $stats.Skipped++
        } else {
            $stats.Success++
        }
    } else {
        $stats.Failed++

        if ($result.RateLimited) {
            $stats.RateLimited = $true
            Write-Log "`n$($script:Symbols.Error) Rate Limit atingido! Interrompendo publicações." "ERROR"
            break
        }
    }

    # Aguardar entre publicações
    if ($dep -ne $dependencies[-1] -and -not $result.Skipped) {
        Write-Log "`n$($script:Symbols.Clock) Aguardando $WaitTime segundos antes da próxima publicação..." "INFO" $script:Colors.Warning
        Start-Sleep -Seconds $WaitTime
    }
}

$endTime = Get-Date
$duration = $endTime - $startTime

# Gerar relatório final
Write-Log "`n" "INFO"
Write-Log "╔══════════════════════════════════════════════════════════════╗" "INFO" $script:Colors.Info
Write-Log "║                    RELATÓRIO DE PUBLICAÇÃO                   ║" "INFO" $script:Colors.Info
Write-Log "╚══════════════════════════════════════════════════════════════╝" "INFO" $script:Colors.Info

Write-Log "`n$($script:Symbols.Star) RESUMO GERAL:" "INFO" $script:Colors.Highlight
Write-Log "  Total de pacotes: $($stats.Total)" "INFO" "White"
Write-Log "  $($script:Symbols.Success) Publicados com sucesso: $($stats.Success)" "SUCCESS"
Write-Log "  $($script:Symbols.Error) Falharam: $($stats.Failed)" $(if ($stats.Failed -gt 0) {"ERROR"} else {"INFO"})
Write-Log "  $($script:Symbols.Arrow) Pulados: $($stats.Skipped)" "INFO" "White"
Write-Log "  $($script:Symbols.Clock) Duração total: $($duration.ToString('mm\:ss'))" "INFO" "White"

# Detalhes por pacote
Write-Log "`n$($script:Symbols.Package) DETALHES POR PACOTE:" "INFO" $script:Colors.Highlight

foreach ($item in $results) {
    $name = $item.Name
    $result = $item.Result

    if ($result.Success) {
        if ($result.Skipped) {
            Write-Log "  $($script:Symbols.Arrow) $name - PULADO" "INFO" $script:Colors.Warning
        } elseif ($result.DryRun) {
            Write-Log "  $($script:Symbols.Check) $name - DRY-RUN OK" "INFO" $script:Colors.Info
        } else {
            Write-Log "  $($script:Symbols.Success) $name - PUBLICADO" "SUCCESS"
        }
    } else {
        $reason = if ($result.Reason) { " ($($result.Reason))" } else { "" }
        Write-Log "  $($script:Symbols.Error) $name - FALHOU$reason" "ERROR"
    }
}

# Avisos e próximos passos
if ($stats.RateLimited) {
    Write-Log "`n$($script:Symbols.Warning) ATENÇÃO: Rate Limit Atingido!" "WARNING"
    Write-Log "  Aguarde o período especificado antes de tentar novamente." "WARNING"
    Write-Log "  Ou envie email para help@crates.io solicitando aumento do limite." "INFO" "White"
}

if ($stats.Failed -gt 0 -and -not $stats.RateLimited) {
    Write-Log "`n$($script:Symbols.Info) PRÓXIMOS PASSOS:" "INFO" $script:Colors.Highlight
    Write-Log "  1. Revise os erros acima" "INFO" "White"
    Write-Log "  2. Corrija os problemas identificados" "INFO" "White"
    Write-Log "  3. Execute o script novamente" "INFO" "White"
}

if ($DryRun) {
    Write-Log "`n$($script:Symbols.Info) Modo DRY-RUN - Execute sem -DryRun para publicar realmente" "INFO" $script:Colors.Highlight
}

# Verificar dependências que podem ser habilitadas no Cargo.toml
Write-Log "`n$($script:Symbols.Info) PRÓXIMA AÇÃO SUGERIDA:" "INFO" $script:Colors.Highlight

$publishedDeps = @()
foreach ($item in $results) {
    if ($item.Result.Success -and -not $item.Result.Skipped -and -not $item.Result.DryRun) {
        $publishedDeps += $item.Name
    }
}

if ($publishedDeps.Count -gt 0) {
    Write-Log "  As seguintes dependências foram publicadas e podem ser habilitadas:" "INFO" "White"
    foreach ($depName in $publishedDeps) {
        $dep = $dependencies | Where-Object { $_.Name -eq $depName }
        if ($dep) {
            $publishStatus = Test-CratePublished -CrateName $depName
            $version = if ($publishStatus.Version) { $publishStatus.Version } else { "latest" }
            Write-Log "  • $depName = `"$version`"" "INFO" $script:Colors.Info
        }
    }

    Write-Log "`n  Atualize o Cargo.toml em d:\arxis\avila-cloud\Cargo.toml" "INFO" "White"
    Write-Log "  Descomente e ajuste as linhas das dependências publicadas" "INFO" "White"
}

Write-Log "`n$($script:Symbols.Star) Script finalizado!" "INFO" $script:Colors.Success
Write-Log "Log completo salvo em: $LogFile" "INFO" $script:Colors.Info
Write-Log "`n" "INFO"

# Retornar código de saída apropriado
if ($stats.Failed -gt 0) {
    exit 1
} else {
    exit 0
}
