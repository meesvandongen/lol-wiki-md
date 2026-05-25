[CmdletBinding()]
param(
    [ValidateSet('workspace', 'release')]
    [string]$ExporterSource = 'workspace',

    [switch]$SkipExport,

    [string]$WikiRoot,

    [string]$GeneratedDir = (Join-Path $PSScriptRoot '..\generated'),

    [switch]$SkipDocsInstall
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$RootDir = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
if ([System.IO.Path]::IsPathRooted($GeneratedDir)) {
    $GeneratedDir = [System.IO.Path]::GetFullPath($GeneratedDir)
}
else {
    $GeneratedDir = [System.IO.Path]::GetFullPath((Join-Path $RootDir $GeneratedDir))
}
$ExportOutDir = Join-Path $GeneratedDir 'wiki-export\out'
$ExportMetaDir = Join-Path $GeneratedDir 'wiki-export\meta'
$MarkdownRoot = Join-Path $GeneratedDir 'markdown'
$DocsContentDir = Join-Path $RootDir 'apps\docs\docs'
$DocsAppDir = Join-Path $RootDir 'apps\docs'
$DownloadCacheDir = Join-Path $RootDir '.cache\apps\exporter'

$RootCategory = if ($env:ROOT_CATEGORY) { $env:ROOT_CATEGORY } else { 'Category:Browse' }
$MaxDepth = if ($env:MAX_DEPTH) { $env:MAX_DEPTH } else { '5' }
$BatchSize = if ($env:BATCH_SIZE) { $env:BATCH_SIZE } else { '200' }
$TraversalDelayMs = if ($env:TRAVERSAL_DELAY_MS) { $env:TRAVERSAL_DELAY_MS } else { '50' }
$BatchDelayMs = if ($env:BATCH_DELAY_MS) { $env:BATCH_DELAY_MS } else { '400' }
$Concurrency = if ($env:CONCURRENCY) { $env:CONCURRENCY } else { '3' }

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory = $true)]
        [string]$FilePath,

        [string[]]$ArgumentList = @()
    )

    & $FilePath @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed with exit code $LASTEXITCODE: $FilePath $($ArgumentList -join ' ')"
    }
}

function Resolve-ReleaseAssetName {
    return 'x86_64-pc-windows-gnu.exe'
}

function Download-ReleaseExporter {
    $releaseApi = 'https://gitlab.com/api/v4/projects/lol-math%2Flol-wiki-export/releases/permalink/latest'
    $assetSuffix = Resolve-ReleaseAssetName

    Write-Host 'Downloading latest lol-wiki-export release metadata...'
    $release = Invoke-RestMethod -Uri $releaseApi
    $tagName = $release.tag_name
    $assetName = "lol-wiki-export-$tagName-$assetSuffix"
    $assetLink = $release.assets.links | Where-Object { $_.name -eq $assetName } | Select-Object -First 1

    if ($null -eq $assetLink) {
        throw "Could not find release asset: $assetName"
    }

    $binDir = Join-Path $DownloadCacheDir $tagName
    $binPath = Join-Path $binDir $assetName
    New-Item -ItemType Directory -Force -Path $binDir | Out-Null

    if (-not (Test-Path $binPath)) {
        Write-Host "Downloading $assetName..."
        Invoke-WebRequest -Uri $assetLink.url -OutFile $binPath
    }

    return $binPath
}

function Build-WorkspaceTools {
    Write-Host 'Building vendored exporter and converter...'
    Invoke-CheckedCommand -FilePath 'cargo' -ArgumentList @('build', '--release', '-p', 'lol-wiki-export', '-p', 'lol_wiki_md', '--bin', 'convert')
}

function Run-Export {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ExporterBin
    )

    Write-Host "Running wiki export into $ExportOutDir"
    Invoke-CheckedCommand -FilePath $ExporterBin -ArgumentList @(
        '--root-category', $RootCategory,
        '--max-depth', $MaxDepth,
        '--out-dir', $ExportOutDir,
        '--meta-dir', $ExportMetaDir,
        '--batch-size', $BatchSize,
        '--traversal-delay-ms', $TraversalDelayMs,
        '--batch-delay-ms', $BatchDelayMs,
        '--concurrency', $Concurrency
    )
}

function Run-Convert {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ConverterBin,

        [Parameter(Mandatory = $true)]
        [string]$SourceRoot
    )

    $targets = @(
        @{ Label = 'champions'; Args = @('--all-champions') },
        @{ Label = 'items'; Args = @('--all-items') },
        @{ Label = 'runes'; Args = @('--all-runes') }
    )

    foreach ($target in $targets) {
        $outputDir = Join-Path $MarkdownRoot $target.Label
        New-Item -ItemType Directory -Force -Path $outputDir | Out-Null
        Write-Host "Converting $($target.Label) into $outputDir"
        Invoke-CheckedCommand -FilePath $ConverterBin -ArgumentList @('--wiki-root', $SourceRoot, '--output', $outputDir) + $target.Args
    }
}

function Install-DocsDependencies {
    if ($SkipDocsInstall) {
        Write-Host 'Skipping docs dependency install'
        return
    }

    Write-Host 'Installing docs app dependencies...'
    if (Test-Path (Join-Path $RootDir 'package-lock.json')) {
        Invoke-CheckedCommand -FilePath 'npm' -ArgumentList @('ci')
    }
    else {
        Invoke-CheckedCommand -FilePath 'npm' -ArgumentList @('install')
    }
}

function Build-Docs {
    Write-Host 'Syncing generated markdown into the Rspress docs tree...'
    Invoke-CheckedCommand -FilePath 'node' -ArgumentList @(
        (Join-Path $RootDir 'scripts\sync-rspress-content.mjs'),
        $MarkdownRoot,
        $DocsContentDir
    )

    Write-Host 'Building static Rspress site...'
    Invoke-CheckedCommand -FilePath 'npm' -ArgumentList @('run', 'docs:build')
    Write-Host "Static docs site available at $(Join-Path $DocsAppDir 'out')"
}

Push-Location $RootDir
try {
    New-Item -ItemType Directory -Force -Path $GeneratedDir, $MarkdownRoot | Out-Null

    if ($SkipExport) {
        if (-not $WikiRoot) {
            $WikiRoot = Join-Path $RootDir 'export_out'
        }
        Write-Host "Skipping export step; using existing wiki root: $WikiRoot"
    }
    else {
        $WikiRoot = $ExportOutDir
    }

    switch ($ExporterSource) {
        'workspace' {
            Build-WorkspaceTools
            $ExporterBin = Join-Path $RootDir 'target\release\lol-wiki-export.exe'
        }
        'release' {
            $ExporterBin = Download-ReleaseExporter
            Invoke-CheckedCommand -FilePath 'cargo' -ArgumentList @('build', '--release', '-p', 'lol_wiki_md', '--bin', 'convert')
        }
        default {
            throw "Unsupported exporter source: $ExporterSource"
        }
    }

    $ConverterBin = Join-Path $RootDir 'target\release\convert.exe'

    if (-not $SkipExport) {
        Run-Export -ExporterBin $ExporterBin
    }

    Run-Convert -ConverterBin $ConverterBin -SourceRoot $WikiRoot
    Install-DocsDependencies
    Build-Docs

    Write-Host 'Pipeline complete.'
}
finally {
    Pop-Location
}
