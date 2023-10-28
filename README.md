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

| PIN | Destination |
|-----|-------------|
| 2   | Display SDA |
| 3   | Display SCL |
| 14  | CubeCell RX |
| 15  | CubeCell TX |
| 16  | Low LED     |
| 17  | DHT11       |
| 20  | Ok LED      |
| 21  | High LED    |

---

### Softat

- Lämpötilan lukija ja API (temperature_reader)
    - Lukee lämpötilan ja julkaisee API:a lämpötilan ja raja-arvojen lukemiseen
    - API:ssa on POST metodi raja-arvojen päivitykseen
    - Laittaa oikean ledin palamaan perustuen lämpötilan raja-arvoihin
- Näytön päivittäjä (pydisplay)
    - Kysyy API:lta lämpötilan ja kirjoittaa sen näytölle
- Web-palvelin (web_server)
    - Jakelee staattista html-sivua
    - Html-sivulla on javascript, joka kyselee API:lta lämpötilan ja raja-arvot
    - Mahdollistaa em. lisäksi uusien raja-arvojen tallentamisen
- Sarjaporttisofta (pyuart)
    - Lukee API:lta lämpötilan ja kirjoittaa sen sarjaporttiin

---

## temperature_reader

- Kielenä rust

### .env

- Tässähän ei kyllä pitäis käyttää .env -tiedostoa vaan jotain YAML tms configia...
- Sleep time
- API:n portti
- Oletusraja-arvot

### Main loop

- Tarkistaa annetut lämpötilarajat
- Lukee anturilta lämpötilan ja tallentaa sen muistiin
- Sammuttaa kaikki LEDit
- Tarkistaa onko lämpötila annettujen parametrien välissä
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
    "t": {
        "temperature": 123,
    },
    "b": {
        "lower_bound": 102,
        "upper_bound": 234
    }
}
```

#### POST `/bounds?lower=<number>&upper=<number>`

- Query parametreina
- Muistetaan, että käsitellään edelleenki celsiuksen kymmenyksiä

---

## web_server

- Kielenä rust, html ja javascript

- `/` get endpoint
    - Jakelee staattista index.html:ää
    - Liipasee get kutsun `/temperature` end-pointtiin ja näyttää sen vastauksen perusteella juttuja
- Nappula, jolla lähetetään uudet raja-arvot query parametreinä

---

## pydisplay

- Kielenä python

- Kyselee API:lta lämpötilan ja kirjoittaa sen näyttömoduulille

---

## pyuart

- Kielenä python

- Kyselee API:lta lämpötilan ja kirjoittaa sen sarjaporttiin

---


