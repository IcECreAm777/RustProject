RMDIR /S /Q .\target\release\resources\
xcopy /S /E .\resources .\target\release\resources\

cargo run --release
