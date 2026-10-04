use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

// ============================================================
//  Агульны каркас старонкі
// ============================================================

fn layout(title: &str, active: &str, body: &str) -> String {
    let nav_item = |href: &str, key: &str, label: &str| {
        let cls = if key == active { " class=\"is-active\"" } else { "" };
        format!("<li><a href=\"{}\"{}>{}</a></li>", href, cls, label)
    };

    let nav = format!(
        "{}{}{}{}{}{}{}",
        nav_item("/", "index", "Галоўная"),
        nav_item("/history", "history", "Гісторыя"),
        nav_item("/grammar", "grammar", "Граматыка"),
        nav_item("/vocabulary", "vocabulary", "Слоўнік"),
        nav_item("/taraskievica", "taraskievica", "Тарашкевіца"),
        nav_item("/dialects", "dialects", "Гаворкі"),
        nav_item("/about", "about", "Пра праект"),
    );

    format!(r##"<!DOCTYPE html>
<html lang="be" data-theme="dark">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} · Мова</title>
<link rel="stylesheet" href="/static/style.css">
<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ctext y='.9em' font-size='90'%3E🇧🇾%3C/text%3E%3C/svg%3E">
</head>
<body>
<div class="orbs" aria-hidden="true">
  <span class="orb orb-1"></span><span class="orb orb-2"></span>
  <span class="orb orb-3"></span><span class="orb orb-4"></span>
</div>

<header class="site-header">
  <nav class="nav glass">
    <a href="/" class="logo"><span class="logo-mark">Б</span><span>Мова</span></a>
    <ul class="nav-links">{nav}</ul>
    <button class="theme-toggle glass" id="themeToggle" aria-label="Зьмяніць тэму">
      <span class="theme-sun">☀</span><span class="theme-moon">☾</span>
    </button>
  </nav>
</header>

<main class="page">{body}</main>

<footer class="site-footer">
  <div class="footer-inner glass">
    <div class="footer-col footer-brand">
      <a href="/" class="footer-logo"><span class="logo-mark">Б</span><span>Мова</span></a>
      <p class="footer-tagline">Жывая, сакавітая, старажытная.<br>Пра мову — зь любоўю й без русызмаў.</p>
      <div class="footer-social">
        <a href="#" class="soc">tg</a><a href="#" class="soc">gh</a>
        <a href="#" class="soc">yt</a><a href="#" class="soc">rss</a>
      </div>
    </div>
    <div class="footer-col"><h4>Мова</h4>
      <a href="/history">Гісторыя</a><a href="/grammar">Граматыка</a>
      <a href="/vocabulary">Слоўнік</a><a href="/dialects">Гаворкі</a>
    </div>
    <div class="footer-col"><h4>Правапіс</h4>
      <a href="/taraskievica">Тарашкевіца</a>
      <a href="/taraskievica&#35;нарк">Наркамаўка</a>
      <a href="/taraskievica&#35;правілы">Асноўныя правілы</a>
    </div>
    <div class="footer-col"><h4>Праект</h4>
      <a href="/about">Пра нас</a><a href="/about&#35;кантакты">Кантакты</a>
      <a href="/about&#35;ліцэнзія">Ліцэнзія</a>
    </div>
    <div class="footer-col footer-newsletter"><h4>Ліставаць</h4>
      <p>Атрымлівай новыя артыкулы пра мову раз на тыдзень.</p>
      <form class="newsletter" onsubmit="event.preventDefault();this.classList.add('is-sent');">
        <input type="email" placeholder="твая@пошта.бел" required>
        <button type="submit">→</button>
        <span class="newsletter-ok">Дзякуй! Хутка будзе ліст.</span>
      </form>
    </div>
  </div>
  <div class="footer-bottom">
    <span>© 2026 Моўны праект «Мова»</span>
    <span>Зроблена з <span class="heart">♥</span> на Расьце</span>
    <span>Клясычны правапіс · Тарашкевіца</span>
  </div>
</footer>
<script src="/static/app.js" defer></script>
</body></html>"##)
}

// ============================================================
//  Галоўная
// ============================================================

async fn index() -> Html<String> {
    let body = r##"
<section class="hero"><div class="hero-inner glass">
  <span class="badge">🇧🇾 Жывая мова · Тарашкевіца</span>
  <h1 class="hero-title">Беларуская мова — <em>сакавітая</em>, старажытная, свабодная.</h1>
  <p class="hero-lead">Мы зьбіраем тут усё пра нашую мову: ад старабеларускіх грамат і статутаў ВКЛ — да сучасных гаворкаў, слоўнікаў і клясычнага правапісу Баляслава Тарашкевіча. Без русызмаў, без кампрамісаў.</p>
  <div class="hero-actions">
    <a href="/history" class="btn btn-primary">Пачаць з гісторыі →</a>
    <a href="/vocabulary" class="btn btn-ghost">Заглянуць у слоўнік</a>
  </div>
  <div class="hero-stats">
    <div class="stat"><span class="stat-num">X</span><span class="stat-label">стагодзьдзяў пісьмовай гісторыі</span></div>
    <div class="stat"><span class="stat-num">3</span><span class="stat-label">галоўныя дыялектныя групы</span></div>
    <div class="stat"><span class="stat-num">6</span><span class="stat-label">склонаў у назоўніку</span></div>
    <div class="stat"><span class="stat-num">↺</span><span class="stat-label">бясконца жывая традыцыя</span></div>
  </div>
</div></section>

<section class="section">
  <header class="section-head"><h2>Што тут ёсьць</h2><p>Сем разьдзелаў — сем падарожжаў у глыб мовы.</p></header>
  <div class="cards">
    <a href="/history" class="card glass"><span class="card-emoji">📜</span><h3>Гісторыя</h3><p>Ад полацкіх грамат X стагодзьдзя — да сёньняшняга дня. Статуты ВКЛ, адраджэньне, рэфэрэндум.</p><span class="card-more">Чытаць →</span></a>
    <a href="/grammar" class="card glass"><span class="card-emoji">🧩</span><h3>Граматыка</h3><p>Шэсьць склонаў, спражэньні, дзеепрыметнікі. Усё тое, што робіць мову гнуткай і дакладнай.</p><span class="card-more">Разьбірацца →</span></a>
    <a href="/vocabulary" class="card glass"><span class="card-emoji">📖</span><h3>Слоўнік</h3><p>Унікальныя беларускія словы, іх этымалёгія, гульні сэнсаў. Ад «вавёркі» да «шуфель».</p><span class="card-more">Гартаць →</span></a>
    <a href="/taraskievica" class="card glass"><span class="card-emoji">✒️</span><h3>Тарашкевіца</h3><p>Клясычны правапіс 1918 году. Чым ён адрозьніваецца ад наркамаўкі й чаму мы яго любім.</p><span class="card-more">Даведацца →</span></a>
    <a href="/dialects" class="card glass"><span class="card-emoji">🗺️</span><h3>Гаворкі</h3><p>Паўночна-ўсходнія, паўднёва-заходнія, сярэднебеларускія. Як гучыць мова ў розных кутох.</p><span class="card-more">Паслухаць →</span></a>
    <a href="/about" class="card glass"><span class="card-emoji">🫱</span><h3>Пра праект</h3><p>Хто мы, навошта гэта ўсё, і як далучыцца да справы адраджэньня мовы.</p><span class="card-more">Пазнаёміцца →</span></a>
  </div>
</section>

<section class="section section-quote">
  <blockquote class="glass">
    <p>«Не пакідайце ж мовы нашае беларускае, каб не ўмёрлі!»</p>
    <cite>— Францішак Багушэвіч, 1891</cite>
  </blockquote>
</section>
"##;
    Html(layout("Галоўная", "index", body))
}

// ============================================================
//  Гісторыя
// ============================================================

async fn history() -> Html<String> {
    let events = [
        ("X–XIII ст.", "Полацкае княства", "Першыя пісьмовыя помнікі: граматы, надпісы, летапісы. Мова Полаччыны й Турава — жывая, гнуткая, з уласнымі рысамі."),
        ("XIV–XVII ст.", "Вялікае Княства Літоўскае", "Старабеларуская — афіцыйная мова ВКЛ. Статуты 1529, 1566, 1588 гадоў напісаныя па-беларуску. Гэта залаты век."),
        ("XVIII ст.", "Заняпад і паланізацыя", "Пасьля падзелаў Рэчы Паспалітай мова выціскаецца з афіцыйнага ўжытку. Але жыве ў вёсцы, у песьні, у казцы."),
        ("XIX ст.", "Адраджэньне", "Дунін-Марцінкевіч, Багушэвіч, Купала, Колас. Мова вяртаецца ў літаратуру — сакавітая, народная, сапраўдная."),
        ("1918", "БНР", "Беларуская Народная Рэспубліка абвяшчае беларускую мову дзяржаўнай. Тарашкевіч выдае «Беларускую граматыку для школ»."),
        ("1920-я", "Беларусізацыя", "Школы, газэты, навука — па-беларуску. Мова квітнее. Але нядоўга."),
        ("1933", "Рэформа й русіфікацыя", "Рэформа правапісу, набліжаная да расейскай. Клясычная традыцыя выціскаецца. Пачынаюцца рэпрэсіі супраць дзеячоў."),
        ("1990", "Дзяржаўная мова", "Вяртаньне статусу. Спробы адраджэньня, вяртаньне тарашкевіцы ў адукацыю й мэдыі."),
        ("1995", "Рэфэрэндум", "Статус мовы зноў звужаецца. Але мова жыве — у хатах, у сеціве, у новых гуртах."),
        ("Сёньня", "Жывая й свабодная", "Мова ў сеціве, у музыкаў, у перакладах, у новых кнігах. Яна ня ўмёрла й не ўмрэ, пакуль мы на ёй гаворым."),
    ];

    let mut items = String::new();
    for (year, title, text) in events {
        items.push_str(&format!(
            r##"<li class="glass"><span class="year">{}</span><h3>{}</h3><p>{}</p></li>"##,
            year, title, text
        ));
    }

    let body = format!(
        r##"
<section class="page-hero glass">
  <span class="badge">📜 Гісторыя</span>
  <h1>Дзесяць стагодзьдзяў жывога слова</h1>
  <p>Мова — гэта ня музэй. Яна жыве, зьмяняецца, змагаецца. Вось яе шлях.</p>
</section>
<section class="section"><ol class="timeline">{}</ol></section>
"##,
        items
    );
    Html(layout("Гісторыя", "history", &body))
}

// ============================================================
//  Граматыка
// ============================================================

async fn grammar() -> Html<String> {
    let body = r##"
<section class="page-hero glass">
  <span class="badge">🧩 Граматыка</span>
  <h1>Гнуткая, як лазіна</h1>
  <p>Шэсьць склонаў, два спражэньні, багатая сыстэма дзеясловаў. Вось асновы.</p>
</section>

<section class="section grid-2">
  <div class="glass panel">
    <h2>Назоўнік</h2>
    <p>Тры роды, два лікі, шэсьць склонаў. Клічны склон — асаблівая прыкмета беларускае мовы:</p>
    <ul class="example-list">
      <li><b>Назоўны:</b> <em>брат, сястра, поле</em></li>
      <li><b>Родны:</b> <em>брата, сястры, поля</em></li>
      <li><b>Давальны:</b> <em>брату, сястры, полю</em></li>
      <li><b>Вінавальны:</b> <em>брата, сястру, поле</em></li>
      <li><b>Творны:</b> <em>братам, сястрой, полем</em></li>
      <li><b>Месны:</b> <em>аб браце, аб сястры, аб полі</em></li>
      <li><b>Клічны:</b> <em>браце! сястро! поле!</em></li>
    </ul>
  </div>
  <div class="glass panel">
    <h2>Дзеяслоў</h2>
    <p>Два спражэньні, тры часы, шмат дзеепрыметнікаў і дзеепрыслоўяў:</p>
    <ul class="example-list">
      <li><b>I спражэньне:</b> <em>іду, ідзеш, ідзе</em></li>
      <li><b>II спражэньне:</b> <em>раблю, робіш, робіць</em></li>
      <li><b>Прошлы:</b> <em>рабіў, рабіла, рабілі</em></li>
      <li><b>Будучы:</b> <em>буду рабіць / зраблю</em></li>
      <li><b>Загадны:</b> <em>рабі!, рабіце!</em></li>
    </ul>
  </div>
  <div class="glass panel">
    <h2>Прыметнік</h2>
    <p>Згаджаецца з назоўнікам у родзе, ліку й склоне. Мае поўныя й кароткія формы:</p>
    <ul class="example-list">
      <li><em>прыгожы</em> — поўная форма</li>
      <li><em>прыгож</em> — кароткая</li>
      <li>Вышэйшая ступень: <em>прыгажэйшы</em></li>
      <li>Найвышэйшая: <em>найпрыгажэйшы</em></li>
    </ul>
  </div>
  <div class="glass panel">
    <h2>Лічэбнік</h2>
    <p>Складаная сыстэма зь лікавымі формамі назоўніка:</p>
    <ul class="example-list">
      <li><em>адзін стол</em></li>
      <li><em>два сталы</em></li>
      <li><em>тры сталы</em></li>
      <li><em>пяць сталоў</em></li>
      <li><em>шмат сталоў</em></li>
    </ul>
  </div>
</section>
"##;
    Html(layout("Граматыка", "grammar", body))
}

// ============================================================
//  Слоўнік
// ============================================================

async fn vocabulary() -> Html<String> {
    let words = [
        ("Вавёрка", "[ва-вёр-ка]", "Ня проста «бялка». У нашым слове чуецца нешта пяшчотнае, сваё."),
        ("Шуфель", "[шу-фель]", "Вялікая драўляная лапата. Ад яе паходзіць «шуфляваць» — варушыць, варочаць."),
        ("Мроіць", "[мро-іць]", "Бачыць у сьне, марыць наяве. Слова, у якім сон і мара — адно."),
        ("Вятрак", "[вят-рак]", "Млын, што круціцца ад ветру. У ім — і вецер, і хлеб, і воля."),
        ("Рунь", "[рунь]", "Маладыя ўсходы, першыя парасткі. Сымбаль пачатку й надзеі."),
        ("Агмень", "[аг-мень]", "Гарачае попел, што яшчэ тлее. Агонь, які ня згас, а схаваўся."),
        ("Крыжы", "[кры-жы]", "Прылада для касьбы. Не блытаць з крыжам-сымбалем — гэта рознае."),
        ("Варта", "[вар-та]", "І «варта рабіць», і «варта — ахова». Адно слова, два сэнсы, адна душа."),
        ("Няўрымсьлівы", "[ня-ў-рым-сьлі-вы]", "Той, каго не ўрымаць. Непакойны, жывы, сапраўдны."),
    ];

    let mut cards = String::new();
    for (word, pron, text) in words {
        cards.push_str(&format!(
            r##"<article class="word glass"><h3>{}</h3><p class="word-pron">{}</p><p>{}</p></article>"##,
            word, pron, text
        ));
    }

    let body = format!(
        r##"
<section class="page-hero glass">
  <span class="badge">📖 Слоўнік</span>
  <h1>Словы, якія гавораць самі за сябе</h1>
  <p>Тут жывуць словы, якія няма як перакласьці на іншыя мовы. Бо яны — нашыя.</p>
</section>
<section class="section"><div class="word-grid">{}</div></section>
"##,
        cards
    );
    Html(layout("Слоўнік", "vocabulary", &body))
}

// ============================================================
//  Тарашкевіца
// ============================================================

async fn taraskievica() -> Html<String> {
    let body = r##"
<section class="page-hero glass">
  <span class="badge">✒️ Тарашкевіца</span>
  <h1>Клясычны правапіс, якому 100+ гадоў</h1>
  <p>У 1918 годзе Баляслаў Тарашкевіч выдаў «Беларускую граматыку для школ». Яна стала асновай клясычнага правапісу.</p>
</section>

<section class="section grid-2">
  <div class="glass panel">
    <h2>Што такое тарашкевіца</h2>
    <p>Гэта клясычны варыянт беларускага правапісу, заснаваны на працы Тарашкевіча й на традыцыі, што ідзе ад пачатку XX стагодзьдзя.</p>
    <ul class="example-list">
      <li>Мяккія зычныя перад <em>е, ё, ю, я, і</em></li>
      <li>Ужываньне <em>-а</em> ў родным склоне множнага ліку</li>
      <li>Пасьлядоўнае <em>ў</em> пасьля галосных</li>
      <li>Канчаткі дзеясловаў <em>ісьці</em>, <em>весьці</em></li>
    </ul>
  </div>
  <div class="glass panel">
    <h2 id="нарк">Чым адрозьніваецца ад наркамаўкі</h2>
    <p>«Наркамаўка» — правапіс 1933 году, набліжаны да расейскага. Вось асноўныя адрозьненьні:</p>
    <table class="diff-table">
      <thead><tr><th>Тарашкевіца</th><th>Наркамаўка</th></tr></thead>
      <tbody>
        <tr><td>сьвет</td><td>свет</td></tr>
        <tr><td>зьмяніць</td><td>змяніць</td></tr>
        <tr><td>коньмі</td><td>каньмі</td></tr>
        <tr><td>ідэя</td><td>ідэя</td></tr>
        <tr><td>лічба</td><td>лічба</td></tr>
      </tbody>
    </table>
  </div>
  <div class="glass panel panel-wide">
    <h2 id="правілы">Асноўныя правілы</h2>
    <ol class="rules">
      <li>Пасьля галосных пішам <b>ў</b>, а ня <b>в</b>: <em>быў, казаў, хлеў</em>.</li>
      <li>Мяккі знак захоўваем там, дзе ён гістарычна быў: <em>сьвет, зьвер, цьвёрды</em>.</li>
      <li>У родным склоне множнага ліку — <b>-а</b> ці <b>-яў</b>: <em>кніг, сталоў, людзей</em>.</li>
      <li>Канчатак <b>-ае</b> ў прыметніках: <em>прыгожае, новае, маладое</em>.</li>
      <li>Захоўваем <b>ё</b> пасьля <em>р</em>: <em>зьвярнуцца, цьвёрды, мёрзлы</em>.</li>
    </ol>
  </div>
</section>
"##;
    Html(layout("Тарашкевіца", "taraskievica", body))
}

// ============================================================
//  Гаворкі
// ============================================================

async fn dialects() -> Html<String> {
    let body = r##"
<section class="page-hero glass">
  <span class="badge">🗺️ Гаворкі</span>
  <h1>Мова гучыць па-рознаму — і гэта цуд</h1>
  <p>Тры вялікія дыялектныя групы. У кожнай — свой рытм, свой сьпеў, свая душа.</p>
</section>

<section class="section grid-3">
  <article class="glass panel">
    <h2>Паўночна-ўсходнія</h2>
    <p>Віцебшчына, Магілёўшчына, паўночны ўсход Меншчыны.</p>
    <ul class="example-list">
      <li>«дзеканьне»: <em>дзеці</em> гучыць мякка</li>
      <li>канчатак <em>-е</em> ў назоўным склоне</li>
      <li>аканства слабейшае</li>
    </ul>
  </article>
  <article class="glass panel">
    <h2>Сярэднебеларускія</h2>
    <p>Цэнтар краіны, Меншчына, частка Гарадзеншчыны й Берасьцейшчыны.</p>
    <ul class="example-list">
      <li>гучная аснова літаратурнае мовы</li>
      <li>баляканьне: <em>бяда</em>, <em>пяць</em></li>
      <li>цеплівае, «мяккое» гучаньне</li>
    </ul>
  </article>
  <article class="glass panel">
    <h2>Паўднёва-заходнія</h2>
    <p>Гарадзеншчына, Берасьцейшчына, паўднёвы захад Меншчыны.</p>
    <ul class="example-list">
      <li>«цьвёрдае» <em>р</em>: <em>рака</em>, <em>радзіма</em></li>
      <li>націск на другім складзе з канца</li>
      <li>захаваныя архаізмы</li>
    </ul>
  </article>
</section>
"##;
    Html(layout("Гаворкі", "dialects", body))
}

// ============================================================
//  Пра праект
// ============================================================

async fn about() -> Html<String> {
    let body = r##"
<section class="page-hero glass">
  <span class="badge">🫱 Пра праект</span>
  <h1>Навошта ўсё гэта</h1>
  <p>Мы верым: мова жыве тады, калі на ёй гавораць, пішуць і думаюць. Гэты сайт — спроба зрабіць мову зручнай, прыгожай і сучаснай.</p>
</section>

<section class="section grid-2">
  <div class="glass panel"><h2>Хто мы</h2>
    <p>Неабыякавыя людзі, якія любяць беларускую мову. Мы ня маем грантаў, ня маем заказчыкаў. Маем толькі жаданьне, каб мова гучала — і ў сеціве, і ў хаце.</p>
  </div>
  <div class="glass panel"><h2>Наш прынцып</h2>
    <p>Клясычны правапіс (тарашкевіца), максымальная дэрусіфікацыя, жывая мова без стылізацыі пад «вёску». Мова — гэта ня фальклёр. Гэта мы, сёньняшнія.</p>
  </div>
  <div class="glass panel"><h2 id="кантакты">Кантакты</h2>
    <p>Пішы на <a href="mailto:hello@mova.by">hello@mova.by</a>, калі ёсьць што сказаць, што паправіць, ці што дадаць.</p>
  </div>
  <div class="glass panel"><h2 id="ліцэнзія">Ліцэнзія</h2>
    <p>Код — на GitHub пад MIT. Тэксты — вольныя для выкарыстаньня з спасылкай.</p>
  </div>
</section>
"##;
    Html(layout("Пра праект", "about", body))
}

// ============================================================
//  Хэлскек
// ============================================================

async fn health_check() -> &'static str {
    "OK"
}

// ============================================================
//  Галоўная функцыя
// ============================================================

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/history", get(history))
        .route("/grammar", get(grammar))
        .route("/vocabulary", get(vocabulary))
        .route("/taraskievica", get(taraskievica))
        .route("/dialects", get(dialects))
        .route("/about", get(about))
        .nest_service("/static", ServeDir::new("static"));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Сэрвэр на http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}