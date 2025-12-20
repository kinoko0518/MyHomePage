$path = "target/dx/new-home-page/debug/web/public/index.html"
if (Test-Path $path) {
    $content = Get-Content $path -Raw
    $content = $content -replace '/\./', './'
    Set-Content $path $content
    Write-Host "Fixed paths in $path"
} else {
    Write-Error "File not found: $path"
}
