# Bagagesorteringssystem
Projektet er en simulering af et bagagesorteringssystem i Rust, der håndterer bagage fra check-in til terminaler (gates) ved brug af tråde og synkronisering.

Systemet anvender Rust’s trådhåndtering og synkroniseringsteknikker til at simulere de forskellige processer i bagagesystemet.
## Funktioner:

### Dataoprettelse:
Opretter og opdaterer reservationssystem og flyveplan.
### Trådhåndtering:
Opretter tråde for skranker, sorteringsanlæg og terminaler.
### Bufferhåndtering:
Bruger Vec<T> og Arc<Mutex<T>> til trådsikker adgang til buffere.
### Simulering af åbning/lukning:
Terminaler og skranker åbnes og lukkes via brugerinput.
### Logning:
Udskriver systemstatus til skærm og logger til filer.
