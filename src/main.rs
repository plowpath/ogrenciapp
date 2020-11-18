//! ÖĞRENCİ APP
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

/// Müşteri yapımız
#[derive(Serialize, Deserialize, Debug)]
pub struct Musteri {
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
    kalanborc: i64,
    kalantaksit: i64,
}

/// Yeni müşteri eklendiğinde seçtiği seçeneklere göre borcunu hesaplayan fonksiyon
#[allow(clippy::too_many_arguments)]
fn calculate_new(
    yemek: bool,
    servis: bool,
    turkce: bool,
    matematik: bool,
    fen: bool,
    sosyal: bool,
    taksit: i64,
    mut borc: i64,
    kalantaksit: i64,
) -> Result<[i64; 4], anyhow::Error> {
    if yemek {
        borc += 300
    } else {
        borc += 0
    }
    if servis {
        borc += 200
    } else {
        borc += 0
    }
    if turkce {
        borc += 2500
    } else {
        borc += 0
    }
    if matematik {
        borc += 2500
    } else {
        borc += 0
    }
    if fen {
        borc += 2500
    } else {
        borc += 0
    }
    if sosyal {
        borc += 2500
    } else {
        borc += 0
    }
    let aylik = borc / taksit;
    let kalanborc;
    if taksit == kalantaksit {
        kalanborc = borc
    } else {
        kalanborc = kalantaksit * aylik
    }

    let para = [borc, aylik, kalanborc, kalantaksit];
    println!("{:?}", para);

    Ok(para)
}

/// Taksit ödemesini yapan müşterinin taksit bilgisini güncellemeye yarayan fonksiyon
fn calculate_update(aylik: i64, kalantaksit: i64) -> (i64, i64) {
    let kalanborc = aylik * kalantaksit;
    (kalantaksit, kalanborc)
}

/// main fonksiyonumuz sadece rocketi çalıştırmalıdır
fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index, api, send, update, delete, getstudent, nuke, api_data, yeni, guncelle,
                tablo, sil
            ],
        )
        .launch();
}
