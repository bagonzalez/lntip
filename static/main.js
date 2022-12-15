// /static/main.js
let interval = null;

$(() => {
  $("#form").collapse("show");
  $("#send-btn").click(sendBtn);
  $("#invoice").collapse("show");
  $("#success-box").collapse("hide");
});

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
        $("#invoice").collapse("show");
        $("#success-box").collapse("show");
      }
    },
    async: false,
  });
};