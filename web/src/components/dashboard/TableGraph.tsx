import * as React from "react";
import { GraphDataType } from "../../utils/data_types"
import BarPlot from "../visualizers/BarPlot";
import { useStore } from "../../utils";

export const TableGraph = () => {
    const [dimensions, setDimensions] = React.useState({ width: 0, height: 0 })
    const sectionEl = React.useRef(null)

    const tables = useStore((state) => state.tables)
    const data: GraphDataType[] = tables.map((t) => ({ label: t.name, value: t.row_count }))

    React.useEffect(() => {
        if (sectionEl.current) {
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
            
            { data.length && <BarPlot data={data} width={dimensions.width} /> }
        </section>
    )
}