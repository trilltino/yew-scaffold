@echo off
REM Development automation script for Windows

if "%1"=="help" goto help
if "%1"=="start" goto start
if "%1"=="build" goto build
if "%1"=="test" goto test
if "%1"=="clean" goto clean
if "%1"=="wallet" goto wallet
if "%1"=="" goto help

:help
echo 🚀 Stellar dApp Development Script
echo.
echo Usage: dev.bat [command]
echo.
echo Commands:
echo   start    - Start both frontend and backend servers
echo   build    - Build both frontend and backend
echo   test     - Run all tests
echo   clean    - Clean build artifacts
echo   wallet   - Test wallet connectivity
echo   help     - Show this help
echo.
goto end

:start
echo 🚀 Starting development servers...
echo Starting backend in new window...
start "Backend" cmd /k "cd backend && cargo run"
timeout /t 3 /nobreak >nul
echo Starting frontend...
cd frontend && trunk serve --port 8080
goto end

:build
echo 🔨 Building project...
echo Building backend...
cd backend && cargo build
if %errorlevel% neq 0 (
    echo ❌ Backend build failed
    goto end
)
echo Building frontend...
cd ..\frontend && trunk build
if %errorlevel% neq 0 (
    echo ❌ Frontend build failed
    goto end
)
echo ✅ Build completed successfully
goto end

:test
echo 🧪 Running tests...
echo Testing backend...
cd backend && cargo test
echo Testing frontend build...
cd ..\frontend && trunk build
echo ✅ Tests completed
goto end

:clean
echo 🧹 Cleaning build artifacts...
cd backend && cargo clean
cd ..\frontend && rm -rf dist
echo ✅ Clean completed
goto end

:wallet
echo 🔍 Testing wallet connectivity...
timeout /t 2 /nobreak >nul
curl -s http://127.0.0.1:3001/health
if %errorlevel% neq 0 (
    echo ❌ Backend not running. Run 'dev.bat start' first
    goto end
)
echo ✅ Backend healthy
curl -s http://127.0.0.1:8080/ >nul
if %errorlevel% neq 0 (
    echo ❌ Frontend not running. Run 'dev.bat start' first
    goto end
)
echo ✅ Frontend healthy
echo 💡 Open http://127.0.0.1:8080/ and check browser console
goto end

:end