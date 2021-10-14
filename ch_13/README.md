## Setup

```shell
$ docker build -t black_hat_rust/ch13:latest .
$ make bundle
```


## Usage


```shell
$ docker run -d -p 1322:22 --name bhr_ch13_ssh black_hat_rust/ch13:latest
$ cargo run -p agent -- 127.0.0.1:1322
```

On Linux, the agent will be installed in `$XDG_DATA_HOME/bhr_ch13` or `$HOME/.local/share/bhr_ch13`


### Inspect container

```shell
$ docker exec -ti bhr_ch13_ssh bash
$ ls -lah /tmp
$ ls -lahR /root/.local/share/
```

### Cleanup

```shell
$ docker rm -f bhr_ch13_ssh
# on linux
$ rm -rf ~/.local/share/bhr_ch13/
# on macOS
$ rm -rf $HOME/Library/Application Support/bhr_ch13/
```


## Build for Linux

```shell
make agent_linux
```

## Build for Windows

```shell
make agent_windows
```
