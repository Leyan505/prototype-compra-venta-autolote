
$(function(){
    fetch("/fetch_sales")
        .then(response => {
            if(!response.ok) {
                throw new Error('Error al obtener ventas' + response.statusText)
            }
            return response.json();
        })
        .then(data => {

            let data_sales = new Array(12).fill(0);
            data.forEach(item => {
            if (item.month && !isNaN(item.month) && item.month >= 1 && item.month <= 12) {
                const monthIndex = parseInt(item.month) - 1;
                
                data_sales[monthIndex] += item.record_count;
            }
            });

            //Ventasdata_sales_chart
            var ctx = document.getElementById('chart-ventas').getContext('2d');
            var myChart = new Chart(ctx, {
                type: 'line',  // Specifies that we are creating a line chart
                data: {
                    labels: ['Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio', 'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre'],  // X-axis labels
                    datasets: [{
                        label: 'Ventas',  // Name for the dataset
                        data: data_sales,  // Data points for the line
                        fill: false,  // Set to 'false' to not fill the area under the line
                        borderColor: 'rgb(75, 192, 192)',  // Line color
                        tension: 0.1,  // Line smoothness (0 is straight, higher values make it smoother)
                        borderWidth: 2, // Line thickness
                        pointRadius: 5, // Size of data points
                        pointBackgroundColor: 'rgb(75, 192, 192)', // Data point color
                    }]
                },
                options: {
                    responsive: true,  // Make the chart responsive to window resizing
                    animation: {
                        duration: 2000,  // Animation duration in milliseconds (2 seconds)
                        easing: 'easeInOutQuad',  // Easing function for smooth animation
                        // Optionally, you can add events
                        onProgress: function(animation) {
                            // You can track progress here if needed
                        },
                        onComplete: function() {
                            console.log('Animation Complete!');
                        }
                    },
                    scales: {
                        x: {
                            beginAtZero: true  // Set the X-axis to start from zero
                        },
                        y: {
                            beginAtZero: true  // Set the Y-axis to start from zero
                        }
                    }
                }
            });
        })
        .catch(error => {
            console.error("Hubo un problema:", error);
        })
        
        
    //ventas por marca
    fetch("/fetch_sales_brands")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        const marcas = data.map(item => item.marca);
        const cantidad = data.map(item => item.cantidad);

        var ctx1 = document.getElementById('chart-marca').getContext('2d');
        var myBarChart = new Chart(ctx1, {
            type: 'pie',  // Specifies the type of chart: bar chart
            data: {
                labels: marcas,  // X-axis labels
                datasets: [{
                    label: 'Ventas',  // Dataset label
                    data: cantidad,  // Data for each bar
                    backgroundColor: 'rgba(75, 192, 192, 0.2)',  // Bar color
                    borderColor: 'rgba(75, 192, 192, 1)',  // Border color of bars
                    borderWidth: 1,  // Border width of bars
                }]
            },
            options: {
                responsive: true,  // Make the chart responsive to resizing
                animation: {
                    duration: 1500,  // Duration of the animation in milliseconds (1.5 seconds)
                    easing: 'easeInOutQuart',  // Easing function for the animation (smooth in and out)
                    onProgress: function(animation) {
                        // Optional: You can track progress here if needed
                    },
                    onComplete: function() {
                        console.log('Animation Complete!');
                    }
                },
                scales: {
                    x: {
                        beginAtZero: true  // Start the X-axis from zero
                    },
                    y: {
                        beginAtZero: true  // Start the Y-axis from zero
                    }
                }
            }
        });
        
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })



    //gastos por mes
    fetch("/fetch_costs_chart")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        let data_costs = new Array(12).fill(0);
        data.forEach(item => {
        if (item.month && !isNaN(item.month) && item.month >= 1 && item.month <= 12) {
            const monthIndex = parseInt(item.month) - 1;
            
            data_costs[monthIndex] += item.record_count;
        }
        });

        var ctx2 = document.getElementById('chart-gastos').getContext('2d');
        var myBarChart = new Chart(ctx2, {
            type: 'bar',  // Specifies the type of chart: bar chart
            data: {
                labels: ['Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio', 'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre'],  // X-axis labels
                datasets: [{
                    label: 'Gastos',  // Dataset label
                    data: data_costs,  // Data for each bar
                    backgroundColor: 'rgba(75, 192, 192, 0.2)',  // Bar color
                    borderColor: 'rgba(75, 192, 192, 1)',  // Border color of bars
                    borderWidth: 1,  // Border width of bars
                }]
            },
            options: {
                responsive: true,  // Make the chart responsive to resizing
                animation: {
                    duration: 1500,  // Duration of the animation in milliseconds (1.5 seconds)
                    easing: 'easeInOutQuart',  // Easing function for the animation (smooth in and out)
                    onProgress: function(animation) {
                        // Optional: You can track progress here if needed
                    },
                    onComplete: function() {
                        console.log('Animation Complete!');
                    }
                },
                scales: {
                    x: {
                        beginAtZero: true  // Start the X-axis from zero
                    },
                    y: {
                        beginAtZero: true  // Start the Y-axis from zero
                    }
                }
            }
        });
        
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })
    
    //gastos por mes
    fetch("/fetch_vehicles_chart")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        let data_buys = new Array(12).fill(0);
        data.forEach(item => {
        if (item.month && !isNaN(item.month) && item.month >= 1 && item.month <= 12) {
            const monthIndex = parseInt(item.month) - 1;
            
            data_buys[monthIndex] += item.record_count;
        }
        });

        //buys
        var ctx3 = document.getElementById('chart-compras').getContext('2d');
        var myBarChart = new Chart(ctx3, {
            type: 'bar',  // Specifies the type of chart: bar chart
            data: {
                labels: ['Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio', 'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre'],  // X-axis labels
                datasets: [{
                    label: 'My First Dataset',  // Dataset label
                    data: data_buys,  // Data for each bar
                    backgroundColor: 'rgba(75, 192, 192, 0.2)',  // Bar color
                    borderColor: 'rgba(75, 192, 192, 1)',  // Border color of bars
                    borderWidth: 1,  // Border width of bars
                }]
            },
            options: {
                responsive: true,  // Make the chart responsive to resizing
                animation: {
                    duration: 1500,  // Duration of the animation in milliseconds (1.5 seconds)
                    easing: 'easeInOutQuart',  // Easing function for the animation (smooth in and out)
                    onProgress: function(animation) {
                        // Optional: You can track progress here if needed
                    },
                    onComplete: function() {
                        console.log('Animation Complete!');
                    }
                },
                scales: {
                    x: {
                        beginAtZero: true  // Start the X-axis from zero
                    },
                    y: {
                        beginAtZero: true  // Start the Y-axis from zero
                    }
                }
            }
        });

        
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })

    //ganancias en el mes
    fetch("/fetch_earnings")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        if(data[0]["total_difference"] != null)
        {
            document.getElementById("ganancias-dashboard").innerHTML = data[0]["total_difference"];
        }
        else{
            document.getElementById("ganancias-dashboard").innerHTML= "0$";
        }
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })

    //compras en el mes
    fetch("/fetch_buys")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        if(data[0]["total_difference"] != null)
        {
            document.getElementById("compras-dashboard").innerHTML = data[0]["total"];
        }
        else{
            document.getElementById("compras-dashboard").innerHTML= "0$";
        }
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })

    //Ventas en el mes
    fetch("/fetch_sells")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        if(data[0]["total_difference"] != null)
        {
            document.getElementById("ventas-dashboard").innerHTML = data[0]["total"];
        }
        else{
            document.getElementById("ventas-dashboard").innerHTML= "0$";
        }
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })


    //vehiculos vendidos en el mes
    fetch("/fetch_vehicles_sold")
    .then(response => {
        if(!response.ok) {
            throw new Error('Error al obtener ventas' + response.statusText)
        }
        return response.json();
    })
    .then(data => {
        if(data[0]["total_difference"] != null)
        {
            document.getElementById("vendidos-dashboard").innerHTML = data[0]["total"];
        }
        else{
            document.getElementById("vendidos-dashboard").innerHTML= "0";
        }
    })
    .catch(error => {
        console.error("Hubo un problema:", error);
    })
});
