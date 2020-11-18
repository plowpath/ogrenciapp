use rocket::response::{content, Redirect};
use std::fs;

/// indeksimiz şimdilik tablo sayfamıza redirect etmektedir
#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!(tablo))
}

/// yeni müşteri kaydı sayfamız
#[get("/yeni")]
pub fn yeni() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/yeni.html")?))
}

/// müşteri güncelleme sayfamız
#[get("/guncelle")]
pub fn guncelle() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/guncelle.html")?))
}

/// tablo sayfamız
#[get("/tablo")]
pub fn tablo() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/tablo.html")?))
}

/// müşteri silme sayfamız
#[get("/sil")]
pub fn sil() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/sil.html")?))
}
