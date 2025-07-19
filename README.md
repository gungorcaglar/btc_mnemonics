# btc mnemonic online hack
Randomly generate BIP39 words and ONLINE check BTC wallets with Blockchain.com - Blockchain Data API



### Build the Docker image:
```bash
sudo docker build -t mnemonic .
 ```
### Run the Docker container:
```bash
sudo docker run -d -it -p 8000:8000 --name mnemonic-container mnemonic
```
### Check Logs:
```bash
sudo docker logs --follow mnemonic-container
```
### Check Bash:
```bash
sudo docker exec -it mnemonic-container bash
```


> [!CAUTION]
> Only For Education Use