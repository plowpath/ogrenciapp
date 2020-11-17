#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod database;

use anyhow::Result;
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
    taksit: i64,
    borc: i64,
    kalanborc: i64,
    kalantaksit: i64,
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
            taksit: 12,
            borc: 500,
            kalanborc: 300,
            kalantaksit: 6,
        }
    }
}

#[get("/api")]
fn api() -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    let a = database::data_hazirlama(&conn)?;
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
    Ok(content::Json(lel))
}

#[get("/api/send?<isim>&<soyisim>&<fatura_adres>&<veli_adres>&<telefon>&<yemek>&<servis>&<turkce>&<matematik>&<fen>&<sosyal>&<taksit>")]
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
    taksit: i64,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
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
        taksit,
        borc: 0,
        kalantaksit: taksit,
        kalanborc: 0,
    };

    let fixingstuff = calculate(
        yemek,
        servis,
        turkce,
        matematik,
        fen,
        sosyal,
        taksit,
        me.borc,
        me.kalanborc,
        me.kalantaksit,
    )?;
    let fnborc = fixingstuff[1];
    let fnkalanborc = fixingstuff[2];
    let fnkalantaksit = fixingstuff[3];

    let conn = database::sqlite_connection()?;
    let checkphonenumber: Result<i64, rusqlite::Error> = conn.query_row(
        r#"SELECT * FROM Musteri WHERE telefon=?"#,
        params![me.telefon],
        |row| row.get(5),
    );
    if checkphonenumber.is_err() {
        conn.execute(
            "INSERT INTO Musteri (isim, soyisim, fatura_adres, veli_adres, telefon, yemek, servis, turkce, matematik, fen, sosyal, taksit, borc, kalanborc, kalantaksit ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
            params![
                me.isim,
                me.soyisim,
                me.fatura_adres,
                me.veli_adres,
                me.telefon,
                me.yemek,
                me.servis,
                me.turkce,
                me.matematik,
                me.fen,
                me.sosyal,
                me.taksit,
                fnborc,
                fnkalanborc,
                fnkalantaksit,
            ],
        )?;
        let b = json!({"success": true});
        Ok(content::Json(b.to_string()))
    } else {
        let b = json!({"success": false});
        Ok(content::Json(b.to_string()))
    }
}

#[get("/api/update?<tel>&<kolum>&<yenim>")]
fn update(
    tel: i64,
    kolum: String,
    yenim: String,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    if kolum == "isim" || kolum == "soyisim" || kolum == "fatura_adres" || kolum == "veli_adres" {
        let hereismysql = "UPDATE Musteri SET ".to_string()
            + kolum.as_str()
            + "='"
            + yenim.as_str()
            + "' WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![])?;
        let b = json!({"success": true});
        Ok(content::Json(b.to_string()))
    } else if kolum == "telefon" && yenim.parse::<i64>().is_ok() {
        let hereismysql = "UPDATE Musteri SET ".to_string()
            + kolum.as_str()
            + "="
            + yenim.as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![])?;
        let b = json!({"success": true});
        Ok(content::Json(b.to_string()))
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
        conn.execute(hereismysql.as_str(), params![])?;
        let c = json!({"success": true});
        Ok(content::Json(c.to_string()))
    } else {
        let b = json!({"success": false});
        Ok(content::Json(b.to_string()))
    }
}

#[get("/api/nuke")]
fn nuke() -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    conn.execute("DELETE FROM Musteri", params![])?;
    let b = json!({"success": true});
    Ok(content::Json(b.to_string()))
}

#[get("/api/delete?<tel>")]
fn delete(tel: i64) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    println!("{}", tel);
    conn.execute("DELETE FROM Musteri WHERE telefon=?", params![tel])?;
    let b = json!({"success": true});
    Ok(content::Json(b.to_string()))
}

#[get("/api/getstudent?<tel>")]
fn getstudent(
    tel: i64,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    let mut statement = conn.prepare("SELECT * FROM Musteri WHERE telefon=?")?;
    let one_student = statement.query_map(params![tel], |row| {
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
            kalanborc: row.get(14)?,
            kalantaksit: row.get(15)?,
        })
    })?;
    let mut bar = Vec::new();
    for person in one_student {
        let footar = json!(person?);
        bar.push(footar);
    }
    let mut lel = String::new();
    lel += "[";
    for b in bar {
        lel += format!("{},", b).as_str();
    }
    lel.pop();
    if lel.is_empty() {
        println!("öğrenci bulunamadı");
        lel = r#"{"success": false}"#.to_string()
    } else {
        lel += "]";
    }
    Ok(content::Json(lel))
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(tablo))
}

#[get("/yeni")]
fn yeni() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/yeni.html")?))
}

#[get("/guncelle")]
fn guncelle() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/guncelle.html")?))
}

#[get("/tablo")]
fn tablo() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/tablo.html")?))
}

#[get("/sil")]
fn sil() -> Result<rocket::response::content::Html<std::string::String>, anyhow::Error> {
    Ok(content::Html(fs::read_to_string("ui/sil.html")?))
}

#[allow(clippy::too_many_arguments)]
fn calculate(
    yemek: bool,
    servis: bool,
    turkce: bool,
    matematik: bool,
    fen: bool,
    sosyal: bool,
    taksit: i64,
    mut borc: i64,
    kalanborc: i64,
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
    let para = [taksit, borc, kalanborc, kalantaksit];

    Ok(para)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, api, send, update, delete, getstudent, nuke, yeni, guncelle, tablo, sil],
        )
        .launch();
}
