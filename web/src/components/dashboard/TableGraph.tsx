import * as React from "react";
import { GraphDataType } from "../../data_desc"
import BarPlot from "../visualizers/BarPlot";

type TableGraphProps = {
    data: GraphDataType[]
}

export const TableGraph: React.FC<TableGraphProps> = ({data}) => {
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
  <section className="visualSection tableRowCount dashboardDisplay" ref={sectionEl}>
    <h3 className="sectionHeader">Table Row Counts</h3>
    <BarPlot data={data} width={dimensions.width} />
  </section>
 )
}