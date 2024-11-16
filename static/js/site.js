

$(function (){
  let table = new DataTable('#vehiculos-table');
  let table2 = new DataTable('#vendedores-table');


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
      $.get("/vehicles/" + getIdFromRow,
        {
            format: 'json',
            ajax: true
        }
      )
        .done(function(data) {
          
        });
    }
  });
});
