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

### API

- Tämä vois olla oma softa, mutta tälleen vältetään tyhjänpäivänen tiedostojen kirjottelu ja lukeminen
- Lämpötiloja käsitellään i16 tyyppisinä, ja ne ovat celsiusasteen kymmenyksiä
    - Web-sivulla nämä muutetaan celsiusasteiksi yhden desimaalin tarkkuudella

#### GET `/temperature`

- Lue globaalit muuttujat `current_temperature`, `lower_bound` ja `upper_bound`
- Palauta json

```json
{
  "temperature": 123,
  "lower_bound": 102,
  "upper_bound": 234
}
```

#### POST `/values`

- Query parametreina
```json
{
  "lower_bound": 123,
  "upper_bound": 234
}
```
- Tallenna arvot globaaleihin muuttujiin `lower_bound` ja `upper_bound`

---

## Web-palvelin

- `/` get endpoint
  - Liipasee get kutsun `/temperature` end-pointtiin ja näyttää sen vastauksen perusteella juttuja
