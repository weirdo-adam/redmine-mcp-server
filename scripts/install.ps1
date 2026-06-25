$ErrorActionPreference = "Stop"

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
} catch {
}

$Repo = if ($env:REDMINE_MCP_REPO) { $env:REDMINE_MCP_REPO } else { "weirdo-adam/redmine-mcp-server" }
$BinaryName = "redmine-mcp-server"
$LocalAppData = if ($env:LOCALAPPDATA) { $env:LOCALAPPDATA } else { Join-Path $HOME "AppData\Local" }
$InstallDir = if ($env:REDMINE_MCP_INSTALL_DIR) { $env:REDMINE_MCP_INSTALL_DIR } else { Join-Path $LocalAppData "redmine-mcp-server\bin" }

function Resolve-RedmineMcpTag {
    if ($env:REDMINE_MCP_VERSION) {
        if ($env:REDMINE_MCP_VERSION.StartsWith("v")) {
            return $env:REDMINE_MCP_VERSION
        }

        return "v$($env:REDMINE_MCP_VERSION)"
    }

    $release = Invoke-RestMethod `
        -Uri "https://api.github.com/repos/$Repo/releases/latest" `
        -Headers @{ "User-Agent" = "redmine-mcp-server-installer" }

    return $release.tag_name
}

function Resolve-RedmineMcpArch {
    $arch = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }

    switch -Regex ($arch) {
        "^(AMD64|x86_64)$" { return "x86_64" }
        "^(ARM64|AARCH64)$" { return "aarch64" }
        default { throw "Unsupported CPU architecture: $arch" }
    }
}

function Download-File {
    param(
        [Parameter(Mandatory = $true)][string]$Url,
        [Parameter(Mandatory = $true)][string]$Output
    )

    Invoke-WebRequest `
        -Uri $Url `
        -OutFile $Output `
        -Headers @{ "User-Agent" = "redmine-mcp-server-installer" } `
        -UseBasicParsing
}

function Add-UserPath {
    param([Parameter(Mandatory = $true)][string]$PathToAdd)

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $entries = @()
    if ($currentPath) {
        $entries = $currentPath -split ";" | Where-Object { $_ }
    }

    $normalizedTarget = $PathToAdd.TrimEnd("\")
    $exists = $entries | Where-Object { $_.TrimEnd("\") -ieq $normalizedTarget }
    if ($exists) {
        return $false
    }

    $newPath = if ($currentPath) { "$currentPath;$PathToAdd" } else { $PathToAdd }
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = "$env:Path;$PathToAdd"
    return $true
}

$Tag = Resolve-RedmineMcpTag
if (-not $Tag) {
    throw "Unable to resolve the latest Redmine MCP server release."
}

$Version = $Tag.TrimStart("v")
$Arch = Resolve-RedmineMcpArch
$Package = "$BinaryName-$Version-windows-$Arch.tar.gz"
$BaseUrl = "https://github.com/$Repo/releases/download/$Tag"
$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) "redmine-mcp-server-install-$([Guid]::NewGuid())"

if (-not (Get-Command tar -ErrorAction SilentlyContinue)) {
    throw "tar is required to extract Redmine MCP server."
}

New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

try {
    $Archive = Join-Path $TempDir $Package
    $Checksum = Join-Path $TempDir "$Package.sha256"

    Write-Host "Installing Redmine MCP server $Version for windows-$Arch..."
    Download-File "$BaseUrl/$Package" $Archive
    Download-File "$BaseUrl/$Package.sha256" $Checksum

    $Expected = ([regex]::Split((Get-Content $Checksum -Raw).Trim(), "\s+"))[0].ToLowerInvariant()
    $Actual = (Get-FileHash -Algorithm SHA256 $Archive).Hash.ToLowerInvariant()
    if ($Expected -ne $Actual) {
        throw "Checksum verification failed for $Package. Expected $Expected, got $Actual."
    }

    tar -xzf $Archive -C $TempDir

    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    $SourceBinary = Join-Path $TempDir "$BinaryName.exe"
    $TargetBinary = Join-Path $InstallDir "$BinaryName.exe"
    Copy-Item $SourceBinary $TargetBinary -Force

    $AddedPath = Add-UserPath $InstallDir

    Write-Host ""
    Write-Host "Installed Redmine MCP server to:"
    Write-Host "  $TargetBinary"
    Write-Host ""
    Write-Host "Required environment:"
    Write-Host "  REDMINE_BASE_URL=https://redmine.example.com"
    Write-Host "  REDMINE_API_KEY=your-api-key"
    Write-Host ""
    if ($AddedPath) {
        Write-Host "Added the install directory to your user PATH. Restart open terminals if needed:"
        Write-Host "  $InstallDir"
        Write-Host ""
    }
    Write-Host "MCP command:"
    Write-Host "  $BinaryName.exe"
} finally {
    Remove-Item $TempDir -Recurse -Force -ErrorAction SilentlyContinue
}
