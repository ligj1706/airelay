# airelay Windows installer
# Run: irm https://raw.githubusercontent.com/ligj1706/airelay/main/install.ps1 | iex

$ErrorActionPreference = "Stop"
$REPO = "ligj1706/airelay"
$VERSION = if ($args[0]) { $args[0] } else { "latest" }

$ARCH = (Get-CimInstance Win32_Processor).Architecture
if ($ARCH -eq 9 -or $ARCH -eq 12) {
    $NAME = "windows-x86_64"
} else {
    Write-Error "Unsupported architecture: $ARCH (only x86_64/ARM64 Windows supported)"
    exit 1
}

$INSTALL_DIR = "$env:USERPROFILE\.local\bin"
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null

if ($VERSION -eq "latest") {
    $URL = "https://github.com/$REPO/releases/latest/download/airelay-$NAME.zip"
} else {
    $URL = "https://github.com/$REPO/releases/download/$VERSION/airelay-$NAME.zip"
}

Write-Host "-> Downloading $URL"
$TMP = "$env:TEMP\airelay_install"
New-Item -ItemType Directory -Force -Path $TMP | Out-Null
$ZIP = "$TMP\airelay.zip"

try {
    Invoke-WebRequest -Uri $URL -OutFile $ZIP -UseBasicParsing
} catch {
    Write-Error "Download failed. Check your network or visit https://github.com/$REPO/releases"
    exit 1
}

Expand-Archive -Path $ZIP -DestinationPath $TMP -Force
Copy-Item "$TMP\airelay.exe" "$INSTALL_DIR\airelay.exe" -Force
Remove-Item -Recurse -Force $TMP -ErrorAction SilentlyContinue

Write-Host "Binary installed to $INSTALL_DIR"

$USER_PATH = [Environment]::GetEnvironmentVariable("Path", "User")
if ($USER_PATH -notlike "*$INSTALL_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$USER_PATH;$INSTALL_DIR", "User")
    $env:Path = "$env:Path;$INSTALL_DIR"
    Write-Host "PATH updated"
}

Write-Host ""
Write-Host "============================================"
Write-Host "  airelay installed!"
Write-Host ""
Write-Host "  Usage:"
Write-Host "    airelay                Start the proxy"
Write-Host "    airelay list           List providers"
Write-Host "    airelay switch <p/m>   Switch model"
Write-Host ""
Write-Host "  Configure:  http://127.0.0.1:8082/admin"
Write-Host ""
Write-Host "  Launch Claude Code:"
Write-Host "    `$env:ANTHROPIC_BASE_URL='http://127.0.0.1:8082'"
Write-Host "    `$env:ANTHROPIC_AUTH_TOKEN='any'"
Write-Host "    claude"
Write-Host "============================================"
Write-Host ""
Write-Host "Restart your terminal or run: refreshenv"
