# 出力先ディレクトリを作成
$outputDir = ".\imgs"
if (-Not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir
}

# 現在のディレクトリ内の TIFF ファイルを取得
$tifFiles = Get-ChildItem -Path . -Filter *.tif

# 各 TIFF ファイルを WebP に変換
foreach ($file in $tifFiles) {
    # 出力ファイルのパスを設定
    $outputFile = Join-Path -Path $outputDir -ChildPath ($file.BaseName + ".webp")

    # すでに WebP ファイルが存在する場合はスキップ
    if (Test-Path $outputFile) {
        Write-Output "Skipping $($file.Name), WebP file already exists."
        continue
    }

    # WebP に変換
    magick $file.FullName -quality 99 -define webp:method=6 -define webp:target-size=250000 -define webp:pass=10 $outputFile
    Write-Output "Converted $($file.Name) to $outputFile."
}
