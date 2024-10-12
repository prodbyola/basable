import * as React from "react";
import { useParams } from 'react-router-dom';

const DatabaseTable = () => {
    const { tableID } = useParams()
    return <>{tableID}</>
}

export default DatabaseTable