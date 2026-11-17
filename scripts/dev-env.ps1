# Sets up the build environment for KikoeLive on Windows (dot-source it):
#   . .\scripts\dev-env.ps1
# Requirements: Rust (rustup, MSVC host), VS 2022 Build Tools with the C++
# workload (provides MSVC + CMake), and optionally CUDA Toolkit 12.x for GPU builds.
# Nothing here changes system configuration; it only edits this shell's variables.

$__kikoePrevEap = $ErrorActionPreference
$ErrorActionPreference = 'Stop'

$cargoBin = Join-Path $env:USERPROFILE '.cargo\bin'
if (Test-Path $cargoBin) { $env:PATH = "$cargoBin;$env:PATH" }

$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) { throw 'Visual Studio Build Tools not found (vswhere.exe missing).' }
$vs = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $vs) { throw 'No Visual Studio installation with the C++ toolchain found.' }
$cmakeBin = Join-Path $vs 'Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin'
if (Test-Path $cmakeBin) { $env:PATH = "$cmakeBin;$env:PATH" }
# bindgen (llama-cpp-sys-2) needs libclang: VS component Microsoft.VisualStudio.Component.VC.Llvm.Clang
$llvmBin = Join-Path $vs 'VC\Tools\Llvm\x64\bin'
if (Test-Path (Join-Path $llvmBin 'libclang.dll')) { $env:LIBCLANG_PATH = $llvmBin }
else { Write-Host 'libclang not found: add the VS "C++ Clang Compiler for Windows" component.' }

$cudaRoot = 'C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA'
$cuda = Get-ChildItem $cudaRoot -Directory -ErrorAction SilentlyContinue | Where-Object { $_.Name -like 'v12*' } | Sort-Object Name -Descending | Select-Object -First 1
if ($cuda) {
    $env:CUDA_PATH = $cuda.FullName
    # MSBuild's CUDA integration resolves the toolkit through CUDA_PATH_Vxx_y
    $verVar = 'CUDA_PATH_V' + ($cuda.Name.TrimStart('v') -replace '\.', '_')
    Set-Item -Path "env:$verVar" -Value $cuda.FullName
    $env:PATH = "$($cuda.FullName)\bin;$env:PATH"
    # Compile CUDA kernels only for the GPU generations we target. 89 = Ada (RTX 40xx).
    # Override with e.g. $env:CUDAARCHS = '75;86;89' for a broader test build.
    if (-not $env:CUDAARCHS) { $env:CUDAARCHS = '89' }
    Write-Host "CUDA: $($cuda.FullName) (archs $env:CUDAARCHS)"
} else {
    Write-Host 'CUDA toolkit not found: build without the cuda feature (CPU only).'
}
Write-Host "MSVC/CMake: $vs"
$ErrorActionPreference = $__kikoePrevEap
