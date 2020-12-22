use crate::Ogrenci;
use anyhow::Result;
use rusqlite::{params, Connection};
use serde_json::json;
use std::path::Path;

/// database olarak sqlite kullanmaktayız, bu fonksiyon varsa database'e bağlanır yoksa bir tane oluşturup bağlanır
pub fn sqlite_connection() -> Result<Connection, anyhow::Error> {
    let conn;
    if Path::new("ogrenciler.db").exists() {
        conn = Connection::open("ogrenciler.db")?;
    } else {
        conn = Connection::open("ogrenciler.db")?;
        conn.execute(
            "CREATE TABLE Ogrenci (
                          id              INTEGER PRIMARY KEY,
                          isim            TEXT NOT NULL,
                          soyisim         TEXT NOT NULL,
                          fatura_adres    TEXT NOT NULL,
                          veli_adres      TEXT NOT NULL,
                          telefon         INTEGER,
                          yemek           INTEGER,
                          servis          INTEGER,
                          turkce          INTEGER,
                          matematik       INTEGER,
                          fen             INTEGER,
                          sosyal          INTEGER,
                          taksit          INTEGER,
                          borc            INTEGER,
                          aylik           INTEGER,
                          kalan_borc       INTEGER,
                          kalan_taksit     INTEGER
                )",
            params![],
        )?;
    }

    Ok(conn)
}

/// database'imizdeki tüm satır ve sütunları sorgulayan fonksiyonumuz (tablomuzu oluşturmaktadır)
pub fn data_hazirlama(conn: &Connection) -> Result<Vec<serde_json::Value>, anyhow::Error> {
    let mut stmt = conn.prepare("SELECT * FROM Ogrenci")?;
    let ogrenci_iter = stmt.query_map(params![], |row| {
        Ok(Ogrenci {
            id: row.get(0)?,
            isim: row.get(1)?,
            soyisim: row.get(2)?,
            fatura_adres: row.get(3)?,
            veli_adres: row.get(4)?,
            telefon: row.get(5)?,
            yemek: row.get(6)?,
            servis: row.get(7)?,
            turkce: row.get(8)?,
            matematik: row.get(9)?,
            fen: row.get(10)?,
            sosyal: row.get(11)?,
            taksit: row.get(12)?,
            borc: row.get(13)?,
            aylik: row.get(14)?,
            kalan_borc: row.get(15)?,
            kalan_taksit: row.get(16)?,
        })
    })?;
    let mut ogrenciler = Vec::new();
    for ogrenci in ogrenci_iter {
        ogrenciler.push(json!(ogrenci?));
    }

    Ok(ogrenciler)
}

/// en ilkel haliyle raporlama yapmamızı sağlayan fonksiyon **todo**
pub fn hesap(istenilen: &str) -> Result<i64, anyhow::Error> {
    let conn = sqlite_connection()?;
    let sqlsorgu = "SELECT SUM(".to_string() + istenilen + ") FROM Ogrenci";
    let sonuc: i64 = conn.query_row(sqlsorgu.as_str(), params![], |row| row.get(0))?;

    Ok(sonuc)
}

type Dondur = (bool, bool, bool, bool, bool, bool, i64, i64);
pub fn calculate_on_update(telefon: i64) -> Result<Dondur, anyhow::Error> {
    let conn = sqlite_connection()?;
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

    let yemek_sonuc: bool =
        conn.query_row(yemek_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;
    let servis_sonuc: bool =
        conn.query_row(servis_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;
    let turkce_sonuc: bool =
        conn.query_row(turkce_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;
    let matematik_sonuc: bool =
        conn.query_row(matematik_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;
    let fen_sonuc: bool =
        conn.query_row(fen_sqlsorgu.as_str(), rusqlite::params![], |row| row.get(0))?;
    let sosyal_sonuc: bool =
        conn.query_row(sosyal_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;
    let taksit_sonuc: i64 =
        conn.query_row(taksit_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;
    let kalan_taksit_sonuc: i64 =
        conn.query_row(kalan_taksit_sqlsorgu.as_str(), rusqlite::params![], |row| {
            row.get(0)
        })?;

    Ok((
        yemek_sonuc,
        servis_sonuc,
        turkce_sonuc,
        matematik_sonuc,
        fen_sonuc,
        sosyal_sonuc,
        taksit_sonuc,
        kalan_taksit_sonuc,
    ))
}
