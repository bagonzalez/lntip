use crate::lightning;
use hex::FromHex;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::*;
use rocket_dyn_templates::Template;
use tonic_openssl_lnd::lnrpc::invoice::InvoiceState;

#[derive(Serialize, Default)]
pub struct InvoiceResponse {
    payment_request: String,
    hash: String,
    paid: bool,
    preimage: String,
    description: String,
    amount: i64,
    creation_date: i64,
    state: i32,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", ())
}

#[get("/create_invoice/<description>/<amount>")]
pub async fn create_invoice(description: &str, amount: u32) -> Json<InvoiceResponse> {
    let client = &mut lightning::connect().await.unwrap();
    let invoice = lightning::create_invoice(client, description, amount)
        .await
        .unwrap();

    let hash_str = invoice
        .r_hash
        .iter()
        .map(|h| format!("{h:02x}"))
        .collect::<Vec<String>>()
        .join("");

    Json(InvoiceResponse {
        payment_request: invoice.payment_request,
        hash: hash_str,
        ..Default::default()
    })
}

#[get("/invoice/<hash>")]
pub async fn lookup_invoice(hash: &str) -> Json<InvoiceResponse> {
    let hash = <[u8; 32]>::from_hex(hash).expect("Decoding failed");
    let invoice = lightning::get_invoice(&hash).await.unwrap();
    let mut preimage = String::new();
    let mut paid = false;
    if let Some(state) = InvoiceState::from_i32(invoice.state) {
        if state == InvoiceState::Settled {
            paid = true;
            preimage = invoice
                .r_preimage
                .iter()
                .map(|h| format!("{h:02x}"))
                .collect::<Vec<String>>()
                .join("");
        }
    }
    Json(InvoiceResponse {
        paid,
        preimage,
        description: invoice.memo,
        ..Default::default()
    })
}

#[get("/list_invoices")]
pub async fn list_invoices() -> Json<Vec<InvoiceResponse>> {
    let mut invoices = lightning::list_invoices().await.unwrap();

    invoices.sort_by(|a, b| b.creation_date.cmp(&a.creation_date));

    let mut invoice_responses = Vec::new();
    for invoice in invoices {
        let hash_str = invoice
            .r_hash
            .iter()
            .map(|h| format!("{h:02x}"))
            .collect::<Vec<String>>()
            .join("");
        let mut preimage = String::new();
        let mut paid = false;
        if let Some(state) = InvoiceState::from_i32(invoice.state) {
            if state == InvoiceState::Settled {
                paid = true;
                preimage = invoice
                    .r_preimage
                    .iter()
                    .map(|h| format!("{h:02x}"))
                    .collect::<Vec<String>>()
                    .join("");
            }
        }
        invoice_responses.push(InvoiceResponse {
            payment_request: invoice.payment_request,
            hash: hash_str,
            paid,
            preimage,
            description: invoice.memo,
            amount: invoice.value,
            creation_date: invoice.creation_date,
            state: invoice.state,
        });
    }
    Json(invoice_responses)
}
