# Evil Twin

On a Raspberry Pi with Rust installed

```shell
$ sudo apt install -y macchanger hostapd dnsmasq sqlite3
$ make -C ../phishing/ && cp -r ../phishing/dist/* .
$ sudo ./server -p 80
$ sudo ./evil_twin.sh
```
