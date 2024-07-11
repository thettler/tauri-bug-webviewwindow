<script lang="ts">
    import {WebviewWindow} from "@tauri-apps/api/webviewWindow"

    function openWindow() {
        const pileWindow = new WebviewWindow('my-label', {
            url: 'https://github.com/tauri-apps/tauri'
        });

        pileWindow.once('tauri://created', function () {
            console.log('Success');
        });
        pileWindow.once('tauri://error', function (e) {
            console.log(e);
        });
    }
</script>
<div>
    <button onclick={openWindow}>Open new Window</button>
</div>
