use dotenv::dotenv;
use std::env;
use tonic_openssl_lnd::{LndClientError, LndLightningClient};
pub async fn connect() -> Result<LndLightningClient, LndClientError> {
    dotenv().ok();
    let port: u32 = env::var("LND_GRPC_PORT")
        .expect("LND_GRPC_PORT must be set")
        .parse()
        .expect("port is not u32");
    let host = env::var("LND_GRPC_HOST").expect("LND_GRPC_HOST mustbe set");
    let cert = env::var("LND_CERT_FILE").expect("LND_CERT_FILE mustbe set");
    let macaroon = env::var("LND_MACAROON_FILE").expect("LND_MACAROON_FILE must beset");
    // Connecting to LND requires only host, port, cert file, and macaroon file
    let client = tonic_openssl_lnd::connect_lightning(host, port, cert, macaroon)
        .await
        .expect("Failed connecting to LND");
    Ok(client)
}

use tonic_openssl_lnd::lnrpc::{AddInvoiceResponse, Invoice}; // <--al inicio
pub async fn create_invoice(
    client: &mut LndLightningClient,
    description: &str,
    amount: u32,
) -> Result<AddInvoiceResponse, LndClientError> {
    let invoice = Invoice {
        memo: description.to_string(),
        value: amount as i64,
        ..Default::default()
    };
    let invoice = client.add_invoice(invoice).await?.into_inner();
    Ok(invoice)
}

use tonic_openssl_lnd::lnrpc::PaymentHash; // <-- agregamos PaymentHash
pub async fn get_invoice(hash: &[u8]) -> Result<Invoice, LndClientError> {
    let mut client = connect().await.unwrap();
    let invoice = client
        .lookup_invoice(PaymentHash {
            r_hash: hash.to_vec(),
            ..Default::default()
        })
        .await?
        .into_inner();
    Ok(invoice)
}

use tonic_openssl_lnd::lnrpc::{ListInvoiceRequest, ListInvoiceResponse};
pub async fn list_invoices() -> Result<Vec<Invoice>, LndClientError> {
    let mut client = connect().await.unwrap();

    let invoices = client
        .list_invoices(ListInvoiceRequest {
            ..Default::default()
        })
        .await?
        .into_inner();

    Ok(invoices.invoices)
}
