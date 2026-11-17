# Sample CPU / memory of KikoeLive processes and GPU usage once per second.
#   powershell -File scripts\sample-resources.ps1 -Names kikoe-eval,kikoe-mt -Seconds 150 -Out eval\results\res.csv
# CPU% is per-process share of one logical core x100 (so 200 = two cores busy).
param([string[]]$Names = @('KikoeLive', 'kikoe-live', 'kikoe-eval', 'kikoe-mt'), [int]$Seconds = 120, [string]$Out = 'resources.csv')
$cores = [Environment]::ProcessorCount
$prev = @{}
$rows = New-Object System.Collections.Generic.List[string]
$rows.Add('t_s,process,pid,cpu_pct,working_set_mb,private_mb,gpu_util_pct,gpu_mem_used_mb,sys_commit_mb')
$t0 = Get-Date
for ($i = 0; $i -lt $Seconds; $i++) {
    $gpu = (& nvidia-smi --query-gpu=utilization.gpu,memory.used --format=csv,noheader,nounits 2>$null) -split ','
    $gu = if ($gpu.Count -ge 2) { $gpu[0].Trim() } else { '' }
    $gm = if ($gpu.Count -ge 2) { $gpu[1].Trim() } else { '' }
    $os = Get-CimInstance Win32_OperatingSystem
    $commit = [math]::Round(($os.TotalVirtualMemorySize - $os.FreeVirtualMemory) / 1024)
    $t = [math]::Round(((Get-Date) - $t0).TotalSeconds, 1)
    foreach ($p in Get-Process -Name $Names -ErrorAction SilentlyContinue) {
        $key = $p.Id
        $cpu = ''
        if ($prev.ContainsKey($key)) {
            $dt = ((Get-Date) - $prev[$key].At).TotalSeconds
            if ($dt -gt 0) { $cpu = [math]::Round(100 * ($p.TotalProcessorTime.TotalSeconds - $prev[$key].Cpu) / $dt, 0) }
        }
        $prev[$key] = @{ Cpu = $p.TotalProcessorTime.TotalSeconds; At = Get-Date }
        $rows.Add("$t,$($p.ProcessName),$($p.Id),$cpu,$([math]::Round($p.WorkingSet64/1MB)),$([math]::Round($p.PrivateMemorySize64/1MB)),$gu,$gm,$commit")
    }
    Start-Sleep -Milliseconds 1000
}
$rows | Set-Content -Encoding UTF8 $Out
Write-Host "wrote $Out ($($rows.Count - 1) samples, $cores logical cores)"
