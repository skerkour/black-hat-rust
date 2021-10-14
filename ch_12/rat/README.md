## Usage


## In shell 1

```shell
$ cargo run -p agent
```

On Linux, the agent will be installed in `$XDG_DATA_HOME/bhr_ch12` or `$HOME/.local/share/bhr_ch12`


## In shell 2

```shell
$ cargo run -p agent
# should exit immediatly
```


## Build for Linux

```shell
make agent_linux
```

## Build for Windows

```shell
make agent_windows
```

## Warning

Don't commit your private keys as in this example ;)
