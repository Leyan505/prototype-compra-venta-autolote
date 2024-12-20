$(function (){
  let table = new DataTable('#vehiculos-table');
  let table2 = new DataTable('#vendedores-table');
  let table3 = new DataTable('#ventas-table');
  let table4 = new DataTable('#clientes-table');
  let table5 = new DataTable('#gastos-table');


  $(".toggle-sidebar").click(function(){
    $(".sidebar").toggleClass('active');
    $(".main").toggleClass('active');
    $(".topbar").toggleClass('active');
  });


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

// ODIO JS VENTAS

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
            <input type="hidden" name="id_venta" id="id_venta" value="${data[0]["id_venta"]}">
            <input type="text" class="form-control" name="matricula" id="matricula" value="${data[0]["matricula"]}" disabled>
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
            <input type="numeric" class="form-control" name="precio_venta" id="precio_venta" value="${data[0]["precio_venta"]}" required>
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


    // CLIENTES
    $('#clientsModal').modal({
      keyboard: true,
      backdrop: "static",
      show: false,
    }).on('show.bs.modal', function (event) {
      var getIdFromRow = $(event.relatedTarget).closest('button').attr('data-id');
      var opc = $(event.relatedTarget).closest('button').attr('id');
    
    if (opc == "insert-clients") {
      $('#clients-header').html(`<h3>Insertar Cliente</h3>`);
      $('#clients-details').html(`<form action="/clients" method="POST" enctype="application/x-www-form-urlencoded">
        <div class="row mb-3">
          <div class="col-12">
            <label for="cedula" class="form-label">Cédula</label>
            <input type="text" class="form-control" name="cedula" id="cedula" placeholder="ej. 001-112205-7896J" required>
          </div>
        </div>
        <div class="row mb-3">
          <div class="col-6">
            <label for="nombre" class="form-label">Nombre</label>
            <input type="text" class="form-control" name="nombre" id="nombre" placeholder="ej. Andrés" required>
          </div>
          <div class="col-6">
            <label for="apellido" class="form-label">Apellido</label>
            <input type="text" class="form-control" name="apellido" id="apellido" placeholder="ej. Guido" required>
          </div>
        </div>
        <div class="flex-btn-modal">
          <button id="btnInsertarCliente" class="btn" type="submit">Insertar</button>
        </div>
      </form>`);
    } else if (opc == "edit-client") {
      $.get("/clients/clientDetails/" + getIdFromRow, {
        format: 'json',
        ajax: true
      }).done(function (data) {
        $('#clients-header').html(`<h3>Editar Venta</h3>`);
        $('#clients-details').html(`<form action="/edit_client/" method="POST" enctype="application/x-www-form-urlencoded">
          <div class="row mb-3">
            <div class="col-12">
              <label for="cedula" class="form-label">Cédula</label>
              <input type="text" class="form-control" name="cedula" value="${data[0]["cedula"]}" required>
              <input type="hidden" class="form-control" name="id_cliente" id="id_cliente" value="${data[0]["id_cliente"]}">
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="nombre" class="form-label">Nombre</label>
              <input type="text" class="form-control" name="nombre" id="nombre" value="${data[0]["nombre"]}" required>
            </div>
            <div class="col-6">
              <label for="apellido" class="form-label">Apellido</label>
              <input type="text" class="form-control" name="apellido" id="apellido" value="${data[0]["apellido"]}" required>
            </div>
          </div>
          <div class="flex-btn-modal">
            <button id="btnEditarCliente" class="btn" type="submit">Editar</button>
          </div>
        </form>`);
      });
    }
  });

  // SIGO ODIANDO JS VENDEDORES

  $('#sellerModal').modal({
    keyboard: true,
    backdrop: "static",
    show: false,
  }).on('show.bs.modal', function (event) {
    var getIdFromRow = $(event.relatedTarget).closest('button').attr('data-id');
    var opc = $(event.relatedTarget).attr('id');
  
    if (opc === "insert-seller") {
      $('#seller-header').html(`<h3>Insertar Vendedor</h3>`);
      $('#seller-details').html(`<form action="/sellers" method="POST" enctype="application/x-www-form-urlencoded">
          <div class="row mb-3">
            <div class="col-6">
              <label for="nombre" class="form-label">Nombre</label>
              <input type="text" class="form-control" name="nombre" id="nombre" placeholder="Nombre" required>
            </div>
            <div class="col-6">
              <label for="apellido" class="form-label">Apellido</label>
              <input type="text" class="form-control" name="apellido" id="apellido" placeholder="Apellido" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-12">
              <label for="cedula" class="form-label">Cédula</label>
              <input type="text" class="form-control" name="cedula" id="cedula" placeholder="Cédula" required>
            </div>
          </div>
          <div class="flex-btn-modal">
            <button id="btnInsertarVendedor" type="submit" class="btn btn-primary">Insertar</button>
          </div>
        </form>
      `);
    } else if ( opc == "view-seller") {
      $.get("/sellers/sellerDetails/" + getIdFromRow, { format: 'json', ajax: true })
        .done(function (data) {
          $('#seller-header').html(`<h3>Detalles del Vendedor</h3>`);
          $('#seller-details').html(`<div class="table_component" role="region" tabindex="0">
              <table>
                <tbody>
                  <tr>
                    <th>Información</th>
                    <th>Valor</th>
                  </tr>
                  <tr>
                    <td>ID Vendedor</td>
                    <td>${data[0]["id_vendedor"]}</td>
                  </tr>
                  <tr>
                    <td>Nombre</td>
                    <td>${data[0]["nombre"]}</td>
                  </tr>
                  <tr>
                    <td>Apellido</td>
                    <td>${data[0]["apellido"]}</td>
                  </tr>
                  <tr>
                    <td>Cédula</td>
                    <td>${data[0]["cedula"]}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          `);
        });
    } else if (opc =="edit-seller") {
      $.get("/sellers/sellerDetails/" + getIdFromRow, 
        { format: 'json', ajax: true })
        .done(function (data) {
          $('#seller-header').html(`<h3>Editar Vendedor</h3>`);
          $('#seller-details').html(`<form action="/edit_sellers/" method="POST" enctype="application/x-www-form-urlencoded">
              <div class="row mb-3">
                <div class="col-6">
                  <label for="nombre" class="form-label">Nombre</label>
                  <input type="hidden" name="id_vendedor" id="id_vendedor" value="${data[0]["id_vendedor"]}">
                  <input type="text" class="form-control" name="nombre" id="nombre" value="${data[0]["nombre"]}" required>
                </div>
                <div class="col-6">
                  <label for="apellido" class="form-label">Apellido</label>
                  <input type="text" class="form-control" name="apellido" id="apellido" value="${data[0]["apellido"]}" required>
                </div>
              </div>
              <div class="row mb-3">
                <div class="col-12">
                  <label for="cedula" class="form-label">Cédula</label>
                  <input type="text" class="form-control" name="cedula" id="cedula" value="${data[0]["cedula"]}" required>
                </div>
              </div>
              <div class="flex-btn-modal">
                <button id="btnEditarVendedor" type="submit" class="btn btn-warning">Guardar Cambios</button>
              </div>
            </form>
          `);
        });
    }
  });


//Gastos

$('#costModal').modal({
  keyboard: true,
  backdrop: "static",
  show: false,
}).on('show.bs.modal', function (event) {
  var getIdFromRow = $(event.relatedTarget).closest('button').attr('data-id');
  var opc = $(event.relatedTarget).closest('button').attr('id');

  if (opc == "view-costs") {
    $.get("/costs/details/" + getIdFromRow, {
      format: 'json',
      ajax: true
    }
  ).done(function (data) {
      $('#cost-header').html( `Gastos`);
      $('#cost-details').html(`<div class="table_component" role="region" tabindex="0">
        <table>
          <tbody>
            <tr>
              <th>Información de Gasto</th>
              <th>Valores</th>
            </tr>
            <tr>
              <td>ID Gasto</td>
              <td>${data["id_gasto"]}</td>
            </tr>
            <tr>
              <td>Matrícula</td>
              <td>${data["matricula"]}</td>
            </tr>
            <tr>
              <td>Tipo de Reparacion</td>
              <td>${data["tipo_reparacion"]}</td>
            </tr>
            <tr>
              <td>Monto de Gasto</td>
              <td>${data["monto"]}</td>
            </tr>
            <tr>
              <th>Informacion Del Taller</th>
              <th>Valores</th>
            </tr> 
            <tr>
              <td>Nombre del Taller</td>
              <td>${data["nombre_taller"]}</td>
            </tr>
            <tr>
              <td>Direccion del Taller</td>
              <td>${data["direccion_taller"]}</td>
            </tr>
            <tr>
              <td>Telefono del Taller</td>
              <td>${data["telefono_taller"]}</td>
            </tr>
          </tbody>
        </table>
      </div>`
    );
    });
  } else if (opc == "insert-costs") {
    $('#cost-header').html(`<h3>Insertar gasto</h3>`);
    $('#cost-details').html(`<form action="/costs" method="POST" enctype="application/x-www-form-urlencoded">
      <div class="row mb-3">
          <div class="col-6">
              <label for="matricula" class="form-label">Matricula</label>
              <input type="text" class="form-control" name="matricula" id="matricula" placeholder="M123456" required>
            </div>
            <div class="col-6">
              <label for="tipo_reparacion" class="form-label">Reparacion</label>
              <input type="text" class="form-control" name="tipo_reparacion" id="tipo_reparacion" placeholder="Cambio de Aceite" required>
            </div>
          </div>
        <div class="row mb-3">
          <div class="col-6">
            <label for="monto" class="form-label">Monto</label>
            <input type="number" class="form-control" name="monto" id="monto" placeholder="ej. 250" required>
          </div>
          <div class="col-6">
            <label for="fecha_finalizacion" class="form-label">Fecha de finalizacion</label>
            <input type="date" class="form-control" name="fecha_finalizacion" id="fecha_finalizacion" placeholder="ej. Yaris" required>
          </div>
        </div>
        <div class="row mb-3">
          <div class="col-6">
            <label for="nombre_taller" class="form-label">Nombre del taller</label>
            <input type="text" class="form-control" name="nombre_taller" id="nombre_taller" placeholder="ej. Taller La tuerca" required>
          </div>
          <div class="col-6">
            <label for="direccion_taller" class="form-label">Direccion del taller</label>
            <input type="text" class="form-control" name="direccion_taller" id="direccion_taller" placeholder="ej. Casa del Obrero media al lago" required>
          </div>
        </div>
        <div class="row mb-3">
          <div class="col-6">
            <label for="telefono_taller" class="form-label">Telefono del taller</label>
            <input type="number" class="form-control" name="telefono_taller" id="telefono_taller" placeholder="ej 22772432" required>
          </div>
        

        </div>
        <div class="flex-btn-modal">
          <button id="btnInsertarGasto" class="btn" type="submit">Insertar</button>
        </div>
        </form>

    `);
  } else if (opc == "edit-costs") {
      $.get("/costs/details/" + getIdFromRow, {
        format: 'json',
        ajax: true
      }).done(function (data) {
        $('#cost-header').html(`<h3>Editar Gasto</h3>`);
        $('#cost-details').html(`<form action="/edit_costs/" method="POST" enctype="application/x-www-form-urlencoded">
          <div class="row mb-3">
            <div class="col-6">
              <label for="matricula" class="form-label">Matrícula</label>
              <input type="text" class="form-control" name="matricula" id="matricula" value="${data["matricula"]}" required>
            </div>
            <div class="col-6">
              <label for="fecha_finalizacion" class="form-label">Fecha de Finalización</label>
              <input type="date" class="form-control" name="fecha_finalizacion" id="fecha_finalizacion" value="${data["fecha_finalizacion"]}" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="monto" class="form-label">Monto</label>
              <input type="number" class="form-control" name="monto" id="monto" value="${data["monto"]}" required>
            </div>
            <div class="col-6">
              <label for="tipo_reparacion" class="form-label">Tipo de Cambio</label>
              <input type="text" class="form-control" name="tipo_reparacion" id="tipo_reparacion" value="${data["tipo_reparacion"]}" required>
            </div>
            <div class="col-6">
              <label for="nombre_taller" class="form-label">Nombre del Taller</label>
              <input type="text" class="form-control" name="nombre_taller" id="nombre_taller" value="${data["nombre_taller"]}" required>
            </div>
          </div>
          <div class="row mb-3">
            <div class="col-6">
              <label for="direccion_taller" class="form-label">Dirección del Taller</label>
              <input type="text" class="form-control" name="direccion_taller" id="direccion_taller" value="${data["direccion_taller"]}" required>
            </div>
            <div class="col-6">
              <label for="telefono_taller" class="form-label">Teléfono del Taller</label>
              <input type="text" class="form-control" name="telefono_taller" id="telefono_taller" value="${data["telefono_taller"]}" required>
            </div>
          </div>
          <div class="flex-btn-modal">
            <button id="btnEditarGasto" class="btn" type="submit">Editar</button>
          </div>
        </form>`);
    });
  }
});
});



