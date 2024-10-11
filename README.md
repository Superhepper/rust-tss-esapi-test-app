# rust-tss-esapi-test-app
Test app using the rust-tss-esapi crate.


# To build on windows
To build on windows it is necessary to build from installation folder.
Ex:
```powershell
$Env:LIBCLANG_PATH = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\bin"
$Env:TPM2_TSS_PATH = "C:\Program Files\tpm2-tss-Win64"
```

To run the app on windows the 'Path' variable needs to contain
the path to all the DLLs that will be needed.
```powershell
$env:Path += ';' + "C:\Program Files\OpenSSL-Win64\bin"
$env:Path += ';' + "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\bin"
$env:Path += ';' + "C:\Program Files\tpm2-tss-Win64\lib" # I chose to put the dlls in the lib folder.
```