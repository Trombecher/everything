import {createSignal, JSX} from "solid-js";

export default ({
                    value,
                    fallback
}: {
    value: Promise<JSX.Element>,
    fallback?: JSX.Element
}) => {
    const [rValue, setRValue] = createSignal<JSX.Element>(fallback);
    value.then(setRValue);
    return rValue;
}