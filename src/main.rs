#[derive(Debug)]
struct Utente {
    nome: String,
    cognome: String,
    secret_id: u32,
}

impl Utente {
    fn nuovo(n: &str, c: &str, id: u32) -> Utente {
        Utente {
            nome: n.to_string(),
            cognome: c.to_string(),
            secret_id: id,
        }
    }

    fn generate_badge(&self) -> String {
        format!(
            "Badge: {} {} - ID: {}",
            self.nome, self.cognome, self.secret_id
        )
    }
}

fn main() {
    let new_user = Utente::nuovo("Mario", "Rossi", 42);
    let badge = new_user.generate_badge();
    println!("{}", badge);
    println!("{:?}", new_user)
}
