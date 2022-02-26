<script>
    export let value = 0;
    export let min;
    export let max;
    export let label;

    let wid = `${Math.max(min.length, max.length) + 1}em`;

    function inc() {
        ++value;
        chkVaule();
    }

    function dec() {
        --value;
        chkVaule();
    }

    function chkVaule() {
        if (value > max) {
            value = max;
            return;
        }
        if (value < min) {
            value = min;
        }
    }

    // chkDigital
</script>

<div style="display: inline-block;">
    <button on:click={inc} style="width: {wid};">+</button>
    <input
        type="number"
        {min}
        {max}
        bind:value
        on:blur={chkVaule}
        style="width: {wid};"
        class="input"
    />
    <span>{label}</span>
    <button on:click={dec} style="width: {wid};">-</button>
</div>

<style>
    /* input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
    } */
    /* input */

    .input {
        behavior: integer;
        -filter: "-+0~9";
        margin: 0;
        display: inline-block;
        cursor: default;
        /* context-menu: url(sciter:menu-edit.htm); */
    }

    :global(.input > caption) {
        display: block;
        height: *;
        width: *;
        flow: text;
        behavior: edit;
        cursor: text;
        white-space: pre;
        overflow-x: hidden-scroll;
    }

    button {
        margin: 0;
        /* padding: 0; */
        display: block;
        border: none;
    }
</style>
