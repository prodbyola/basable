import * as React from "react";
import { GraphDataType } from "../../utils";
import { BarChart } from '@mui/x-charts/BarChart';

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
  
  const x = data.map(d => d.label)
  const y = data.map(d => d.value)

  return <BarChart 
    xAxis={[{ scaleType: 'band', data: x }]}
    series={[{data: y}]}
    width={ width }
    height={400}
    colors={[color]}
    borderRadius={50}
  />;
};

export default BarPlot;
