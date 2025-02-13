<script>
    import { invoke } from "@tauri-apps/api/core";

    let name = '';
    let msg = '';
    
    async function greet() {
        try {
            msg = await invoke('greet', { name });
            console.log("Greeted!");
        } catch (err) {
            console.error("Failed to greet", err);
            msg = "Failed to communicate with backend."
        }
    }
</script>


<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name}/>
    <button on:click="{greet}">Greet!</button>
    <p>{msg}</p>
</div>