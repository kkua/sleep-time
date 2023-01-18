import { createEffect, createSignal } from "solid-js";

export default function TimeSection(props) {
    const max = props.max;
    const min = Number.parseInt(props.min);
    const [val, setVal] = createSignal(props.init);
    props.bind(val);
    let timer = null;

    return (
        <div class="d-inline-block" style="padding-right:1rem">
            <i class="icon btn btn-link icon-plus" ontouchstart={onTouchInc} ontouchend={onTouchEnd} onclick={onClickInc}></i>
            <div class="d-inline-block" style="width:3rem">
                <input class='form-input d-inline-block no-input-arrow' type='text' value={toInputValue()} oninput={onInputVal} />
            </div>
            <i class="icon btn btn-link icon-minus" ontouchstart={onTouchDec} ontouchend={onTouchEnd} onclick={onClickDec}></i>{props.label}
        </div>
    );


    function onTouchInc(event) {
        console.log("touch")
        timer = setTimeout(() => {
            timer = setInterval(() => {
                incVal();
                createEffect(() => console.log("touch", val()))
            }, 100);
        }, 300);

    }

    function onTouchDec(event) {
        console.log("touch")
        timer = setTimeout(() => {
            timer = setInterval(() => {
                decVal();
            }, 100);
        }, 300);
    }

    function onTouchEnd() {
        console.log("end touch")
        clearTimeout(timer);
        timer = undefined;
    }

    function onClickInc(event) {
        if (timer) {
            return;
        }
        incVal();
    }

    function onClickDec(event) {
        if (timer) {
            return;
        }
        decVal();
    }

    function incVal() {
        let value = val();
        if (value >= max) {
            setVal(min);
        } else {
            setVal(val() + 1);
        }
    }

    function decVal() {
        let value = val();
        if (value <= min) {
            setVal(max);
        } else {
            setVal(val() - 1);
        }
    }


    function onInputVal() {
        let value = this.value;
        let valueStr = value.replace(/[^0-9]/g, '');
        let v = Number.parseInt(valueStr);
        console.log(v);
        if (Number.isNaN(v)) {
            v = val();
        }
        if (value > max) {
            setVal(max);
        } else if (value < min) {
            setVal(min);
        } else {
            setVal(v);
        }
        this.value = toInputValue();
    }


    function toInputValue() {
        return val().toString().padStart(2, '0');
    }
}