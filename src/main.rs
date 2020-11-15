#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod database;

use rocket::response::{content, Redirect};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

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

#[get("/api")]
fn api() -> rocket::response::content::Json<std::string::String> {
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

#[get("/api/send?<isim>&<soyisim>&<fatura_adres>&<veli_adres>&<telefon>&<yemek>&<servis>&<turkce>&<matematik>&<fen>&<sosyal>")]
#[allow(clippy::too_many_arguments)]
fn send(
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
) -> rocket::response::content::Json<std::string::String> {
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
    let checkphonenumber: Result<i64, rusqlite::Error> = conn.query_row(
        r#"SELECT * FROM Musteri WHERE telefon=?"#,
        params![me.telefon],
        |row| row.get(5),
    );
    if checkphonenumber.is_err() {
        conn.execute(
            "INSERT INTO Musteri (isim, soyisim, fatura_adres, veli_adres, telefon, yemek, servis, turkce, matematik, fen, sosyal) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![me.isim, me.soyisim, me.fatura_adres, me.veli_adres, me.telefon, me.yemek, me.servis, me.turkce, me.matematik, me.fen, me.sosyal],
        ).expect("telefon kontrolünde sıkıntı");
        let b = json!({"success": true});
        content::Json(b.to_string())
    } else {
        let b = json!({"success": false});
        content::Json(b.to_string())
    }
}

#[get("/api/update?<tel>&<kolum>&<yenim>")]
fn update(
    tel: i64,
    kolum: String,
    yenim: String,
) -> rocket::response::content::Json<std::string::String> {
    let conn = database::sqlite_connection();
    if kolum == "isim" || kolum == "soyisim" || kolum == "fatura_adres" || kolum == "veli_adres" {
        let hereismysql = "UPDATE Musteri SET ".to_string()
            + kolum.as_str()
            + "='"
            + yenim.as_str()
            + "' WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![]).unwrap();
        let b = json!({"success": true});
        content::Json(b.to_string())
    } else if kolum == "telefon" && yenim.parse::<i64>().is_ok() {
        let hereismysql = "UPDATE Musteri SET ".to_string()
            + kolum.as_str()
            + "="
            + yenim.as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![]).unwrap();
        let b = json!({"success": true});
        content::Json(b.to_string())
    } else if (kolum == "yemek"
        || kolum == "servis"
        || kolum == "turkce"
        || kolum == "matematik"
        || kolum == "fen"
        || kolum == "sosyal")
        && (yenim == "0" || yenim == "1")
    {
        let hereismysql = "UPDATE Musteri SET ".to_string()
            + kolum.as_str()
            + "="
            + yenim.as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![]).unwrap();
        let c = json!({"success": true});
        content::Json(c.to_string())
    } else {
        let b = json!({"success": false});
        content::Json(b.to_string())
    }
}

#[get("/api/nuke")]
fn nuke() -> rocket::response::content::Json<std::string::String> {
    let conn = database::sqlite_connection();
    conn.execute("DELETE FROM Musteri", params![])
        .expect("müşterileri silemedik");
    let b = json!({"success": true});
    content::Json(b.to_string())
}

#[get("/api/delete?<tel>")]
fn delete(tel: i64) -> rocket::response::content::Json<std::string::String> {
    let conn = database::sqlite_connection();
    println!("{}", tel);
    conn.execute("DELETE FROM Musteri WHERE telefon=?", params![tel])
        .expect("müşterileri silemedik");
    let b = json!({"success": true});
    content::Json(b.to_string())
}

#[get("/api/getstudent?<tel>")]
fn getstudent(tel: i64) -> rocket::response::content::Json<std::string::String> {
    let conn = database::sqlite_connection();
    let mut statement = conn
        .prepare("SELECT * FROM Musteri WHERE telefon=?")
        .expect("cant create the statemtn");
    let one_student = statement
        .query_map(params![tel], |row| {
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
        .unwrap();
    let mut bar = Vec::new();
    for person in one_student {
        let footar = json!(person.expect("json serializasyonu"));
        bar.push(footar);
    }
    let mut lel = String::new();
    lel += "[";
    for b in bar {
        lel += format!("{},", b).as_str();
    }
    lel.pop();
    if lel.is_empty() {
        println!("öğrenci bulunamadı")
    } else {
        lel += "]";
    }
    content::Json(lel)
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(tablo))
}

#[get("/yeni")]
fn yeni() -> rocket::response::content::Html<std::string::String> {
    content::Html(fs::read_to_string("ui/yeni.html").expect("kayıt sayfası yok"))
}

#[get("/guncelle")]
fn guncelle() -> rocket::response::content::Html<std::string::String> {
    content::Html(fs::read_to_string("ui/guncelle.html").expect("güncelleme sayfası yok"))
}

#[get("/tablo")]
fn tablo() -> rocket::response::content::Html<std::string::String> {
    content::Html(fs::read_to_string("ui/tablo.html").expect("tablo sayfası yok"))
}

#[get("/sil")]
fn sil() -> rocket::response::content::Html<std::string::String> {
    content::Html(fs::read_to_string("ui/sil.html").expect("silme sayfası yok"))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, api, send, update, delete, getstudent, nuke, yeni, guncelle, tablo, sil],
        )
        .launch();
}
