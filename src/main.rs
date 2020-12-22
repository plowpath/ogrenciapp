#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod database;
mod ui;

use crate::api::*;
use crate::ui::*;
use database::calculate_on_update;
use serde::{Deserialize, Serialize};

/// Öğrenci yapımız
#[derive(Serialize, Deserialize, Debug)]
pub struct Ogrenci {
    id: i32,
    isim: String,
    soyisim: String,
    fatura_adres: String,
    veli_adres: String,
    telefon: i64,
    yemek: bool,
    servis: bool,
    turkce: bool,
    matematik: bool,
    fen: bool,
    sosyal: bool,
    taksit: i64,
    borc: i64,
    aylik: i64,
    kalan_borc: i64,
    kalan_taksit: i64,
}

/// Yeni öğrenci eklendiğinde seçtiği seçeneklere göre borcunu hesaplayan fonksiyon
fn calculate_new(
    yemek: bool,
    servis: bool,
    turkce: bool,
    matematik: bool,
    fen: bool,
    sosyal: bool,
    taksit: i64,
) -> [i64; 4] {
    let mut borc = 0;
    let kalantaksit = taksit;

    match yemek {
        true => borc += 300,
        false => borc += 0,
    }
    match servis {
        true => borc += 300,
        false => borc += 0,
    }
    match turkce {
        true => borc += 300,
        false => borc += 0,
    }
    match matematik {
        true => borc += 300,
        false => borc += 0,
    }
    match fen {
        true => borc += 300,
        false => borc += 0,
    }
    match sosyal {
        true => borc += 300,
        false => borc += 0,
    }

    let aylik = borc / taksit;
    let mut kalanborc = 0;

    if taksit == kalantaksit {
        kalanborc = borc
    }

    [borc, aylik, kalanborc, kalantaksit]
}

/// Taksit ödemesini yapan öğrencinin taksit bilgisini güncellemeye yarayan fonksiyon
fn calculate_update(aylik: i64, kalantaksit: i64) -> i64 {
    aylik * kalantaksit
}

fn calculate_update_lesson(telefon: i64) -> Result<(i64, i64, i64), anyhow::Error> {
    let mut borc: i64 = 0;
    let (
        yemek_sonuc,
        servis_sonuc,
        turkce_sonuc,
        matematik_sonuc,
        fen_sonuc,
        sosyal_sonuc,
        taksit_sonuc,
        kalan_taksit_sonuc,
    ) = calculate_on_update(telefon)?;

    match yemek_sonuc {
        true => borc += 300,
        false => borc += 0,
    }
    match servis_sonuc {
        true => borc += 300,
        false => borc += 0,
    }
    match turkce_sonuc {
        true => borc += 300,
        false => borc += 0,
    }
    match matematik_sonuc {
        true => borc += 300,
        false => borc += 0,
    }
    match fen_sonuc {
        true => borc += 300,
        false => borc += 0,
    }
    match sosyal_sonuc {
        true => borc += 300,
        false => borc += 0,
    }

    let aylik = borc / taksit_sonuc;
    let kalan_borc = calculate_update(aylik, kalan_taksit_sonuc);

    Ok((borc, aylik, kalan_borc))
}

/// main fonksiyonumuz sadece rocketi çalıştırmalıdır
fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index, yeni, guncelle, tablo, sil, api, new, update, delete, getstudent, nuke,
                api_data
            ],
        )
        .launch();
}
