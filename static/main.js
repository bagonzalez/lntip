// /static/main.js
let interval = null;

$(() => {
  $("#form").collapse("show");
  $("#send-btn").click(sendBtn);
  $("#invoice").collapse("hide");
  $("#success-box").collapse("hide");
  $("#refresh-btn").click(getListaPayments);
  getListaPayments();
});

const getListaPayments = async () => {
  $.ajax({
    url: `http://127.0.0.1:8000/list_invoices`,
    success: function (invoices) {
      console.log(invoices);
      let body_table = "";

      invoices.forEach((invoice) => {
        let date = new Date(0);

        let status = "";

        if (invoice.state == 0) {
          status = "Open";
        } else if (invoice.state == 1) {
          status = "Completado";
        } else if (invoice.state == 2) {
          status = "Canceled";
        } else {
          status = "Accepted";
        }

        date.setUTCSeconds(invoice.creation_date);
        body_table += `<tr>
          <td>${invoice.description}</td>
          <td>${invoice.amount}</td>
          <td>${invoice.paid}</td>
          <td>${date}</td>
          <td>${invoice.state === 2 ? `<font color="red">${status}</font>` : `<font color="green">${status}</font>`}</td>             
        </tr>`;

        if (invoice.state === 2) {
          body_table += `<tr>
            <td colspan="5">
              <div class="alert alert-danger" role="alert">
                <strong>Cancelado!</strong> El pago fue cancelado por el usuario.
              </div>
            </td>
          </tr>`;
        }

        if (invoice.state === 1) {
          body_table += `<tr>
            <td colspan="5">
              <div class="alert alert-success" role="alert">
                <strong>Completado!</strong> El pago fue completado.
              </div>
            </td>
          </tr>`;
        }

        if (invoice.state === 0) {
          body_table += `<tr>
            <td colspan="5">
              <div class="alert alert-warning" role="alert">
                <strong>Abierto!</strong> El pago está abierto.                
              </div>    
              <div style="max-width:1250px;">
               <code>${invoice.payment_request}</code>
              </div>         
            </td>
          </tr>
          `;
        }
      });

      $("#lista").empty();

      $("#lista").append(
        `<table class="table">
            <thead>
              <tr>
                <th scope="col">Descripción</th>
                <th scope="col">Monto</th>
                <th scope="col">Pagada</th>
                <th scope="col">Fecha</th>
                <th scope="col">Estado</th>
              </tr>
            </thead>
            <tbody>
            ${body_table}
            </tbody>
          </table>`
      );

    },
    async: false,
  });

}

const sendBtn = async () => {
  const amount = $("#amount").val();
  const description = $("#description").val();
  $.ajax({
    url:
      `http://127.0.0.1:8000/create_invoice/${description}/${amount}`,
    success: function (invoice) {
      console.log(invoice);
      $("#form").collapse("hide");
      $("#invoice-amount").text(amount);
      $("#invoice-text").text(invoice.payment_request);
      $("#invoice").collapse("show");
      $("#success-box").collapse("hide");
      interval = setInterval(waitPayment, 1000, invoice.hash);
    },
    async: false,
  });
  getListaPayments();
};


const waitPayment = async (hash) => {
  $.ajax({
    url: `http://127.0.0.1:8000/invoice/${hash}`,
    success: function (invoice) {
      console.log(invoice);
      if (invoice.paid) {
        console.log("pago realizado");
        clearInterval(interval);
        interval = null;
        $("#form").collapse("hide");
        $("#invoice").collapse("hide");
        $("#success-box").collapse("show");
      }
    },
    async: false,
  });
};