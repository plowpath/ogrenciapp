#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod database;
mod ui;

use crate::api::*;
use crate::ui::*;
use anyhow::Result;
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
) -> Result<[i64; 4], anyhow::Error> {
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

    let para = [borc, aylik, kalanborc, kalantaksit];

    Ok(para)
}

/// Taksit ödemesini yapan öğrencinin taksit bilgisini güncellemeye yarayan fonksiyon
fn calculate_update(aylik: i64, kalantaksit: i64) -> i64 {
    aylik * kalantaksit
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
