//! # Vulnerability Dictionary (`ferrox-selftest::vocabulary`)
//!
//! Educational and explanatory dictionary mapping OWASP WSTG vulnerability codes to
//! plain-language explanations, practical business risks, and OWASP remediation guidelines.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Comprehensive entry explaining a specific OWASP security check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityExplanation {
    pub wstg_id: &'static str,
    pub title: &'static str,
    pub plain_concept: &'static str,
    pub practical_business_risk: &'static str,
    pub remediation_guide: &'static str,
}

/// Vulnerability Dictionary lookup provider
pub struct VulnerabilityDictionary;

impl VulnerabilityDictionary {
    /// Returns the complete map of OWASP WSTG vulnerability explanations
    pub fn get_dictionary() -> HashMap<&'static str, VulnerabilityExplanation> {
        let mut map = HashMap::new();

        map.insert("WSTG-CONF-001", VulnerabilityExplanation {
            wstg_id: "WSTG-CONF-001",
            title: "HTTP Strict Transport Security (HSTS) Header Missing",
            plain_concept: "HSTS è una direttiva di sicurezza che obbliga il browser a comunicare SOLO tramite connessione cifrata HTTPS, impedendo qualsiasi tentativo di downgrade della connessione a HTTP in chiaro.",
            practical_business_risk: "Senza HSTS, un attaccante sulla stessa rete Wi-Fi o punto d'accesso può intercettare il traffico della vittima tramite attacchi Man-in-the-Middle (MitM) e rubare credenziali o cookie di sessione.",
            remediation_guide: "Configura il server web o il middleware per restituire l'header 'Strict-Transport-Security: max-age=31536000; includeSubDomains; preload'.",
        });

        map.insert("WSTG-CONF-002", VulnerabilityExplanation {
            wstg_id: "WSTG-CONF-002",
            title: "Content Security Policy (CSP) Header Missing",
            plain_concept: "La CSP è una difesa fondamentale che definisce quali risorse (script, stili, immagini) il browser è autorizzato a caricare ed eseguire all'interno della pagina.",
            practical_business_risk: "L'assenza di CSP facilita gli attacchi Cross-Site Scripting (XSS), permettendo a uno script malevolo iniettato di rubare dati riservati o eseguire azioni a nome dell'utente.",
            remediation_guide: "Imposta l'header 'Content-Security-Policy' con direttive restrittive (es. default-src 'self'; script-src 'self').",
        });

        map.insert("WSTG-CONF-003", VulnerabilityExplanation {
            wstg_id: "WSTG-CONF-003",
            title: "Clickjacking Protection (X-Frame-Options) Missing",
            plain_concept: "L'header X-Frame-Options impedisce a siti web terzi e potenzialmente malevoli di incorporare la tua applicazione all'interno di un iframe nascosto.",
            practical_business_risk: "Un attaccante può ingannare gli utenti spingendoli a cliccare su elementi invisibili che eseguono azioni dannose sul tuo sito a loro insaputa (Clickjacking).",
            remediation_guide: "Invia l'header 'X-Frame-Options: DENY' o 'X-Frame-Options: SAMEORIGIN'.",
        });

        map.insert("WSTG-CONF-004", VulnerabilityExplanation {
            wstg_id: "WSTG-CONF-004",
            title: "MIME-Type Sniffing Protection (X-Content-Type-Options) Missing",
            plain_concept: "Impedisce al browser di ignorare l'header Content-Type dichiarato e di tentare di indovinare (sniffing) il formato del file caricato.",
            practical_business_risk: "Senza questo header, un file di testo o un'immagine contenente codice JavaScript malevolo potrebbe essere eseguita dal browser come script.",
            remediation_guide: "Configura l'header 'X-Content-Type-Options: nosniff'.",
        });

        map.insert("WSTG-SESS-002", VulnerabilityExplanation {
            wstg_id: "WSTG-SESS-002",
            title: "Cookie Security Flags Missing (HttpOnly & Secure)",
            plain_concept: "I flag HttpOnly e Secure proteggono i cookie di autenticazione garantendo che non siano accessibili da JavaScript e siano trasmessi solo su canali cifrati.",
            practical_business_risk: "Un cookie di sessione senza HttpOnly può essere rubato istantaneamente da qualsiasi vulnerabilità XSS presente nella pagina.",
            remediation_guide: "Imposta sempre le direttive '; HttpOnly; Secure; SameSite=Strict' su tutti i cookie di autenticazione.",
        });

        map.insert("WSTG-ERRH-001", VulnerabilityExplanation {
            wstg_id: "WSTG-ERRH-001",
            title: "Detailed Server Stack Trace Leakage",
            plain_concept: "Si verifica quando il server mostra messaggi d'errore grezzi, dettagli del codice o stack trace del database agli utenti finali.",
            practical_business_risk: "Gli attaccanti utilizzano questi dettagli tecnici per comprendere l'architettura interna e trovare vulnerabilità specifiche da sfruttare.",
            remediation_guide: "Intercetta tutte le eccezioni con un middleware di gestione errori centralizzato e mostra solo messaggi generici all'utente.",
        });

        map.insert("WSTG-ATHN-001", VulnerabilityExplanation {
            wstg_id: "WSTG-ATHN-001",
            title: "Weak Password or Rate Limit Policy",
            plain_concept: "Indica che il sistema di login non limita il numero di tentativi di autenticazione errati consecutivi.",
            practical_business_risk: "Espone gli account degli utenti ad attacchi di Brute Force e Credential Stuffing automatizzati ad alta velocità.",
            remediation_guide: "Implementa un Rate Limiter rigoroso o una politica di blocco temporaneo dell'account (es. Sentinel VelocityTracker).",
        });

        map
    }

    /// Looks up explanation for a given WSTG ID
    pub fn lookup(wstg_id: &str) -> Option<VulnerabilityExplanation> {
        Self::get_dictionary().get(wstg_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_lookup() {
        let entry = VulnerabilityDictionary::lookup("WSTG-CONF-001");
        assert!(entry.is_some());
        let e = entry.unwrap();
        assert_eq!(e.title, "HTTP Strict Transport Security (HSTS) Header Missing");
        assert!(e.plain_concept.contains("HTTPS"));
    }
}
