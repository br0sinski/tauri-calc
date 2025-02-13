<script lang="ts">
    /*
    Goal of this component is to call the add, sub, div, mult and
    square method of the Rust Backend in order to calculate :)!
    Here, in the Frontend. We will see how that works!
    */
    import { invoke } from "@tauri-apps/api/core";
    import type { CalculationInput, CalculationResult, Operation} from "../types/types";

    const numbers = [
        7, 8, 9,
        4, 5, 6,
        1, 2, 3,
        0
    ];

    const operations: Array<{symbol: string, op: Operation}> = [
        {symbol: '+', op: "add"},
        {symbol: '-', op: "sub"},
        {symbol: '/', op: "div"},
        {symbol: '*', op: "mult"},
        {symbol: '^', op: "square"}
    ]

    let displayValue: string = '';
    let pendingOperation: Operation | null = null;
    let firstNum: number | null = null;
    let secondNum: number | null = null;
    let result: number | null = null;
    let error: string | null = null;

    function handleNumberClick(num: number) {
        if(pendingOperation == null) {
            displayValue = displayValue === '0' ? num.toString() : displayValue + num;
            firstNum = Number(displayValue);
        } else {
            displayValue = displayValue === '0' ? num.toString() : displayValue + num;
            secondNum = Number(displayValue);
        }
    }

    function handleOperationClick(op: Operation) {
        if (firstNum !== null && secondNum !== null) {
            calculate();
        }
        pendingOperation = op;
        if (firstNum === null) {
            firstNum = Number(displayValue);
        }
        displayValue = '';
    }

    async function calculate(): Promise<void> {
    if (firstNum === null || secondNum === null || pendingOperation === null) {
        return;
    }

    try {
        error = null;
        if (pendingOperation === 'div' && secondNum === 0) {
            throw new Error('Division by zero not possible');
        }

        // For division, handle the Result type differently
        if (pendingOperation === 'div') {
            try {
                const response = await invoke<number>(pendingOperation, {
                    a: firstNum,
                    b: secondNum
                });
                result = response;
                displayValue = result.toString();
            } catch (divErr) {
                error = divErr as string;
                result = null;
            }
        } else {

            result = await invoke<number>(pendingOperation, {
                a: firstNum,
                b: secondNum
            });
            displayValue = result.toString();
        }

        if (result !== null) {
            firstNum = result;
            secondNum = null;
            pendingOperation = null;
        }
    } catch (err: unknown) {
        error = err instanceof Error ? err.message : "Unknown error";
        result = null;
    }
}

    function clear() {
        displayValue = '';
        firstNum = null;
        secondNum = null;
        pendingOperation = null;
        result = null;
        error = null;
    }
</script>

<div class="calc">
    <input type="text" class="result" placeholder="0" value="{error ?? displayValue}" readonly>

    <div class="calc-grid">
        <div class="num-grid">
            {#each numbers as i}
                <button on:click={() => handleNumberClick(i)}>{i}</button>
            {/each}
            <button on:click={clear}>C</button>
            <button on:click={calculate}>=</button>
        </div>
        
        <div class="op-grid">
            {#each operations as {symbol, op}}
                <button on:click={() => handleOperationClick(op)} class:active={pendingOperation === op}>{symbol}</button>
            {/each}
        </div>
    </div>
</div>

<style>
    .calc-grid {
        display: grid;
        grid-template-columns: 3fr 1fr;
        gap: 10px;
    }

    .num-grid {
        display: grid;
        gap: 2px;
        grid-template-columns: repeat(3, 1fr);
    }

    .op-grid {
        display: grid;
        gap: 2px;
        grid-template-columns: 1fr;
    }

    button {
        padding: 0.5rem;
        font-size: 1.2rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        background: #f8f9fa;
        cursor: pointer;
    }

    button:hover {
        background: #e9ecef;
    }

    .calc {
        max-width: 400px;
        margin: 0 auto;
        padding: 1rem;
    }

    .result {
        width: 100%;
        box-sizing: border-box;
        margin-bottom: 1rem;
        padding: 0.75rem;
        font-size: 1.2rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        background: #fff;
    }
</style>