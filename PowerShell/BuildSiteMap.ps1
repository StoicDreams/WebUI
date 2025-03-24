
$domain = "https://webui.stoicdreams.com"

$urls = [System.Collections.ArrayList]@();

$appsettings = Get-Content "webapp/root_files/nav.json" -Raw

if ($appsettings.Length -eq 0) {
	throw "webapp/root_files/nav.json file not found";
}

$appsettings | Select-String -Pattern '(?<="url":[ ]*)("([^"]*)")' -AllMatches | ForEach-Object { $_.Matches } | ForEach-Object {
    $item = @{url = $_.Value};
    $urls.Add($item) | Out-Null
}

function buildSitemap
{
    begin
    {
        '<?xml version="1.0" encoding="utf-8" ?>'
        '<urlset xmlns="https://www.sitemaps.org/schemas/sitemap/0.9">'
    }

    process
    {
        '    <url>'
        "        <loc>$($domain)$($_.url)</loc>"
        "        <lastmod>{0:yyyy-MM-dd}</lastmod>" -f (Get-Date).Date
        '    </url>'
    }

    end
    {
        "</urlset>"
    }
}


$urls | buildSitemap | Out-File webapp/root_files/sitemap.xml -Encoding ascii
Write-Host "Sitemap Updated!"
