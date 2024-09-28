import * as d3 from "d3";
import { useEffect } from "react";
import * as React from "react";
import { GraphDataType } from "../..";

type BarProps = {
  data: GraphDataType[];
  width?: number;
  color?: string;
};

const BarPlot: React.FC<BarProps> = ({
  color = "rgb(68, 81, 202)",
  width = 460,
  data,
}) => {
  const max = data.reduce(
    (max, obj) => (obj.value > max.value ? obj : max),
    data[0]
  );

  useEffect(() => {
    // Set up dimensions
    const margin = { top: 30, right: 30, bottom: 70, left: 60 };
    const w = width - margin.left - margin.right;
    const h = 400 - margin.top - margin.bottom;

    // Create the SVG container
    const svg = d3
      .select("#barplot")
      .append("svg")
      .attr("width", w + margin.left + margin.right)
      .attr("height", h + margin.top + margin.bottom)
      .append("g")
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    // X axis
    var x = d3
      .scaleBand()
      .range([0, w])
      .domain(
        data.map(function (d) {
          return d.label;
        })
      )
      .padding(0.2);

    svg
      .append("g")
      .attr("transform", "translate(0," + h + ")")
      .attr("class", "barplot_x")
      .call(d3.axisBottom(x))
      .selectAll("text")
      .attr("transform", "translate(-10,0) rotate(-45)")
      .style("text-anchor", "end");

    // Add Y axis
    var y = d3.scaleLinear().domain([0, max.value]).range([h, 0]);

    svg.append("g").attr("class", "barplot_y").call(d3.axisLeft(y));

    const rx = 12;
    const ry = 12;

    // Bars
    svg
      .selectAll("bar")
      .data(data)
      .enter()
      .append("path")
      .attr("d", (item: GraphDataType) => {

        if(item.value) {
          const offset = h - y(item.value)

          let my = y(item.value)
          if(offset > ry) my += ry

          let vy = h - y(item.value)
          if(offset > ry) vy -= ry
          
          return `
            M${x(item.label)},${my}
            a${rx},${ry} 0 0 1 ${rx},${-ry}
            h${x.bandwidth() - 2 * rx}
            a${rx},${ry} 0 0 1 ${rx},${ry}
            v${vy}
            h${-x.bandwidth()}Z
          `
        }
      })
      .attr("fill", color);

    // Clean-up function to remove the SVG on component unmount or re-render
    return () => {
      d3.select("#barplot").select("svg").remove();
    };
  });

  return <div id="barplot"></div>;
};

export default BarPlot;
