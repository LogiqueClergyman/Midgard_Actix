@echo off

REM Try to get the container ID for "postgres" (including stopped containers)
for /f "delims=" %%i in ('docker ps -a -q -f "name=postgres"') do set CONTAINER_ID=%%i
REM If the container does not exist, create a new one
if "%CONTAINER_ID%"=="" (
    echo Container ID inside condition 1: %CONTAINER_ID%
    echo No container with the name "postgres" exists, creating a new container...

    REM Run the PostgreSQL container
    docker run --name postgres -e POSTGRES_USER=admin -e POSTGRES_PASSWORD=admin -e POSTGRES_DB=Midgard -p 5432:5432 -d postgres

    REM Wait for the container to initialize
    timeout /t 5 /nobreak

    REM Check if the container is now running
    docker inspect --format '{{.State.Running}}' postgres | findstr /i "true" > nul
    if errorlevel 1 (
        echo Failed to start PostgreSQL container.
    ) else (
        echo PostgreSQL container started successfully.
    )
) else (
    echo Container ID inside condition 2: %CONTAINER_ID%
    REM If the container exists, check if it is stopped
    docker ps -q -f "name=postgres" > nul
    if "%ERRORLEVEL%"=="0" (
        echo PostgreSQL container exists but is stopped. Starting the container...

        REM Start the stopped container
        docker start postgres

        REM Check if the container is now running
        for /f "delims=" %%i in ('docker ps -q -f "name=postgres"') do set CONTAINER_ID=%%i
        if not "%CONTAINER_ID%"=="" (
            echo PostgreSQL container started successfully.
        ) else (
            echo Failed to start PostgreSQL container.
        )
    ) else (
        echo PostgreSQL container is already running.
    )
)
REM After handling the container, connect to PostgreSQL interactive terminal
echo Connecting to PostgreSQL interactive terminal...

docker exec -it postgres psql -U admin -d test_db