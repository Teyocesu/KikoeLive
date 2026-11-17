# Build a Windows x64 test artifact of KikoeLive (unsigned).
#   powershell -ExecutionPolicy Bypass -File scripts\package-windows.ps1 [-Cpu] [-Bundle]
# Output: dist\windows\KikoeLive\ (portable folder) and, with -Bundle, the NSIS installer
# under target\release\bundle\nsis\.
param([switch]$Cpu, [switch]$Bundle)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
. "$root\scripts\dev-env.ps1"
$ErrorActionPreference = 'Continue'

$features = if ($Cpu) { @() } else { @('--features', 'cuda') }
& cmd /c "cargo build --release -p kikoe-mt $($features -join ' ') 2>&1"
if ($LASTEXITCODE -ne 0) { throw 'kikoe-mt build failed' }

$triple = 'x86_64-pc-windows-msvc'
New-Item -ItemType Directory -Force "$root\app\src-tauri\binaries" | Out-Null
Copy-Item "$root\target\release\kikoe-mt.exe" "$root\app\src-tauri\binaries\kikoe-mt-$triple.exe" -Force

Push-Location "$root\app"
$tauriArgs = @('tauri', 'build')
if (-not $Cpu) { $tauriArgs += @('--features', 'cuda') }
if (-not $Bundle) { $tauriArgs += '--no-bundle' }
& cmd /c "npx $($tauriArgs -join ' ') 2>&1"
$code = $LASTEXITCODE
Pop-Location
if ($code -ne 0) { throw 'tauri build failed' }

$out = "$root\dist\windows\KikoeLive"
New-Item -ItemType Directory -Force $out | Out-Null
Copy-Item "$root\target\release\kikoe-live.exe" "$out\KikoeLive.exe" -Force
Copy-Item "$root\target\release\kikoe-mt.exe" "$out\kikoe-mt.exe" -Force
if (-not $Cpu -and $env:CUDA_PATH) {
    # CUDA runtime libraries, redistributable under the NVIDIA CUDA EULA (Attachment A).
    foreach ($dll in 'cudart64_12.dll', 'cublas64_12.dll', 'cublasLt64_12.dll') {
        Copy-Item (Join-Path $env:CUDA_PATH "bin\$dll") $out -Force
    }
}
Copy-Item "$root\THIRD-PARTY-NOTICES.md" $out -Force
Get-ChildItem $out | Select-Object Name, Length | Format-Table -AutoSize
Write-Host "Portable build: $out (unsigned test artifact)"
