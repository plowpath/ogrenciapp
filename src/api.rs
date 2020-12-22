use crate::{
    calculate_new, calculate_update, calculate_update_lesson,
    database::{self, hesap},
    Ogrenci,
};
use rocket::response::content;
use rusqlite::params;
use serde_json::json;

/// tüm databesi döndüren ana api
#[get("/api")]
pub fn api() -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    let ogrenciler_iter = database::data_hazirlama(&conn)?;
    let mut ogrenciler = String::new();
    ogrenciler += "[";
    for ogrenci in ogrenciler_iter {
        ogrenciler += format!("{},", ogrenci).as_str();
    }
    ogrenciler.pop();
    if !ogrenciler.is_empty() {
        ogrenciler += "]";
    }
    Ok(content::Json(ogrenciler))
}

/// öğrenci eklememizi sağlayan apimiz
#[get("/api/new?<isim>&<soyisim>&<fatura_adres>&<veli_adres>&<telefon>&<yemek>&<servis>&<turkce>&<matematik>&<fen>&<sosyal>&<taksit>")]
#[allow(clippy::too_many_arguments)]
pub fn new(
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
    let me = Ogrenci {
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
        aylik: 0,
        borc: 0,
        kalan_borc: 0,
        kalan_taksit: taksit,
    };

    let fixingstuff = calculate_new(yemek, servis, turkce, matematik, fen, sosyal, taksit);
    let fnborc = fixingstuff[0];
    let fnaylik = fixingstuff[1];
    let fnkalanborc = fixingstuff[2];
    let fnkalantaksit = fixingstuff[3];

    let conn = database::sqlite_connection()?;
    let checkphonenumber: Result<i64, rusqlite::Error> = conn.query_row(
        r#"SELECT * FROM Ogrenci WHERE telefon=?"#,
        params![me.telefon],
        |row| row.get(5),
    );
    if checkphonenumber.is_err() {
        conn.execute(
            "INSERT INTO Ogrenci (isim, soyisim, fatura_adres, veli_adres, telefon, yemek, servis, turkce, matematik, fen, sosyal, taksit, borc, aylik, kalan_borc, kalan_taksit ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)",
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
                fnaylik,
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
/// öğrenci güncellememizi sağlayan api
///
/// deneme
pub fn update(
    tel: i64,
    kolum: String,
    yenim: String,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    if kolum == "isim" || kolum == "soyisim" || kolum == "fatura_adres" || kolum == "veli_adres" {
        let hereismysql = "UPDATE Ogrenci SET ".to_string()
            + kolum.as_str()
            + "='"
            + yenim.as_str()
            + "' WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![])?;
        let b = json!({"success": true});
        Ok(content::Json(b.to_string()))
    } else if kolum == "telefon" && yenim.parse::<i64>().is_ok() {
        let hereismysql = "UPDATE Ogrenci SET ".to_string()
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
        let hereismysql = "UPDATE Ogrenci SET ".to_string()
            + kolum.as_str()
            + "="
            + yenim.as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![])?;
        let (borc, aylik, kalan_borc) = calculate_update_lesson(tel)?;
        let guncelle_bakalim_borc = "UPDATE Ogrenci SET borc=".to_string()
            + borc.to_string().as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(guncelle_bakalim_borc.as_str(), rusqlite::params![])?;

        let guncelle_bakalim_aylik = "UPDATE Ogrenci SET aylik=".to_string()
            + aylik.to_string().as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(guncelle_bakalim_aylik.as_str(), rusqlite::params![])?;

        let guncelle_bakalim_kalan_borc = "UPDATE Ogrenci SET kalan_borc=".to_string()
            + kalan_borc.to_string().as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(guncelle_bakalim_kalan_borc.as_str(), rusqlite::params![])?;

        let c = json!({"success": true});
        Ok(content::Json(c.to_string()))
    } else if kolum == "kalan_taksit" && yenim.parse::<i64>().is_ok() {
        let aylik: i64 = conn.query_row(
            "SELECT * FROM Ogrenci WHERE telefon=?",
            params![tel],
            |row| row.get(14),
        )?;
        let kalanborc = calculate_update(aylik, yenim.parse::<i64>()?);
        let hereismysql = "UPDATE Ogrenci SET ".to_string()
            + kolum.as_str()
            + "="
            + yenim.as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql.as_str(), params![])?;
        let hereismysql2 = "UPDATE Ogrenci SET kalan_borc=".to_string()
            + kalanborc.to_string().as_str()
            + " WHERE telefon="
            + tel.to_string().as_str();
        conn.execute(hereismysql2.as_str(), params![])?;
        let b = json!({"success": true});
        Ok(content::Json(b.to_string()))
    } else {
        let b = json!({"success": false});
        Ok(content::Json(b.to_string()))
    }
}

/// **geliştirme amaçlıdır**
/// tüm database i silen apimiz
#[get("/api/nuke")]
pub fn nuke() -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    conn.execute("DELETE FROM Ogrenci", params![])?;
    let b = json!({"success": true});
    Ok(content::Json(b.to_string()))
}

/// bireysel olarak öğrenci silmemizi sağlar
#[get("/api/delete?<tel>")]
pub fn delete(
    tel: i64,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    conn.execute("DELETE FROM Ogrenci WHERE telefon=?", params![tel])?;
    let b = json!({"success": true});
    Ok(content::Json(b.to_string()))
}

/// sadece bir öğrencinin döndürülmesini sağlayan api
#[get("/api/getstudent?<tel>")]
pub fn getstudent(
    tel: i64,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let conn = database::sqlite_connection()?;
    let mut statement = conn.prepare("SELECT * FROM Ogrenci WHERE telefon=?")?;
    let one_student = statement.query_map(params![tel], |row| {
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
        lel = r#"{"success": false}"#.to_string()
    } else {
        lel += "]";
    }
    Ok(content::Json(lel))
}

/// basit raporlama apisi
#[get("/api/data?<istiyorum>")]
pub fn api_data(
    istiyorum: String,
) -> Result<rocket::response::content::Json<std::string::String>, anyhow::Error> {
    let sonuc = hesap(&istiyorum)?;
    Ok(content::Json(json!({ istiyorum: sonuc }).to_string()))
}
