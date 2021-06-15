# tonic-server-stream-err-bug-repro
Minimal repository reproducing a bug found when streaming error on purpose to clients with `tonic`

# How to use to reproduce bug?

- Build the executables (`cargo build`).
- Open 3 terminals at the build output directory (Ex. `\target\debug`).
- Run `server.exe`.
- In a seperate terminal Run `trace_client.exe`.
- In a seperate terminal Run `err_client.exe` to cause bug.
