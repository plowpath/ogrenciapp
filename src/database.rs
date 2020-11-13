use crate::Musteri;
use rusqlite::{params, Connection};
use serde_json::json;
use std::path::Path;

pub fn sqlite_connection() -> Connection {
    let conn;
    if Path::new("deneme.db").exists() {
        conn = Connection::open("deneme.db").expect("olması gereken database yok");
    } else {
        conn = Connection::open("deneme.db").expect("olmaması gereken database oluşturulamıyor");
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
                          sosyal          INTEGER

                          )",
            params![],
        )
        .expect("yeni database oluştururken data girişini yapamadık");
    }
    conn
}

pub fn data_hazirlama(conn: &Connection) -> Vec<serde_json::Value> {
    let mut stmt = conn
        .prepare("SELECT * FROM Musteri")
        .expect("tüm listeye select atamadık");
    let person_iter = stmt
        .query_map(params![], |row| {
            Ok(Musteri {
                id: row.get(0).expect("id sütunu"),
                isim: row.get(1).expect("isim sütunu"),
                soyisim: row.get(2).expect("soyisim sütunu"),
                fatura_adres: row.get(3).expect("fatura_adres sütunu"),
                veli_adres: row.get(4).expect("veli_adres sütunu"),
                telefon: row.get(5).expect("telefon sütunu"),
                yemek: row.get(6).expect("yemek sütunu"),
                servis: row.get(7).expect("servis sütunu"),
                turkce: row.get(8).expect("turkce sütunu"),
                matematik: row.get(9).expect("matematik sütunu"),
                fen: row.get(10).expect("fen sütunu"),
                sosyal: row.get(11).expect("sosyal sütunu"),
            })
        })
        .expect("rusqlite tamamını iter ederken sıkıntı");
    let mut bar = Vec::new();
    for person in person_iter {
        let footar = json!(person.expect("json serializasyonu"));
        bar.push(footar);
    }
    bar
}
