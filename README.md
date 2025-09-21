# dns-server-minimal
DNS Server Minimale


Pseudo DNS Server, da una dig fa il parse header, qname, qtype, qclass
Fornendo i vari dati estratti dai vari offset

é tutto molto basilare

# TODO

- Manca la risposta


/funzione response(buf_query, lunghezza_query):
    # 1. Creare buffer vuoto per la risposta
    risposta = []

    # 2. HEADER
    copia primi 2 byte (ID) dalla query
    aggiungi Flags con QR=1, AA=1, RD copiato dalla query
    set QDCOUNT = 1
    set ANCOUNT = 1
    set NSCOUNT = 0
    set ARCOUNT = 0
    append all'header

    # 3. QUESTION
    copia QNAME, QTYPE, QCLASS dalla query e append

    # 4. ANSWER
    NAME = puntatore a QNAME (0xC0 0x0C)
    TYPE = 0x0001 (A record)
    CLASS = 0x0001 (IN)
    TTL = 60 secondi (0x0000003C)
    RDLENGTH = 4 (dimensione IPv4)
    RDATA = 127.0.0.1 (7F 00 00 01)
    append all'answer

    # 5. INVIARE RISPOSTA
    socket.send_to(risposta, indirizzo_client)
