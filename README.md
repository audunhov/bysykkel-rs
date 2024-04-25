# Bysykkel-rs

Enkelt verktøy for å hente status fra forhåndsvalgte bysykkel-stasjoner. Bidra gjerne!

## Dependencies

Du må ha cargo innstallert og på $PATH

## Innstallasjon

```sh
git clone https://github.com/audunhov/bysykkel-rs
cd bysykkel-rs
cargo install --path=.
```

## Konfigurasjon

`config.json` har et felt med stations som er en liste over stasjonsnavn. 
Legg til flere navn, så dukker de opp etter rekompilering.
Stasjonsnavn finner du på Oslo Bysykkels nettsider, eller her: [Stasjoner](https://oslobysykkel.no/stasjoner)

NB! Du må kjøre `cargo install --path=.` på nytt etter endring i config.json, siden den inkluderes ved kompilering. 
Det burde være enkelt å fikse, men har ikke tatt meg bryet enda, siden jeg aldri endrer oppsettet mitt.
