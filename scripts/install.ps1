# install.ps1 — copies the post-commit hook into the current repo's .git/hooks/

$HookSrc = Join-Path $PSScriptRoot "..\hooks\post-commit"
$HookDst = ".git\hooks\post-commit"

if (-not (Test-Path ".git\hooks")) {
    Write-Error "'.git\hooks' not found. Run this script from the root of a git repository."
    exit 1
}

if (-not (Test-Path $HookSrc)) {
    Write-Error "Source hook not found at: $HookSrc"
    exit 1
}

Copy-Item -Path $HookSrc -Destination $HookDst -Force
Write-Host "Hook installed at $HookDst"
