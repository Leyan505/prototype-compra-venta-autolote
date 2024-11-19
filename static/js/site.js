$(function (){
  let table = new DataTable('#vehiculos-table');
  let table2 = new DataTable('#vendedores-table');
  let table3 = new DataTable('#ventas-table');

  $('#vehicleModal').modal({
    keyboard: true,
    backdrop: "static",
    show:false,
  }).on('show.bs.modal', function(event){
    var getIdFromRow = $(event.relatedTarget).closest('button').attr('data-id');
    var opc = $(event.relatedTarget).closest('button').attr('id');
    
    if(opc == "view-vehicles")
    {
      $.get("vehicles/vehicleDetails/" + getIdFromRow,
        {
            format: 'json',
            ajax: true
        }
      )
        .done(function(data) {
          $('#vehicle-header').html(`${data[0]["marca"]} ` + `${data[0]["modelo"]} `+ `${data[0]["anio"]} `);
          $('#vehicle-details').html(`<div class="table_component" role="region" tabindex="0">
            <table>
                <tbody>
                    <tr>
                      <th>Informacion vehiculo</th>
                      <th>Valores</th>
                    </tr>
                    <tr>
                        <td>Matricula</td>
                        <td>${data[0]["matricula"]}</td>
                    </tr>
                    <tr>
                        <td>Nro_chasis</td>
                        <td>${data[0]["nro_chasis"]}</td>
                    </tr>
                    <tr>
                        <td>Marca</td>
                        <td>${data[0]["marca"]}</td>
                    </tr>
                    <tr>
                        <td>Modelo</td>
                        <td>${data[0]["modelo"]}</td>
                    </tr>
                    <tr>
                        <td>Color</td>
                        <td>${data[0]["color"]}</td>
                    </tr>
                    <tr>
                        <td>Año</td>
                        <td>${data[0]["anio"]}</td>
                    </tr>            
                    <tr>
                      <th>Informacion compra</th>
                      <th>Valores</th>
                    </tr>      
                    <tr>
                        <td>Fecha de compra</td>
                        <td>${data[0]["fecha_compra"]}</td>
                    </tr>                  <tr>
                        <td>Precio de compra</td>
                        <td>${data[0]["precio_compra"]}</td>
                    </tr>
                </tbody>
            </table>`);
        });
    }
    else if(opc == "insert-vehicles")
    {
      $('#vehicle-header').html(`<h3>Insertar vehículo</h3>`);
      $('#vehicle-details').html(`<form action="/vehicles" method="POST" enctype="application/x-www-form-urlencoded">
        <div class="row mb-3">
            <div class="col-6">
                <label for="matricula" class="form-label">Matricula</label>
                <input type="text" class="form-control" name="matricula" id="matricula" placeholder="M123456" required>
              </div>
              <div class="col-6">
                <label for="nro_chasis" class="form-label">Nro. Chasis</label>
                <input type="text" class="form-control" name="nro_chasis" id="nro_chasis" placeholder="1G1RC6E42BUXXXXXX" required>
              </div>
            </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="marca" class="form-label">Marca</label>
              <input type="text" class="form-control" name="marca" id="marca" placeholder="ej. Toyota" required>
            </div>
            <div class="col-6">
              <label for="modelo" class="form-label">Modelo</label>
              <input type="text" class="form-control" name="modelo" id="modelo" placeholder="ej. Yaris" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="color" class="form-label">Color</label>
              <input type="text" class="form-control" name="color" id="color" placeholder="ej. Blanco" required>
            </div>
            <div class="col-6">
              <label for="anio" class="form-label">Año</label>
              <input type="number" class="form-control" name="anio" id="anio" placeholder="ej. 2020" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="fecha_compra" class="form-label">Fecha de compra</label>
              <input type="date" class="form-control" name="fecha_compra" id="fecha_compra" placeholder="dd/mm/aaaa" required>
            </div>
            <div class="col-6">
              <label for="precio_compra" class="form-label">Precio de compra</label>
              <input type="numeric" class="form-control" name="precio_compra" id="precio_compra" placeholder="0.0$" required>
            </div>
            <input type="hidden" value="DISPONIBLE" class="form-control" name="estado">

          </div>
          <div class="flex-btn-modal">
            <button id="btnInsertarVehiculo" class="btn" type="submit">Insertar</button>
          </div>
          </form>
        `);
    }
    else if(opc == "edit-vehicles")
    {
      $.get("vehicles/vehicleDetails/" + getIdFromRow,
        {
            format: 'json',
            ajax: true
        }
      )
        .done(function(data) {
      $('#vehicle-header').html(`<h3>Editar vehículo</h3>`);
      $('#vehicle-details').html(`<form action="/edit_vehicles/" method="POST" enctype="application/x-www-form-urlencoded">
        <div class="row mb-3">
            <div class="col-6">
                <label for="matricula" class="form-label">Matricula</label>
                <input type="text" class="form-control" value="${data[0]["matricula"]}" disabled>
                <input type="hidden" class="form-control" name="matricula" id="matricula" value="${data[0]["matricula"]}">
              </div>
              <div class="col-6">
                <label for="nro_chasis" class="form-label">Nro. Chasis</label>
                <input type="text" class="form-control" value="${data[0]["nro_chasis"]}"disabled>
                <input type="hidden" class="form-control" name="nro_chasis" id="nro_chasis" value="${data[0]["nro_chasis"]}">
              </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="marca" class="form-label">Marca</label>
              <input type="text" class="form-control" name="marca" id="marca" value="${data[0]["marca"]}" required>
            </div>
            <div class="col-6">
              <label for="modelo" class="form-label">Modelo</label>
              <input type="text" class="form-control" name="modelo" id="modelo" value="${data[0]["modelo"]}" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="color" class="form-label">Color</label>
              <input type="text" class="form-control" name="color" id="color" value="${data[0]["color"]}" required>
            </div>
            <div class="col-6">
              <label for="anio" class="form-label">Año</label>
              <input type="number" class="form-control" name="anio" id="anio" value="${data[0]["anio"]}" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="fecha_compra" class="form-label">Fecha de compra</label>
              <input type="date" class="form-control" name="fecha_compra" id="fecha_compra" value="${data[0]["fecha_compra"]}" required>
            </div>
            <div class="col-6">
              <label for="precio_compra" class="form-label">Precio de compra</label>
              <input type="numeric" class="form-control" name="precio_compra" id="precio_compra" value="${data[0]["precio_compra"]}" required>
            </div>       
          </div>
          <div class="row mb-3">
            <div class="col-12">
              <label for="estado" class="form-label">Estado</label>
              <select class="form-select" value="${data[0]["estado"]}" class="form-control" name="estado">
                <option value="DISPONIBLE">Disponible</option>
                <option value="ENPROCESO"> En proceso</option>
                <option value="RESERVADO">Reservado</option>
                <option value="VENDIDO">Vendido</option>
              </select>
            </div>
          </div>
          <div class="flex-btn-modal">
            <button id="btnEditarVehiculo" class="btn" type="submit">Editar</button>
          </div>
          </form>`);
      });
    }
  });



  // ODIO JS

  $('#salesModal').modal({
    keyboard: true,
    backdrop: "static",
    show: false,
  }).on('show.bs.modal', function (event) {
    var getIdFromRow = $(event.relatedTarget).closest('button').attr('data-id');
    var opc = $(event.relatedTarget).closest('button').attr('id');
  
    if (opc == "view-sales") {
      $.get("/sales/salesDetails/" + getIdFromRow, {
        format: 'json',
        ajax: true
      }
    ).done(function (data) {
        $('#sales-header').html( `Venta`);
        $('#sales-details').html(`<div class="table_component" role="region" tabindex="0">
          <table>
            <tbody>
              <tr>
                <th>Información Venta</th>
                <th>Valores</th>
              </tr>
              <tr>
                <td>ID Venta</td>
                <td>${data[0]["id_venta"]}</td>
              </tr>
              <tr>
                <td>Matrícula</td>
                <td>${data[0]["matricula"]}</td>
              </tr>
              <tr>
                <td>Fecha de Venta</td>
                <td>${data[0]["fecha_venta"]}</td>
              </tr>
              <tr>
                <td>Precio Venta</td>
                <td>${data[0]["precio_venta"]}</td>
              </tr>
              <tr>
                <td>ID Cliente</td>
                <td>${data[0]["id_cliente"]}</td>
              </tr>
              <tr>
                <td>ID Vendedor</td>
                <td>${data[0]["id_vendedor"]}</td>
              </tr>
            </tbody>
          </table>
        </div>`);
      });
    } else if (opc == "insert-sales") {
      $('#sales-header').html(`<h3>Insertar Venta</h3>`);
      $('#sales-details').html(`<form action="/sales" method="POST" enctype="application/x-www-form-urlencoded">
        <div class="row mb-3">
          <div class="col-6">
            <label for="matricula" class="form-label">Matrícula</label>
            <input type="text" class="form-control" name="matricula" id="matricula" placeholder="ej. M123456" required>
          </div>
          <div class="col-6">
            <label for="fecha_venta" class="form-label">Fecha de Venta</label>
            <input type="date" class="form-control" name="fecha_venta" id="fecha_venta" required>
          </div>
        </div>
        <div class="row mb-3">
          <div class="col-6">
            <label for="precio_venta" class="form-label">Precio de Venta</label>
            <input type="number" class="form-control" name="precio_venta" id="precio_venta" placeholder="ej. 230.34" required>
          </div>
          <div class="col-6">
            <label for="id_cliente" class="form-label">ID Cliente</label>
            <input type="text" class="form-control" name="id_cliente" id="id_cliente" placeholder="ID del cliente" required>
          </div>
        </div>
        <div class="row mb-3">
          <div class="col-6">
            <label for="id_vendedor" class="form-label">ID Vendedor</label>
            <input type="text" class="form-control" name="id_vendedor" id="id_vendedor" placeholder="ID del vendedor" required>
          </div>
        </div>
        <div class="flex-btn-modal">
          <button id="btnInsertarVenta" class="btn" type="submit">Insertar</button>
        </div>
      </form>`);
    } else if (opc == "edit-sales") {
      $.get("/sales/salesDetails/" + getIdFromRow, {
        format: 'json',
        ajax: true
      }).done(function (data) {
        $('#sales-header').html(`<h3>Editar Venta</h3>`);
        $('#sales-details').html(`<form action="/edit_sales/" method="POST" enctype="application/x-www-form-urlencoded">
          <div class="row mb-3">
            <div class="col-6">
              <label for="matricula" class="form-label">Matrícula</label>
              <input type="text" class="form-control" value="${data[0]["matricula"]}" required>
              <input type="hidden" class="form-control" name="matricula" id="matricula" value="${data[0]["matricula"]}">
            </div>
            <div class="col-6">
              <label for="fecha_venta" class="form-label">Fecha de Venta</label>
              <input type="date" class="form-control" name="fecha_venta" id="fecha_venta" value="${data[0]["fecha_venta"]}" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="precio_venta" class="form-label">Precio de Venta</label>
              <input type="number" class="form-control" name="precio_venta" id="precio_venta" value="${data[0]["precio_venta"]}" required>
            </div>
            <div class="col-6">
              <label for="id_cliente" class="form-label">ID Cliente</label>
              <input type="text" class="form-control" name="id_cliente" id="id_cliente" value="${data[0]["id_cliente"]}" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="id_vendedor" class="form-label">ID Vendedor</label>
              <input type="text" class="form-control" name="id_vendedor" id="id_vendedor" value="${data[0]["id_vendedor"]}" required>
            </div>
          </div>
          <div class="flex-btn-modal">
            <button id="btnEditarVenta" class="btn" type="submit">Editar</button>
          </div>
        </form>`);
      });
    }
  });
});

