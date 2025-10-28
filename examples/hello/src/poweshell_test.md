$urls = @(
    "http://127.0.0.1:8000/?emoji",
    "http://127.0.0.1:8000/?name=Rocketeer",
    "http://127.0.0.1:8000/?lang=ру",
    "http://127.0.0.1:8000/?lang=ру&emoji",
    "http://127.0.0.1:8000/?emoji&lang=en",
    "http://127.0.0.1:8000/?name=Rocketeer&lang=en",
    "http://127.0.0.1:8000/?emoji&name=Rocketeer",
    "http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji",
    "http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer"
)

$counter = 0
foreach ($url in $urls) {
    $counter++
    Write-Host "Test #$counter : $url"
    curl.exe -s -w "`nHTTP Status: %{http_code}`n" $url
    Start-Sleep -Milliseconds 500
    Write-Host "`n"
}

Write-Host "All tests completed!"
Read-Host "Press Enter to continue..."
