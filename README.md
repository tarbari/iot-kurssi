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

- Simppeli