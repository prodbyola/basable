import * as d3 from "d3";
import { useEffect } from "react";
import * as React from "react";

const tables = [
  {
    name: "data_dictionary",
    row_count: 65,
    col_count: 3,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "encounters",
    row_count: 186,
    col_count: 14,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "organizations",
    row_count: 0,
    col_count: 8,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "patients",
    row_count: 974,
    col_count: 20,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "payers",
    row_count: 10,
    col_count: 7,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "procedures",
    row_count: 65,
    col_count: 9,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
];

const BasableBarChart = () => {
  const data = tables.map((t) => {
    return { label: t.name, value: t.row_count };
  });

  const max = data.reduce(
    (max, obj) => (obj.value > max.value ? obj : max),
    data[0]
  );

  useEffect(() => {
    // Set up dimensions
    const margin = { top: 30, right: 30, bottom: 70, left: 60 };
    const width = 460 - margin.left - margin.right;
    const height = 400 - margin.top - margin.bottom;

    // Create the SVG container
    const svg = d3
      .select("#barchart")
      .append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    // X axis
    var x = d3
      .scaleBand()
      .range([0, width])
      .domain(
        data.map(function (d) {
          return d.label;
        })
      )
      .padding(0.2);

    svg
      .append("g")
      .attr("transform", "translate(0," + height + ")")
      .call(d3.axisBottom(x))
      .selectAll("text")
      .attr("transform", "translate(-10,0) rotate(-45)")
      .style("text-anchor", "end");

    // Add Y axis
    var y = d3
      .scaleLinear()
      .domain([0, max.value + 50])
      .range([height, 0]);

    svg.append("g").call(d3.axisLeft(y));

    // Bars
    svg
      .selectAll("mybar")
      .data(data)
      .enter()
      .append("rect")
      .attr("x", function (d) {
        return x(d.label);
      })
      .attr("y", function (d) {
        return y(d.value);
      })
      .attr("width", x.bandwidth())
      .attr("height", function (d) {
        return height - y(d.value);
      })
      .attr("fill", "#69b3a2");
  });

  return <div id="barchart"></div>;
};

export default BasableBarChart;
