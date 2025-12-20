param (
    [switch]$Release
)

$buildParams = @("build", "--ssg")
$flavor = "debug"

if ($Release) {
    $buildParams += "--release"
    $flavor = "release"
    Write-Host "Building in RELEASE mode..."
}
else {
    Write-Host "Building in DEBUG mode..."
}

Write-Host "Running: dx $buildParams"
# Invoke dx command
& dx $buildParams

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

# Determine target path
$targetHtml = "target/dx/new-home-page/$flavor/web/public/index.html"

if (-not (Test-Path $targetHtml)) {
    # Fallback/Check dist if dx behavior changes, but target is standard
    if (Test-Path "dist/index.html") {
        $targetHtml = "dist/index.html"
    }
    else {
        Write-Error "File not found: $targetHtml"
        exit 1
    }
}

Write-Host "Fixing asset paths in $targetHtml..."
$content = Get-Content $targetHtml -Raw
# Replace /./ with ./ and /MyHomePage/ with ./
$content = $content -replace '/\./', './' -replace '/MyHomePage/', './'
Set-Content $targetHtml $content

Write-Host "Build and Fix Complete!"
