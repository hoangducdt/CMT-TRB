@echo off
chcp 65001 >nul
title TRB Helper - Local Server
cd /d "%~dp0"

set PORT=8080
set URL=http://localhost:%PORT%/

if not exist "index.html" (
    echo [LOI] Khong tim thay index.html trong thu muc nay.
    echo Hay dat file .cmd cung thu muc voi index.html.
    pause
    exit /b 1
)

echo ==============================================
echo   TRB Helper - dang khoi dong may chu cuc bo
echo ==============================================
echo.
echo   Link mo web (Ctrl+Click neu can):
echo   %URL%
echo.
echo   Dong cua so nay de tat may chu.
echo ==============================================
echo.

rem Mo trinh duyet sau 2 giay (cho server kip khoi dong)
start "" powershell -NoProfile -WindowStyle Hidden -Command "Start-Sleep 2; Start-Process '%URL%'"

rem --- Uu tien 1: Python ---
python --version >nul 2>&1
if %errorlevel%==0 (
    python -m http.server %PORT%
    goto :end
)

rem --- Uu tien 2: Python launcher ---
py --version >nul 2>&1
if %errorlevel%==0 (
    py -m http.server %PORT%
    goto :end
)

rem --- Uu tien 3: Node.js ---
node --version >nul 2>&1
if %errorlevel%==0 (
    call npx --yes serve -l %PORT% .
    goto :end
)

rem --- Du phong: PowerShell (co san tren Windows, khong can cai gi) ---
echo [Thong tin] Khong thay Python/Node, dung PowerShell lam may chu.
echo.
powershell -NoProfile -ExecutionPolicy Bypass -Command "$root=(Get-Location).Path; $l=New-Object Net.HttpListener; $l.Prefixes.Add('http://localhost:'+$env:PORT+'/'); $l.Start(); $mime=@{'.html'='text/html; charset=utf-8';'.js'='text/javascript';'.wasm'='application/wasm';'.svg'='image/svg+xml';'.css'='text/css';'.json'='application/json';'.png'='image/png';'.webp'='image/webp';'.md'='text/plain; charset=utf-8'}; while($l.IsListening){ $c=$l.GetContext(); try{ $f=Join-Path $root ([Uri]::UnescapeDataString($c.Request.Url.AbsolutePath.TrimStart('/'))); if(Test-Path $f -PathType Container){$f=Join-Path $f 'index.html'}; if(Test-Path $f -PathType Leaf){ $t=$mime[[IO.Path]::GetExtension($f).ToLower()]; if(-not $t){$t='application/octet-stream'}; $b=[IO.File]::ReadAllBytes($f); $c.Response.ContentType=$t; $c.Response.ContentLength64=$b.Length; $c.Response.OutputStream.Write($b,0,$b.Length) } else { $c.Response.StatusCode=404 } } catch {} finally { $c.Response.Close() } }"

:end
pause
