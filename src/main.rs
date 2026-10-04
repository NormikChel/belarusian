use askama::Template;
use std::fs;
use std::path::Path;

// ============================================
// ШАБЛЁНЫ
// ============================================

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmpl {
    active: &'static str,
}

#[derive(Template)]
#[template(path = "history.html")]
struct HistoryTmpl {
    active: &'static str,
}

#[derive(Template)]
#[template(path = "grammar.html")]
struct GrammarTmpl {
    active: &'static str,
}

#[derive(Template)]
#[template(path = "vocabulary.html")]
struct VocabularyTmpl {
    active: &'static str,
}

#[derive(Template)]
#[template(path = "taraskievica.html")]
struct TaraskievicaTmpl {
    active: &'static str,
}

#[derive(Template)]
#[template(path = "dialects.html")]
struct DialectsTmpl {
    active: &'static str,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTmpl {
    active: &'static str,
}

// ============================================
// УТЫЛІТЫ
// ============================================

fn write_page(filename: &str, html: String) {
    let out = Path::new("dist").join(filename);
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).expect("не стварыць папку");
    }
    fs::write(&out, html).expect("не запісаць файл");
    println!("  ✓ dist/{}", filename);
}

fn copy_dir(from: &str, to: &str) {
    fs::create_dir_all(to).expect("не стварыць тэчка");
    let entries = fs::read_dir(from).unwrap_or_else(|_| panic!("няма папкі {}", from));
    for entry in entries {
        let entry = entry.expect("дрэнны запіс у тэчку");
        let path = entry.path();
        let dest = Path::new(to).join(entry.file_name());
        if path.is_dir() {
            copy_dir(path.to_str().unwrap(), dest.to_str().unwrap());
        } else {
            fs::copy(&path, &dest).expect("не скапіяваць файл");
            println!("  ✓ {}", dest.display());
        }
    }
}

// ============================================
// ГАЛОЎНАЕ
// ============================================

fn main() {
    println!("🔨 Чышчу dist/...");
    let _ = fs::remove_dir_all("dist");
    fs::create_dir_all("dist").expect("не стварыць dist");

    println!("\n📄 Генэрую старонкі...");
    write_page("index.html",        IndexTmpl        { active: "index" }.render().unwrap());
    write_page("history.html",      HistoryTmpl      { active: "history" }.render().unwrap());
    write_page("grammar.html",      GrammarTmpl      { active: "grammar" }.render().unwrap());
    write_page("vocabulary.html",   VocabularyTmpl   { active: "vocabulary" }.render().unwrap());
    write_page("taraskievica.html", TaraskievicaTmpl { active: "taraskievica" }.render().unwrap());
    write_page("dialects.html",     DialectsTmpl     { active: "dialects" }.render().unwrap());
    write_page("about.html",        AboutTmpl        { active: "about" }.render().unwrap());

    println!("\n🎨 Капірую статыку...");
    copy_dir("static", "dist/static");

    // .nojekyll — каб GitHub Pages не чапаў файлы, што пачынаюцца з _
    fs::write("dist/.nojekyll", "").expect("не стварыць .nojekyll");
    println!("  ✓ dist/.nojekyll");

    println!("\n✅ Гатово! Старонкі ў dist/");
    println!("   Залей утрыманьне dist/ на GitHub Pages.");
}