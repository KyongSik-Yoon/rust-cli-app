$OUTDIR = "tests/expected"
if (-not (Test-Path -Path $OUTDIR)) {
    New-Item -ItemType Directory -Path $OUTDIR
}

target/debug/echor "Hello there" | Out-File -FilePath "$OUTDIR/hello1.txt"
target/debug/echor "Hello"  "there" | Out-File -FilePath "$OUTDIR/hello2.txt"
target/debug/echor -n "Hello  there" | Out-File -FilePath "$OUTDIR/hello1.n.txt"
target/debug/echor -n "Hello"  "there" | Out-File -FilePath "$OUTDIR/hello2.n.txt"