$Bin = "socks5-proxy"
if ($env:TARGET) {
  $Target = $env:TARGET
} else {
  $Target = (rustc -vV | Select-String "host:").ToString().Split(" ")[1]
}

cargo build -p $Bin --release --target $Target
New-Item -ItemType Directory -Force src-tauri\bin | Out-Null
Move-Item `
  "target\$Target\release\$Bin.exe" `
  "src-tauri\bin\$Bin-$Target.exe"
