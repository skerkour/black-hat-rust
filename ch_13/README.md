## Setup

```shell
$ docker build -t black_hat_rust/ch13:latest .
```


## Usage


### In shell 1

```shell
$ docker run -d -p 1322:22 --name bhr_ch13_ssh black_hat_rust/ch13:latest
$ cargo run -p agent --
```

On Linux, the agent will be installed in `$XDG_DATA_HOME/bhr_ch13` or `$HOME/.local/share/bhr_ch13`


### In shell 2

```shell
$ cargo run -p agent --
# should exit immediatly
```

### Inspect container

```shell
$ docker exec -ti bhr_ch13_ssh bash
```

### Cleanup

```shell
$ docker rm -f bhr_ch13_ssh
```


## Build for Linux

```shell
make agent_linux
```

## Build for Windows

```shell
make agent_windows
```
