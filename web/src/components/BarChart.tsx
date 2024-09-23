import * as d3 from "d3"
import { useEffect, useRef } from "react"
import React = require("react")

const data = [
    {
        "name": "data_dictionary",
        "row_count": 65,
        "col_count": 3,
        "created": "2024-09-22",
        "updated": "2024-09-22"
    },
    {
        "name": "encounters",
        "row_count": 186,
        "col_count": 14,
        "created": "2024-09-22",
        "updated": "2024-09-22"
    },
    {
        "name": "organizations",
        "row_count": 0,
        "col_count": 8,
        "created": "2024-09-22",
        "updated": "2024-09-22"
    },
    {
        "name": "patients",
        "row_count": 974,
        "col_count": 20,
        "created": "2024-09-22",
        "updated": "2024-09-22"
    },
    {
        "name": "payers",
        "row_count": 10,
        "col_count": 7,
        "created": "2024-09-22",
        "updated": "2024-09-22"
    },
    {
        "name": "procedures",
        "row_count": 65,
        "col_count": 9,
        "created": "2024-09-22",
        "updated": "2024-09-22"
    }
]

const BarChart = () => {
    const svgRef = useRef()

    useEffect(() => {
        // Set up dimensions
        const width = 600;
        const height = 400;
        const margin = { top: 20, right: 30, bottom: 40, left: 40 };

        // Create the SVG container
        const svg = d3
            .select(svgRef.current)
            .attr("width", width)
            .attr("height", height)
            .style("background", "#f0f0f0")
            .style("border", "1px solid black");

        // Set up scales
        const x = d3
            .scaleBand()
            .domain(data.map((d) => d.name))
            .range([margin.left, width - margin.right])
            .padding(0.2);

        const y = d3
            .scaleLinear()
            .domain([0, d3.max(data, (d: typeof data[number]) => d.row_count)])
            .nice()
            .range([height - margin.bottom, margin.top]);

        // Clear previous content before re-rendering
        svg.selectAll("*").remove();

        // Append x-axis
        svg
            .append("g")
            .attr("transform", `translate(0, ${height - margin.bottom})`)
            .call(d3.axisBottom(x));

        // Append y-axis
        svg
            .append("g")
            .attr("transform", `translate(${margin.left}, 0)`)
            .call(d3.axisLeft(y));

        // Add labels (optional)
        svg
            .selectAll(".label")
            .data(data)
            .join("text")
            .attr("class", "label")
            .attr("x", (d) => x(d.name) + x.bandwidth() / 2)
            .attr("y", (d) => y(d.row_count) - 5)
            .attr("text-anchor", "middle")
            .text((d) => d.row_count);
    }, data)

    return <svg ref={svgRef}></svg>;
}

export default BarChart