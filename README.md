magick xxx.tif -quality 99 -define webp:method=6 -define webp:target-size=250000 -define webp:pass=10 imgs/yyy.webp

exiv2 -M"set Xmp.dc.description テスト" imgs/xxx.webp

exiftool -overwrite_original -UserComment="test"  imgs/xxx.jpg

cargo run