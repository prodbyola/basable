import type { ApiService } from "@/request";

export class Table {
    private _columnList: string[] = []
    private _rows: unknown[] = []
    private _rowCount = 0

    createdAtColumn: string | undefined

    get name(){
        return this._name
    }

    get columnList(){
        return this._columnList
    }

    get rows() {
        return this._rows
    }

    get rowCount(){
        return this._rowCount
    }

    constructor(private _name: string, private svc: ApiService){}

    async loadColumList(){
        await this.svc.request({
            path: `columns?table=${this.name}`,
            method: 'GET'
        }).then((resp) => this._columnList = resp?.data as string[])
    }

    async loadDashboard(){
        let path = `dashboard?table=${this.name}`
        if(this.createdAtColumn){
            path += `&create_at=${this.createdAtColumn}`
        }
        
        await this.svc.request({
            path,
            method: 'GET'
        }).then((resp) => {
            const data = resp?.data
            console.log(data)
            
            if(data) {
                this._rowCount = data['row_count']
            }
        })
    }
}