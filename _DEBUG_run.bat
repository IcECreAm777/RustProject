RMDIR /S /Q .\target\debug\resources\
xcopy /S /E .\resources .\target\debug\resources\

cargo run
