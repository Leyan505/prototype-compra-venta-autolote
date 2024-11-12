

$(function (){
  let table = new DataTable('#vehiculos-table');
  let table2 = new DataTable('#vendedores-table');


  $('#vehicleModal').modal({
    keyboard: true,
    backdrop: "static",
    show:false,
  }).on('show.bs.modal', function(event){
    var getIdFromRow = $(event.relatedTarget).closest('button').attr('data-id');
    $.get("vehicles/vehicleDetails/" + getIdFromRow,
      {
          format: 'json',
          ajax: true
      }
    )
      .done(function(data) {
        $(this).find('#vehicleDetails').html($('<b> Nro_chasis: '+ data.nro_chasis + '</b>'))
      });
  });
});
