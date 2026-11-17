# Reproduce the measurements in EVIDENCE.md (Windows). Requires the release
# builds of kikoe-mt and kikoe-eval, the models, and the evaluation data:
#   python -I scripts/prepare_fleurs.py eval/data/fleurs 40
#   powershell -File scripts\make_tts_corpus.ps1 eval\corpus\conversational.jsonl eval\data\tts\lines
#   python -I scripts/build_tts_mix.py eval/corpus/conversational.jsonl eval/data/tts/lines eval/data/tts/clean none
#   python -I scripts/build_tts_mix.py eval/corpus/conversational.jsonl eval/data/tts/lines eval/data/tts/bgm 10
#
#   powershell -ExecutionPolicy Bypass -File scripts\run-eval.ps1 -Stage mt|asr|pipeline|live|all
param(
    [ValidateSet('mt', 'asr', 'pipeline', 'live', 'all')] [string]$Stage = 'all',
    [string]$Asr = "$env:LOCALAPPDATA\KikoeLive\models\ggml-large-v3-turbo-q5_0.bin",
    [string]$AsrSmall = "$env:LOCALAPPDATA\KikoeLive\models\ggml-small-q5_1.bin",
    [string]$Vad = "$env:LOCALAPPDATA\KikoeLive\models\ggml-silero-v6.2.0.bin",
    [string]$Mt = "$env:LOCALAPPDATA\KikoeLive\models\Qwen3.5-4B-Q4_K_M.gguf",
    [string[]]$MtCandidates = @("$env:LOCALAPPDATA\KikoeLive\models\Qwen3-4B-Q4_K_M.gguf", "$env:LOCALAPPDATA\KikoeLive\models\Qwen3.5-4B-Q4_K_M.gguf")
)
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
if ($env:CUDA_PATH) { $env:PATH = "$env:CUDA_PATH\bin;$env:PATH" }
$eval = "$root\target\release\kikoe-eval.exe"
$out = "$root\eval\results"
New-Item -ItemType Directory -Force $out | Out-Null
function Run([string[]]$a) { Write-Host ">> kikoe-eval $($a -join ' ')"; & $eval @a; if ($LASTEXITCODE -ne 0) { throw "kikoe-eval failed: $($a -join ' ')" } }
function Tag([string]$p) { [IO.Path]::GetFileNameWithoutExtension($p).ToLower() }

if ($Stage -in 'mt', 'all') {
    foreach ($m in $MtCandidates) {
        Run @('mt', '--corpus', 'eval/corpus/conversational.jsonl', '--model', $m, '--lang', 'both', '--out', "$out/mt-conversational-$(Tag $m)")
        Run @('mt', '--corpus', 'eval/corpus/fleurs_dev_subset.jsonl', '--model', $m, '--lang', 'both', '--out', "$out/mt-fleurs-$(Tag $m)")
    }
}
if ($Stage -in 'asr', 'all') {
    Run @('asr', '--manifest', 'eval/data/fleurs/manifest.jsonl', '--asr', $Asr, '--out', "$out/asr-fleurs-turbo")
    Run @('asr', '--manifest', 'eval/data/fleurs/manifest.jsonl', '--asr', $AsrSmall, '--out', "$out/asr-fleurs-small")
    Run @('asr', '--manifest', 'eval/data/tts/lines_manifest.jsonl', '--asr', $Asr, '--out', "$out/asr-tts-turbo")
}
if ($Stage -in 'pipeline', 'all') {
    $common = @('--asr', $Asr, '--vad', $Vad, '--mt', $Mt)
    Run (@('pipeline', '--wav', 'eval/data/tts/clean.wav', '--truth', 'eval/data/tts/clean.truth.jsonl', '--lang', 'en', '--out', "$out/pipeline-clean-en") + $common)
    Run (@('pipeline', '--wav', 'eval/data/tts/clean.wav', '--truth', 'eval/data/tts/clean.truth.jsonl', '--lang', 'es', '--out', "$out/pipeline-clean-es") + $common)
    Run (@('pipeline', '--wav', 'eval/data/tts/bgm.wav', '--truth', 'eval/data/tts/bgm.truth.jsonl', '--lang', 'en', '--out', "$out/pipeline-bgm-en") + $common)
    Run (@('pipeline', '--wav', 'eval/data/tts/bgm.wav', '--truth', 'eval/data/tts/bgm.truth.jsonl', '--lang', 'es', '--out', "$out/pipeline-bgm-es") + $common)
    Run (@('pipeline', '--wav', 'eval/data/tts/clean.wav', '--truth', 'eval/data/tts/clean.truth.jsonl', '--lang', 'en', '--switch', '40:es', '--switch', '85:en', '--out', "$out/pipeline-clean-switch") + $common)
    Run (@('pipeline', '--wav', 'eval/data/tts/bgm.wav', '--truth', 'eval/data/tts/bgm.truth.jsonl', '--lang', 'en', '--vad-kind', 'energy', '--out', "$out/pipeline-bgm-en-energyvad") + $common)
}
if ($Stage -in 'live', 'all') {
    # Plays the clean test stream through the default output device (audible!) and captures it via WASAPI loopback.
    $common = @('--asr', $Asr, '--vad', $Vad, '--mt', $Mt)
    Run (@('live', '--seconds', '140', '--play', 'eval/data/tts/clean.wav', '--truth', 'eval/data/tts/clean.truth.jsonl', '--lang', 'en', '--switch', '60:es', '--out', "$out/live-wasapi-switch") + $common)
}
