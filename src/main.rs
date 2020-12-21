#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod database;
mod ui;

use crate::api::*;
use crate::ui::*;
use database::sqlite_connection;
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

fn calculate_update_lesson(telefon: i64) {
    let conn = sqlite_connection().unwrap();
    let yemek_sqlsorgu =
        "SELECT yemek FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let servis_sqlsorgu =
        "SELECT servis FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let turkce_sqlsorgu =
        "SELECT turkce FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let matematik_sqlsorgu =
        "SELECT matematik FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let fen_sqlsorgu =
        "SELECT fen FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let sosyal_sqlsorgu =
        "SELECT sosyal FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let taksit_sqlsorgu =
        "SELECT taksit FROM Ogrenci WHERE telefon=".to_string() + telefon.to_string().as_str();
    let kalan_taksit_sqlsorgu = "SELECT kalan_taksit FROM Ogrenci WHERE telefon=".to_string()
        + telefon.to_string().as_str();

    let yemek_sonuc: bool = conn
        .query_row(yemek_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();
    let servis_sonuc: bool = conn
        .query_row(servis_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();
    let turkce_sonuc: bool = conn
        .query_row(turkce_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();
    let matematik_sonuc: bool = conn
        .query_row(matematik_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();
    let fen_sonuc: bool = conn
        .query_row(fen_sqlsorgu.as_str(), rusqlite::params![], |row| row.get(0))
        .unwrap();
    let sosyal_sonuc: bool = conn
        .query_row(sosyal_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();
    let taksit_sonuc: i64 = conn
        .query_row(taksit_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();
    let kalan_taksit_sonuc: i64 = conn
        .query_row(kalan_taksit_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })
        .unwrap();

    let mut borc: i64 = 0;

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

    let guncelle_bakalim_borc = "UPDATE Ogrenci SET borc=".to_string()
        + borc.to_string().as_str()
        + " WHERE telefon="
        + telefon.to_string().as_str();
    conn.execute(guncelle_bakalim_borc.as_str(), rusqlite::params![])
        .unwrap();

    let guncelle_bakalim_aylik = "UPDATE Ogrenci SET aylik=".to_string()
        + aylik.to_string().as_str()
        + " WHERE telefon="
        + telefon.to_string().as_str();
    conn.execute(guncelle_bakalim_aylik.as_str(), rusqlite::params![])
        .unwrap();

    let guncelle_bakalim_kalan_borc = "UPDATE Ogrenci SET kalan_borc=".to_string()
        + kalan_borc.to_string().as_str()
        + " WHERE telefon="
        + telefon.to_string().as_str();
    conn.execute(guncelle_bakalim_kalan_borc.as_str(), rusqlite::params![])
        .unwrap();

    println!("{}", yemek_sonuc)
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
