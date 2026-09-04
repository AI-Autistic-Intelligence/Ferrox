use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use sys_locale::get_locale;
use std::collections::HashMap;

/// Ferrox Enterprise CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new Ferrox project
    Init {
        /// Optional project name
        name: Option<String>,
    },
}

struct I18n {
    lang: String,
    dict: HashMap<&'static str, HashMap<&'static str, &'static str>>,
}

impl I18n {
    fn new() -> Self {
        let mut dict = HashMap::new();

        // English
        let mut en = HashMap::new();
        en.insert("welcome", "Welcome to Ferrox CLI! Let's scaffold your Enterprise App.");
        en.insert("project_name", "What is the name of your project?");
        en.insert("database", "Which database do you want to use?");
        en.insert("success", "Project scaffolded successfully!");
        dict.insert("en", en);

        // Italian
        let mut it = HashMap::new();
        it.insert("welcome", "Benvenuto nella CLI di Ferrox! Iniziamo lo scaffolding.");
        it.insert("project_name", "Qual è il nome del tuo progetto?");
        it.insert("database", "Quale database desideri utilizzare?");
        it.insert("success", "Progetto generato con successo!");
        dict.insert("it", it);

        // Chinese (Simplified)
        let mut zh = HashMap::new();
        zh.insert("welcome", "欢迎使用 Ferrox CLI！让我们搭建您的企业级应用。");
        zh.insert("project_name", "您的项目名称是什么？");
        zh.insert("database", "您想使用哪个数据库？");
        zh.insert("success", "项目搭建成功！");
        dict.insert("zh", zh);

        // Spanish (Mexico)
        let mut es = HashMap::new();
        es.insert("welcome", "¡Bienvenido a la CLI de Ferrox! Vamos a crear tu aplicación.");
        es.insert("project_name", "¿Cuál es el nombre de tu proyecto?");
        es.insert("database", "¿Qué base de datos deseas usar?");
        es.insert("success", "¡Proyecto creado con éxito!");
        dict.insert("es", es);

        // Auto-detect language (fallback to "en")
        let locale = get_locale().unwrap_or_else(|| String::from("en"));
        let lang = if locale.starts_with("it") { "it" }
            else if locale.starts_with("zh") { "zh" }
            else if locale.starts_with("es") { "es" }
            else { "en" }.to_string();

        Self { lang, dict }
    }

    fn t(&self, key: &str) -> &str {
        self.dict.get(self.lang.as_str())
            .and_then(|lang_dict| lang_dict.get(key))
            .unwrap_or(&key)
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let i18n = I18n::new();

    match &cli.command {
        Commands::Init { name } => {
            println!("🚀 {}", i18n.t("welcome"));

            let project_name = match name {
                Some(n) => n.clone(),
                None => Text::new(i18n.t("project_name")).prompt().unwrap(),
            };

            let options = vec!["PostgreSQL (SeaORM)", "MongoDB", "Redis Only"];
            let _db_choice = Select::new(i18n.t("database"), options).prompt().unwrap();

            // Scaffolding simulation...
            println!("✅ {} ({})", i18n.t("success"), project_name);
        }
    }
}
