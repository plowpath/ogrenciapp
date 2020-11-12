#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod database;

use rocket::response::content;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Musteri {
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
}

impl Musteri {
    fn _new() -> Self {
        Musteri {
            id: 0,
            isim: "hello".to_string(),
            soyisim: "world".to_string(),
            fatura_adres: "ankara".to_string(),
            veli_adres: "istanbul".to_string(),
            telefon: 5672341212,
            yemek: true,
            servis: false,
            turkce: true,
            matematik: true,
            fen: true,
            sosyal: false,
        }
    }
}

#[get("/")]
fn index() -> rocket::response::content::Json<std::string::String> {
    let conn = database::sqlite_connection();
    let a = database::data_hazirlama(&conn);
    let mut lel = String::new();
    lel += "[";
    for b in a {
        lel += format!("{},", b).as_str();
    }
    lel.pop();
    if lel.is_empty() {
        println!("nothing to do")
    } else {
        lel += "]";
    }
    content::Json(lel)
}

#[get("/nuke")]
fn nuke() -> String {
    let conn = database::sqlite_connection();
    conn.execute("DELETE FROM Musteri", params![]).unwrap();
    "nuked".to_string()
}

fn _data_insert(conn: &Connection, me: &Musteri) {
    conn.execute(
        "INSERT INTO Musteri (isim, soyisim, fatura_adres, veli_adres, telefon, yemek, servis, turkce, matematik, fen, sosyal) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
        params![me.isim, me.soyisim, me.fatura_adres, me.veli_adres, me.telefon, me.yemek, me.servis, me.turkce, me.matematik, me.fen, me.sosyal],
    ).unwrap();
}

#[get("/gonder?<isim>&<soyisim>&<fatura_adres>&<veli_adres>&<telefon>&<yemek>&<servis>&<turkce>&<matematik>&<fen>&<sosyal>")]
#[allow(clippy::too_many_arguments)]
fn proper_data_insert(
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
) -> String {
    let me = Musteri {
        id: 0,
        isim,
        soyisim,
        fatura_adres,
        veli_adres,
        telefon,
        yemek,
        servis,
        turkce,
        matematik,
        fen,
        sosyal,
    };
    let conn = database::sqlite_connection();
    conn.execute(
        "INSERT INTO Musteri (isim, soyisim, fatura_adres, veli_adres, telefon, yemek, servis, turkce, matematik, fen, sosyal) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
        params![me.isim, me.soyisim, me.fatura_adres, me.veli_adres, me.telefon, me.yemek, me.servis, me.turkce, me.matematik, me.fen, me.sosyal],
    ).unwrap();
    "done".to_string()
}

#[get("/new")]
fn new() -> rocket::response::content::Html<std::string::String> {
    content::Html(fs::read_to_string("ui/new.html").expect("dosya yok"))
}

#[get("/table")]
fn table() -> rocket::response::content::Html<std::string::String> {
    content::Html(fs::read_to_string("ui/table.html").expect("dosya yok"))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, proper_data_insert, nuke, new, table])
        .launch();
}
