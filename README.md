R-Cli 是极客时间rust训练营的学习项目

## 作业2
- sign
  默认使用硬编码sercet，不支持命令行参数key
```bash
➜  r-cli git:(main) cargo run -- jwt sign --sub acme --aud device1 --exp 14d
   Compiling rcli v0.1.0 (/home/ke/rustProject/r-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 6.07s
     Running `target/debug/rcli jwt sign --sub acme --aud device1 --exp 14d`
JwtSignOpts { sub: "acme", aud: "device1", exp: 1717035806s }
"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhY21lIiwiYXVkIjoiZGV2aWNlMSIsImV4cCI6MTcxNzAzNTgwNn0.Hld0PnecqVlvisxIL4g0wuwB1GHJ4RJWpgW7YhgiNso"
```
- verify
```bash
➜  r-cli git:(main) ✗ cargo run -- jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhY21lIiwiYXVkIjoiZGV2aWNlMSIsImV4cCI6MTcxNzAzNTgwNn0.Hld0PnecqVlvisxIL4g0wuwB1GHJ4RJWpgW7YhgiNso
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/rcli jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhY21lIiwiYXVkIjoiZGV2aWNlMSIsImV4cCI6MTcxNzAzNTgwNn0.Hld0PnecqVlvisxIL4g0wuwB1GHJ4RJWpgW7YhgiNso`
Ok
```
![jwo.io](assets/img/jwtIo.png)

## 作业3
- http serve
```bash
➜  r-cli git:(main) ✗ RUST_LOG=debug cargo run -- http serve
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/rcli http serve`
2024-05-16T02:42:04.656320Z  INFO rcli::process::http_serve: Serving "." on port0.0.0.0:8080
```
![](assets/img/restfulTestIndex.png)
- index

![serve_index](assets/img/httpServeIndex.png)
