# Laboratorijska vježba 2

U ovoj vježbi kreiran je jednostavna implementacija blockchaina. Program se pokreće sa `node Main.js` i u konzoli ispisuje stanja walleta nakon svake transakcije a cijeli blockchain se nalazi u [blockchain.json](./blockchain.json). Dolje se nalazi primjer ispisa konzole za kreirani blockchain. Za svako novo pokretanje programa kreira se novi blockchain.

## Opis rada

Svaka transakcija mora imati validan potpis od svog pošiljatelja. Također, u ovom primjeru simulirano je računanje stanja walleta tako što se propagira kroz transakcije i računa stanje za public key koji se koristi.

## Primjer ispisa

```
WALLET AMOUNTS AFTER TREASURY TRANSFER:


-----------------------------------------------------------
            
Wallet Professor balance: 40
            
Wallet Professor public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEB9UAue4iCOddw6o23+S5FqbL4mu9I5I/
8NMHwfANXKLEi0PKGvV5vzhmdcScOU6lW2q6P2mkwPThpMEH2AGtWg==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet Student balance: 40
            
Wallet Student public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEbrLcOEo6YdMHx52DoSyr9lOsynoTmdRY
U2CyS6hChBcYxSSNryzSBFgGvjbmqm0y/kBTgxzGOPU22ygnB5kBQw==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet CryptoWhale balance: 40
            
Wallet CryptoWhale public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAE9CQpZkj2l3ZLntVyT+atztQsl5A8d1Y3
gnlyl+DHxkGeAGZoDzYwny7L3xwWrz7ck/pbTWUIfqOGpWvPKOOe0Q==
-----END PUBLIC KEY-----

WALLET AMOUNTS AFTER PROFFESOR TO STUDENT 10:


-----------------------------------------------------------
            
Wallet Professor balance: 30
            
Wallet Professor public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEB9UAue4iCOddw6o23+S5FqbL4mu9I5I/
8NMHwfANXKLEi0PKGvV5vzhmdcScOU6lW2q6P2mkwPThpMEH2AGtWg==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet Student balance: 50
            
Wallet Student public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEbrLcOEo6YdMHx52DoSyr9lOsynoTmdRY
U2CyS6hChBcYxSSNryzSBFgGvjbmqm0y/kBTgxzGOPU22ygnB5kBQw==
-----END PUBLIC KEY-----

WALLET AMOUNTS AFTER STUDENT TO CRYPTOWHALE 30:


-----------------------------------------------------------
            
Wallet Student balance: 20
            
Wallet Student public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEbrLcOEo6YdMHx52DoSyr9lOsynoTmdRY
U2CyS6hChBcYxSSNryzSBFgGvjbmqm0y/kBTgxzGOPU22ygnB5kBQw==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet CryptoWhale balance: 70
            
Wallet CryptoWhale public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAE9CQpZkj2l3ZLntVyT+atztQsl5A8d1Y3
gnlyl+DHxkGeAGZoDzYwny7L3xwWrz7ck/pbTWUIfqOGpWvPKOOe0Q==
-----END PUBLIC KEY-----

WALLET AMOUNTS AFTER CRYPTOWHALE TO PROFESSOR 10:


-----------------------------------------------------------
            
Wallet Professor balance: 40
            
Wallet Professor public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEB9UAue4iCOddw6o23+S5FqbL4mu9I5I/
8NMHwfANXKLEi0PKGvV5vzhmdcScOU6lW2q6P2mkwPThpMEH2AGtWg==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet CryptoWhale balance: 60
            
Wallet CryptoWhale public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAE9CQpZkj2l3ZLntVyT+atztQsl5A8d1Y3
gnlyl+DHxkGeAGZoDzYwny7L3xwWrz7ck/pbTWUIfqOGpWvPKOOe0Q==
-----END PUBLIC KEY-----

WALLET AMOUNTS AFTER 2 TRIES OF INVALID TRANSACTIONS:

Issuficient balance to complete transaction!
Invalid signature!

-----------------------------------------------------------
            
Wallet Professor balance: 40
            
Wallet Professor public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEB9UAue4iCOddw6o23+S5FqbL4mu9I5I/
8NMHwfANXKLEi0PKGvV5vzhmdcScOU6lW2q6P2mkwPThpMEH2AGtWg==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet Student balance: 20
            
Wallet Student public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEbrLcOEo6YdMHx52DoSyr9lOsynoTmdRY
U2CyS6hChBcYxSSNryzSBFgGvjbmqm0y/kBTgxzGOPU22ygnB5kBQw==
-----END PUBLIC KEY-----


-----------------------------------------------------------
            
Wallet CryptoWhale balance: 60
            
Wallet CryptoWhale public key: -----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAE9CQpZkj2l3ZLntVyT+atztQsl5A8d1Y3
gnlyl+DHxkGeAGZoDzYwny7L3xwWrz7ck/pbTWUIfqOGpWvPKOOe0Q==
-----END PUBLIC KEY-----

BLOCKCHAIN INFO AVAILABLE IN blockchain.json
```