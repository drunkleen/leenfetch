param(
    [switch]$Update,
    [switch]$Force
)

$Repo = "drunkleen/leenfetch"
$AppName = "leenfetch"
$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\leenfetch"
$BinDir = Join-Path $env:LOCALAPPDATA "Programs\leenfetch"

function Write-Step($Message) {
    Write-Host "==> $Message" -ForegroundColor Cyan
}

function Get-LatestVersion {
    $url = "https://api.github.com/repos/$Repo/releases/latest"
    try {
        $release = Invoke-RestMethod -Uri $url -ErrorAction Stop
        return $release.tag_name
    } catch {
        Write-Error "Failed to fetch latest version. Check your internet connection."
        exit 1
    }
}

function Get-LocalVersion {
    $exe = Join-Path $BinDir "leenfetch.exe"
    if (Test-Path $exe) {
        try {
            $ver = & $exe --version 2>&1 | Select-Object -First 1
            if ($ver -match "(\d+\.\d+\.\d+)") { return "v$($matches[1])" }
        } catch {}
    }
    return $null
}

function Add-ToPath {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$BinDir*") {
        $newPath = "$userPath;$BinDir"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = "$env:Path;$BinDir"
        Write-Step "Added to PATH"
    }
}

Write-Step "Detecting system..."

$Arch = $env:PROCESSOR_ARCHITECTURE.ToLower()
if ($Arch -eq "amd64") { $Arch = "x86_64" }
elseif ($Arch -eq "arm64") { $Arch = "arm64" }
else {
    Write-Error "Unsupported architecture: $Arch"
    exit 1
}

$CurrentVer = Get-LocalVersion
$LatestVer = Get-LatestVersion

if ($CurrentVer) {
    Write-Step "Current version: $CurrentVer"
    if (-not $Update -and -not $Force) {
        Write-Host "leenfetch is already installed (v$CurrentVer). Use -Update to upgrade or -Force to reinstall."
        exit 0
    }
    if ($Update -and $CurrentVer -eq $LatestVer -and -not $Force) {
        Write-Host "Already at latest version ($CurrentVer)."
        exit 0
    }
}

Write-Step "Version: $LatestVer"
Write-Step "Architecture: $Arch"

$FileName = "leenfetch-v$($LatestVer.Substring(1))-windows-$Arch.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$LatestVer/$FileName"
$TempZip = Join-Path $env:TEMP $FileName

Write-Step "Downloading..."
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -ErrorAction Stop
} catch {
    Write-Error "Download failed. URL: $DownloadUrl"
    exit 1
}

Write-Step "Extracting..."
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

try {
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force -ErrorAction Stop
} catch {
    Microsoft.PowerShell.Archive\Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force -ErrorAction Stop
}

$ExePath = Join-Path $InstallDir "leenfetch.exe"
if (-not (Test-Path $ExePath)) {
    $extracted = Get-ChildItem -Recurse -Filter "leenfetch.exe" -Path $InstallDir | Select-Object -First 1
    if ($extracted) {
        Move-Item -Path $extracted.FullName -Destination $ExePath -Force
        Get-ChildItem -Path $InstallDir -Directory | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    }
}

Add-ToPath

Remove-Item -Path $TempZip -Force -ErrorAction SilentlyContinue

Write-Step "Successfully installed leenfetch $LatestVer"
Write-Host "Run 'leenfetch' in a new terminal to get started!" -ForegroundColor Green
