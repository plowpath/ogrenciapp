use anyhow::Result;
use rocket::response::{content::Html, Redirect};
use std::fs::read_to_string;

/// indeksimiz şimdilik tablo sayfamıza redirect etmektedir
#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!(tablo))
}

/// yeni öğrenci kaydı sayfamız
#[get("/yeni")]
pub fn yeni() -> Result<Html<String>> {
    Ok(Html(read_to_string("ui/yeni.html")?))
}

/// öğrenci güncelleme sayfamız
#[get("/guncelle")]
pub fn guncelle() -> Result<Html<String>> {
    Ok(Html(read_to_string("ui/guncelle.html")?))
}

/// tablo sayfamız
#[get("/tablo")]
pub fn tablo() -> Result<Html<String>> {
    Ok(Html(read_to_string("ui/tablo.html")?))
}

/// öğrenci silme sayfamız
#[get("/sil")]
pub fn sil() -> Result<Html<String>> {
    Ok(Html(read_to_string("ui/sil.html")?))
}
