# Sample CPU / memory of KikoeLive processes and GPU usage once per second.
#   powershell -File scripts\sample-resources.ps1 -Names kikoe-eval,kikoe-mt -Seconds 150 -Out eval\results\res.csv
# Rows are appended as they are taken, so stopping the sampler early keeps the data.
# cpu_pct is the share of one logical core x100 (200 = two cores busy).
param([string[]]$Names = @('KikoeLive', 'kikoe-live', 'kikoe-eval', 'kikoe-mt'), [int]$Seconds = 120, [string]$Out = 'resources.csv')
# accept both -Names a,b and -Names 'a,b' (Start-Process passes one string)
$Names = @($Names | ForEach-Object { $_ -split ',' } | ForEach-Object { $_.Trim() } | Where-Object { $_ })
$prev = @{}
Set-Content -Encoding UTF8 $Out 'unix_ms,process,pid,cpu_pct,working_set_mb,private_mb,gpu_util_pct,gpu_mem_used_mb,sys_commit_mb'
for ($i = 0; $i -lt $Seconds; $i++) {
    $gpu = (& nvidia-smi --query-gpu=utilization.gpu,memory.used --format=csv,noheader,nounits 2>$null) -split ','
    $gu = if ($gpu.Count -ge 2) { $gpu[0].Trim() } else { '' }
    $gm = if ($gpu.Count -ge 2) { $gpu[1].Trim() } else { '' }
    $os = Get-CimInstance Win32_OperatingSystem
    $commit = [math]::Round(($os.TotalVirtualMemorySize - $os.FreeVirtualMemory) / 1024)
    $now = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
    $lines = @()
    foreach ($p in Get-Process -Name $Names -ErrorAction SilentlyContinue) {
        $cpu = ''
        if ($prev.ContainsKey($p.Id)) {
            $dt = ($now - $prev[$p.Id].At) / 1000.0
            if ($dt -gt 0) { $cpu = [math]::Round(100 * ($p.TotalProcessorTime.TotalSeconds - $prev[$p.Id].Cpu) / $dt, 0) }
        }
        $prev[$p.Id] = @{ Cpu = $p.TotalProcessorTime.TotalSeconds; At = $now }
        $lines += "$now,$($p.ProcessName),$($p.Id),$cpu,$([math]::Round($p.WorkingSet64/1MB)),$([math]::Round($p.PrivateMemorySize64/1MB)),$gu,$gm,$commit"
    }
    if ($lines.Count -gt 0) { Add-Content -Encoding UTF8 $Out $lines }
    Start-Sleep -Milliseconds 1000
}
