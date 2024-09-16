exiv2 -M"set Xmp.dc.description テスト" imgs/xxx.webp

exiftool -overwrite_original -UserComment="test"  imgs/xxx.jpg

cargo run