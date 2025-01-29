import {Value, VL_DATE_TIME} from "../adapter/model.ts";

export default ({value}: {value: Value}) => {
    switch(value._) {
        case VL_DATE_TIME: return <span>{"" + new Date(value.v)}</span>
    }
}