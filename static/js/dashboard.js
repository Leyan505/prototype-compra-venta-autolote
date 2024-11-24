// Data
const data = [
    { date: '2020-01-01', value: 30 },
    { date: '2020-02-01', value: 80 },
    { date: '2020-03-01', value: 45 },
    { date: '2020-04-01', value: 60 },
    { date: '2020-05-01', value: 90 },
    { date: '2020-06-01', value: 120 },
];

// Set dimensions and margins
const width = 100;
const height = 100;
const margin = { top: 20, right: 30, bottom: 40, left: 40 };

// Create the SVG canvas
const svg = d3.select('svg')
    .attr('width', width)
    .attr('height', height)
    .append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);

// Parse date
const parseDate = d3.timeParse('%Y-%m-%d');

// Scales
const x = d3.scaleTime()
    .domain(d3.extent(data, d => parseDate(d.date)))
    .range([0, width - margin.left - margin.right]);

const y = d3.scaleLinear()
    .domain([0, d3.max(data, d => d.value)])
    .range([height - margin.top - margin.bottom, 0]);

// Define axes
const xAxis = d3.axisBottom(x);
const yAxis = d3.axisLeft(y);

// Append X and Y axes
svg.append('g')
    .attr('class', 'x axis')
    .attr('transform', `translate(0,${height - margin.top - margin.bottom})`)
    .call(xAxis);

svg.append('g')
    .attr('class', 'y axis')
    .call(yAxis);

// Line generator function
const line = d3.line()
    .x(d => x(parseDate(d.date)))
    .y(d => y(d.value));

// Append the line path with transition for animation
svg.append('path')
    .data([data])
    .attr('class', 'line')
    .attr('d', line)
    .attr('stroke-dasharray', function() {
        const length = this.getTotalLength(); // Get the total length of the path
        return length; // Set the dasharray to the path length to make it appear from start to end
    })
    .attr('stroke-dashoffset', function() {
        return this.getTotalLength(); // Set the dash offset to the length to hide the line initially
    })
    .transition() // Start the transition
    .duration(2000) // Duration of 2 seconds
    .ease(d3.easeBounceOut) // Easing function for smooth transition
    .attr('stroke-dashoffset', 0); // Animate the stroke-dashoffset to 0 to reveal the line