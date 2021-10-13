## Setup

```shell
$ docker run --name bhr_ch12_postgres -d -e POSTGRES_PASSWORD=black_hat_rust -e POSTGRES_USER=ch12 -p 5432:5432 postgres:13
$ cp env.example .env
```

## In shell 1

```shell
$ cargo run -p server
```

## In shell 2

```shell
$ cargo run -p agent
```

On Linux, the agent will be installed in `$XDG_DATA_HOME/bhr_ch12` or `$HOME/.local/share/bhr_ch12`


## In shell 3

```shell
$ cargo run -p client -- agents
+--------------------------------------+--------------------------------+--------------------------------+----------------------------------------------+----------------------------------------------+
| Agent ID                             | Created At                     | Last Seen At                   | Identity Public Key                          | Public Prekey                                |
+--------------------------------------+--------------------------------+--------------------------------+----------------------------------------------+----------------------------------------------+
| eeeb0586-d55c-4086-b9cd-55aae794bab2 | 2021-05-19 10:37:22.135356 UTC | 2021-05-19 13:25:10.890342 UTC | ttL7sPbyuyEmZ12E7JrD47BcMe2WLnnHP0DJufmVIAY= | G7l4VXnB1/aXKrAa9soyDkbUKr1xa+ldX+EjaINj5wY= |
+--------------------------------------+--------------------------------+--------------------------------+----------------------------------------------+----------------------------------------------+
$ cargo run -p client -- exec -a eeeb0586-d55c-4086-b9cd-55aae794bab2 ls
Cargo.lock
Cargo.toml
Dockerfile
Dockerfile.agent
Makefile
README.md
agent
client
common
env.example
server
target
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
