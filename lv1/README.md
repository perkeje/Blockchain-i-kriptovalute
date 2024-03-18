# Laboratorijska vježba 1

Ovaj program je primjer kako se koriste različite kriptografske funkcionalnosti u Rustu, uključujući kodiranje i dekodiranje Base58, generiranje SHA-256 hash-a, te korištenje RSA algoritma za šifriranje, dešifriranje, i potpisivanje poruka. Kod s komentarima se nalazi u src direktoriju.

## Testovi

Svaki zadatak je iskomentiran i ima napisane testove za zadane ulazne primjere.

## Preduvjeti

Za kompajliranje i pokretanje programa potrebno je imati instaliran rust i cargo. Program bi se trebao moći pokrenuti i bez potrebe za cargom pokretanjem naredbe:

`
./lv1 <naredba> <argumenti>
`

## Pokretanje

### Kodiranje Base58

`
cargo run encode '<ulazni_tekst>'
`

### Dekodiranje Base58

`
cargo run decode '<ulazni_tekst>'
`

### Generiranje SHA-256 hash-a

`
cargo run sha256 '<ulazni_tekst>'
`

### Generiranje RSA ključeva

`
cargo run rsa_generate
`

Ova naredba generira par RSA ključeva (javni i privatni) i ispisuje ih. Ključevi se dalje koriste za iduće naredbe i kopiraju se zajedno sa

`-----BEGIN RSA PUBLIC KEY-----`
i
`-----END RSA PUBLIC KEY-----`
dijelovima

### Šifriranje poruke

`
cargo run encrypt '<poruka>' '<javni_ključ>'
`

### Dešifriranje poruke

`
cargo run decrypt '<šifrirana_poruka>' '<privatni_ključ>'
`
