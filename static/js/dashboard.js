$(function(){

    //Ventas
    var ctx = document.getElementById('chart-ventas').getContext('2d');
    var myChart = new Chart(ctx, {
        type: 'line',  // Specifies that we are creating a line chart
        data: {
            labels: ['January', 'February', 'March', 'April', 'May', 'June'],  // X-axis labels
            datasets: [{
                label: 'My First Dataset',  // Name for the dataset
                data: [65, 59, 80, 81, 56, 55],  // Data points for the line
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

    //ventas por marca
    var ctx1 = document.getElementById('chart-marca').getContext('2d');
    var myBarChart = new Chart(ctx1, {
        type: 'pie',  // Specifies the type of chart: bar chart
        data: {
            labels: ['January', 'February', 'March', 'April', 'May', 'June'],  // X-axis labels
            datasets: [{
                label: 'My First Dataset',  // Dataset label
                data: [65, 59, 80, 81, 56, 55],  // Data for each bar
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

    //gastos
    var ctx2 = document.getElementById('chart-gastos').getContext('2d');
    var myBarChart = new Chart(ctx2, {
        type: 'bar',  // Specifies the type of chart: bar chart
        data: {
            labels: ['January', 'February', 'March', 'April', 'May', 'June'],  // X-axis labels
            datasets: [{
                label: 'My First Dataset',  // Dataset label
                data: [65, 59, 80, 81, 56, 55],  // Data for each bar
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

    //compras
    var ctx3 = document.getElementById('chart-compras').getContext('2d');
    var myBarChart = new Chart(ctx3, {
        type: 'bar',  // Specifies the type of chart: bar chart
        data: {
            labels: ['January', 'February', 'March', 'April', 'May', 'June'],  // X-axis labels
            datasets: [{
                label: 'My First Dataset',  // Dataset label
                data: [65, 59, 80, 81, 56, 55],  // Data for each bar
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

});
