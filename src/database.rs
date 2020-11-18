use crate::Musteri;
use anyhow::Result;
use rusqlite::{params, Connection};
use serde_json::json;
use std::path::Path;

/// database olarak sqlite kullanmaktayız, bu fonksiyon varsa database'e bağlanır yoksa bir tane oluşturup bağlanır
pub fn sqlite_connection() -> Result<Connection, anyhow::Error> {
    let conn;
    if Path::new("deneme.db").exists() {
        conn = Connection::open("deneme.db")?;
    } else {
        conn = Connection::open("deneme.db")?;
        conn.execute(
            "CREATE TABLE Musteri (
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
                          kalanborc       INTEGER,
                          kalantaksit     INTEGER

                          )",
            params![],
        )?;
    }
    Ok(conn)
}

/// database'imizdeki tüm satır ve sütunları sorgulayan fonksiyonumuz (tablomuzu oluşturmaktadır)
pub fn data_hazirlama(conn: &Connection) -> Result<Vec<serde_json::Value>, anyhow::Error> {
    let mut stmt = conn.prepare("SELECT * FROM Musteri")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Musteri {
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
            kalanborc: row.get(15)?,
            kalantaksit: row.get(16)?,
        })
    })?;
    let mut bar = Vec::new();
    for person in person_iter {
        let footar = json!(person?);
        bar.push(footar);
    }
    Ok(bar)
}

/// en ilkel haliyle raporlama yapmamızı sağlayan fonksiyon **todo**
pub fn hesap(kalanborc: String) -> Result<i64, anyhow::Error> {
    let conn = sqlite_connection()?;
    let sqlsorgu = "SELECT SUM(".to_string() + kalanborc.as_str() + ") FROM Musteri";
    let stt: i64 = conn.query_row(sqlsorgu.as_str(), params![], |row| row.get(0))?;
    println!("{}", stt);

    Ok(stt)
}
