# README

Zápočtový program na Programování v ChatGPT.

Chat vybral Rust & téma, Chat píše kód, já píšu, zdali to funguje a jak se to ovládá.

Tématem je hra podobná Space Invaders. To by Chata mělo na chvilku zabavit.

## Instalace
S Rustem na svém počítači si stáhněte projekt, v adresáři s Cargo.toml pak napište do terminálu `cargo build --release` aby se projekt zkompiloval. Nakonec do vytvořeného adresáře `target/release` zkopírujte adresář `assets`.

Pokud máte Windows a Intel x86_64 procesor, můžete rovnou zkusit rozbalit zip soubor s buildem pro takový systém.

## Minimalist Space Shooter
### Popis
Hra je podobná Space Invaders, hráč se snaží zastavit padající asteroidy, přitom může získat power-upy a zvyšovat skóre. Čím déle hráč hraje, tím více asteroidů spadne. Hru lze pozastavit či ukončit.

### Gameplay
Hráč kontroluje zelený čtvereček šipkami doleva a doprava. Střílí pomocí mezerníku. Hru lze pozastavit stisknutím klávesy P. Hru lze ihned ukončit stisknutím klávesy ESC. Střílením asteroidů (rotující šestiúhelníky) ze zvyšuje skóre, za každý asteroid o 1 bod.

Každých 10 projektilem zničených asteroidů vytvoří power-up, buď ve formě bomby (zničí všechny asteroidy a za každý zvýší skóre o 1), nebo triple-shotu (hráč střílí 3 projektily naráz), nebo ve formě auto-shootu (hráč střílí rychleji a bez potřeby stisknout mezerník).

Hra končí, pokud hráč stiskne klávesu ESC nebo pokud asteroid narazí na bariéru pod hráčem (červená čára). Pokud se tak stane, hráči se zbrazí dosažené skóre s nápisem GAME OVER. Hru lze pak jenom ukončit přes ESC.

### Použité crates
- piston_window (verze 0.120.0)
- rand (verze 0.8.4)

#### Program byl za lidského dohledu napsán umělou inteligencí ChatGPT.