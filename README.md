# IoT-kurssi

---

## Järjestelmän kuvaus

### Fyysiset komponentit

- Lämpötila-anturi
- 3kpl LED
	- 2x punainen
    - 1x vihreä
- Näyttö
- 2x Lora-moduuleja
- Raspberry Pi 4 (2GB)

#### GPIO

- GPIO 26 -> RX from DHT11
- 16, 20, 21 -> LED output

---

### Softat

- Lämpötilan lukija
  - Välittää lämpötilan tiedon web-palvelimelle ja lora-verkkoon
  - Välittää nykyiset raja-arvot web-palvelimelle ja lora-verkkoon
  - Ottaa vastaan web-palvelimelta lämpötilan raja-arvot
  - Laittaa oikean ledin palamaan perustuen lämpötilan raja-arvoihin
  - Kirjoittaa lämpötilan näytölle
- Web-palvelin
  - Ottaa vastaan lämpötilatiedot lukijalta
  - Mahdollistaa raja-arvojen asettamisen ja lähettämisen lukijalle
- Lora-client
  - Samat toiminnallisuudet, kuin web-palvelimella
  - Ottaa ja lähettää dataa lora-verkon yli

---

## Lämpötilan lukija

### .env

- Sleep time
- Lora-verkon clientin osote
- Web-palvelimen osote
- Käytetäänkö loraa, webbiä vai molempia

### Main loop

- Tarkistaa annetut lämpötilarajat
- Lukee anturilta lämpötilan ja tallentaa sen muistiin
- Tarkistaa onko lämpötila annettujen parametrien välissä
- Sammuttaa kaikki LEDit
- Sytyttää asianmukaisen LEDin
- Odotetaan .envissä määritelty aika

### Async api

- Tämä vois olla oma softa, mutta tälleen vältetään tyhjänpäivänen tiedostojen kirjottelu ja lukeminen

#### GET `/temperature`

- Lue globaalit muuttujat `current_temperature`, `lower_bound` ja `upper_bound`
- Palauta json

```json
{
  "temperature": 12.34,
  "lower_bound": 10.23,
  "upper_bound": 23.45
}
```

#### POST `/values`

- Query parametreina
```json
{
  "lower_bound": 12.34,
  "upper_bound": 23.45
}
```
- Tallenna arvot globaaleihin muuttujiin `lower_bound` ja `upper_bound`

---

## Web-palvelin

- `/` get endpoint
  - Liipasee get kutsun 
