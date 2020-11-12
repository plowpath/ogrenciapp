#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod database;

use rocket::response::content;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

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
fn index() -> rocket::response::content::Html<std::string::String> /*rocket::response::content::Json<std::string::String>*/
{
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
    //let zal = content::Json(lel);
    let alsana1 = r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
      <meta charset="utf-8">
      <meta name="viewport" content="width=device-width, 
                     initial-scale=1, 
                     shrink-to-fit=no">
      <title>
        tüm liste
      </title>
    
      <!-- Include Bootstrap for styling -->
      <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/mdb-ui-kit/1.0.0/mdb.min.css">
    
      <!-- Include the Bootstrap Table CSS 
      for the table -->
      <link rel="stylesheet" href="https://unpkg.com/bootstrap-table@1.16.0/dist/bootstrap-table.min.css">
    </head>
    
    <body>
      <div class="container">
        <h1 class="text text-success text-center ">
          ogrenciapp
        </h1>
        <table class="table-striped border-success">
          <thead>
            <tr>
              <th data-field="id">
                <span class="text-success">
                  id
                </span>
              </th>
              <th data-field="isim">
                <span class="text-success">
                  isim
                </span>
              </th>
              <th data-field="soyisim">
                <span class="text-success">
                  soyisim
                </span>
                <th data-field="telefon">
                <span class="text-success">
                  telefon
                </span>
                <th data-field="yemek">
                <span class="text-success">
                  yemek
                </span>
                <th data-field="servis">
                <span class="text-success">
                  servis
                </span>
                <th data-field="turkce">
                <span class="text-success">
                  turkce
                </span>
                <th data-field="matematik">
                <span class="text-success">
                  matematik
                </span>
                <th data-field="fen">
                <span class="text-success">
                  fen
                </span>
                <th data-field="sosyal">
                <span class="text-success">
                  sosyal
                </span>
              </th>
            </tr>
          </thead>
        </table>
      </div>
    
      <!-- Include jQuery and other required 
      files for Bootstrap -->
      <script src="https://code.jquery.com/jquery-3.3.1.min.js">
      </script>
      <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.7/umd/popper.min.js">
      </script>
      <script src="https://cdnjs.cloudflare.com/ajax/libs/mdb-ui-kit/1.0.0/mdb.min.css">
      </script>
    
      <!-- Include the JavaScript file 
      for Bootstrap table -->
      <script src="https://unpkg.com/bootstrap-table@1.16.0/dist/bootstrap-table.min.js">
      </script>
      <script type="text/javascript">
        $(document).ready(function () {
    
         
          $('table').bootstrapTable({
            data: mydata
          });
        });
    
        var mydata ="#;
    let alsana3 = r#";
    </script>
  </body>
  
  </html>"#;
    content::Html(format!("{}{}{}", alsana1, lel, alsana3))
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
    content::Html(
        r#"<!doctype html>
<html lang="en">
        
        <head>
          <!-- Required meta tags -->
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        
          <!-- Font Awesome -->
          <link href="https://use.fontawesome.com/releases/v5.8.2/css/all.css" rel="stylesheet" />
          <!-- Google Fonts -->
          <link href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap" rel="stylesheet" />
          <!-- MDB -->
          <link href="https://cdnjs.cloudflare.com/ajax/libs/mdb-ui-kit/1.0.0/mdb.min.css" rel="stylesheet" />
        
          <title>Öğrenci</title>
        </head>
        
        <body>
          <div class="container">
            <form action="/gonder" method="get">
              <label for="isim">isim:</label>
              <input class="form-control" type="text" id="isim" name="isim" required>
              <label for="soyisim">soyisim:</label>
              <input class="form-control" type="text" id="soyisim" name="soyisim" required>
              <label for="fatura_adres">fatura adres:</label>
              <input class="form-control" type="text" id="fatura_adres" name="fatura_adres" required>
              <label for="veli_adres">veli adres:</label>
              <input class="form-control" type="text" id="veli_adres" name="veli_adres" required>
              <label for="telefon">telefon:</label>
              <input class="form-control" type="tel" id="telefon" name="telefon" required>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="yemek" name="yemek" value="true" checked>
                <label class="form-check-label" for="yemek">yemek</label>
              </div>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="servis" name="servis" value="true" checked>
                <label class="form-check-label" for="servis">servis</label>
              </div>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="turkce" name="turkce" value="true" checked>
                <label class="form-check-label" for="turkce">türkçe</label>
              </div>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="matematik" name="matematik" value="true" checked>
                <label class="form-check-label" for="matematik">matematik</label>
              </div>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="fen" name="fen" value="true" checked>
                <label class="form-check-label" for="fen">fen</label>
              </div>
              <div class="form-check">
                <input type="checkbox" class="form-check-input" id="sosyal" name="sosyal" value="true" checked>
                <label class="form-check-label" for="sosyal">sosyal</label>
              </div>
        
              <button type="submit" class="btn btn-primary">Gönder</button>
              <a class="btn btn-danger" href="/nuke" role="button">NUKE</a>
            </form>
        </div>
    </body>
</html>"#
        .to_string(),
    )
}
fn main() {
    rocket::ignite()
        .mount("/", routes![index, proper_data_insert, nuke, new])
        .launch();
}
