<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  async function next() {
    await invoke("next");
  }

  async function play() {
    await invoke("play");
  }

  async function prev() {
    await invoke("next");
  }
</script>

<div>
  <div class="row">
    <button on:click={prev}>
      {"<<"}
    </button>
    <button on:click={play}>Play</button>
    <button on:click={next}>
      {">>"}
    </button>
  </div>
</div>

