# Synthesise each line of a JSONL corpus with the Japanese OneCore voices that
# ship with Windows (Ayumi, Haruka, Ichiro, Sayaka). Dev tooling only: the audio
# is synthetic, clearly labelled as such in EVIDENCE.md, and never committed.
#
#   powershell -ExecutionPolicy Bypass -File scripts\make_tts_corpus.ps1 eval\corpus\conversational.jsonl eval\data\tts\lines
param([string]$Corpus, [string]$OutDir)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force $OutDir | Out-Null
Add-Type -AssemblyName System.Runtime.WindowsRuntime
$null = [Windows.Media.SpeechSynthesis.SpeechSynthesizer, Windows.Media.SpeechSynthesis, ContentType = WindowsRuntime]
$null = [Windows.Storage.Streams.DataReader, Windows.Storage.Streams, ContentType = WindowsRuntime]
$asTask = ([System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {
    $_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1' })[0]
function Await($op, [Type]$t) { $task = $asTask.MakeGenericMethod($t).Invoke($null, @($op)); $task.Wait(-1) | Out-Null; $task.Result }

$voices = [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices | Where-Object { $_.Language -eq 'ja-JP' }
if (-not $voices) { throw 'No Japanese voices installed.' }
$synth = New-Object Windows.Media.SpeechSynthesis.SpeechSynthesizer
$i = 0
$meta = @()
foreach ($line in Get-Content -Encoding UTF8 $Corpus) {
    if (-not $line.Trim()) { continue }
    $item = $line | ConvertFrom-Json
    $voice = $voices[$i % $voices.Count]
    # every third line is fast speech, as streamers often talk quickly
    $rate = if ($i % 3 -eq 2) { '1.35' } else { '1.1' }
    $synth.Voice = $voice
    $text = [System.Security.SecurityElement]::Escape($item.ja)
    $ssml = "<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='ja-JP'><prosody rate='$rate'>$text</prosody></speak>"
    $stream = Await ($synth.SynthesizeSsmlToStreamAsync($ssml)) ([Windows.Media.SpeechSynthesis.SpeechSynthesisStream])
    $reader = New-Object Windows.Storage.Streams.DataReader($stream.GetInputStreamAt(0))
    $size = [uint32]$stream.Size
    $null = Await ($reader.LoadAsync($size)) ([uint32])
    $bytes = New-Object byte[] $size
    $reader.ReadBytes($bytes)
    $path = Join-Path $OutDir "$($item.id).wav"
    [IO.File]::WriteAllBytes($path, $bytes)
    $meta += [pscustomobject]@{ id = $item.id; voice = $voice.DisplayName; rate = $rate }
    $i++
}
$meta | ConvertTo-Json | Set-Content -Encoding UTF8 (Join-Path $OutDir 'voices.json')
Write-Host "synthesised $i lines with $($voices.Count) voices into $OutDir"
