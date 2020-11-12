use crate::Musteri;
use rusqlite::{params, Connection};
use serde_json::json;
use std::path::Path;

pub fn sqlite_connection() -> Connection {
    let conn;
    if Path::new("deneme.db").exists() {
        conn = Connection::open("deneme.db").unwrap();
    } else {
        conn = Connection::open("deneme.db").unwrap();
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
        .unwrap();
    }
    conn
}

pub fn data_hazirlama(conn: &Connection) -> Vec<serde_json::Value> {
    let mut stmt = conn.prepare("SELECT * FROM Musteri").unwrap();
    let person_iter = stmt
        .query_map(params![], |row| {
            Ok(Musteri {
                id: row.get(0).unwrap(),
                isim: row.get(1).unwrap(),
                soyisim: row.get(2).unwrap(),
                fatura_adres: row.get(3).unwrap(),
                veli_adres: row.get(4).unwrap(),
                telefon: row.get(5).unwrap(),
                yemek: row.get(6).unwrap(),
                servis: row.get(7).unwrap(),
                turkce: row.get(8).unwrap(),
                matematik: row.get(9).unwrap(),
                fen: row.get(10).unwrap(),
                sosyal: row.get(11).unwrap(),
            })
        })
        .unwrap();
    let mut bar = Vec::new();
    for person in person_iter {
        let footar = json!(person.unwrap());
        bar.push(footar);
    }
    bar
}
