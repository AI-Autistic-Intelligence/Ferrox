---
id: 1m-rps
title: 1M RPS Benchmark Whitepaper
sidebar_position: 1
---

# 1 Million Requests / Second Benchmark Whitepaper
## Studio Comparativo delle Prestazioni Multi-Runtime nell'Ecosistema Ferrox

**Autori**: Ferrox Engineering Team & Enterprise Architecture Group  
**Data**: Ottobre 2026 • **Versione**: 2.0 Enterprise  
**Repository Benchmark**: [`1MilionRequestPerSecond`](https://github.com/AI-Autistic-Intelligence/1MilionRequestPerSecond)

---

## Executive Summary

Il progetto **1 Million Requests Per Second (1M RPS)** è un'indagine ingegneristica empirica volta a misurare e confrontare il comportamento sotto stress estremo dei cinque runtime supportati dall'ecosistema Ferrox: **Rust**, **Node.js**, **PHP**, **Python** e **Java**.

A differenza dei benchmark sintetici banali ("Hello World" in memoria), il test Ferrox riproduce un **workload transazionale enterprise reale**:
1. **Rate Limiting Distribuito**: Ogni richiesta invoca un comando atomico Redis (`INCR` + `EXPIRE`) per verificare la quota del chiamante.
2. **Accesso a Database Relazionale**: Ogni richiesta esegue una query `SELECT` parametrizzata su una tabella PostgreSQL popolata con **100.000 record** indicizzati.
3. **Isolamento delle Risorse**: Ogni container applicativo è vincolato rigidamente a **8.0 vCPU** tramite cgroups Docker, con pool di connessioni calibrato a 100 connessioni per replica.

### Risultati Chiave
- **Throughput Assoluto**: **ferrox-node** (Fastify + cluster nativo) e **ferrox-rust** (Axum + Tokio) dominano la classifica, elaborando rispettivamente **14.781 RPS** e **13.469 RPS**.
- **Consistenza & Latenza p99**: **ferrox-rust** vince nettamente sull'affidabilità con una latenza p99 di **333 ms** (più di **2x inferiore** rispetto a Node e Python), evidenziando l'assenza totale di stop-the-world Garbage Collection.
- **Efficienza di Memoria (RAM)**: **ferrox-rust** consuma appena **38 MB** a pieno carico, rispetto ai 320 MB di Node.js e agli 890 MB della JVM Java.

---

## 📊 Grafici Comparativi SVG

### 1. Throughput Reale (Requests / Second)
*Misurato su finestra di stress test continuo di 60s con Autocannon e concorrenza elevata.*

<div style={{ margin: '1.5rem 0', textAlign: 'center' }}>
<svg viewBox="0 0 760 260" style={{ width: '100%', maxWidth: '760px', height: 'auto', background: '#090d16', borderRadius: '12px', border: '1px solid #1e293b', padding: '16px' }}>
  <line x1="160" y1="30" x2="720" y2="30" stroke="#1e293b" strokeDasharray="3 3"/>
  <line x1="160" y1="75" x2="720" y2="75" stroke="#1e293b" strokeDasharray="3 3"/>
  <line x1="160" y1="120" x2="720" y2="120" stroke="#1e293b" strokeDasharray="3 3"/>
  <line x1="160" y1="165" x2="720" y2="165" stroke="#1e293b" strokeDasharray="3 3"/>
  <line x1="160" y1="210" x2="720" y2="210" stroke="#1e293b" strokeDasharray="3 3"/>

  <text x="145" y="48" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-node</text>
  <rect x="160" y="34" width="530" height="22" rx="4" fill="url(#grad-node-p)"/>
  <text x="700" y="50" fill="#38bdf8" fontSize="12" fontWeight="bold">14,781 RPS</text>

  <text x="145" y="93" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-rust</text>
  <rect x="160" y="79" width="482" height="22" rx="4" fill="url(#grad-rust-p)"/>
  <text x="652" y="95" fill="#f59e0b" fontSize="12" fontWeight="bold">13,469 RPS</text>

  <text x="145" y="138" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-php</text>
  <rect x="160" y="124" width="380" height="22" rx="4" fill="url(#grad-php-p)"/>
  <text x="550" y="140" fill="#a855f7" fontSize="12" fontWeight="bold">10,627 RPS</text>

  <text x="145" y="183" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-py</text>
  <rect x="160" y="169" width="255" height="22" rx="4" fill="url(#grad-py-p)"/>
  <text x="425" y="185" fill="#3b82f6" fontSize="12" fontWeight="bold">7,120 RPS</text>

  <text x="145" y="228" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-java</text>
  <rect x="160" y="214" width="130" height="22" rx="4" fill="url(#grad-java-p)"/>
  <text x="300" y="230" fill="#f43f5e" fontSize="12" fontWeight="bold">3,639 RPS</text>

  <defs>
    <linearGradient id="grad-node-p" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stopColor="#0284c7"/><stop offset="100%" stopColor="#38bdf8"/>
    </linearGradient>
    <linearGradient id="grad-rust-p" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stopColor="#ea580c"/><stop offset="100%" stopColor="#f59e0b"/>
    </linearGradient>
    <linearGradient id="grad-php-p" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stopColor="#7c3aed"/><stop offset="100%" stopColor="#a855f7"/>
    </linearGradient>
    <linearGradient id="grad-py-p" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stopColor="#1d4ed8"/><stop offset="100%" stopColor="#60a5fa"/>
    </linearGradient>
    <linearGradient id="grad-java-p" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stopColor="#be123c"/><stop offset="100%" stopColor="#f43f5e"/>
    </linearGradient>
  </defs>
</svg>
</div>

---

### 2. Latenza p99 (Millisecondi - Valore Minore è Migliore)

<div style={{ margin: '1.5rem 0', textAlign: 'center' }}>
<svg viewBox="0 0 760 260" style={{ width: '100%', maxWidth: '760px', height: 'auto', background: '#090d16', borderRadius: '12px', border: '1px solid #1e293b', padding: '16px' }}>
  <text x="145" y="48" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-rust</text>
  <rect x="160" y="34" width="190" height="22" rx="4" fill="#10b981"/>
  <text x="360" y="50" fill="#10b981" fontSize="12" fontWeight="bold">333 ms (Miglior Consistenza)</text>

  <text x="145" y="93" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-php</text>
  <rect x="160" y="79" width="340" height="22" rx="4" fill="#a855f7"/>
  <text x="510" y="95" fill="#a855f7" fontSize="12" fontWeight="bold">594 ms</text>

  <text x="145" y="138" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-java</text>
  <rect x="160" y="124" width="376" height="22" rx="4" fill="#f43f5e"/>
  <text x="546" y="140" fill="#f43f5e" fontSize="12" fontWeight="bold">657 ms</text>

  <text x="145" y="183" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-node</text>
  <rect x="160" y="169" width="388" height="22" rx="4" fill="#38bdf8"/>
  <text x="558" y="185" fill="#38bdf8" fontSize="12" fontWeight="bold">677 ms (V8 GC Jitter)</text>

  <text x="145" y="228" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-py</text>
  <rect x="160" y="214" width="466" height="22" rx="4" fill="#60a5fa"/>
  <text x="636" y="230" fill="#60a5fa" fontSize="12" fontWeight="bold">815 ms (GIL Wait)</text>
</svg>
</div>

---

### 3. Consumo di Memoria RAM (Megabyte - Valore Minore è Migliore)

<div style={{ margin: '1.5rem 0', textAlign: 'center' }}>
<svg viewBox="0 0 760 260" style={{ width: '100%', maxWidth: '760px', height: 'auto', background: '#090d16', borderRadius: '12px', border: '1px solid #1e293b', padding: '16px' }}>
  <text x="145" y="48" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-rust</text>
  <rect x="160" y="34" width="35" height="22" rx="4" fill="#10b981"/>
  <text x="205" y="50" fill="#10b981" fontSize="12" fontWeight="bold">38 MB (Zero Allocator Bloat)</text>

  <text x="145" y="93" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-php</text>
  <rect x="160" y="79" width="115" height="22" rx="4" fill="#a855f7"/>
  <text x="285" y="95" fill="#a855f7" fontSize="12" fontWeight="bold">180 MB</text>

  <text x="145" y="138" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-py</text>
  <rect x="160" y="124" width="156" height="22" rx="4" fill="#60a5fa"/>
  <text x="326" y="140" fill="#60a5fa" fontSize="12" fontWeight="bold">245 MB</text>

  <text x="145" y="183" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-node</text>
  <rect x="160" y="169" width="205" height="22" rx="4" fill="#38bdf8"/>
  <text x="375" y="185" fill="#38bdf8" fontSize="12" fontWeight="bold">320 MB (V8 Heap Cluster)</text>

  <text x="145" y="228" fill="#e2e8f0" fontSize="13" fontWeight="bold" textAnchor="end">ferrox-java</text>
  <rect x="160" y="214" width="570" height="22" rx="4" fill="#f43f5e"/>
  <text x="590" y="230" fill="#f43f5e" fontSize="12" fontWeight="bold">890 MB (JVM Metaspace + Heap)</text>
</svg>
</div>

---

## 🔬 Dati Tabellari di Benchmark

| Framework / Runtime | Stack Tecnologico | Throughput (RPS) | Latenza Media | Latenza p99 | Errori / Timeouts | RAM Tipica |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **ferrox-node** | Fastify 5.0 + Native Cluster + `pg` | **14,781.99** | 138.13 ms | 677 ms | **0** | ~320 MB |
| **ferrox-rust** | Axum 0.8 + Tokio + Deadpool PG | **13,469.20** | 148.87 ms | **333 ms** | **0** | **~38 MB** |
| **ferrox-php** | FrankenPHP / Swoole + PDO Pool | **10,627.04** | 188.42 ms | 594 ms | **0** | ~180 MB |
| **ferrox-py** | FastAPI + Asyncpg + Gunicorn | **7,120.69** | 281.23 ms | 815 ms | **0** | ~245 MB |
| **ferrox-java** | Spring Boot 3.3 + Virtual Threads + HikariCP | **3,639.44** | 548.00 ms | 657 ms | **0** | ~890 MB |

---

## 🏗️ Architettura del Test & Isolamento del Carico

```mermaid
graph LR
    Generator["Autocannon Test Runner<br/>(Client Concorrente)"] --> HAProxy["HAProxy Load Balancer<br/>(Algoritmo Round-Robin)"]
    HAProxy --> R1["Backend Replica 1<br/>(Max 8 vCPU)"]
    HAProxy --> R2["Backend Replica 2<br/>(Max 8 vCPU)"]
    HAProxy --> R3["Backend Replica 3<br/>(Max 8 vCPU)"]
    R1 --> Redis[("Redis 7.2 Cluster<br/>Rate-Limiting (INCR)")]
    R2 --> Redis
    R3 --> Redis
    R1 --> PG[("PostgreSQL 16 Enterprise<br/>100,000 Righe Seeded")]
    R2 --> PG
    R3 --> PG
```

### Specifiche dell'Ambiente di Prova
- **CPU Allocata**: Limite rigido a 8.0 vCPU per container applicativo.
- **Database Engine**: PostgreSQL 16 con `max_connections = 2000`, `shared_buffers = 2GB`.
- **Cache Engine**: Redis 7.2 Alpine con persistenza AOF disattivata per massimizzare il throughput I/O in memoria.
- **Connection Pool**: 100 connessioni simultanee mantenute calde per replica.
- **Durata della Sessione**: 60 secondi di stress costante per ogni runtime, preceduti da 10 secondi di warm-up.

---

## 🛠️ Come Riprodurre il Benchmark in Locale

```bash
# 1. Clona o spostati nella directory del benchmark
cd 1MilionRequestPerSecond

# 2. Avvia lo stack completo (PostgreSQL, Redis, HAProxy e le 5 app)
docker-compose up --build -d

# 3. Attendi il popolamento del database (100,000 record)
docker-compose logs -f db-seeder

# 4. Esegui il test suite automatizzato contro tutti i runtime
docker-compose exec test-runner node test_runner.js

# 5. Genera i report grafici HTML aggiornati
node report_generator.js
```
