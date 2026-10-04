use askama::Template;
use std::fs;
use std::path::Path;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "history.html")]
struct HistoryTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "grammar.html")]
struct GrammarTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "vocabulary.html")]
struct VocabTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "taraskievica.html")]
struct TaraTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "dialects.html")]
struct DialTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTmpl { active: &'static str }

fn write_page(path: &str, html: String) {
    let out = Path::new("dist").join(path);
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&out, html).unwrap();
    println!("✓ {}", out.display());
}

fn main() {
    fs::create_dir_all("dist").unwrap();

    write_page("index.html",         IndexTmpl { active: "index" }.render().unwrap());
    write_page("history/index.html", HistoryTmpl { active: "history" }.render().unwrap());
    write_page("grammar/index.html", GrammarTmpl { active: "grammar" }.render().unwrap());
    write_page("vocabulary/index.html", VocabTmpl { active: "vocabulary" }.render().unwrap());
    write_page("taraskievica/index.html", TaraTmpl { active: "taraskievica" }.render().unwrap());
    write_page("dialects/index.html", DialTmpl { active: "dialects" }.render().unwrap());
    write_page("about/index.html",   AboutTmpl { active: "about" }.render().unwrap());

    // Копируем статику
    copy_dir("static", "dist/static");

    println!("\n🚀 Гатово! Старонкі ў dist/");
}

fn copy_dir(from: &str, to: &str) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let dest = Path::new(to).join(entry.file_name());
        if path.is_dir() {
            copy_dir(path.to_str().unwrap(), dest.to_str().unwrap());
        } else {
            fs::copy(&path, &dest).unwrap();
            println!("✓ {}", dest.display());
        }
    }
}