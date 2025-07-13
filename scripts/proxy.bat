@echo off
chcp 65001 >nul
title Windows 系统代理一键设置工具（适配端口 10808）

:MENU
cls
echo ================================
echo     Windows 系统代理设置工具
echo ================================
echo 1. 设置系统 HTTP/HTTPS 代理 (127.0.0.1:10808)
echo 2. 取消系统代理
echo 3. 查看当前 WinHTTP 代理状态
echo 0. 退出
echo ================================
set /p choice=请输入选项 [0-3]:

if "%choice%"=="1" goto SET_PROXY
if "%choice%"=="2" goto UNSET_PROXY
if "%choice%"=="3" goto SHOW_PROXY
if "%choice%"=="0" goto EXIT
goto MENU

:SET_PROXY
echo 设置环境变量代理...
setx http_proxy http://127.0.0.1:10808 /m
setx https_proxy http://127.0.0.1:10808 /m

echo 设置 WinHTTP 代理...
netsh winhttp set proxy 127.0.0.1:10808

echo ✅ 系统代理已设置为 http://127.0.0.1:10808
pause
goto MENU

:UNSET_PROXY
echo 清除环境变量代理...
setx http_proxy "" /m
setx https_proxy "" /m

echo 清除 WinHTTP 代理...
netsh winhttp reset proxy

echo ✅ 系统代理已取消
pause
goto MENU

:SHOW_PROXY
echo 当前 WinHTTP 代理状态:
netsh winhtt
