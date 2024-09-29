import * as React from "react";
import BarPlot from "./BarPlot";
import { GraphDataType } from "../../utils/data_types";

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

export const VisualSection = () => {
  const data: GraphDataType[] = tables.map((t) => ({ label: t.name, value: t.row_count }))
  
  const [dimensions, setDimensions] = React.useState({width: 0, height: 0})
  const sectionEl = React.useRef(null)

  React.useEffect(() => {
    if(sectionEl.current) {
        const observer = new ResizeObserver((entries) => {
            entries.forEach(entry => {
                setDimensions({
                    width: entry.contentRect.width,
                    height: entry.contentRect.height
                })
            })
        })

        observer.observe(sectionEl.current)

        return () => {
            observer.disconnect()
        }
    }
  }, [])
  
  return (
    <section ref={sectionEl} className="visualSection">
        <BarPlot data={data} width={dimensions.width} />
    </section>
    )
}