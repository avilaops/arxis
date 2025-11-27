param(
    [string]$ConfigPath = "$PSScriptRoot\config.json",
    [switch]$Once,
    [int]$IntervalMs
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if (-not (Test-Path -LiteralPath $ConfigPath)) {
    throw "Arquivo de configuração não encontrado: $ConfigPath"
}

try {
    $configContent = Get-Content -LiteralPath $ConfigPath -Raw
    $config = $configContent | ConvertFrom-Json
} catch {
    throw "Falha ao carregar '$ConfigPath': $($_.Exception.Message)"
}

function Get-ConfigProperty {
    param([string]$Name)
    return $config.PSObject.Properties[$Name]
}

if (-not $IntervalMs) {
    $pollingProp = Get-ConfigProperty -Name 'pollingIntervalMs'
    if ($pollingProp -and $null -ne $pollingProp.Value) {
        $IntervalMs = [int]$pollingProp.Value
    } else {
        $IntervalMs = 750
    }
}

Add-Type -AssemblyName System.Windows.Forms

$nativeCode = @"
using System;
using System.Runtime.InteropServices;

public static class VsCodeWindowNative
{
    [DllImport("user32.dll")]
    public static extern bool SetWindowPos(IntPtr hWnd, IntPtr hWndInsertAfter, int X, int Y, int cx, int cy, uint uFlags);

    [DllImport("user32.dll")]
    public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);

    [DllImport("user32.dll")]
    public static extern bool IsIconic(IntPtr hWnd);

    public static readonly IntPtr HWND_TOP = new IntPtr(0);
    public const uint SWP_NOZORDER = 0x0004;
    public const uint SWP_NOACTIVATE = 0x0010;
    public const uint SWP_SHOWWINDOW = 0x0040;
    public const int SW_RESTORE = 9;
}
"@

try {
    Add-Type -TypeDefinition $nativeCode -Language CSharp -ErrorAction Stop | Out-Null
} catch [System.Exception] {
    if (-not ($_ -match 'type name vsCodeWindowNative has already been defined')) {
        throw
    }
}

function Get-Screen {
    param([int]$Index)

    $screens = [System.Windows.Forms.Screen]::AllScreens
    if ($screens.Length -eq 0) {
        throw 'Nenhum monitor disponível foi detectado.'
    }

    if ($Index -lt 0 -or $Index -ge $screens.Length) {
        Write-Warning "Monitor $Index não encontrado. Usando monitor principal."
        return [System.Windows.Forms.Screen]::PrimaryScreen
    }

    return $screens[$Index]
}

function New-VsCodeLayoutSlots {
    param(
        [pscustomobject]$Layout,
        [System.Windows.Forms.Screen]$Screen
    )

    $layoutProps = $Layout.PSObject.Properties.Name

    $columns = if ($layoutProps -contains 'columns' -and $null -ne $Layout.columns) { [int]$Layout.columns } else { 2 }
    $rows = if ($layoutProps -contains 'rows' -and $null -ne $Layout.rows) { [int]$Layout.rows } else { 2 }
    $padding = if ($layoutProps -contains 'padding' -and $null -ne $Layout.padding) { [int]$Layout.padding } else { 12 }

    if ($columns -le 0 -or $rows -le 0) {
        throw 'Valores de rows/columns no layout precisam ser maiores que zero.'
    }

    $work = $Screen.WorkingArea
    $slotList = New-Object System.Collections.Generic.List[object]

    $totalHorizontalPadding = $padding * ($columns - 1)
    $totalVerticalPadding = $padding * ($rows - 1)

    $baseWidth = [math]::Floor(($work.Width - $totalHorizontalPadding) / $columns)
    $baseHeight = [math]::Floor(($work.Height - $totalVerticalPadding) / $rows)

    for ($row = 0; $row -lt $rows; $row++) {
        for ($column = 0; $column -lt $columns; $column++) {
            $x = $work.Left + $column * ($baseWidth + $padding)
            $y = $work.Top + $row * ($baseHeight + $padding)

            # Ajusta última coluna/linha para cobrir qualquer sobra de pixels
            if ($column -eq $columns - 1) {
                $width = $work.Right - $x
            } else {
                $width = $baseWidth
            }

            if ($row -eq $rows - 1) {
                $height = $work.Bottom - $y
            } else {
                $height = $baseHeight
            }

            $slotList.Add([pscustomobject]@{
                Index = $slotList.Count
                X = [int]$x
                Y = [int]$y
                Width = [int]$width
                Height = [int]$height
                Column = $column
                Row = $row
            }) | Out-Null
        }
    }

    return ,$slotList.ToArray()
}

function New-VsCodeExplicitSlots {
    param(
        [System.Array]$Slots,
        [System.Windows.Forms.Screen]$Screen
    )

    $work = $Screen.WorkingArea
    $result = New-Object System.Collections.Generic.List[object]

    for ($i = 0; $i -lt $Slots.Count; $i++) {
        $slot = $Slots[$i]

        $slotProps = $slot.PSObject.Properties.Name
        $rawX = if ($slotProps -contains 'x') { [double]$slot.x } else { 0.0 }
        $rawY = if ($slotProps -contains 'y') { [double]$slot.y } else { 0.0 }
        $rawWidth = if ($slotProps -contains 'width') { [double]$slot.width } else { 0.5 }
        $rawHeight = if ($slotProps -contains 'height') { [double]$slot.height } else { 0.5 }

        $x = if ($rawX -le 1) { $work.Left + [math]::Round($work.Width * $rawX) } else { $work.Left + [int]$rawX }
        $y = if ($rawY -le 1) { $work.Top + [math]::Round($work.Height * $rawY) } else { $work.Top + [int]$rawY }
        $width = if ($rawWidth -le 1) { [math]::Round($work.Width * $rawWidth) } else { [int]$rawWidth }
        $height = if ($rawHeight -le 1) { [math]::Round($work.Height * $rawHeight) } else { [int]$rawHeight }

        $result.Add([pscustomobject]@{
            Index = $result.Count
            X = [int]$x
            Y = [int]$y
            Width = [int]$width
            Height = [int]$height
            Column = if ($slotProps -contains 'column') { $slot.column } else { $null }
            Row = if ($slotProps -contains 'row') { $slot.row } else { $null }
        }) | Out-Null
    }

    return ,$result.ToArray()
}

function Get-ConfiguredSlots {
    $monitorProp = Get-ConfigProperty -Name 'monitor'
    $monitorIndex = if ($monitorProp -and $null -ne $monitorProp.Value) { [int]$monitorProp.Value } else { 0 }
    $screen = Get-Screen -Index $monitorIndex

    $slotsProp = Get-ConfigProperty -Name 'slots'
    if ($slotsProp -and $null -ne $slotsProp.Value) {
        return New-VsCodeExplicitSlots -Slots $slotsProp.Value -Screen $screen
    }

    $layoutProp = Get-ConfigProperty -Name 'layout'
    $layout = if ($layoutProp -and $null -ne $layoutProp.Value) {
        $layoutProp.Value
    } else {
        [pscustomobject]@{ columns = 2; rows = 2; padding = 12 }
    }

    return New-VsCodeLayoutSlots -Layout $layout -Screen $screen
}

$script:SlotDefinitions = Get-ConfiguredSlots
if (-not $script:SlotDefinitions -or $script:SlotDefinitions.Length -eq 0) {
    throw 'Nenhum slot foi configurado. Verifique config.json.'
}

$defaultOrder = for ($i = 0; $i -lt $script:SlotDefinitions.Length; $i++) { $i }
$slotOrderProp = Get-ConfigProperty -Name 'slotOrder'
$slotOrderRaw = if ($slotOrderProp) { $slotOrderProp.Value } else { $null }
$slotOrderArray = @()
if ($slotOrderRaw -ne $null) {
    if ($slotOrderRaw -is [System.Collections.IEnumerable] -and -not ($slotOrderRaw -is [string])) {
        $slotOrderArray = @($slotOrderRaw)
    } else {
        $slotOrderArray = @($slotOrderRaw)
    }
}

$slotOrderFiltered = $slotOrderArray | Where-Object { $_ -ne $null }
$script:SlotOrder = if ($slotOrderFiltered.Count -gt 0) {
    $slotOrderFiltered | ForEach-Object { [int]$_ }
} else {
    $defaultOrder
}

$script:SlotOrder = $script:SlotOrder | Where-Object { $_ -ge 0 -and $_ -lt $script:SlotDefinitions.Length }
if ($script:SlotOrder.Count -eq 0) {
    $script:SlotOrder = $defaultOrder
}

$script:NextRoundRobinIndex = -1
$script:Assignments = New-Object 'System.Collections.Generic.Dictionary[int, hashtable]'
$processNamesProp = Get-ConfigProperty -Name 'processNames'
$processNamesRaw = if ($processNamesProp) { $processNamesProp.Value } else { $null }

if ($processNamesRaw -eq $null) {
    $script:ProcessNames = @('Code')
} else {
    if ($processNamesRaw -is [string]) {
        $script:ProcessNames = @($processNamesRaw)
    } else {
        $processArray = @($processNamesRaw) | Where-Object { $_ }
        if ($processArray.Count -eq 0) {
            $script:ProcessNames = @('Code')
        } else {
            $script:ProcessNames = $processArray
        }
    }
}

function Get-NextSlotIndex {
    $occupied = @($script:Assignments.Values | ForEach-Object { $_.Slot })
    foreach ($slotIndex in $script:SlotOrder) {
        if ($occupied -notcontains $slotIndex) {
            return $slotIndex
        }
    }

    if ($script:SlotOrder.Count -eq 0) {
        return 0
    }

    $script:NextRoundRobinIndex = ($script:NextRoundRobinIndex + 1) % $script:SlotOrder.Count
    return $script:SlotOrder[$script:NextRoundRobinIndex]
}

function Set-VsCodeWindowPosition {
    param(
        [IntPtr]$Handle,
        [pscustomobject]$Slot
    )

    if ($Handle -eq [IntPtr]::Zero) {
        return
    }

    if ([VsCodeWindowNative]::IsIconic($Handle)) {
        [VsCodeWindowNative]::ShowWindow($Handle, [VsCodeWindowNative]::SW_RESTORE) | Out-Null
        Start-Sleep -Milliseconds 120
    } else {
        [VsCodeWindowNative]::ShowWindow($Handle, [VsCodeWindowNative]::SW_RESTORE) | Out-Null
    }

    [void][VsCodeWindowNative]::SetWindowPos(
        $Handle,
        [VsCodeWindowNative]::HWND_TOP,
        $Slot.X,
        $Slot.Y,
        $Slot.Width,
        $Slot.Height,
        [VsCodeWindowNative]::SWP_NOZORDER -bor [VsCodeWindowNative]::SWP_NOACTIVATE -bor [VsCodeWindowNative]::SWP_SHOWWINDOW
    )
}

function Wait-ForMainWindowHandle {
    param(
        [System.Diagnostics.Process]$Process,
        [int]$TimeoutMs = 15000
    )

    $elapsed = 0
    $increment = 200
    while ($elapsed -lt $TimeoutMs) {
        try {
            $Process.Refresh()
            $handle = $Process.MainWindowHandle
            if ($handle -and $handle -ne [IntPtr]::Zero) {
                return $handle
            }
        } catch {
            return [IntPtr]::Zero
        }
        Start-Sleep -Milliseconds $increment
        $elapsed += $increment
    }

    return [IntPtr]::Zero
}

function Set-VsCodeWindowAssignment {
    param([System.Diagnostics.Process]$Process)

    if ($script:Assignments.ContainsKey($Process.Id)) {
        return
    }

    $handle = Wait-ForMainWindowHandle -Process $Process
    if ($handle -eq [IntPtr]::Zero) {
        Write-Verbose "Não foi possível obter handle principal do processo $($Process.Id) ($($Process.ProcessName))."
        return
    }

    $slotIndex = Get-NextSlotIndex
    $slot = $script:SlotDefinitions[$slotIndex]

    Set-VsCodeWindowPosition -Handle $handle -Slot $slot

    $script:Assignments.Add($Process.Id, @{
        Slot = $slotIndex
        Handle = $handle
        AssignedAt = Get-Date
        ProcessName = $Process.ProcessName
    })

    Write-Verbose "Janela PID $($Process.Id) atribuída ao slot $slotIndex."
}

function Get-TargetProcesses {
    $list = New-Object System.Collections.Generic.List[System.Diagnostics.Process]
    foreach ($name in $script:ProcessNames) {
        try {
            $procs = Get-Process -Name $name -ErrorAction Stop
            foreach ($proc in $procs) {
                $null = $list.Add($proc)
            }
        } catch [System.Exception] {
            continue
        }
    }

    return $list.ToArray() | Sort-Object Id
}

function Remove-ClosedAssignments {
    param([int[]]$ActiveProcessIds)

    foreach ($processId in @($script:Assignments.Keys)) {
        if ($ActiveProcessIds -notcontains $processId) {
            $null = $script:Assignments.Remove($processId)
            Write-Verbose "Removendo atribuição para PID $processId (janela encerrada)."
        }
    }
}

$slotSummary = ($script:SlotDefinitions | ForEach-Object {
    "Slot $($_.Index): $($_.Width)x$($_.Height) em ($($_.X),$($_.Y))"
}) -join '; '
Write-Verbose "Slots configurados: $slotSummary"

function Update-AllWindows {
    $processes = Get-TargetProcesses
    $activeIds = @($processes | ForEach-Object { $_.Id })

    Remove-ClosedAssignments -ActiveProcessIds $activeIds

    foreach ($process in $processes) {
        Set-VsCodeWindowAssignment -Process $process
    }
}

Update-AllWindows

if ($Once) {
    return
}

Write-Verbose "Monitorando novas janelas do VS Code. Pressione Ctrl+C para finalizar."

while ($true) {
    Start-Sleep -Milliseconds $IntervalMs
    Update-AllWindows
}
